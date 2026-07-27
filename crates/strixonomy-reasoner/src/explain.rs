use crate::error::{ReasonerError, Result};
use crate::hierarchy::asserted_hierarchy_from_ontology;
use crate::result::{ExplanationResult, ExplanationStep};
use ontologos_core::{
    EntityId, EntityKind, InferenceTrace, Ontology, TraceConclusion, TracePremise, TraceStep,
};
use ontologos_dl::DlClassifier;
use ontologos_el::ElClassifier;
use ontologos_explain::{build_proof_graph, render_text, ProofGraph};
use ontologos_rl::rdfs::RdfsEngine;
use ontologos_rl::RlEngine;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn explain_unsatisfiable_alternatives(
    profile: crate::adapter::ReasonerId,
    ontology: &Ontology,
    class_iri: &str,
    max_justifications: usize,
) -> Result<Vec<ExplanationResult>> {
    let max = max_justifications.clamp(1, 8);
    let mut results = match profile {
        crate::adapter::ReasonerId::El => {
            explain_unsatisfiable_el_alternatives(ontology, class_iri, max)?
        }
        crate::adapter::ReasonerId::Rl => vec![explain_unsatisfiable_rl(ontology, class_iri)?],
        crate::adapter::ReasonerId::Rdfs => vec![explain_unsatisfiable_rdfs(ontology, class_iri)?],
        crate::adapter::ReasonerId::Dl => vec![explain_unsatisfiable_dl(ontology, class_iri)?],
        crate::adapter::ReasonerId::Auto => {
            // Match CLI AutoAdapter::explain: use the concrete engine Auto classify selects.
            let concrete = crate::auto::resolve_auto_reasoner_id(ontology)
                .map_err(|e| ReasonerError::Explain(e.to_string()))?;
            return explain_unsatisfiable_alternatives(concrete, ontology, class_iri, max);
        }
    };

    // Basic de-duplication by rendered text.
    let mut seen = std::collections::HashSet::new();
    results.retain(|r| seen.insert(r.text.clone()));
    if results.is_empty() {
        return Err(ReasonerError::ExplanationUnavailable(class_iri.to_string()));
    }
    Ok(results)
}

pub fn explain_unsatisfiable_el(ontology: &Ontology, class_iri: &str) -> Result<ExplanationResult> {
    let class_id = ontology
        .lookup_entity(class_iri)
        .ok_or_else(|| ReasonerError::ClassNotFound(class_iri.to_string()))?;

    let report = ElClassifier::new()
        .classify_with_options(ontology, true)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?;

    if let Some(bottom) = find_bottom_subsumption(ontology, class_id, &report.trace) {
        let graph = explain_unsatisfiable_trace(ontology, class_id, bottom, &report.trace)
            .map_err(|e| ReasonerError::Explain(e.to_string()))?;
        return map_proof_graph(ontology, class_iri, graph);
    }

    explain_via_composed_ancestor(ontology, class_iri, &report.trace)
}

pub fn explain_unsatisfiable_el_alternatives(
    ontology: &Ontology,
    class_iri: &str,
    max: usize,
) -> Result<Vec<ExplanationResult>> {
    let class_id = ontology
        .lookup_entity(class_iri)
        .ok_or_else(|| ReasonerError::ClassNotFound(class_iri.to_string()))?;

    let report = ElClassifier::new()
        .classify_with_options(ontology, true)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?;

    let Some(bottom) = find_bottom_subsumption(ontology, class_id, &report.trace) else {
        // Expansion-only unsat: compose a single honest justification.
        return Ok(vec![explain_via_composed_ancestor(ontology, class_iri, &report.trace)?]);
    };

    let target_idxs: Vec<usize> = report
        .trace
        .steps
        .iter()
        .enumerate()
        .filter_map(|(idx, s)| {
            if conclusion_matches_subsumption(ontology, &s.conclusion, class_id, bottom) {
                Some(idx)
            } else {
                None
            }
        })
        .take(max)
        .collect();

    let mut out = Vec::new();
    for idx in target_idxs {
        let subgraph = InferenceTrace { steps: hst_prune(&report.trace, idx) };
        let graph = build_proof_graph(ontology, &subgraph)
            .map_err(|e| ReasonerError::Explain(e.to_string()))?;
        out.push(map_proof_graph(ontology, class_iri, graph)?);
    }
    if out.is_empty() {
        out.push(explain_unsatisfiable_el(ontology, class_iri)?);
    }
    Ok(out)
}

