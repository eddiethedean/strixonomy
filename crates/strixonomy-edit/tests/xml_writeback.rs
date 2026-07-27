//! Adapter smoke for RDF/XML write-back via Transaction.

use std::collections::BTreeMap;
use std::path::PathBuf;
use strixonomy_edit::{apply_transaction_to_text_as, EditFormat, Transaction};
use strixonomy_owl::PatchOp;

fn fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/protege-roundtrip")
        .join(name);
    std::fs::read_to_string(path).expect("fixture")
}

#[test]
fn rdf_xml_transaction_set_label() {
    let source = fixture("organization.owl");
    let txn = Transaction::from_turtle(vec![PatchOp::SetLabel {
        entity_iri: "http://example.org/org#Department".into(),
        value: "Department Via Txn".into(),
    }]);
    let result =
        apply_transaction_to_text_as(&txn, &source, true, &BTreeMap::new(), EditFormat::RdfXml)
            .expect("apply");
    let text = result.preview_text.expect("preview");
    assert!(text.contains("Department Via Txn"));
}

#[test]
fn owl_xml_transaction_create_class() {
    let source = fixture("example.owx");
    let txn = Transaction::from_turtle(vec![
        PatchOp::CreateEntity {
            entity_iri: "http://example.org/org#Team".into(),
            kind: strixonomy_owl::PatchEntityKind::Class,
        },
        PatchOp::SetLabel {
            entity_iri: "http://example.org/org#Team".into(),
            value: "Team".into(),
        },
    ]);
    let result =
        apply_transaction_to_text_as(&txn, &source, true, &BTreeMap::new(), EditFormat::OwlXml)
            .expect("apply");
    let text = result.preview_text.expect("preview");
    assert!(text.contains("Team") || text.contains("#Team"));
}

#[test]
fn rdf_xml_transaction_set_ontology_iri_keeps_version_iri_text() {
    let source = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:owl="http://www.w3.org/2002/07/owl#"
     xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
     xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#">
    <owl:Ontology rdf:about="http://example.org/ont">
        <owl:versionIRI rdf:resource="http://example.org/ont/1.0"/>
    </owl:Ontology>
</rdf:RDF>"#;
    let txn = Transaction::from_turtle(vec![PatchOp::SetOntologyIri {
        ontology_iri: "http://example.org/ont-renamed".into(),
    }]);
    let result =
        apply_transaction_to_text_as(&txn, source, true, &BTreeMap::new(), EditFormat::RdfXml)
            .expect("apply");
    let text = result.preview_text.expect("preview");
    assert!(text.contains("http://example.org/ont-renamed"));
    assert!(
        text.contains("http://example.org/ont/1.0"),
        "version IRI must survive SetOntologyIri (#303): {text}"
    );
}
