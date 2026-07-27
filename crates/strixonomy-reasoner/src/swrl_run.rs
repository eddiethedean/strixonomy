//! Optional SWRL-aware classification: materialize DLSafe rules then classify.

use crate::error::{ReasonerError, Result};
use crate::input::ReasonerInput;
use crate::result::{
    build_inferred_hierarchy, new_inferences, taxonomy_equivalence_clusters, taxonomy_to_iri_edges,
    unsatisfiable_iris, ClassificationResult, ReasonerWarning,
};
use ontologos_core::{EntityKind, Ontology, SwrlAtom, SwrlDArg, SwrlIArg, SwrlRule};
use std::time::Instant;

const ONTOCORE_SWRL_PRED: &str = "http://ontocode.dev/ns#swrlRule";

/// Classify with SWRL materialization when the ontology contains SWRL rules.
pub fn classify_with_swrl(input: &ReasonerInput) -> Result<ClassificationResult> {
    let started = Instant::now();
    let (ontology, mut warnings) = prepare_swrl_ontology(input)?;
    let (taxonomy, _) = ontologos_swrl::classify_with_swrl(&ontology)
        .map_err(|e| ReasonerError::Classify(format!("SWRL classify: {e}")))?;

    let iri_edges = taxonomy_to_iri_edges(&ontology, &taxonomy).map_err(ReasonerError::Classify)?;
    let equivalences =
        taxonomy_equivalence_clusters(&ontology, &taxonomy).map_err(ReasonerError::Classify)?;
    let reported = unsatisfiable_iris(&ontology, &taxonomy).map_err(ReasonerError::Classify)?;
    let inferred = build_inferred_hierarchy(&iri_edges, &reported, &input.asserted_hierarchy);
    let unsatisfiable = inferred.unsatisfiable.clone();
    let new_inferences = new_inferences(&input.asserted_hierarchy, &inferred.edges);

    // Consistency must run on the *materialized* ontology (#358), not the pre-SWRL input.
    let mut consistent = unsatisfiable.is_empty();
    if let Ok(detail) = crate::abox::check_full_consistency(
        &ontology,
        crate::adapter::ReasonerId::Dl,
        &unsatisfiable,
    ) {
        consistent = detail.consistent;
        for clash in &detail.abox_clashes {
            warnings.push(ReasonerWarning {
                code: "abox_clash".into(),
                message: clash.clone(),
                suggested_profile: None,
            });
        }
        if !detail.complete {
            warnings.push(ReasonerWarning {
                code: "consistency_incomplete".into(),
                message: "consistency check did not complete (budget or cancel)".into(),
                suggested_profile: None,
            });
        }
    }

    Ok(ClassificationResult {
        profile_used: "swrl".to_string(),
        consistent,
        unsatisfiable,
        inferred,
        new_inferences,
        warnings,
        duration_ms: started.elapsed().as_millis() as u64,
        subsumption_count: taxonomy.subsumption_count(),
        inferred_axiom_count: taxonomy.subsumption_count(),
        equivalences,
    })
}

/// Clone input ontology, inject authored rules, and materialize SWRL inferences.
///
/// Returns the post-materialization ontology plus inject/materialize warnings.
pub fn prepare_swrl_ontology(input: &ReasonerInput) -> Result<(Ontology, Vec<ReasonerWarning>)> {
    let mut ontology = input.ontology.clone();
    let mut warnings = Vec::new();
    // Prefer rules already injected at WorkspaceInputLoader time; re-inject from live
    // overrides only when the store is still empty (e.g. load path missed Turtle).
    if ontology.swrl_rules().is_empty() {
        let (authored, inject_warnings) = inject_authored_swrl_from_input(&mut ontology, input);
        warnings.extend(inject_warnings);
        if authored > 0 {
            warnings.push(ReasonerWarning {
                code: "swrl_authored_injected".into(),
                message: format!(
                    "injected {authored} Strixonomy-authored SWRL rule(s) from strixonomy:swrlRule annotations"
                ),
                suggested_profile: None,
            });
        }
    }
    let report = ontologos_swrl::materialize_swrl_rules(&mut ontology)
        .map_err(|e| ReasonerError::Classify(format!("SWRL materialization: {e}")))?;
    if report.inferences_added > 0 || report.rules_found > 0 {
        warnings.push(ReasonerWarning {
            code: "swrl_materialized".into(),
            message: format!(
                "SWRL: {} rule(s), {} inferred axiom(s) materialized",
                report.rules_found, report.inferences_added
            ),
            suggested_profile: None,
        });
    }
    Ok((ontology, warnings))
}

/// True when Ontologos already has SWRL rules on the ontology.
pub fn ontology_has_swrl_rules(ontology: &Ontology) -> bool {
    !ontology.swrl_rules().is_empty()
}

/// True when classify should take the SWRL path.
pub fn input_has_swrl_rules(input: &ReasonerInput) -> bool {
    ontology_has_swrl_rules(&input.ontology) || input_mentions_authored_swrl(input)
}

fn input_mentions_authored_swrl(input: &ReasonerInput) -> bool {
    input.document_overrides.values().any(|t| t.contains(ONTOCORE_SWRL_PRED))
}