pub fn explain_unsatisfiable_rl(ontology: &Ontology, class_iri: &str) -> Result<ExplanationResult> {
    let class_id = ontology
        .lookup_entity(class_iri)
        .ok_or_else(|| ReasonerError::ClassNotFound(class_iri.to_string()))?;
    let mut mutable = ontology.clone();
    let trace = RlEngine::try_new(1)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?
        .with_traces(true)
        .saturate(&mut mutable)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?
        .trace;
    if let Some(bottom) = find_bottom_subsumption(&mutable, class_id, &trace) {
        let graph = explain_unsatisfiable_trace(&mutable, class_id, bottom, &trace)
            .map_err(|e| ReasonerError::Explain(e.to_string()))?;
        return map_proof_graph(ontology, class_iri, graph);
    }
    explain_via_composed_ancestor(&mutable, class_iri, &trace)
}

pub fn explain_unsatisfiable_rdfs(
    ontology: &Ontology,
    class_iri: &str,
) -> Result<ExplanationResult> {
    let class_id = ontology
        .lookup_entity(class_iri)
        .ok_or_else(|| ReasonerError::ClassNotFound(class_iri.to_string()))?;
    let mut mutable = ontology.clone();
    let trace = RdfsEngine::new()
        .with_traces(true)
        .materialize(&mut mutable)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?
        .trace;
    if let Some(bottom) = find_bottom_subsumption(&mutable, class_id, &trace) {
        let graph = explain_unsatisfiable_trace(&mutable, class_id, bottom, &trace)
            .map_err(|e| ReasonerError::Explain(e.to_string()))?;
        return map_proof_graph(ontology, class_iri, graph);
    }
    explain_via_composed_ancestor(&mutable, class_iri, &trace)
}

fn explain_unsatisfiable_trace(
    ontology: &Ontology,
    class: EntityId,
    bottom: EntityId,
    trace: &InferenceTrace,
) -> std::result::Result<ProofGraph, ontologos_explain::Error> {
    let subgraph = minimal_subsumption_trace(ontology, trace, class, bottom)?;
    build_proof_graph(ontology, &subgraph)
}

fn minimal_subsumption_trace(
    ontology: &Ontology,
    trace: &InferenceTrace,
    sub: EntityId,
    sup: EntityId,
) -> std::result::Result<InferenceTrace, ontologos_explain::Error> {
    let step_idx = trace
        .steps
        .iter()
        .position(|s| conclusion_matches_subsumption(ontology, &s.conclusion, sub, sup))
        .ok_or(ontologos_explain::Error::Core(ontologos_core::Error::Message(format!(
            "no inference step concludes {sub:?} ⊑ {sup:?}"
        ))))?;

    Ok(InferenceTrace { steps: hst_prune(trace, step_idx) })
}

fn hst_prune(trace: &InferenceTrace, target_idx: usize) -> Vec<TraceStep> {
    let mut needed = HashSet::new();
    let mut queue = VecDeque::from([target_idx]);

    while let Some(idx) = queue.pop_front() {
        if !needed.insert(idx) {
            continue;
        }
        let step = &trace.steps[idx];
        if step.premises.is_empty() {
            continue;
        }
        if let Some(premise_idx) = find_premise_step(trace, &step.premises[0]) {
            queue.push_back(premise_idx);
        }
        for premise in step.premises.iter().skip(1) {
            if let Some(premise_idx) = find_premise_step(trace, premise) {
                queue.push_back(premise_idx);
            }
        }
    }

    let mut ordered = needed.into_iter().collect::<Vec<_>>();
    ordered.sort_unstable();
    ordered.into_iter().map(|idx| trace.steps[idx].clone()).collect()
}

fn find_premise_step(trace: &InferenceTrace, premise: &TracePremise) -> Option<usize> {
    trace.steps.iter().position(|step| match (&step.conclusion, premise) {
        (
            TraceConclusion::SubClassOf { sub, sup },
            TracePremise::SubClassOf { sub: psub, sup: psup },
        ) => sub == psub && sup == psup,
        (TraceConclusion::Axiom { id }, TracePremise::Axiom { id: pid }) => id == pid,
        _ => false,
    })
}

