use crate::model::{
    AnnotationChange, AxiomChange, BreakingChange, BreakingReason, DiffResult, EntityChange,
    EntityChangeKind, ImportChange,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use strixonomy_catalog::{IndexBuilder, OntologyCatalog};
use strixonomy_core::{
    Annotation, Axiom, Entity, Import, AXIOM_KIND_DOMAIN, AXIOM_KIND_RANGE, AXIOM_KIND_SUB_CLASS_OF,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiffError {
    #[error("catalog error: {0}")]
    Catalog(#[from] strixonomy_catalog::CatalogError),

    #[error("{0}")]
    Message(String),
}

pub type Result<T> = std::result::Result<T, DiffError>;

pub fn diff_directories(left: &Path, right: &Path) -> Result<DiffResult> {
    let left_cat = IndexBuilder::new().workspace(left).build()?;
    let right_cat = IndexBuilder::new().workspace(right).build()?;
    Ok(diff_catalogs(&left_cat, &right_cat))
}

pub fn diff_catalogs(base: &OntologyCatalog, head: &OntologyCatalog) -> DiffResult {
    let mut result = DiffResult::default();
    diff_entities(base, head, &mut result);
    diff_annotations(base, head, &mut result);
    detect_renames(&mut result);
    diff_axioms(base, head, &mut result);
    diff_imports(base, head, &mut result);
    detect_breaking(&mut result);
    result
}

fn diff_entities(base: &OntologyCatalog, head: &OntologyCatalog, result: &mut DiffResult) {
    let base_map: BTreeMap<&str, &Entity> =
        base.data().entities.iter().map(|e| (e.iri.as_str(), e)).collect();
    let head_map: BTreeMap<&str, &Entity> =
        head.data().entities.iter().map(|e| (e.iri.as_str(), e)).collect();

    for (iri, entity) in &head_map {
        if !base_map.contains_key(iri) {
            result.entity_changes.push(EntityChange {
                kind: EntityChangeKind::Added,
                iri: iri.to_string(),
                previous_iri: None,
                labels: entity.labels.clone(),
            });
        }
    }

    for (iri, entity) in &base_map {
        if !head_map.contains_key(iri) {
            result.entity_changes.push(EntityChange {
                kind: EntityChangeKind::Removed,
                iri: iri.to_string(),
                previous_iri: None,
                labels: entity.labels.clone(),
            });
        }
    }

    for (iri, head_entity) in &head_map {
        if let Some(base_entity) = base_map.get(iri) {
            if head_entity.deprecated && !base_entity.deprecated {
                result.entity_changes.push(EntityChange {
                    kind: EntityChangeKind::Deprecated,
                    iri: iri.to_string(),
                    previous_iri: None,
                    labels: head_entity.labels.clone(),
                });
            }
        }
    }
}

fn detect_renames(result: &mut DiffResult) {
    let added: Vec<EntityChange> = result
        .entity_changes
        .iter()
        .filter(|c| c.kind == EntityChangeKind::Added)
        .cloned()
        .collect();
    let removed: Vec<EntityChange> = result
        .entity_changes
        .iter()
        .filter(|c| c.kind == EntityChangeKind::Removed)
        .cloned()
        .collect();
    let mut drop_removed = BTreeSet::new();
    let mut drop_added = BTreeSet::new();
    for r in &removed {
        for a in &added {
            if drop_added.contains(&a.iri) {
                continue;
            }
            // Require a newly added structural identity link (#20 / #389).
            let sameas = result.annotation_changes.iter().any(|ann| {
                ann.change == "added"
                    && is_rename_link_predicate(&ann.predicate)
                    && ((ann.subject == r.iri && ann.object == a.iri)
                        || (ann.subject == a.iri && ann.object == r.iri))
            });
            if sameas {
                result.entity_changes.push(EntityChange {
                    kind: EntityChangeKind::Renamed,
                    iri: a.iri.clone(),
                    previous_iri: Some(r.iri.clone()),
                    labels: a.labels.clone(),
                });
                drop_removed.insert(r.iri.clone());
                drop_added.insert(a.iri.clone());
                break;
            }
        }
    }
    result.entity_changes.retain(|c| {
        !((c.kind == EntityChangeKind::Removed && drop_removed.contains(&c.iri))
            || (c.kind == EntityChangeKind::Added && drop_added.contains(&c.iri)))
    });
}

/// Exact identity-link predicates used for rename detection (#389).
fn is_rename_link_predicate(pred: &str) -> bool {
    matches!(
        pred,
        "http://www.w3.org/2002/07/owl#sameAs"
            | "owl:sameAs"
            | "http://www.w3.org/2004/02/skos/core#exactMatch"
            | "skos:exactMatch"
    )
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
struct AnnotationKey(String, String, String, String);

fn annotation_key(a: &Annotation) -> AnnotationKey {
    AnnotationKey(a.ontology_id.clone(), a.subject.clone(), a.predicate.clone(), a.object.clone())
}

fn diff_annotations(base: &OntologyCatalog, head: &OntologyCatalog, result: &mut DiffResult) {
    let base_set: BTreeSet<AnnotationKey> =
        base.data().annotations.iter().map(annotation_key).collect();
    let head_set: BTreeSet<AnnotationKey> =
        head.data().annotations.iter().map(annotation_key).collect();

    for ann in &head.data().annotations {
        if !base_set.contains(&annotation_key(ann)) {
            result.annotation_changes.push(AnnotationChange {
                change: "added".to_string(),
                subject: ann.subject.clone(),
                predicate: ann.predicate.clone(),
                object: ann.object.clone(),
            });
        }
    }
    for ann in &base.data().annotations {
        if !head_set.contains(&annotation_key(ann)) {
            result.annotation_changes.push(AnnotationChange {
                change: "removed".to_string(),
                subject: ann.subject.clone(),
                predicate: ann.predicate.clone(),
                object: ann.object.clone(),
            });
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
struct AxiomKey(String, String, String, String, String);

fn axiom_key(a: &Axiom) -> AxiomKey {
    AxiomKey(
        a.ontology_id.clone(),
        a.axiom_kind.clone(),
        a.subject.clone(),
        a.predicate.clone(),
        a.object.clone(),
    )
}

fn diff_axioms(base: &OntologyCatalog, head: &OntologyCatalog, result: &mut DiffResult) {
    let base_set: BTreeSet<AxiomKey> = base.data().axioms.iter().map(axiom_key).collect();
    let head_set: BTreeSet<AxiomKey> = head.data().axioms.iter().map(axiom_key).collect();

    for ax in &head.data().axioms {
        if !base_set.contains(&axiom_key(ax)) {
            result.axiom_changes.push(AxiomChange {
                change: "added".to_string(),
                subject: ax.subject.clone(),
                predicate: ax.predicate.clone(),
                object: ax.object.clone(),
                axiom_kind: ax.axiom_kind.clone(),
            });
        }
    }
    for ax in &base.data().axioms {
        if !head_set.contains(&axiom_key(ax)) {
            result.axiom_changes.push(AxiomChange {
                change: "removed".to_string(),
                subject: ax.subject.clone(),
                predicate: ax.predicate.clone(),
                object: ax.object.clone(),
                axiom_kind: ax.axiom_kind.clone(),
            });
            if ax.axiom_kind == AXIOM_KIND_SUB_CLASS_OF {
                result.breaking_changes.push(BreakingChange {
                    reason: BreakingReason::RemovedSuperclass,
                    message: format!(
                        "removed subclass axiom: {} subClassOf {}",
                        ax.subject, ax.object
                    ),
                    entity_iri: Some(ax.subject.clone()),
                });
            }
            if ax.axiom_kind == AXIOM_KIND_DOMAIN || ax.axiom_kind == AXIOM_KIND_RANGE {
                result.breaking_changes.push(BreakingChange {
                    reason: BreakingReason::DomainRangeChange,
                    message: format!("removed {} axiom on {}", ax.axiom_kind, ax.subject),
                    entity_iri: Some(ax.subject.clone()),
                });
            }
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
struct ImportKey(String, String);

fn import_key(i: &Import) -> ImportKey {
    ImportKey(i.ontology_id.clone(), i.import_iri.clone())
}

fn diff_imports(base: &OntologyCatalog, head: &OntologyCatalog, result: &mut DiffResult) {
    let base_set: BTreeSet<ImportKey> = base.data().imports.iter().map(import_key).collect();
    let head_set: BTreeSet<ImportKey> = head.data().imports.iter().map(import_key).collect();

    for imp in &head.data().imports {
        if !base_set.contains(&import_key(imp)) {
            result.import_changes.push(ImportChange {
                change: "added".to_string(),
                ontology_id: imp.ontology_id.clone(),
                import_iri: imp.import_iri.clone(),
            });
        }
    }
    for imp in &base.data().imports {
        if !head_set.contains(&import_key(imp)) {
            result.import_changes.push(ImportChange {
                change: "removed".to_string(),
                ontology_id: imp.ontology_id.clone(),
                import_iri: imp.import_iri.clone(),
            });
            result.breaking_changes.push(BreakingChange {
                reason: BreakingReason::RemovedImport,
                message: format!("removed import: {}", imp.import_iri),
                entity_iri: None,
            });
        }
    }
}

fn detect_breaking(result: &mut DiffResult) {
    let mut seen = BTreeSet::new();
    for change in &result.entity_changes {
        match change.kind {
            EntityChangeKind::Removed => {
                let key = format!("removed:{}", change.iri);
                if seen.insert(key) {
                    result.breaking_changes.push(BreakingChange {
                        reason: BreakingReason::RemovedEntity,
                        message: format!("removed entity: {}", change.iri),
                        entity_iri: Some(change.iri.clone()),
                    });
                }
            }
            EntityChangeKind::Renamed => {
                let key = format!("renamed:{}", change.iri);
                if seen.insert(key) {
                    result.breaking_changes.push(BreakingChange {
                        reason: BreakingReason::RenamedIri,
                        message: format!(
                            "renamed entity: {} -> {}",
                            change.previous_iri.as_deref().unwrap_or("?"),
                            change.iri
                        ),
                        entity_iri: Some(change.iri.clone()),
                    });
                }
            }
            _ => {}
        }
    }
}

/// Merge unsatisfiability sets into inference + breaking sections.
pub fn apply_unsat_diff(result: &mut DiffResult, base_unsat: &[String], head_unsat: &[String]) {
    let base: BTreeSet<&str> = base_unsat.iter().map(String::as_str).collect();
    let head: BTreeSet<&str> = head_unsat.iter().map(String::as_str).collect();
    for iri in head.difference(&base) {
        result.inference_changes.push(crate::model::InferenceChange {
            class_iri: (*iri).to_string(),
            change: "became_unsatisfiable".to_string(),
            detail: "class is unsatisfiable in head but not base".to_string(),
        });
        result.breaking_changes.push(BreakingChange {
            reason: BreakingReason::UnsatisfiableClass,
            message: format!("class became unsatisfiable: {iri}"),
            entity_iri: Some((*iri).to_string()),
        });
    }
    for iri in base.difference(&head) {
        result.inference_changes.push(crate::model::InferenceChange {
            class_iri: (*iri).to_string(),
            change: "became_satisfiable".to_string(),
            detail: "class is satisfiable in head but was unsatisfiable in base".to_string(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixtures() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
    }

    #[test]
    fn diff_same_directory_is_empty() {
        let path = fixtures();
        let cat = IndexBuilder::new().workspace(&path).build().expect("index");
        let diff = diff_catalogs(&cat, &cat);
        assert!(diff.is_empty());
    }
}