/// Inject authored SWRL from live document overrides into the Ontologos store.
pub fn inject_authored_swrl_from_input(
    ontology: &mut Ontology,
    input: &ReasonerInput,
) -> (usize, Vec<ReasonerWarning>) {
    let mut injected = 0usize;
    let mut warnings = Vec::new();
    for text in input.document_overrides.values() {
        let (n, w) = inject_swrl_from_turtle(ontology, text);
        injected += n;
        warnings.extend(w);
    }
    (injected, warnings)
}

/// Parse `strixonomy:swrlRule` JSON literals from Turtle and push convertible rules.
///
/// Returns `(injected_count, warnings)`. Rules with BuiltIn/DataRange atoms are skipped
/// with an explicit warning (#357) instead of silently disappearing.
pub fn inject_swrl_from_turtle(
    ontology: &mut Ontology,
    text: &str,
) -> (usize, Vec<ReasonerWarning>) {
    let mut injected = 0usize;
    let mut warnings = Vec::new();
    for rule in strixonomy_swrl::rules_from_turtle_document(text) {
        if !rule.enabled {
            continue;
        }
        let rule_id = rule.id.as_deref().unwrap_or("<anonymous>");
        match convert_authored_rule(ontology, &rule) {
            Ok(converted) => {
                if ontology.push_swrl_rule(converted).is_ok() {
                    injected += 1;
                }
            }
            Err(e) => {
                warnings.push(ReasonerWarning {
                    code: "swrl_rule_skipped".into(),
                    message: format!("enabled SWRL rule {rule_id} was not injected: {e}"),
                    suggested_profile: None,
                });
            }
        }
    }
    (injected, warnings)
}

fn convert_authored_rule(
    ontology: &mut Ontology,
    rule: &strixonomy_swrl::SwrlRule,
) -> Result<SwrlRule> {
    let mut body = Vec::new();
    for atom in &rule.body {
        body.push(convert_atom(ontology, atom)?);
    }
    let mut head = Vec::new();
    for atom in &rule.head {
        head.push(convert_atom(ontology, atom)?);
    }
    Ok(SwrlRule { body, head })
}

fn convert_atom(ontology: &mut Ontology, atom: &strixonomy_swrl::SwrlAtom) -> Result<SwrlAtom> {
    use strixonomy_swrl::SwrlAtom as A;
    match atom {
        A::Class { class, arg } => Ok(SwrlAtom::Class {
            class: ontology
                .entity_id(class, EntityKind::Class)
                .map_err(|e| ReasonerError::Classify(e.to_string()))?,
            arg: convert_iarg(ontology, arg)?,
        }),
        A::ObjectProperty { property, subject, object } => Ok(SwrlAtom::ObjectProperty {
            property: ontology
                .entity_id(property, EntityKind::ObjectProperty)
                .map_err(|e| ReasonerError::Classify(e.to_string()))?,
            subject: convert_iarg(ontology, subject)?,
            object: convert_iarg(ontology, object)?,
        }),
        A::DataProperty { property, subject, value } => Ok(SwrlAtom::DataProperty {
            property: ontology
                .entity_id(property, EntityKind::DataProperty)
                .map_err(|e| ReasonerError::Classify(e.to_string()))?,
            subject: convert_iarg(ontology, subject)?,
            value: convert_darg(ontology, value)?,
        }),
        A::SameIndividual { left, right } => Ok(SwrlAtom::SameIndividual(
            convert_iarg(ontology, left)?,
            convert_iarg(ontology, right)?,
        )),
        A::DifferentIndividuals { left, right } => Ok(SwrlAtom::DifferentIndividuals(
            convert_iarg(ontology, left)?,
            convert_iarg(ontology, right)?,
        )),
        A::BuiltIn { .. } => Err(ReasonerError::Classify(
            "BuiltIn SWRL atoms are not executable via Ontologos materialization".into(),
        )),
        A::DataRange { .. } => Err(ReasonerError::Classify(
            "DataRange SWRL atoms are not mapped for Ontologos injection".into(),
        )),
    }
}

fn convert_iarg(ontology: &mut Ontology, arg: &strixonomy_swrl::SwrlIArg) -> Result<SwrlIArg> {
    match arg {
        strixonomy_swrl::SwrlIArg::Variable(v) => Ok(SwrlIArg::Variable(v.clone())),
        strixonomy_swrl::SwrlIArg::Individual(iri) => Ok(SwrlIArg::Individual(
            ontology
                .entity_id(iri, EntityKind::Individual)
                .map_err(|e| ReasonerError::Classify(e.to_string()))?,
        )),
    }
}

fn convert_darg(ontology: &mut Ontology, arg: &strixonomy_swrl::SwrlDArg) -> Result<SwrlDArg> {
    match arg {
        strixonomy_swrl::SwrlDArg::Variable(v) => Ok(SwrlDArg::Variable(v.clone())),
        strixonomy_swrl::SwrlDArg::Literal { lexical, datatype } => {
            let datatype = match datatype {
                Some(iri) => Some(
                    ontology
                        .entity_id(iri, EntityKind::Datatype)
                        .map_err(|e| ReasonerError::Classify(e.to_string()))?,
                ),
                None => None,
            };
            Ok(SwrlDArg::Literal { lexical: lexical.clone(), datatype })
        }
    }
}
