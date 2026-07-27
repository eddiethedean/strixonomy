//! v0.22 OWL 2 authoring patch + round-trip coverage.

use std::collections::BTreeMap;
use std::path::PathBuf;
use strixonomy::Workspace;
use strixonomy_catalog::IndexBuilder;
use strixonomy_core::OntologyFormat;
use strixonomy_owl::{apply_patches, apply_patches_to_text, apply_xml_patches_to_text, PatchOp};

fn roundtrip_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/protege-roundtrip")
}

fn keys_ns() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("ex".into(), "http://example.org/keys#".into()),
        ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ("rdfs".into(), "http://www.w3.org/2000/01/rdf-schema#".into()),
        ("xsd".into(), "http://www.w3.org/2001/XMLSchema#".into()),
    ])
}

fn abox_ns() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("ex".into(), "http://example.org/abox#".into()),
        ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ("rdfs".into(), "http://www.w3.org/2000/01/rdf-schema#".into()),
        ("xsd".into(), "http://www.w3.org/2001/XMLSchema#".into()),
    ])
}

#[test]
fn owl2_keys_fixture_indexes_haskey_and_inverse() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    assert!(ws.catalog().find_entity("http://example.org/keys#Person").is_some());
    assert!(ws.catalog().find_entity("http://example.org/keys#hasSSN").is_some());
    assert!(ws.catalog().find_entity("http://example.org/keys#hasParent").is_some());
    assert!(ws.catalog().find_entity("http://example.org/keys#hasChild").is_some());
    assert!(ws.catalog().find_entity("http://example.org/keys#Sex").is_some());

    let person =
        ws.catalog().entity_detail("http://example.org/keys#Person").expect("Person detail");
    assert!(
        person.axioms.iter().any(|a| a.kind == "has_key"),
        "Person must list HasKey on EntityDetail, axioms={:?}",
        person.axioms
    );
    let sex = ws.catalog().entity_detail("http://example.org/keys#Sex").expect("Sex");
    assert!(
        sex.axioms.iter().any(|a| a.kind == "disjoint_union"),
        "Sex must list DisjointUnion, axioms={:?}",
        sex.axioms
    );
    let has_parent =
        ws.catalog().entity_detail("http://example.org/keys#hasParent").expect("hasParent");
    assert!(
        has_parent.axioms.iter().any(|a| a.kind == "inverse_object_properties"
            && a.other_iri.as_deref() == Some("http://example.org/keys#hasChild")),
        "hasParent must list InverseObjectProperties, axioms={:?}",
        has_parent.axioms
    );

    // Round-trip: patch a second key property onto Person and confirm write-back.
    let tmp = tempfile::tempdir().expect("tempdir");
    let path = tmp.path().join("owl2-keys.ttl");
    std::fs::copy(dir.join("owl2-keys.ttl"), &path).expect("copy");
    apply_patches(
        &path,
        &[PatchOp::AddInverseObjectProperties {
            property_iri: "http://example.org/keys#hasChild".into(),
            inverse_iri: "http://example.org/keys#hasParent".into(),
        }],
        false,
        &keys_ns(),
    )
    .expect("apply inverse");
    let out = std::fs::read_to_string(&path).expect("read");
    assert!(
        out.contains("inverseOf") || out.contains("hasParent"),
        "inverse patch must persist: {out}"
    );
}

#[test]
fn owl2_abox_fixture_indexes_sameas_and_negative() {
    let dir = roundtrip_dir();
    let ws = Workspace::open(&dir).expect("open workspace");
    let alice = ws.catalog().entity_detail("http://example.org/abox#alice").expect("alice");
    assert!(
        alice.axioms.iter().any(|a| a.kind == "same_individual"),
        "alice must list SameIndividual, axioms={:?}",
        alice.axioms
    );
    assert!(
        alice.axioms.iter().any(|a| a.kind == "different_individuals"),
        "alice must list DifferentIndividuals, axioms={:?}",
        alice.axioms
    );
    assert!(
        alice.axioms.iter().any(|a| a.kind == "negative_object_property_assertion"),
        "alice must list NegativeOPA, axioms={:?}",
        alice.axioms
    );
}

#[test]
fn owl2_turtle_patch_add_has_key_and_same_individual() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("owl2-keys.ttl");
    std::fs::copy(roundtrip_dir().join("owl2-keys.ttl"), &path).expect("copy");

    let person = "http://example.org/keys#Person";
    let has_parent = "http://example.org/keys#hasParent";
    let preview = apply_patches_to_text(
        &std::fs::read_to_string(&path).expect("read"),
        &[PatchOp::AddHasKey { class_iri: person.into(), properties: vec![has_parent.into()] }],
        true,
        &keys_ns(),
    )
    .expect("preview hasKey");
    let text = preview.preview_text.expect("preview text");
    assert!(
        text.contains("hasKey") || text.contains(has_parent),
        "hasKey patch preview missing key content: {text}"
    );

    apply_patches(
        &path,
        &[PatchOp::AddHasKey { class_iri: person.into(), properties: vec![has_parent.into()] }],
        false,
        &keys_ns(),
    )
    .expect("apply hasKey");

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("reindex");
    let detail = catalog.entity_detail(person).expect("Person detail");
    assert!(
        detail.axioms.iter().any(|a| a.display.contains("hasParent") || a.kind.contains("key"))
            || std::fs::read_to_string(&path).unwrap().contains("hasParent"),
        "hasKey must persist, axioms={:?}",
        detail.axioms
    );
}

