//! Horned/catalog reparse oracles for OWL patch success paths.
//!
//! These replace string-contains checks: a no-op rewrite that leaves Turtle looking
//! similar but does not update the Horned catalog must fail here.

mod support;

use std::collections::BTreeMap;
use std::path::PathBuf;
use strixonomy_core::{AXIOM_KIND_CLASS_ASSERTION, AXIOM_KIND_DOMAIN, AXIOM_KIND_PROPERTY_CHAIN};
use strixonomy_owl::{PatchEntityKind, PatchOp};

fn fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures").join(name);
    std::fs::read_to_string(path).expect("read fixture")
}

fn people_ns() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("ex".to_string(), "http://example.org/people#".to_string()),
        ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
    ])
}

fn clinic_ns() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("ex".to_string(), "http://example.org/clinic#".to_string()),
        ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
    ])
}

fn org_ns() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("ex".to_string(), "http://example.org/org#".to_string()),
        ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
    ])
}

#[test]
fn delete_entity_removes_all_statements() {
    let person = "http://example.org/people#Person";
    let (_dir, _path, catalog) = support::apply_and_reindex(
        &fixture("example.ttl"),
        &[PatchOp::DeleteEntity { entity_iri: person.to_string() }],
        &people_ns(),
    );
    assert!(catalog.find_entity(person).is_none(), "Person must be absent after reindex");
}

#[test]
fn remove_complex_subclass_keeps_named_parent() {
    let patient = "http://example.org/clinic#Patient";
    let clinic_person = "http://example.org/clinic#ClinicPerson";
    let (_dir, _path, catalog) = support::apply_and_reindex(
        &fixture("complex-classes.ttl"),
        &[PatchOp::RemoveComplexSubClassOf {
            entity_iri: patient.to_string(),
            manchester: "ex:hasRecord some ex:MedicalRecord".to_string(),
        }],
        &clinic_ns(),
    );
    let hierarchy = catalog.class_hierarchy();
    let parents = hierarchy.parents.get(patient).cloned().unwrap_or_default();
    assert!(
        parents.iter().any(|p| p == clinic_person),
        "named parent ClinicPerson must remain: {parents:?}"
    );
    assert!(
        parents.iter().all(|p| p.starts_with("http")),
        "complex/restriction parents must be gone: {parents:?}"
    );
    let detail = catalog.entity_detail(patient).expect("Patient detail");
    assert!(
        !detail
            .axioms
            .iter()
            .any(|a| a.manchester.as_deref().is_some_and(|m| m.contains("hasRecord"))),
        "complex subclass manchester must be gone: {:?}",
        detail.axioms
    );
}

#[test]
fn add_and_remove_domain() {
    let chases = "http://example.org/org#chases";
    let cat = "http://example.org/org#Cat";
    let dog = "http://example.org/org#Dog";
    let ns = org_ns();
    let (dir, path, catalog) = support::apply_and_reindex(
        &fixture("disjoint-classes.ttl"),
        &[PatchOp::AddDomain { entity_iri: chases.to_string(), class_iri: cat.to_string() }],
        &ns,
    );
    let detail = catalog.entity_detail(chases).expect("chases");
    assert!(
        detail.axioms.iter().any(|a| {
            a.kind == AXIOM_KIND_DOMAIN
                && (a.parent_iri.as_deref() == Some(cat)
                    || a.display.contains(cat)
                    || a.display.contains("Cat"))
        }),
        "Cat domain missing: {:?}",
        detail.axioms
    );

    let catalog = support::reapply_and_reindex(
        dir.path(),
        &path,
        &[PatchOp::RemoveDomain { entity_iri: chases.to_string(), class_iri: dog.to_string() }],
        &ns,
    );
    let detail = catalog.entity_detail(chases).expect("chases after remove Dog");
    assert!(
        detail.axioms.iter().any(|a| {
            a.kind == AXIOM_KIND_DOMAIN
                && (a.parent_iri.as_deref() == Some(cat)
                    || a.display.contains(cat)
                    || a.display.contains("Cat"))
        }),
        "Cat domain must remain: {:?}",
        detail.axioms
    );
    assert!(
        !detail.axioms.iter().any(|a| {
            a.kind == AXIOM_KIND_DOMAIN
                && (a.parent_iri.as_deref() == Some(dog)
                    || a.display.contains(dog)
                    || a.display.contains("Dog"))
        }),
        "Dog domain must be absent: {:?}",
        detail.axioms
    );
}

#[test]
fn add_property_chain() {
    let chases = "http://example.org/org#chases";
    let composed = "http://example.org/org#composed";
    let (_dir, _path, catalog) = support::apply_and_reindex(
        &fixture("disjoint-classes.ttl"),
        &[PatchOp::AddPropertyChain {
            entity_iri: chases.to_string(),
            properties: vec![chases.to_string(), composed.to_string()],
        }],
        &org_ns(),
    );
    let detail = catalog.entity_detail(chases).expect("chases");
    let chain = detail
        .axioms
        .iter()
        .find(|a| a.kind == AXIOM_KIND_PROPERTY_CHAIN)
        .expect("property_chain axiom");
    assert_eq!(chain.properties, vec![chases.to_string(), composed.to_string()]);
}

