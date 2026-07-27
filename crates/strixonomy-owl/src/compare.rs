//! Cross-format semantic comparator for v0.21 round-trip gates.

use crate::bridge::{bridge_ontology, OwlBridgeResult};
use horned_owl::model::{RcAnnotatedComponent, RcStr};
use horned_owl::ontology::component_mapped::ComponentMappedOntology;
use std::collections::{BTreeMap, BTreeSet};
use strixonomy_core::AXIOM_KIND_SUB_CLASS_OF;

/// Diff between two semantic snapshots (empty means equivalent for compared facets).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SemanticDiff {
    pub missing_entities: Vec<String>,
    pub extra_entities: Vec<String>,
    pub label_mismatches: Vec<String>,
    pub parent_mismatches: Vec<String>,
    pub import_mismatches: Vec<String>,
    pub ontology_iri_mismatch: Option<(Option<String>, Option<String>)>,
}

impl SemanticDiff {
    pub fn is_empty(&self) -> bool {
        self.missing_entities.is_empty()
            && self.extra_entities.is_empty()
            && self.label_mismatches.is_empty()
            && self.parent_mismatches.is_empty()
            && self.import_mismatches.is_empty()
            && self.ontology_iri_mismatch.is_none()
    }
}

/// Compare two Horned ontologies for the v0.21 acceptance facets.
pub fn compare_ontologies(
    left: ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    right: ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
) -> SemanticDiff {
    let left_bridge = bridge_ontology(left, "left", "", &BTreeMap::new());
    let right_bridge = bridge_ontology(right, "right", "", &BTreeMap::new());
    compare_bridges(&left_bridge, &right_bridge)
}

pub fn compare_bridges(left: &OwlBridgeResult, right: &OwlBridgeResult) -> SemanticDiff {
    let mut diff = SemanticDiff::default();

    if left.base_iri != right.base_iri {
        diff.ontology_iri_mismatch = Some((left.base_iri.clone(), right.base_iri.clone()));
    }

    let left_iris: BTreeSet<_> = left.entities.iter().map(|e| e.iri.clone()).collect();
    let right_iris: BTreeSet<_> = right.entities.iter().map(|e| e.iri.clone()).collect();
    for iri in left_iris.difference(&right_iris) {
        diff.missing_entities.push(iri.clone());
    }
    for iri in right_iris.difference(&left_iris) {
        diff.extra_entities.push(iri.clone());
    }

    let right_by_iri: BTreeMap<_, _> = right.entities.iter().map(|e| (e.iri.clone(), e)).collect();
    for le in &left.entities {
        if let Some(re) = right_by_iri.get(&le.iri) {
            let mut ll = le.labels.clone();
            let mut rl = re.labels.clone();
            ll.sort();
            rl.sort();
            if ll != rl {
                diff.label_mismatches.push(le.iri.clone());
            }
        }
    }

    let left_parents = parents_map(left);
    let right_parents = parents_map(right);
    let subjects: BTreeSet<_> = left_parents.keys().chain(right_parents.keys()).cloned().collect();
    for subject in subjects {
        let lp = left_parents.get(&subject).cloned().unwrap_or_default();
        let rp = right_parents.get(&subject).cloned().unwrap_or_default();
        if lp != rp {
            diff.parent_mismatches.push(subject);
        }
    }

    let left_imports: BTreeSet<_> = left.imports.iter().map(|i| i.import_iri.clone()).collect();
    let right_imports: BTreeSet<_> = right.imports.iter().map(|i| i.import_iri.clone()).collect();
    for iri in left_imports.symmetric_difference(&right_imports) {
        diff.import_mismatches.push(iri.clone());
    }

    diff
}

fn parents_map(bridge: &OwlBridgeResult) -> BTreeMap<String, BTreeSet<String>> {
    let mut map: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for ax in &bridge.axioms {
        if ax.axiom_kind == AXIOM_KIND_SUB_CLASS_OF {
            map.entry(ax.subject.clone()).or_default().insert(ax.object.clone());
        }
    }
    map
}