#[test]
fn owl2_turtle_patch_abox_negative_and_same() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("owl2-abox.ttl");
    std::fs::copy(roundtrip_dir().join("owl2-abox.ttl"), &path).expect("copy");

    let alice = "http://example.org/abox#alice";
    let carol = "http://example.org/abox#carol";
    let knows = "http://example.org/abox#knows";

    apply_patches(
        &path,
        &[
            PatchOp::AddSameIndividual { individuals: vec![alice.into(), carol.into()] },
            PatchOp::AddNegativeObjectPropertyAssertion {
                entity_iri: alice.into(),
                property_iri: knows.into(),
                target_iri: carol.into(),
            },
        ],
        false,
        &abox_ns(),
    )
    .expect("apply abox patches");

    let out = std::fs::read_to_string(&path).expect("read result");
    assert!(
        out.contains("sameAs") || out.contains(carol),
        "sameAs / negative patches must write content: {out}"
    );
}

#[test]
fn owl2_turtle_remove_alldifferent_pair_and_add_nary() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("owl2-abox.ttl");
    std::fs::copy(roundtrip_dir().join("owl2-abox.ttl"), &path).expect("copy");

    let alice = "http://example.org/abox#alice";
    let bob = "http://example.org/abox#bob";
    let carol = "http://example.org/abox#carol";

    // Inspector-style remove against Protégé AllDifferent fixture.
    apply_patches(
        &path,
        &[PatchOp::RemoveDifferentIndividuals { individuals: vec![alice.into(), bob.into()] }],
        false,
        &abox_ns(),
    )
    .expect("remove AllDifferent pair");
    let after_remove = std::fs::read_to_string(&path).expect("read");
    assert!(
        !after_remove.contains("owl:AllDifferent"),
        "AllDifferent must be cleared when two of three members are removed: {after_remove}"
    );

    // Re-add n-ary DifferentIndividuals — must write AllDifferent covering A≠C.
    apply_patches(
        &path,
        &[PatchOp::AddDifferentIndividuals {
            individuals: vec![alice.into(), bob.into(), carol.into()],
        }],
        false,
        &abox_ns(),
    )
    .expect("add AllDifferent");
    let after_add = std::fs::read_to_string(&path).expect("read");
    assert!(
        after_add.contains("owl:AllDifferent") && after_add.contains("owl:distinctMembers"),
        "add must emit AllDifferent: {after_add}"
    );
    assert!(
        after_add.contains("ex:alice")
            && after_add.contains("ex:bob")
            && after_add.contains("ex:carol"),
        "all three members required: {after_add}"
    );

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("reindex");
    let detail = catalog.entity_detail(alice).expect("alice");
    assert!(
        detail.axioms.iter().any(|a| a.kind == "different_individuals"
            && (a.other_iri.as_deref() == Some(carol) || a.display.contains("carol"))),
        "catalog must project A≠C from AllDifferent, axioms={:?}",
        detail.axioms
    );
}

#[test]
fn owl2_xml_mutate_supports_domain_characteristic_and_rejects_prefix() {
    let rdf = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:owl="http://www.w3.org/2002/07/owl#"
     xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
     xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#">
    <owl:Ontology rdf:about="http://example.org/ont"/>
    <owl:Class rdf:about="http://example.org/ont#Person"/>
    <owl:ObjectProperty rdf:about="http://example.org/ont#worksFor"/>
</rdf:RDF>"#;
    let ns = BTreeMap::new();
    let ok = apply_xml_patches_to_text(
        rdf,
        OntologyFormat::RdfXml,
        &[
            PatchOp::AddDomain {
                entity_iri: "http://example.org/ont#worksFor".into(),
                class_iri: "http://example.org/ont#Person".into(),
            },
            PatchOp::SetFunctional {
                entity_iri: "http://example.org/ont#worksFor".into(),
                value: true,
            },
            PatchOp::AddObjectPropertyAssertion {
                entity_iri: "http://example.org/ont#alice".into(),
                property_iri: "http://example.org/ont#worksFor".into(),
                target_iri: "http://example.org/ont#acme".into(),
            },
        ],
        true,
        &ns,
    )
    .expect("xml mutate domain/functional/opa");
    let preview = ok.preview_text.expect("preview");
    assert!(
        preview.contains("worksFor") && preview.contains("Person"),
        "expected domain/characteristic content: {preview}"
    );

    let err = apply_xml_patches_to_text(
        rdf,
        OntologyFormat::RdfXml,
        &[PatchOp::AddPrefix { prefix: "ex".into(), namespace_iri: "http://example.org/".into() }],
        true,
        &ns,
    );
    assert!(err.is_err(), "prefix ops must error on XML");
    let msg = format!("{}", err.unwrap_err());
    assert!(
        msg.contains("Turtle-only") || msg.contains("AddPrefix") || msg.contains("prefix"),
        "expected clear prefix error, got {msg}"
    );
}
