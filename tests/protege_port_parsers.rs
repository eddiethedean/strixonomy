//! Protégé Wave 1 port: literal/OBO/ontology-ID extract (+ idranges fixture load).
//! Upstream: OWLLiteralParser_TestCase, OboUtilities_TestCase, RDFExtractorTest,
//! IdPolicyParser_*_TestCase.

mod support;

use std::collections::BTreeMap;
use strixonomy_owl::PatchOp;
use support::protege_port::{copy_ported_workspace, index_workspace, standard_ns};

#[test]
fn parsers_ontology_iri_from_turtle_versioned_fixture() {
    let (dir, _) = copy_ported_workspace("versioned_ontology.ttl");
    let catalog = index_workspace(dir.path());
    let doc = catalog
        .data()
        .documents
        .iter()
        .find(|d| d.path.ends_with("versioned_ontology.ttl"))
        .expect("doc");
    let iri = doc.base_iri.clone().unwrap_or_else(|| doc.id.clone());
    assert!(
        iri.contains("example.org/versioned"),
        "expected versioned ontology IRI, got {iri} (id={})",
        doc.id
    );
    assert_eq!(
        doc.version_iri.as_deref(),
        Some("http://example.org/versioned/1.0.0"),
        "version IRI should be indexed on OntologyDocument"
    );
}

#[test]
fn parsers_ontology_iri_from_rdfxml_versioned_fixture() {
    let (dir, _) = copy_ported_workspace("versioned_ontology.owl");
    let catalog = index_workspace(dir.path());
    let doc = catalog
        .data()
        .documents
        .iter()
        .find(|d| d.path.ends_with("versioned_ontology.owl"))
        .expect("doc");
    let iri = doc.base_iri.clone().unwrap_or_else(|| doc.id.clone());
    assert!(iri.contains("example.org/versioned"), "expected ontology IRI from RDF/XML, got {iri}");
    assert!(
        catalog.find_entity("http://example.org/versioned#Thing").is_some(),
        "Thing class should index from RDF/XML"
    );
}

#[test]
fn parsers_typed_literal_via_data_property_assertion() {
    let ttl = r#"
@prefix ex: <http://example.org/tree#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ex:Ontology a owl:Ontology .
ex:age a owl:DatatypeProperty .
ex:alice a owl:NamedIndividual .
"#;
    let (_dir, _path, catalog) = support::apply_and_reindex(
        ttl,
        &[PatchOp::AddDataPropertyAssertion {
            entity_iri: "http://example.org/tree#alice".to_string(),
            property_iri: "http://example.org/tree#age".to_string(),
            value: "\"42\"^^xsd:integer".to_string(),
        }],
        &standard_ns(),
    );
    let hit = catalog.data().axioms.iter().any(|a| {
        a.subject.contains("alice") && (a.object.contains("42") || a.predicate.contains("age"))
    }) || catalog
        .data()
        .annotations
        .iter()
        .any(|a| a.subject.contains("alice") && a.object.contains("42"));
    assert!(
        hit,
        "expected typed literal 42 indexed; axioms={:?} annotations={:?}",
        catalog.data().axioms,
        catalog.data().annotations
    );
}

#[test]
fn parsers_obo_id_roundtrip_via_obo_document() {
    let obo = "\
format-version: 1.2
ontology: ex

[Term]
id: EX:0001
name: Sample term
";
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("sample.obo");
    std::fs::write(&path, obo).unwrap();
    let catalog = index_workspace(dir.path());
    let entity = catalog
        .data()
        .entities
        .iter()
        .find(|e| {
            e.obo_id.as_deref() == Some("EX:0001")
                || e.iri.contains("EX_0001")
                || e.iri.contains("EX:0001")
        })
        .expect("EX:0001 entity");
    assert_eq!(entity.obo_id.as_deref(), Some("EX:0001"));
    assert!(
        entity.iri.contains("EX_0001") || entity.iri.contains("EX:0001"),
        "IRI should encode OBO id, got {}",
        entity.iri
    );
}

#[test]
fn parsers_ambiguous_name_ontology_iri_not_file_base() {
    let (dir, _) = copy_ported_workspace("ambiguous_name.owl");
    let catalog = index_workspace(dir.path());
    let doc = catalog
        .data()
        .documents
        .iter()
        .find(|d| d.path.ends_with("ambiguous_name.owl"))
        .expect("doc");
    let iri = doc.base_iri.clone().unwrap_or_else(|| doc.id.clone());
    assert!(
        iri.contains("right.owl") || iri.contains("example.org/right"),
        "ontology IRI should be the declared rdf:about, not the Ambiguous file base; got {iri}"
    );
    assert!(
        !iri.contains("Ambiguous"),
        "must not prefer Ambiguous xml:base over ontology IRI; got {iri}"
    );
}

#[test]
fn parsers_idranges_minimal_fixture_loads() {
    let (dir, _) = copy_ported_workspace("idranges_minimal.ttl");
    let catalog = index_workspace(dir.path());
    assert!(!catalog.data().documents.is_empty(), "idranges fixture should parse as a document");
    let doc = &catalog.data().documents[0];
    assert_eq!(
        doc.parse_status,
        strixonomy_core::ParseStatus::Ok,
        "expected successful parse; message={:?}",
        doc.parse_message
    );
}

#[test]
fn parsers_set_version_iri_patch() {
    let ttl = r#"
@prefix owl: <http://www.w3.org/2002/07/owl#> .
<http://example.org/v> a owl:Ontology .
"#;
    let ns: BTreeMap<String, String> =
        BTreeMap::from([("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string())]);
    let (dir, path, _) = support::apply_and_reindex(
        ttl,
        &[PatchOp::SetVersionIri {
            ontology_iri: "http://example.org/v".to_string(),
            version_iri: "http://example.org/v/2.0".to_string(),
        }],
        &ns,
    );
    let text = std::fs::read_to_string(&path).expect("read");
    assert!(
        text.contains("http://example.org/v/2.0"),
        "version IRI should appear after SetVersionIri: {text}"
    );
    let _ = dir;
}
