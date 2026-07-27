//! ABox reasoning: consistency, realization, instance checking, inferred assertions.

use crate::adapter::ReasonerId;
use crate::error::{ReasonerError, Result};
use crate::result::{
    entity_iri, ConsistencyDetail, InferredAssertions, InstanceCheckResult, RealizationEntry,
    RealizationResult, SameAsCluster,
};
use ontologos_core::{EntityId, EntityKind, Ontology, Profile, Reasoner, ReasonerConfig};
use ontologos_facade::{
    check_consistency as facade_check_consistency, is_entailed_axiom, EntailmentCheck,
};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Cap named individuals processed in a single realize pass.
const MAX_REALIZE_INDIVIDUALS: usize = 500;
/// Cap total class-assertion entailment checks across a realize pass.
const MAX_REALIZE_ENTAILMENT_CHECKS: usize = 50_000;

/// Thread-local cancel flag installed for reasoner runs (engine-level cancel).
static ENGINE_CANCEL: std::sync::Mutex<Option<Arc<AtomicBool>>> = std::sync::Mutex::new(None);

/// Install a cancel flag for subsequent Ontologos operations on this thread.
pub fn install_cancel_flag(flag: Arc<AtomicBool>) {
    ontologos_core::set_current_cancel(Some(Arc::clone(&flag)));
    if let Ok(mut guard) = ENGINE_CANCEL.lock() {
        *guard = Some(flag);
    }
}

/// Clear the cancel flag after a reasoner run completes.
pub fn clear_cancel_flag() {
    ontologos_core::set_current_cancel(None);
    if let Ok(mut guard) = ENGINE_CANCEL.lock() {
        *guard = None;
    }
}

/// Returns true when an in-flight reasoner run should abort.
pub fn cancel_requested() -> bool {
    if ontologos_core::cancel_requested() {
        return true;
    }
    ENGINE_CANCEL
        .lock()
        .ok()
        .and_then(|g| g.as_ref().map(|f| f.load(Ordering::Acquire)))
        .unwrap_or(false)
}

fn profile_for(id: ReasonerId) -> Profile {
    match id {
        ReasonerId::El => Profile::El,
        ReasonerId::Rl => Profile::Rl,
        ReasonerId::Rdfs => Profile::Rdfs,
        ReasonerId::Dl => Profile::Dl,
        ReasonerId::Auto => Profile::Auto,
    }
}

fn build_reasoner(ontology: &Ontology, id: ReasonerId) -> Result<Reasoner> {
    let config = ReasonerConfig { explanations: true, ..ReasonerConfig::default() };
    Reasoner::builder()
        .profile(profile_for(id))
        .config(config)
        .build(ontology.clone())
        .map_err(|e| ReasonerError::Classify(e.to_string()))
}

/// Full consistency: TBox unsatisfiable classes + ABox / ontology consistency via Ontologos.
pub fn check_full_consistency(
    ontology: &Ontology,
    id: ReasonerId,
    unsatisfiable: &[String],
) -> Result<ConsistencyDetail> {
    if cancel_requested() {
        return Err(ReasonerError::Cancelled);
    }
    let mut reasoner = build_reasoner(ontology, id)?;
    let facade = facade_check_consistency(&mut reasoner)
        .map_err(|e| ReasonerError::Classify(e.to_string()))?;

    let mut abox_clashes = Vec::new();
    if matches!(id, ReasonerId::Rl | ReasonerId::Rdfs | ReasonerId::Auto | ReasonerId::El) {
        let (clashes, _) = ontologos_rl::abox::collect_same_as_clashes(ontology);
        abox_clashes.extend(clashes);
        if let Ok(ok) = ontologos_rl::abox::is_abox_consistent(ontology) {
            if !ok && abox_clashes.is_empty() {
                abox_clashes.push("ABox is inconsistent (sameAs/differentFrom clash)".into());
            }
        }
    }

    let ontology_consistent = facade.complete && facade.consistent && abox_clashes.is_empty();
    let consistent = ontology_consistent && unsatisfiable.is_empty();

    Ok(ConsistencyDetail {
        consistent,
        complete: facade.complete,
        ontology_consistent,
        abox_clashes,
        unsatisfiable: unsatisfiable.to_vec(),
    })
}

