//! Protégé Wave 1 port: reference finder + defined-class predicate.
//! Upstream: ReferenceFinder_TestCase, DefinedClassPredicate_TestCase.

mod support;

use std::collections::BTreeMap;
use strixonomy_core::AXIOM_KIND_EQUIVALENT_CLASS;
use strixonomy_owl::PatchOp;
use strixonomy_refactor::find_usages;
use support::protege_port::{index_workspace, standard_ns};

fn dep_ns() -> BTreeMap<String, String> {
    let mut ns = standard_ns();
    ns.insert("ex".to_string(), "http://example.org/refs#".to_string());
    ns
}

#[test]
fn refs_find_usages_for_superclass() {
    let ttl = r#"
@prefix ex: <http://example.org/refs#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ex:Ontology a owl:Ontology .
ex:Person a owl:Class .
ex:Employee a owl:Class ; rdfs:subClassOf ex:Person .
"#;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("refs.ttl");
    std::fs::write(&path, ttl).unwrap();
    let catalog = index_workspace(dir.path());
    let usages = find_usages(&catalog, "http://example.org/refs#Person");
    assert!(!usages.is_empty(), "Person should have usages, got {usages:?}");
    assert!(
        usages.iter().any(|u| u.context.contains("Employee") || u.context.contains("Person")),
        "expected Employee/Person context in usages: {usages:?}"
    );
}

#[test]
fn refs_defined_class_has_equivalent_class_axiom() {
    let ttl = r#"
@prefix ex: <http://example.org/refs#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ex:Ontology a owl:Ontology .
ex:Parent a owl:Class .
ex:Child a owl:Class .
ex:Defined a owl:Class .
"#;
    let defined = "http://example.org/refs#Defined";
    let (_dir, _path, catalog) = support::apply_and_reindex(
        ttl,
        &[PatchOp::AddEquivalentClass {
            entity_iri: defined.to_string(),
            manchester: "ex:Parent and ex:Child".to_string(),
        }],
        &dep_ns(),
    );

    let is_defined = catalog
        .data()
        .axioms
        .iter()
        .any(|a| a.axiom_kind == AXIOM_KIND_EQUIVALENT_CLASS && a.subject == defined);
    assert!(
        is_defined,
        "Defined class should have equivalent_class axiom; axioms={:?}",
        catalog.data().axioms.iter().filter(|a| a.subject == defined).collect::<Vec<_>>()
    );

    // Primitive class Parent should not be "defined"
    let parent_defined = catalog.data().axioms.iter().any(|a| {
        a.axiom_kind == AXIOM_KIND_EQUIVALENT_CLASS && a.subject == "http://example.org/refs#Parent"
    });
    assert!(!parent_defined, "Parent should remain primitive");
}

#[test]
fn refs_disjoint_union_counts_as_defined_style_construct() {
    let ttl = r#"
@prefix ex: <http://example.org/refs#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Ontology a owl:Ontology .
ex:Color a owl:Class .
ex:Red a owl:Class .
ex:Blue a owl:Class .
"#;
    let (_dir, _path, catalog) = support::apply_and_reindex(
        ttl,
        &[PatchOp::AddDisjointUnion {
            class_iri: "http://example.org/refs#Color".to_string(),
            members: vec![
                "http://example.org/refs#Red".to_string(),
                "http://example.org/refs#Blue".to_string(),
            ],
        }],
        &dep_ns(),
    );
    let has_du =
        catalog.data().axioms.iter().any(|a| {
            a.axiom_kind == "disjoint_union" && a.subject == "http://example.org/refs#Color"
        });
    assert!(has_du, "expected DisjointUnion on Color");
}
