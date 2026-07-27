//! Protégé Wave 1 port: entity deprecation (+ replaced-by / reason annotations).
//! Upstream: EntityDeprecator_TestCase, GOProfile_TestCase, OBIProfile_TestCase.

mod support;

use std::collections::BTreeMap;
use strixonomy_owl::PatchOp;
use support::protege_port::standard_ns;

fn ns_dep() -> BTreeMap<String, String> {
    let mut ns = standard_ns();
    ns.insert("ex".to_string(), "http://example.org/dep#".to_string());
    ns.insert("IAO".to_string(), "http://purl.obolibrary.org/obo/IAO_".to_string());
    ns
}

#[test]
fn deprecation_sets_owl_deprecated_flag() {
    let ttl = r#"
@prefix ex: <http://example.org/dep#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Ontology a owl:Ontology .
ex:Old a owl:Class .
"#;
    let person = "http://example.org/dep#Old";
    let (_dir, _path, catalog) = support::apply_and_reindex(
        ttl,
        &[PatchOp::SetDeprecated { entity_iri: person.to_string(), value: true }],
        &ns_dep(),
    );
    let e = catalog.find_entity(person).expect("entity");
    assert!(e.deprecated, "expected owl:deprecated true");
}

#[test]
fn deprecation_clears_flag() {
    let ttl = r#"
@prefix ex: <http://example.org/dep#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Ontology a owl:Ontology .
ex:Old a owl:Class ;
    owl:deprecated true .
"#;
    let person = "http://example.org/dep#Old";
    let (_dir, _path, catalog) = support::apply_and_reindex(
        ttl,
        &[PatchOp::SetDeprecated { entity_iri: person.to_string(), value: false }],
        &ns_dep(),
    );
    let e = catalog.find_entity(person).expect("entity");
    assert!(!e.deprecated, "expected deprecated cleared");
}

#[test]
fn deprecation_with_replaced_by_and_reason_annotations() {
    let ttl = r#"
@prefix ex: <http://example.org/dep#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix IAO: <http://purl.obolibrary.org/obo/IAO_> .

ex:Ontology a owl:Ontology .
ex:Old a owl:Class .
ex:New a owl:Class .
"#;
    let old = "http://example.org/dep#Old";
    let new = "http://example.org/dep#New";
    let replaced_by = "http://purl.obolibrary.org/obo/IAO_0100001";

    let (_dir, path, catalog) = support::apply_and_reindex(
        ttl,
        &[
            PatchOp::SetDeprecated { entity_iri: old.to_string(), value: true },
            PatchOp::AddAnnotation {
                entity_iri: old.to_string(),
                predicate: replaced_by.to_string(),
                value: format!("<{new}>"),
            },
            PatchOp::AddAnnotation {
                entity_iri: old.to_string(),
                predicate: "http://www.w3.org/2000/01/rdf-schema#comment".to_string(),
                value: "obsolete: use New".to_string(),
            },
        ],
        &ns_dep(),
    );

    let e = catalog.find_entity(old).expect("old");
    assert!(e.deprecated);
    assert!(
        e.comments.iter().any(|c| c.contains("obsolete")),
        "expected reason comment, got {:?}",
        e.comments
    );

    let detail = catalog.entity_detail(old).expect("detail");
    let in_detail = detail.annotations.iter().any(|a| {
        a.predicate.contains("IAO_0100001") && (a.value == new || a.value.contains("New"))
    });
    let in_raw = catalog.data().annotations.iter().any(|a| {
        a.subject == old
            && a.predicate.contains("IAO_0100001")
            && (a.object == new || a.object.contains("New"))
    });
    let text = std::fs::read_to_string(&path).expect("read");
    assert!(
        in_detail || in_raw || text.contains("IAO_0100001") || text.contains("IAO:0100001"),
        "expected replaced-by annotation; detail={:?} raw={:?} text={text}",
        detail.annotations,
        catalog.data().annotations.iter().filter(|a| a.subject == old).collect::<Vec<_>>()
    );
}
