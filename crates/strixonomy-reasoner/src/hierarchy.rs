use ontologos_core::{EntityKind, Ontology};
use strixonomy_catalog::{ClassHierarchy, SubclassEdge};

/// Asserted subclass edges taken from the ontology actually loaded for reasoning
/// (including open-buffer overrides), not from a separately indexed catalog.
pub fn asserted_hierarchy_from_ontology(ontology: &Ontology) -> ClassHierarchy {
    let mut edges = Vec::new();
    for (id, entity) in ontology.entities().iter() {
        if entity.kind != EntityKind::Class {
            continue;
        }
        let Ok(child) = ontology.resolve_iri(entity.iri).map(|s| s.to_string()) else {
            continue;
        };
        for &parent_id in ontology.direct_superclasses(id) {
            let Ok(parent_entity) = ontology.entity(parent_id) else {
                continue;
            };
            let Ok(parent) = ontology.resolve_iri(parent_entity.iri).map(|s| s.to_string()) else {
                continue;
            };
            edges.push(SubclassEdge { child: child.clone(), parent });
        }
    }
    crate::result::hierarchy_from_edges(edges)
}

pub fn subclass_edges_from_ontology(
    ontology: &Ontology,
    asserted: &ClassHierarchy,
) -> Vec<(String, String)> {
    let asserted_set: std::collections::BTreeSet<(String, String)> =
        asserted.edges.iter().map(|e| (e.child.clone(), e.parent.clone())).collect();

    let mut edges = Vec::new();
    for (id, entity) in ontology.entities().iter() {
        if entity.kind != EntityKind::Class {
            continue;
        }
        let child = match ontology.resolve_iri(entity.iri) {
            Ok(s) => s.to_string(),
            Err(_) => continue,
        };
        for &parent_id in ontology.direct_superclasses(id) {
            let parent_entity = match ontology.entity(parent_id) {
                Ok(e) => e,
                Err(_) => continue,
            };
            let parent = match ontology.resolve_iri(parent_entity.iri) {
                Ok(s) => s.to_string(),
                Err(_) => continue,
            };
            if !asserted_set.contains(&(child.clone(), parent.clone())) {
                edges.push((child.clone(), parent));
            }
        }
    }
    edges
}