/// Check whether `individual` is an instance of `class` under the given profile.
pub fn check_instance(
    ontology: &Ontology,
    id: ReasonerId,
    individual_iri: &str,
    class_iri: &str,
) -> Result<InstanceCheckResult> {
    if cancel_requested() {
        return Err(ReasonerError::Cancelled);
    }
    let started = Instant::now();
    let mut reasoner = build_reasoner(ontology, id)?;
    let entailed = is_entailed_axiom(
        &mut reasoner,
        EntailmentCheck::ClassAssertion {
            individual: individual_iri.to_string(),
            class: class_iri.to_string(),
        },
    )
    .map_err(map_entailment_error)?;

    Ok(InstanceCheckResult {
        individual_iri: individual_iri.to_string(),
        class_iri: class_iri.to_string(),
        entailed,
        profile_used: id.as_str().to_string(),
        duration_ms: started.elapsed().as_millis() as u64,
    })
}

fn map_entailment_error(e: impl std::fmt::Display) -> ReasonerError {
    let msg = e.to_string();
    if cancel_requested() || msg.to_ascii_lowercase().contains("cancel") {
        ReasonerError::Cancelled
    } else {
        ReasonerError::Classify(msg)
    }
}

fn named_individuals(ontology: &Ontology) -> Result<Vec<(EntityId, String)>> {
    let mut out = Vec::new();
    for (id, record) in ontology.entities().iter() {
        if record.kind != EntityKind::Individual {
            continue;
        }
        let iri = entity_iri(ontology, id).map_err(ReasonerError::Ontology)?;
        if iri.starts_with("http://www.w3.org/2002/07/owl#")
            || iri.starts_with("http://www.w3.org/1999/02/22-rdf-syntax-ns#")
        {
            continue;
        }
        out.push((id, iri));
    }
    out.sort_by(|a, b| a.1.cmp(&b.1));
    Ok(out)
}

fn named_classes(ontology: &Ontology) -> Result<Vec<(EntityId, String)>> {
    let mut out = Vec::new();
    for (id, record) in ontology.entities().iter() {
        if record.kind != EntityKind::Class && record.kind != EntityKind::ClassIndividual {
            continue;
        }
        let iri = entity_iri(ontology, id).map_err(ReasonerError::Ontology)?;
        if iri == "http://www.w3.org/2002/07/owl#Nothing"
            || iri == "http://www.w3.org/2002/07/owl#Thing"
        {
            continue;
        }
        out.push((id, iri));
    }
    out.sort_by(|a, b| a.1.cmp(&b.1));
    Ok(out)
}

fn most_specific_types(
    ontology: &Ontology,
    hierarchy_parents: &BTreeMap<String, Vec<String>>,
    types: &[String],
) -> Vec<String> {
    let type_set: BTreeSet<&str> = types.iter().map(|s| s.as_str()).collect();
    types
        .iter()
        .filter(|t| {
            // Keep type T if no other entailed type is a strict subclass of T.
            !types.iter().any(|other| {
                if other == *t {
                    return false;
                }
                is_subclass_of(hierarchy_parents, other, t) || {
                    let Some(other_id) = ontology.lookup_entity(other) else {
                        return false;
                    };
                    let Some(t_id) = ontology.lookup_entity(t) else {
                        return false;
                    };
                    ontology.direct_superclasses(other_id).contains(&t_id)
                        && type_set.contains(other.as_str())
                }
            })
        })
        .cloned()
        .collect()
}

fn is_subclass_of(parents: &BTreeMap<String, Vec<String>>, child: &str, ancestor: &str) -> bool {
    if child == ancestor {
        return false;
    }
    let mut stack = vec![child.to_string()];
    let mut seen = BTreeSet::new();
    while let Some(cur) = stack.pop() {
        if !seen.insert(cur.clone()) {
            continue;
        }
        let Some(supers) = parents.get(&cur) else {
            continue;
        };
        for p in supers {
            if p == ancestor {
                return true;
            }
            stack.push(p.clone());
        }
    }
    false
}