fn conclusion_matches_subsumption(
    ontology: &Ontology,
    conclusion: &TraceConclusion,
    sub: EntityId,
    sup: EntityId,
) -> bool {
    match conclusion {
        TraceConclusion::SubClassOf { sub: s, sup: p } => *s == sub && *p == sup,
        TraceConclusion::Axiom { id } => ontology.axiom(*id).ok().is_some_and(|axiom| {
            matches!(
                axiom,
                ontologos_core::Axiom::SubClassOf { subclass, superclass }
                    if *subclass == sub && *superclass == sup
            )
        }),
        _ => false,
    }
}

fn find_bottom_subsumption(
    ontology: &Ontology,
    class: EntityId,
    trace: &InferenceTrace,
) -> Option<EntityId> {
    trace.steps.iter().find_map(|step| {
        let TraceConclusion::SubClassOf { sub, sup } = step.conclusion else {
            return None;
        };
        if sub != class {
            return None;
        }
        let record = ontology.entity(sup).ok()?;
        if record.kind != EntityKind::Class {
            return None;
        }
        let iri = ontology.resolve_iri(record.iri).ok()?;
        if is_owl_nothing(iri) {
            Some(sup)
        } else {
            None
        }
    })
}

/// When Ontologos has no direct `C ⊑ ⊥` trace (common for expansion-only unsats),
/// walk asserted parents to the nearest ancestor `U` that has a bottom proof, or to
/// `owl:Nothing`, and compose an honest subclass-chain justification.
fn explain_via_composed_ancestor(
    ontology: &Ontology,
    class_iri: &str,
    trace: &InferenceTrace,
) -> Result<ExplanationResult> {
    match find_composed_ancestor(ontology, class_iri, trace) {
        Some(ComposeTarget::ViaAncestor { chain, ancestor_iri, ancestor_id, bottom }) => {
            let graph = explain_unsatisfiable_trace(ontology, ancestor_id, bottom, trace)
                .map_err(|e| ReasonerError::Explain(e.to_string()))?;
            let ancestor_result = map_proof_graph(ontology, &ancestor_iri, graph)?;
            Ok(compose_subclass_chain_explanation(class_iri, chain, Some(ancestor_result)))
        }
        Some(ComposeTarget::ToNothing { chain }) => {
            Ok(compose_subclass_chain_explanation(class_iri, chain, None))
        }
        None => Err(ReasonerError::ExplanationUnavailable(class_iri.to_string())),
    }
}

enum ComposeTarget {
    /// Named ancestor with an Ontologos `U ⊑ ⊥` trace.
    ViaAncestor {
        chain: Vec<(String, String)>,
        ancestor_iri: String,
        ancestor_id: EntityId,
        bottom: EntityId,
    },
    /// Asserted path reaches `owl:Nothing` (no Ontologos justification available).
    ToNothing { chain: Vec<(String, String)> },
}

/// BFS parents from `class_iri`; prefer a named ancestor with a bottom trace, else `owl:Nothing`.
fn find_composed_ancestor(
    ontology: &Ontology,
    class_iri: &str,
    trace: &InferenceTrace,
) -> Option<ComposeTarget> {
    let hierarchy = asserted_hierarchy_from_ontology(ontology);
    let mut queue = VecDeque::from([class_iri.to_string()]);
    let mut visited = HashSet::from([class_iri.to_string()]);
    // parent → child we came from (first reach)
    let mut came_from: HashMap<String, String> = HashMap::new();
    let mut nothing_path: Option<Vec<(String, String)>> = None;

    while let Some(current) = queue.pop_front() {
        let Some(parents) = hierarchy.parents.get(&current) else {
            continue;
        };
        for parent in parents {
            if !visited.insert(parent.clone()) {
                continue;
            }
            came_from.insert(parent.clone(), current.clone());

            if is_owl_nothing(parent) {
                if nothing_path.is_none() {
                    nothing_path = reconstruct_subclass_chain(&came_from, class_iri, parent);
                }
                continue;
            }

            if let Some(parent_id) = ontology.lookup_entity(parent) {
                if let Some(bottom) = find_bottom_subsumption(ontology, parent_id, trace) {
                    let chain = reconstruct_subclass_chain(&came_from, class_iri, parent)?;
                    return Some(ComposeTarget::ViaAncestor {
                        chain,
                        ancestor_iri: parent.clone(),
                        ancestor_id: parent_id,
                        bottom,
                    });
                }
            }
            queue.push_back(parent.clone());
        }
    }

    nothing_path.map(|chain| ComposeTarget::ToNothing { chain })
}

