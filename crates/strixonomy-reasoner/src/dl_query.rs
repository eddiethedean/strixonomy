//! Protégé-style DL Query over a workspace reasoner input.

use crate::error::{ReasonerError, Result};
use crate::input::ReasonerInput;
use crate::result::{ClassificationResult, RealizationResult};
use crate::{classify, realize, ReasonerId};
use horned_owl::model::ClassExpression;
use ontologos_bridge::{core_to_triples_all, merge_triples_into_ontology};
use ontologos_core::Ontology;
use ontologos_parser::load_ontology;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;
use strixonomy_catalog::ClassHierarchy;
use strixonomy_owl::{class_expression_to_turtle_value, parse_class_expression};

/// Temporary named class used to materialize anonymous Manchester expressions.
pub const DL_QUERY_CLASS_IRI: &str = "urn:strixonomy:dl-query#Q";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DlQueryMode {
    #[default]
    Inferred,
    Asserted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlQueryResult {
    pub expression: String,
    pub normalized: String,
    pub query_class_iri: String,
    pub subclasses: Vec<String>,
    pub superclasses: Vec<String>,
    pub equivalents: Vec<String>,
    pub instances: Vec<String>,
    pub profile: String,
    pub mode: DlQueryMode,
    pub duration_ms: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<String>,
}

/// Run a Manchester class-expression DL Query against `input`.
///
/// Named class expressions use the hierarchy (asserted or inferred) and realization
/// directly. Anonymous expressions are evaluated by injecting a temporary
/// `EquivalentClasses(Q, CE)` axiom into a cloned ontology (never written to disk).
pub fn run_dl_query(
    profile: ReasonerId,
    input: &ReasonerInput,
    expression: &str,
    namespaces: &BTreeMap<String, String>,
    mode: DlQueryMode,
) -> Result<DlQueryResult> {
    let started = Instant::now();
    let parsed = parse_class_expression(expression, namespaces)
        .map_err(|e| ReasonerError::Classify(format!("Manchester parse failed: {e}")))?;
    let diagnostics: Vec<String> = parsed.diagnostics.iter().map(|d| d.message.clone()).collect();

    let named_iri = named_class_iri(&parsed.expression);
    let is_named = named_iri.is_some();
    let (query_iri, query_input, mut warnings) = if let Some(iri) = named_iri {
        (iri, input.clone_shallow(), Vec::new())
    } else {
        let (augmented, w) = inject_temp_equivalent(input, &parsed.expression, namespaces)?;
        (DL_QUERY_CLASS_IRI.to_string(), augmented, w)
    };

    let (hierarchy, profile_used, realization, equiv_clusters) = match mode {
        DlQueryMode::Asserted => {
            (query_input.asserted_hierarchy.clone(), "asserted".to_string(), None, Vec::new())
        }
        DlQueryMode::Inferred => {
            let classification = classify(profile, &query_input, true)?;
            let realization = match realize(profile, &query_input) {
                Ok(result) => Some(result),
                Err(err) => {
                    // #371: do not treat realize failures as empty Instances.
                    warnings.push(format!(
                        "realization unavailable; Instances may be incomplete: {err}"
                    ));
                    None
                }
            };
            collect_inferred(&classification, realization)
        }
    };

    let mut subclasses = collect_descendants(&hierarchy, &query_iri);
    let mut superclasses = collect_ancestors(&hierarchy, &query_iri);
    let mut equivalents =
        collect_equivalents(&hierarchy, &query_input.ontology, &query_iri, &equiv_clusters);
    subclasses.retain(|iri| iri != &query_iri && iri != DL_QUERY_CLASS_IRI);
    superclasses.retain(|iri| {
        iri != &query_iri
            && iri != DL_QUERY_CLASS_IRI
            && iri != "http://www.w3.org/2002/07/owl#Thing"
    });
    equivalents.retain(|iri| iri != &query_iri && iri != DL_QUERY_CLASS_IRI);

    // #372: Protégé-style — equivalents only under Equivalents, not Sub/Super.
    let equiv_set: BTreeSet<_> = equivalents.iter().cloned().collect();
    subclasses.retain(|iri| !equiv_set.contains(iri));
    superclasses.retain(|iri| !equiv_set.contains(iri));

    let mut instances = Vec::new();
    if let Some(realization) = realization {
        for entry in realization.individuals {
            if entry.types.iter().any(|t| t == &query_iri) {
                instances.push(entry.individual_iri);
            }
        }
    } else if mode == DlQueryMode::Asserted {
        if is_named {
            instances = asserted_instances_of_class(&query_input, &hierarchy, &query_iri);
        } else {
            warnings.push(
                "asserted mode cannot materialize instances for anonymous class expressions; use inferred mode"
                    .to_string(),
            );
        }
    }

    instances.sort();
    instances.dedup();

    Ok(DlQueryResult {
        expression: expression.to_string(),
        normalized: parsed.normalized,
        query_class_iri: query_iri,
        subclasses,
        superclasses,
        equivalents,
        instances,
        profile: profile_used,
        mode,
        duration_ms: started.elapsed().as_millis() as u64,
        warnings,
        diagnostics,
    })
}

impl ReasonerInput {
    /// Clone ontology + hierarchy without re-scanning the workspace.
    fn clone_shallow(&self) -> Self {
        Self {
            workspace: self.workspace.clone(),
            content_hash: self.content_hash.clone(),
            ontology: self.ontology.clone(),
            asserted_hierarchy: self.asserted_hierarchy.clone(),
            document_overrides: self.document_overrides.clone(),
        }
    }
}

fn collect_inferred(
    classification: &ClassificationResult,
    realization: Option<RealizationResult>,
) -> (ClassHierarchy, String, Option<RealizationResult>, Vec<Vec<String>>) {
    (
        classification.inferred.combined.clone(),
        classification.profile_used.clone(),
        realization,
        classification.equivalences.clone(),
    )
}

/// Collect individuals with an asserted type of `class_iri`, an asserted equivalent,
/// or any asserted descendant of those classes (#373).
fn asserted_instances_of_class(
    input: &ReasonerInput,
    hierarchy: &ClassHierarchy,
    class_iri: &str,
) -> Vec<String> {
    let mut class_iris: BTreeSet<String> =
        collect_descendants(hierarchy, class_iri).into_iter().collect();
    class_iris.insert(class_iri.to_string());
    close_under_asserted_equivalents(&input.ontology, &mut class_iris);
    // Descendants of equivalents (and equivalents of descendants) matter too.
    let seeds: Vec<String> = class_iris.iter().cloned().collect();
    for seed in seeds {
        for child in collect_descendants(hierarchy, &seed) {
            class_iris.insert(child);
        }
    }
    close_under_asserted_equivalents(&input.ontology, &mut class_iris);

    let mut out = BTreeSet::new();
    for iri in &class_iris {
        let Some(class_id) = input.ontology.lookup_entity(iri) else {
            continue;
        };
        for ind_id in input.ontology.individuals_of(class_id) {
            if let Ok(ind_iri) = crate::result::entity_iri(&input.ontology, *ind_id) {
                out.insert(ind_iri);
            }
        }
    }

    // Fallback: walk ClassAssertion axioms (including load-marked "inferred" ones).
    if out.is_empty() {
        use ontologos_core::Axiom;
        for (_id, axiom) in input.ontology.axioms().iter() {
            let Axiom::ClassAssertion { individual, class } = axiom else {
                continue;
            };
            let Ok(class_s) = crate::result::entity_iri(&input.ontology, *class) else {
                continue;
            };
            if !class_iris.contains(&class_s) {
                continue;
            }
            if let Ok(ind_s) = crate::result::entity_iri(&input.ontology, *individual) {
                out.insert(ind_s);
            }
        }
    }

    out.into_iter().collect()
}

fn close_under_asserted_equivalents(ontology: &Ontology, class_iris: &mut BTreeSet<String>) {
    let seeds: Vec<String> = class_iris.iter().cloned().collect();
    for seed in seeds {
        let Some(id) = ontology.lookup_entity(&seed) else {
            continue;
        };
        let Some(equivs) = ontology.equivalents_of(id) else {
            continue;
        };
        for eid in equivs {
            if let Ok(iri) = crate::result::entity_iri(ontology, *eid) {
                class_iris.insert(iri);
            }
        }
    }
}

fn named_class_iri(expr: &ClassExpression<horned_owl::model::RcStr>) -> Option<String> {
    match expr {
        ClassExpression::Class(c) => Some(c.to_string()),
        _ => None,
    }
}

fn inject_temp_equivalent(
    input: &ReasonerInput,
    expr: &ClassExpression<horned_owl::model::RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> Result<(ReasonerInput, Vec<String>)> {
    let ce_turtle = class_expression_to_turtle_value(expr, namespaces, 0).map_err(|e| {
        ReasonerError::Classify(format!("failed to serialize class expression: {e}"))
    })?;
    let supplement = build_query_supplement(&ce_turtle, namespaces);
    let loaded = load_ontology_from_temp_text(&supplement)?;
    let mut ontology = input.ontology.clone();
    let triples =
        core_to_triples_all(&loaded).map_err(|e| ReasonerError::Ontology(e.to_string()))?;
    merge_triples_into_ontology(&mut ontology, &triples, &[])
        .map_err(|e| ReasonerError::Ontology(e.to_string()))?;
    let asserted_hierarchy = crate::hierarchy::asserted_hierarchy_from_ontology(&ontology);
    Ok((
        ReasonerInput {
            workspace: input.workspace.clone(),
            content_hash: format!("{}:dl-query", input.content_hash),
            ontology,
            asserted_hierarchy,
            document_overrides: input.document_overrides.clone(),
        },
        vec!["evaluated via temporary equivalent class (not written to disk)".to_string()],
    ))
}

fn build_query_supplement(ce_turtle: &str, namespaces: &BTreeMap<String, String>) -> String {
    let mut out = String::new();
    out.push_str("@prefix owl: <http://www.w3.org/2002/07/owl#> .\n");
    out.push_str("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n");
    out.push_str("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n");
    out.push_str("@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n");
    for (prefix, iri) in namespaces {
        if matches!(prefix.as_str(), "owl" | "rdf" | "rdfs" | "xsd") {
            continue;
        }
        out.push_str(&format!("@prefix {prefix}: <{iri}> .\n"));
    }
    out.push_str(&format!(
        "<{DL_QUERY_CLASS_IRI}> a owl:Class ;\n  owl:equivalentClass {ce_turtle} .\n"
    ));
    out
}

fn load_ontology_from_temp_text(text: &str) -> Result<ontologos_core::Ontology> {
    let tmp = tempfile::Builder::new()
        .suffix(".ttl")
        .tempfile()
        .map_err(|e| ReasonerError::Ontology(e.to_string()))?;
    std::fs::write(tmp.path(), text).map_err(|e| ReasonerError::Ontology(e.to_string()))?;
    load_ontology(tmp.path()).map_err(|e| ReasonerError::Ontology(e.to_string()))
}

fn collect_descendants(hierarchy: &ClassHierarchy, root: &str) -> Vec<String> {
    let mut out = BTreeSet::new();
    let mut stack = vec![root.to_string()];
    let mut seen = BTreeSet::new();
    while let Some(current) = stack.pop() {
        if !seen.insert(current.clone()) {
            continue;
        }
        if let Some(children) = hierarchy.children.get(&current) {
            for child in children {
                if child != root {
                    out.insert(child.clone());
                }
                stack.push(child.clone());
            }
        }
    }
    out.into_iter().collect()
}

fn collect_ancestors(hierarchy: &ClassHierarchy, root: &str) -> Vec<String> {
    let mut out = BTreeSet::new();
    let mut stack = vec![root.to_string()];
    let mut seen = BTreeSet::new();
    while let Some(current) = stack.pop() {
        if !seen.insert(current.clone()) {
            continue;
        }
        if let Some(parents) = hierarchy.parents.get(&current) {
            for parent in parents {
                if parent != root {
                    out.insert(parent.clone());
                }
                stack.push(parent.clone());
            }
        }
    }
    out.into_iter().collect()
}

/// Equivalents = mutual SubClassOf ∩ ∪ asserted `equivalents_of` ∪ taxonomy clusters (#370).
fn collect_equivalents(
    hierarchy: &ClassHierarchy,
    ontology: &Ontology,
    root: &str,
    inferred_clusters: &[Vec<String>],
) -> Vec<String> {
    let descendants: BTreeSet<_> = collect_descendants(hierarchy, root).into_iter().collect();
    let ancestors: BTreeSet<_> = collect_ancestors(hierarchy, root).into_iter().collect();
    let mut out: BTreeSet<String> = descendants.intersection(&ancestors).cloned().collect();

    if let Some(id) = ontology.lookup_entity(root) {
        if let Some(equivs) = ontology.equivalents_of(id) {
            for eid in equivs {
                if let Ok(iri) = crate::result::entity_iri(ontology, *eid) {
                    out.insert(iri);
                }
            }
        }
    }

    for cluster in inferred_clusters {
        if cluster.iter().any(|iri| iri == root) {
            for iri in cluster {
                if iri != root {
                    out.insert(iri.clone());
                }
            }
        }
    }

    out.into_iter().collect()
}