fn check_class_assertion(
    reasoner: &mut Reasoner,
    individual_iri: &str,
    class_iri: &str,
) -> Result<bool> {
    if cancel_requested() {
        return Err(ReasonerError::Cancelled);
    }
    match is_entailed_axiom(
        reasoner,
        EntailmentCheck::ClassAssertion {
            individual: individual_iri.to_string(),
            class: class_iri.to_string(),
        },
    ) {
        Ok(v) => Ok(v),
        Err(e) => {
            if cancel_requested() || e.to_string().to_ascii_lowercase().contains("cancel") {
                Err(ReasonerError::Cancelled)
            } else {
                // Incomplete / unsupported entailment → treat as not entailed, but surface cancel.
                Err(ReasonerError::Classify(e.to_string()))
            }
        }
    }
}

/// Realize all named individuals under the given profile.
pub fn realize(ontology: &Ontology, id: ReasonerId) -> Result<RealizationResult> {
    if cancel_requested() {
        return Err(ReasonerError::Cancelled);
    }
    let started = Instant::now();
    let mut reasoner = build_reasoner(ontology, id)?;
    let mut individuals = named_individuals(ontology)?;
    let classes = named_classes(ontology)?;

    let mut truncated = false;
    if individuals.len() > MAX_REALIZE_INDIVIDUALS {
        individuals.truncate(MAX_REALIZE_INDIVIDUALS);
        truncated = true;
    }

    // Build hierarchy parents from asserted structure.
    let mut parents: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (class_id, class_iri) in &classes {
        for &sup in ontology.direct_superclasses(*class_id) {
            if let Ok(sup_iri) = entity_iri(ontology, sup) {
                parents.entry(class_iri.clone()).or_default().push(sup_iri);
            }
        }
    }

    let mut entries = Vec::new();
    let mut entailment_checks = 0usize;
    let mut entailment_errors = 0usize;
    'individuals: for (ind_id, ind_iri) in &individuals {
        if cancel_requested() {
            return Err(ReasonerError::Cancelled);
        }
        let asserted: Vec<String> = ontology
            .classes_of(*ind_id)
            .iter()
            .filter_map(|cid| entity_iri(ontology, *cid).ok())
            .filter(|iri| {
                iri != "http://www.w3.org/2002/07/owl#Thing"
                    && iri != "http://www.w3.org/2002/07/owl#NamedIndividual"
            })
            .collect();

        let mut types = BTreeSet::new();
        for a in &asserted {
            types.insert(a.clone());
        }

        let mut candidates: BTreeSet<String> = asserted.iter().cloned().collect();
        for (_, class_iri) in &classes {
            candidates.insert(class_iri.clone());
        }

        for class_iri in &candidates {
            if types.contains(class_iri) {
                continue;
            }
            if entailment_checks >= MAX_REALIZE_ENTAILMENT_CHECKS {
                truncated = true;
                break 'individuals;
            }
            entailment_checks += 1;
            match check_class_assertion(&mut reasoner, ind_iri, class_iri) {
                Ok(true) => {
                    types.insert(class_iri.clone());
                }
                Ok(false) => {}
                Err(ReasonerError::Cancelled) => return Err(ReasonerError::Cancelled),
                // Non-cancel entailment failures: count as incomplete, do not treat as not-entailed (#361).
                Err(_) => {
                    entailment_errors += 1;
                    truncated = true;
                }
            }
        }

        let type_list: Vec<String> = types.into_iter().collect();
        let most_specific = most_specific_types(ontology, &parents, &type_list);
        let inferred: Vec<String> =
            type_list.iter().filter(|t| !asserted.contains(t)).cloned().collect();

        entries.push(RealizationEntry {
            individual_iri: ind_iri.clone(),
            types: type_list,
            most_specific,
            asserted,
            inferred,
        });

        if entailment_checks >= MAX_REALIZE_ENTAILMENT_CHECKS {
            truncated = true;
            break;
        }
    }

    Ok(RealizationResult {
        profile_used: id.as_str().to_string(),
        individuals: entries,
        duration_ms: started.elapsed().as_millis() as u64,
        truncated,
        entailment_errors,
    })
}