fn reconstruct_subclass_chain(
    came_from: &HashMap<String, String>,
    start: &str,
    goal: &str,
) -> Option<Vec<(String, String)>> {
    let mut edges = Vec::new();
    let mut cur = goal.to_string();
    while cur != start {
        let prev = came_from.get(&cur)?.clone();
        edges.push((prev.clone(), cur.clone()));
        cur = prev;
    }
    edges.reverse();
    Some(edges)
}

fn compose_subclass_chain_explanation(
    class_iri: &str,
    chain: Vec<(String, String)>,
    ancestor: Option<ExplanationResult>,
) -> ExplanationResult {
    let chain_to_nothing = ancestor.is_none() && ancestor_is_nothing_chain(&chain);
    let mut steps =
        Vec::with_capacity(chain.len() + ancestor.as_ref().map_or(0, |a| a.steps.len()));
    let mut text_lines = Vec::new();

    for (child, parent) in &chain {
        let index = steps.len() + 1;
        steps.push(ExplanationStep {
            index,
            rule: "composed_subclass_chain".into(),
            display: format!("{child} SubClassOf {parent}"),
            subject_iri: Some(child.clone()),
            object_iri: Some(parent.clone()),
        });
        text_lines.push(format!("{child} SubClassOf {parent}"));
    }

    if let Some(ancestor) = ancestor {
        let offset = steps.len();
        for mut step in ancestor.steps {
            step.index += offset;
            steps.push(step);
        }
        if !ancestor.text.is_empty() {
            text_lines.push(ancestor.text);
        }
    }

    let header = if chain_to_nothing {
        "Composed justification (asserted subclass chain to owl:Nothing)"
    } else {
        "Composed justification (subclass chain to unsatisfiable ancestor)"
    };

    ExplanationResult {
        class_iri: class_iri.to_string(),
        steps,
        text: format!("{header}\n\n{}", text_lines.join("\n")),
    }
}

fn ancestor_is_nothing_chain(chain: &[(String, String)]) -> bool {
    chain.last().is_some_and(|(_, parent)| is_owl_nothing(parent))
}

fn map_proof_graph(
    ontology: &Ontology,
    class_iri: &str,
    graph: ProofGraph,
) -> Result<ExplanationResult> {
    if graph.nodes.is_empty() {
        return Err(ReasonerError::ExplanationUnavailable(class_iri.to_string()));
    }

    let text = render_text(ontology, &graph);
    let mut steps = Vec::new();
    for (index, node) in graph.nodes.iter().enumerate() {
        let (subject_iri, object_iri, display) = format_node(ontology, node);
        steps.push(ExplanationStep {
            index: index + 1,
            rule: node.rule.clone(),
            display,
            subject_iri,
            object_iri,
        });
    }

    Ok(ExplanationResult { class_iri: class_iri.to_string(), steps, text })
}

fn format_node(
    ontology: &Ontology,
    node: &ontologos_explain::ProofNode,
) -> (Option<String>, Option<String>, String) {
    if let Some((sub, sup)) = node.conclusion_sub {
        let sub_iri = entity_iri_opt(ontology, sub);
        let sup_iri = entity_iri_opt(ontology, sup);
        let display = match (&sub_iri, &sup_iri) {
            (Some(a), Some(b)) => format!("{a} SubClassOf {b}"),
            _ => node.rule.clone(),
        };
        return (sub_iri, sup_iri, display);
    }
    if let Some(id) = node.conclusion_axiom {
        if let Ok(axiom) = ontology.axiom(id) {
            return (None, None, format!("{axiom:?} ({})", node.rule));
        }
    }
    (None, None, node.rule.clone())
}

