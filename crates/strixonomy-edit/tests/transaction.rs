use strixonomy_edit::{SemanticChange, Transaction};
use strixonomy_owl::PatchOp;

const TURTLE_FIXTURE: &str = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:A a owl:Class .
"#;

#[test]
fn empty_transaction_fails_validation() {
    let txn = Transaction::default();
    assert!(txn.validate().is_err());
}

#[test]
fn compose_same_format() {
    let a = Transaction::from_turtle(vec![PatchOp::AddLabel {
        entity_iri: "http://example.org/A".into(),
        value: "One".into(),
    }]);
    let b = Transaction::from_turtle(vec![PatchOp::AddComment {
        entity_iri: "http://example.org/A".into(),
        value: "note".into(),
    }]);
    let merged = a.compose(b).expect("compose");
    assert_eq!(merged.changes.len(), 2);
}

#[test]
fn compose_mixed_format_fails() {
    let turtle = Transaction::from_turtle(vec![PatchOp::SetLabel {
        entity_iri: "http://example.org/A".into(),
        value: "A".into(),
    }]);
    let obo = Transaction::from_obo(vec![strixonomy_obo::OboPatchOp::SetName {
        term_id: "EX:0000001".into(),
        value: "term".into(),
    }]);
    assert!(turtle.compose(obo).is_err());
}

#[test]
fn invert_add_remove_label_roundtrip_text() {
    let txn = Transaction::from_turtle(vec![PatchOp::AddLabel {
        entity_iri: "http://example.org/A".into(),
        value: "Example".into(),
    }]);
    let namespaces = std::collections::BTreeMap::new();
    let applied = txn.apply_to_text(TURTLE_FIXTURE, false, &namespaces).expect("apply");
    let text = applied.preview_text.expect("preview");
    assert!(text.contains("rdfs:label"));

    let undo = txn.invert().expect("invert");
    let restored = undo.apply_to_text(&text, false, &namespaces).expect("undo apply");
    let restored_text = restored.preview_text.expect("restored");
    assert!(!restored_text.contains("rdfs:label"));
}

#[test]
fn transaction_json_roundtrip() {
    let txn = Transaction::from_turtle(vec![PatchOp::SetDeprecated {
        entity_iri: "http://example.org/A".into(),
        value: true,
    }]);
    let json = serde_json::to_string(&txn).expect("serialize");
    let parsed: Transaction = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(txn, parsed);
}

#[test]
fn legacy_patch_array_parses_as_transaction() {
    let value = serde_json::json!([{
        "op": "add_label",
        "entity_iri": "http://example.org/A",
        "value": "X"
    }]);
    let txn = strixonomy_edit::parse_turtle_input(value).expect("parse");
    assert_eq!(txn.changes.len(), 1);
    assert!(matches!(txn.changes[0], SemanticChange::Turtle { .. }));
}

#[test]
fn turtle_envelope_with_raw_ops_parses() {
    let value = serde_json::json!({
        "transaction": {
            "changes": [{
                "op": "add_label",
                "entity_iri": "http://example.org/A",
                "value": "X"
            }]
        }
    });
    let txn = strixonomy_edit::parse_turtle_input(value).expect("parse");
    assert_eq!(txn.changes.len(), 1);
    assert!(matches!(txn.changes[0], SemanticChange::Turtle { .. }));
}

#[test]
fn turtle_path_rejects_obo_envelope() {
    let value = serde_json::json!({
        "transaction": {
            "changes": [{
                "format": "obo",
                "change": {
                    "op": "set_name",
                    "term_id": "EX:1",
                    "value": "x"
                }
            }]
        }
    });
    let err =
        strixonomy_edit::parse_turtle_input(value).expect_err("must reject OBO on turtle path");
    assert!(err.to_string().contains("OBO") || err.to_string().contains("Turtle"));
}

#[test]
fn obo_path_rejects_turtle_envelope() {
    let value = serde_json::json!({
        "transaction": {
            "changes": [{
                "format": "turtle",
                "change": {
                    "op": "add_label",
                    "entity_iri": "http://example.org/A",
                    "value": "X"
                }
            }]
        }
    });
    let err = strixonomy_edit::parse_obo_input(value).expect_err("must reject Turtle on obo path");
    assert!(err.to_string().contains("Turtle") || err.to_string().contains("OBO"));
}

#[test]
fn adapter_matches_direct_patch_apply() {
    let patches = vec![PatchOp::AddComment {
        entity_iri: "http://example.org/A".into(),
        value: "hello".into(),
    }];
    let namespaces = std::collections::BTreeMap::new();
    let direct =
        strixonomy_owl::apply_patches_to_text(TURTLE_FIXTURE, &patches, true, &namespaces).unwrap();
    let via_txn =
        Transaction::from_turtle(patches).apply_to_text(TURTLE_FIXTURE, true, &namespaces).unwrap();
    assert_eq!(direct.preview_text, via_txn.preview_text);
    assert_eq!(direct.applied, via_txn.applied);
}