/// Collect inferred ABox assertions, reusing an existing realization when available.
pub fn inferred_assertions_from_realization(
    ontology: &Ontology,
    id: ReasonerId,
    realization: &RealizationResult,
) -> Result<InferredAssertions> {
    if cancel_requested() {
        return Err(ReasonerError::Cancelled);
    }

    let mut class_assertions = Vec::new();
    for entry in &realization.individuals {
        for class_iri in &entry.inferred {
            class_assertions.push(crate::result::InferredClassAssertion {
                individual_iri: entry.individual_iri.clone(),
                class_iri: class_iri.clone(),
            });
        }
    }

    let asserted_ops: BTreeSet<(String, String, String)> = {
        let mut set = BTreeSet::new();
        let inds = named_individuals(ontology)?;
        for (subj_id, subj_iri) in &inds {
            for &(prop_id, obj_id) in ontology.object_assertions_of(*subj_id) {
                let Ok(prop_iri) = entity_iri(ontology, prop_id) else {
                    continue;
                };
                let Ok(obj_iri) = entity_iri(ontology, obj_id) else {
                    continue;
                };
                set.insert((subj_iri.clone(), prop_iri, obj_iri));
            }
        }
        set
    };

    let mut object_property_assertions = Vec::new();
    let mut working = ontology.clone();
    // RL ABox materialization is the shared path for role inferences today.
    if matches!(
        id,
        ReasonerId::Rl | ReasonerId::Rdfs | ReasonerId::Auto | ReasonerId::El | ReasonerId::Dl
    ) {
        let _ = ontologos_rl::abox::materialize_abox(&mut working);
    }
    if cancel_requested() {
        return Err(ReasonerError::Cancelled);
    }

    let individuals = named_individuals(&working)?;
    let mut props = Vec::new();
    for (pid, record) in working.entities().iter() {
        if record.kind == EntityKind::ObjectProperty {
            if let Ok(iri) = entity_iri(&working, pid) {
                props.push((pid, iri));
            }
        }
    }
    for (subj_id, subj_iri) in &individuals {
        if cancel_requested() {
            return Err(ReasonerError::Cancelled);
        }
        for (prop_id, prop_iri) in &props {
            let Ok(objs) =
                ontologos_rl::abox::object_property_values(&mut working, *subj_id, *prop_id)
            else {
                continue;
            };
            for obj_id in objs {
                let Ok(obj_iri) = entity_iri(&working, obj_id) else {
                    continue;
                };
                if !asserted_ops.contains(&(subj_iri.clone(), prop_iri.clone(), obj_iri.clone())) {
                    object_property_assertions.push(
                        crate::result::InferredObjectPropertyAssertion {
                            subject_iri: subj_iri.clone(),
                            property_iri: prop_iri.clone(),
                            object_iri: obj_iri,
                        },
                    );
                }
            }
        }
    }

    let closure = ontologos_rl::abox::same_as_closure(ontology);
    let mut same_as_clusters = Vec::new();
    for cluster in &closure.clusters {
        if cluster.len() < 2 {
            continue;
        }
        let mut iris: Vec<String> =
            cluster.iter().filter_map(|id| entity_iri(ontology, *id).ok()).collect();
        iris.sort();
        iris.dedup();
        if iris.len() >= 2 {
            same_as_clusters.push(SameAsCluster { individuals: iris });
        }
    }

    Ok(InferredAssertions { class_assertions, object_property_assertions, same_as_clusters })
}

/// Collect inferred ABox assertions (class types, object properties, sameAs clusters).
pub fn inferred_assertions(ontology: &Ontology, id: ReasonerId) -> Result<InferredAssertions> {
    let realization = realize(ontology, id)?;
    inferred_assertions_from_realization(ontology, id, &realization)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancel_requested_aborts_realize() {
        let flag = Arc::new(AtomicBool::new(true));
        install_cancel_flag(Arc::clone(&flag));
        let ontology = Ontology::new();
        let err = realize(&ontology, ReasonerId::El).expect_err("cancelled");
        clear_cancel_flag();
        assert!(matches!(err, ReasonerError::Cancelled));
    }
}
