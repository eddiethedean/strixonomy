//! Protégé Wave 1 port: fresh axiom / entity location across imports.
//! Upstream: ActiveOntologyLocationStrategy, SubjectDefinitionLocationStrategy,
//! FreshAxiomLocation.

mod support;

use std::collections::BTreeMap;
use strixonomy_owl::PatchOp;
use support::protege_port::{
    apply_patches_reindex, copy_ported_tree, index_workspace, standard_ns,
};

fn home_ns() -> BTreeMap<String, String> {
    let mut ns = standard_ns();
    ns.insert("ex".to_string(), "http://example.org/home#".to_string());
    ns
}

#[test]
fn axiom_location_entities_carry_owning_ontology_id() {
    let dir = copy_ported_tree("imports_home");
    let catalog = index_workspace(dir.path());

    let main = catalog.find_entity("http://example.org/home#MainClass").expect("MainClass");
    let lib = catalog.find_entity("http://example.org/home#LibClass").expect("LibClass");
    assert_ne!(
        main.ontology_id, lib.ontology_id,
        "entities in different documents should have distinct ontology_id"
    );

    let docs = &catalog.data().documents;
    assert!(docs.len() >= 2, "expected main+lib documents, got {}", docs.len());
    let main_doc = docs.iter().find(|d| d.path.ends_with("main.ttl")).expect("main.ttl");
    let lib_doc = docs.iter().find(|d| d.path.ends_with("lib.ttl")).expect("lib.ttl");
    assert!(
        main_doc.imports.iter().any(|i| i.contains("home/lib")
            || i == &lib_doc.id
            || i == "http://example.org/home/lib"),
        "main should import lib; imports={:?} lib.id={}",
        main_doc.imports,
        lib_doc.id
    );
}

#[test]
fn axiom_location_patch_targets_active_document() {
    // Active-ontology strategy: patches applied to main.ttl attach axioms to main's ontology_id.
    let dir = copy_ported_tree("imports_home");
    let main_path = dir.path().join("main.ttl");
    let ns = home_ns();
    let child = "http://example.org/home#MainClass".to_string();
    let parent = "http://example.org/home#Shared".to_string();

    let catalog = apply_patches_reindex(
        dir.path(),
        &main_path,
        &[PatchOp::AddSubClassOf { entity_iri: child.clone(), parent_iri: parent.clone() }],
        &ns,
    );

    let main_entity = catalog.find_entity(&child).expect("MainClass");
    let axiom = catalog
        .data()
        .axioms
        .iter()
        .find(|a| a.axiom_kind == "sub_class_of" && a.subject == child && a.object == parent)
        .expect("SubClassOf axiom");
    assert_eq!(
        axiom.ontology_id, main_entity.ontology_id,
        "new axiom should live in the ontology of the actively patched document"
    );
}

#[test]
fn axiom_location_subject_definition_stays_in_lib_when_patching_lib() {
    // Subject-definition-style: edits against the defining ontology file keep axioms there.
    let dir = copy_ported_tree("imports_home");
    let lib_path = dir.path().join("lib.ttl");
    let ns = home_ns();
    let child = "http://example.org/home#LibClass".to_string();
    let parent = "http://example.org/home#Shared".to_string();

    let catalog = apply_patches_reindex(
        dir.path(),
        &lib_path,
        &[PatchOp::AddSubClassOf { entity_iri: child.clone(), parent_iri: parent.clone() }],
        &ns,
    );
    let lib_entity = catalog.find_entity(&child).expect("LibClass");
    let axiom = catalog
        .data()
        .axioms
        .iter()
        .find(|a| a.axiom_kind == "sub_class_of" && a.subject == child && a.object == parent)
        .expect("SubClassOf");
    assert_eq!(axiom.ontology_id, lib_entity.ontology_id);
}
