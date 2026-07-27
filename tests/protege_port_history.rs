//! Protégé Wave 1 port: history / change-list algebra via semantic transactions.
//! Upstream: HistoryTest, ChangeListMinimizer_TestCase.

mod support;

use strixonomy_edit::{invert_patch_op, Transaction};
use strixonomy_owl::PatchOp;
use support::protege_port::{assert_not_parent_of, assert_parent_of, standard_ns};

#[test]
fn history_invert_add_subclass_restores_text() {
    let ttl = r#"
@prefix ex: <http://example.org/tree#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ex:Ontology a owl:Ontology .
ex:A a owl:Class .
ex:B a owl:Class .
"#;
    let namespaces = standard_ns();
    let add = PatchOp::AddSubClassOf {
        entity_iri: "http://example.org/tree#B".to_string(),
        parent_iri: "http://example.org/tree#A".to_string(),
    };
    let txn = Transaction::from_turtle(vec![add.clone()]);
    let applied = txn.apply_to_text(ttl, false, &namespaces).expect("apply add");
    let text = applied.preview_text.expect("preview");
    assert!(
        text.contains("subClassOf") || text.contains("rdfs:subClassOf"),
        "expected subclass after add: {text}"
    );

    let inv = invert_patch_op(&add).expect("invert AddSubClassOf");
    let undo = Transaction::from_turtle(vec![inv]);
    let restored = undo.apply_to_text(&text, false, &namespaces).expect("apply invert");
    let restored_text = restored.preview_text.expect("restored");
    let (_dir, _path, catalog) = support::apply_and_reindex(&restored_text, &[], &namespaces);
    assert_not_parent_of(&catalog, "http://example.org/tree#B", "http://example.org/tree#A");
}

#[test]
fn history_compose_add_then_remove_cancels() {
    let ttl = r#"
@prefix ex: <http://example.org/tree#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ex:Ontology a owl:Ontology .
ex:A a owl:Class .
ex:B a owl:Class .
"#;
    let namespaces = standard_ns();
    let add = PatchOp::AddSubClassOf {
        entity_iri: "http://example.org/tree#B".to_string(),
        parent_iri: "http://example.org/tree#A".to_string(),
    };
    let remove = PatchOp::RemoveSubClassOf {
        entity_iri: "http://example.org/tree#B".to_string(),
        parent_iri: "http://example.org/tree#A".to_string(),
    };
    let composed = Transaction::from_turtle(vec![add])
        .compose(Transaction::from_turtle(vec![remove]))
        .expect("compose");
    let out = composed.apply_to_text(ttl, false, &namespaces).expect("apply composed");
    let text = out.preview_text.expect("preview");
    let (_dir, _path, catalog) = support::apply_and_reindex(&text, &[], &namespaces);
    assert_not_parent_of(&catalog, "http://example.org/tree#B", "http://example.org/tree#A");
    assert!(catalog.find_entity("http://example.org/tree#A").is_some());
    assert!(catalog.find_entity("http://example.org/tree#B").is_some());
}

#[test]
fn history_undo_redo_roundtrip_on_catalog() {
    let ttl = r#"
@prefix ex: <http://example.org/tree#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ex:Ontology a owl:Ontology .
ex:A a owl:Class .
ex:B a owl:Class .
"#;
    let ns = standard_ns();
    let add = [PatchOp::AddSubClassOf {
        entity_iri: "http://example.org/tree#B".to_string(),
        parent_iri: "http://example.org/tree#A".to_string(),
    }];
    let (dir, path, catalog) = support::apply_and_reindex(ttl, &add, &ns);
    assert_parent_of(&catalog, "http://example.org/tree#B", "http://example.org/tree#A");

    let undo = [PatchOp::RemoveSubClassOf {
        entity_iri: "http://example.org/tree#B".to_string(),
        parent_iri: "http://example.org/tree#A".to_string(),
    }];
    let catalog = support::reapply_and_reindex(dir.path(), &path, &undo, &ns);
    assert_not_parent_of(&catalog, "http://example.org/tree#B", "http://example.org/tree#A");

    let catalog = support::reapply_and_reindex(dir.path(), &path, &add, &ns);
    assert_parent_of(&catalog, "http://example.org/tree#B", "http://example.org/tree#A");
}