#[test]
fn add_class_assertion_to_individual() {
    let ttl = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Alice a owl:NamedIndividual .
ex:Person a owl:Class .
"#;
    let alice = "http://example.org/org#Alice";
    let person = "http://example.org/org#Person";
    let (_dir, _path, catalog) = support::apply_and_reindex(
        ttl,
        &[PatchOp::AddClassAssertion {
            entity_iri: alice.to_string(),
            class_iri: person.to_string(),
        }],
        &org_ns(),
    );
    let detail = catalog.entity_detail(alice).expect("Alice");
    assert!(
        detail.axioms.iter().any(|a| {
            a.kind == AXIOM_KIND_CLASS_ASSERTION && a.parent_iri.as_deref() == Some(person)
        }),
        "class_assertion Alice→Person missing: {:?}",
        detail.axioms
    );
}

#[test]
fn set_functional_property() {
    let chases = "http://example.org/org#chases";
    let (_dir, _path, catalog) = support::apply_and_reindex(
        &fixture("disjoint-classes.ttl"),
        &[PatchOp::SetFunctional { entity_iri: chases.to_string(), value: true }],
        &org_ns(),
    );
    let detail = catalog.entity_detail(chases).expect("chases");
    assert!(detail.characteristics.functional, "expected functional characteristic");
}

#[test]
fn add_and_remove_import() {
    let ttl = r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

<http://example.org/people> a owl:Ontology ;
    rdfs:label "People" .
"#;
    let ontology_iri = "http://example.org/people";
    let import_iri = "http://example.org/org";
    let ns = people_ns();
    let (dir, path, catalog) = support::apply_and_reindex(
        ttl,
        &[PatchOp::AddImport {
            ontology_iri: ontology_iri.to_string(),
            import_iri: import_iri.to_string(),
        }],
        &ns,
    );
    let imports: Vec<_> =
        catalog.data().documents.iter().flat_map(|d| d.imports.iter().cloned()).collect();
    assert!(imports.iter().any(|i| i == import_iri), "import missing after add: {imports:?}");

    let catalog = support::reapply_and_reindex(
        dir.path(),
        &path,
        &[PatchOp::RemoveImport {
            ontology_iri: ontology_iri.to_string(),
            import_iri: import_iri.to_string(),
        }],
        &ns,
    );
    let imports: Vec<_> =
        catalog.data().documents.iter().flat_map(|d| d.imports.iter().cloned()).collect();
    assert!(
        !imports.iter().any(|i| i == import_iri),
        "import must be gone after remove: {imports:?}"
    );
}

#[test]
fn add_generic_annotation() {
    let ttl = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix skos: <http://www.w3.org/2004/02/skos/core#> .

skos:definition a owl:AnnotationProperty .
ex:Cat a owl:Class .
"#;
    let mut ns = org_ns();
    ns.insert("skos".to_string(), "http://www.w3.org/2004/02/skos/core#".to_string());
    let cat = "http://example.org/org#Cat";
    let pred = "http://www.w3.org/2004/02/skos/core#definition";
    let (_dir, _path, catalog) = support::apply_and_reindex(
        ttl,
        &[PatchOp::AddAnnotation {
            entity_iri: cat.to_string(),
            predicate: "skos:definition".to_string(),
            value: "A feline animal".to_string(),
        }],
        &ns,
    );
    let detail = catalog.entity_detail(cat).expect("Cat");
    assert!(
        detail.annotations.iter().any(|a| a.predicate == pred && a.value == "A feline animal"),
        "skos:definition annotation missing: {:?}",
        detail.annotations
    );
}

#[test]
fn remove_subclass_from_trailing_triple() {
    let person = "http://example.org/people#Person";
    let thing = "http://example.org/people#Thing";
    let (_dir, _path, catalog) = support::apply_and_reindex(
        &fixture("example.ttl"),
        &[PatchOp::RemoveSubClassOf {
            entity_iri: person.to_string(),
            parent_iri: thing.to_string(),
        }],
        &people_ns(),
    );
    assert!(catalog.find_entity(person).is_some(), "Person class must remain");
    let parents = catalog.class_hierarchy().parents.get(person).cloned().unwrap_or_default();
    assert!(!parents.iter().any(|p| p == thing), "Thing parent must be removed: {parents:?}");
}

#[test]
fn set_deprecated_false_removes_trailing_flag() {
    let ttl = r#"@prefix ex: <http://example.org/people#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class .
ex:Person owl:deprecated true .
"#;
    let person = "http://example.org/people#Person";
    let (_dir, _path, catalog) = support::apply_and_reindex(
        ttl,
        &[PatchOp::SetDeprecated { entity_iri: person.to_string(), value: false }],
        &people_ns(),
    );
    let entity = catalog.find_entity(person).expect("Person");
    assert!(!entity.deprecated, "deprecated must be false after clear");
}

#[test]
fn create_then_delete_entity_oracle() {
    // Sanity: create is indexed, then delete removes it (guards helper itself).
    let iri = "http://example.org/people#TempClass";
    let ns = people_ns();
    let (dir, path, catalog) = support::apply_and_reindex(
        &fixture("example.ttl"),
        &[PatchOp::CreateEntity { entity_iri: iri.to_string(), kind: PatchEntityKind::Class }],
        &ns,
    );
    assert!(catalog.find_entity(iri).is_some());
    let catalog = support::reapply_and_reindex(
        dir.path(),
        &path,
        &[PatchOp::DeleteEntity { entity_iri: iri.to_string() }],
        &ns,
    );
    assert!(catalog.find_entity(iri).is_none());
}