fn entity_iri_opt(ontology: &Ontology, id: EntityId) -> Option<String> {
    let entity = ontology.entity(id).ok()?;
    ontology.resolve_iri(entity.iri).ok().map(|s| s.to_string())
}

/// True for the OWL bottom class (`owl:Nothing`), not IRIs that merely contain "Nothing".
fn is_owl_nothing(iri: &str) -> bool {
    iri == "http://www.w3.org/2002/07/owl#Nothing" || iri == "owl:Nothing"
}

pub fn explain_unsatisfiable_dl(ontology: &Ontology, class_iri: &str) -> Result<ExplanationResult> {
    let taxonomy = DlClassifier::new()
        .classify(ontology)
        .map_err(|e| ReasonerError::Explain(e.to_string()))?;
    let class_id = ontology
        .lookup_entity(class_iri)
        .ok_or_else(|| ReasonerError::ClassNotFound(class_iri.to_string()))?;
    if !taxonomy.unsatisfiable.contains(&class_id) {
        return Err(ReasonerError::ExplanationUnavailable(class_iri.to_string()));
    }

    // DL-first: use native only when an asserted ⊥ path exists; else annotated fallback.
    if let Some(native) = explain_unsatisfiable_dl_native(ontology, class_iri, class_id) {
        return Ok(native);
    }

    // Secondary: weaker-engine traces, clearly labeled as non-primary.
    let (result, via) = if let Ok(r) = explain_unsatisfiable_el(ontology, class_iri) {
        (r, "EL")
    } else if let Ok(r) = explain_unsatisfiable_rl(ontology, class_iri) {
        (r, "RL")
    } else if let Ok(r) = explain_unsatisfiable_rdfs(ontology, class_iri) {
        (r, "RDFS")
    } else {
        return Err(ReasonerError::ExplanationUnavailable(format!(
            "{class_iri}: DL reports unsatisfiable but no native or EL/RL/RDFS justification is available"
        )));
    };
    Ok(annotate_dl_fallback_explanation(result, via))
}

/// Native DL explanation: only when an asserted ⊥ path is found.
/// Returns `None` so callers can fall back to EL/RL/RDFS justifications.
fn explain_unsatisfiable_dl_native(
    ontology: &Ontology,
    class_iri: &str,
    class_id: EntityId,
) -> Option<ExplanationResult> {
    let nothing = "http://www.w3.org/2002/07/owl#Nothing";
    let mut steps = Vec::new();
    let mut index = 0usize;
    let mut reached_bottom = false;

    // Walk asserted superclass chain toward owl:Nothing.
    let mut frontier = vec![class_id];
    let mut seen = HashSet::new();
    while let Some(cur) = frontier.pop() {
        if !seen.insert(cur) {
            continue;
        }
        let Some(cur_iri) = entity_iri_opt(ontology, cur) else {
            continue;
        };
        for &sup in ontology.direct_superclasses(cur) {
            let Some(sup_iri) = entity_iri_opt(ontology, sup) else {
                continue;
            };
            steps.push(ExplanationStep {
                index,
                rule: "dl_asserted_subclass".into(),
                display: format!("{cur_iri} SubClassOf {sup_iri}"),
                subject_iri: Some(cur_iri.clone()),
                object_iri: Some(sup_iri.clone()),
            });
            index += 1;
            if is_owl_nothing(&sup_iri) || ontology.lookup_entity(nothing) == Some(sup) {
                steps.push(ExplanationStep {
                    index,
                    rule: "dl_bottom".into(),
                    display: format!("{cur_iri} is unsatisfiable (⊑ owl:Nothing)"),
                    subject_iri: Some(cur_iri.clone()),
                    object_iri: Some(nothing.into()),
                });
                reached_bottom = true;
                break;
            }
            frontier.push(sup);
        }
        if reached_bottom {
            break;
        }
    }

    if !reached_bottom {
        return None;
    }

    let text = format!(
        "DL native explanation for {class_iri}\n{}",
        steps
            .iter()
            .map(|s| format!("{}. {}", s.index + 1, s.display))
            .collect::<Vec<_>>()
            .join("\n")
    );

    Some(ExplanationResult { class_iri: class_iri.to_string(), steps, text })
}

fn annotate_dl_fallback_explanation(mut result: ExplanationResult, via: &str) -> ExplanationResult {
    result.text = format!(
        "DL classification (unsatisfiable); justification via {via} Ontologos traces\n\n{}",
        result.text
    );
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::WorkspaceInputLoader;
    use std::path::PathBuf;

    #[test]
    fn dl_fallback_annotation_names_actual_profile() {
        let result = annotate_dl_fallback_explanation(
            ExplanationResult {
                class_iri: "http://ex#A".into(),
                steps: vec![],
                text: "A SubClassOf owl:Nothing".into(),
            },
            "EL",
        );
        assert!(result.text.starts_with(
            "DL classification (unsatisfiable); justification via EL Ontologos traces"
        ));
        assert!(!result.text.contains("DL profile justification"));
    }

    fn unsat_ontology() -> Ontology {
        let dir = tempfile::tempdir().expect("tempdir");
        let src =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/reasoner-unsat.ttl");
        std::fs::copy(&src, dir.path().join("reasoner-unsat.ttl")).expect("copy");
        WorkspaceInputLoader::new(dir.path()).load().expect("load").ontology
    }

    #[test]
    fn explain_el_chain_to_nothing_for_b() {
        let ontology = unsat_ontology();
        let b = "http://example.org/reasoner-unsat#B";
        let nothing = "http://www.w3.org/2002/07/owl#Nothing";
        let result = explain_unsatisfiable_el(&ontology, b).expect("explain B");
        assert_eq!(result.class_iri, b);
        assert!(
            result.steps.iter().any(|s| s.rule == "composed_subclass_chain"
                && s.subject_iri.as_deref() == Some(b)
                && s.object_iri.as_deref() == Some(nothing)),
            "expected composed B ⊑ owl:Nothing (Ontologos traces empty for this fixture): {:?}",
            result.steps
        );
    }

    #[test]
    fn explain_el_composes_chain_for_invalid() {
        let ontology = unsat_ontology();
        let invalid = "http://example.org/reasoner-unsat#Invalid";
        let b = "http://example.org/reasoner-unsat#B";
        let nothing = "http://www.w3.org/2002/07/owl#Nothing";
        let result = explain_unsatisfiable_el(&ontology, invalid).expect("explain Invalid");
        assert_eq!(result.class_iri, invalid);
        assert!(
            result.steps.iter().any(|s| s.rule == "composed_subclass_chain"
                && s.subject_iri.as_deref() == Some(invalid)
                && s.object_iri.as_deref() == Some(b)),
            "expected composed Invalid ⊑ B step, got {:?}",
            result.steps
        );
        assert!(
            result.steps.iter().any(|s| s.object_iri.as_deref() == Some(nothing)),
            "expected chain to reach owl:Nothing, got {:?}",
            result.steps
        );
        assert!(
            result.steps.len() >= 2,
            "composed explanation must include chain through B to ⊥: {:?}",
            result.steps
        );
        assert!(result.text.contains("Composed justification"));
    }

    #[test]
    fn dl_native_only_when_asserted_bottom_path_exists() {
        let ontology = unsat_ontology();
        let nothing = "http://www.w3.org/2002/07/owl#Nothing";
        let b = "http://example.org/reasoner-unsat#B";
        let b_id = ontology.lookup_entity(b).expect("B");
        // Fixture asserts B ⊑ owl:Nothing directly → native path.
        let native = explain_unsatisfiable_dl_native(&ontology, b, b_id);
        assert!(
            native.is_some(),
            "B has an asserted path to owl:Nothing and should explain natively"
        );
        let native = native.expect("native");
        assert!(native.steps.iter().any(|s| s.rule == "dl_bottom"));
        assert!(native.steps.iter().any(|s| s.object_iri.as_deref() == Some(nothing)));

        // Class with no asserted ⊑ Nothing chain → native declines (caller may fall back).
        let mut ont = Ontology::new();
        let orphan = "http://example.org/orphan#C";
        let cid = ont.entity_id(orphan, ontologos_core::EntityKind::Class).expect("declare");
        assert!(explain_unsatisfiable_dl_native(&ont, orphan, cid).is_none());
    }
}
