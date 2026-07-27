//! SWRL AddSwrlRule / RemoveSwrlRule / ReplaceSwrlRule patch round-trip on a temp Turtle file.

use std::collections::BTreeMap;
use std::fs;
use strixonomy_owl::{apply_patches_to_text, PatchOp};
use strixonomy_swrl::rules_from_turtle_document;
use tempfile::tempdir;

fn sample_rule_json(id: &str) -> String {
    format!(
        r#"{{"id":"{id}","body":[{{"kind":"class","class":"http://example.org/swrl#Person","arg":{{"variable":"x"}}}}],"head":[{{"kind":"class","class":"http://example.org/swrl#Human","arg":{{"variable":"x"}}}}],"enabled":true}}"#
    )
}

#[test]
fn add_and_remove_swrl_rule_on_temp_ttl() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("swrl.ttl");
    let ontology_iri = "http://example.org/swrl";
    let initial = format!(
        r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix ex: <http://example.org/swrl#> .

<{ontology_iri}> a owl:Ontology .

ex:Person a owl:Class .
ex:Human a owl:Class .
"#
    );
    fs::write(&path, &initial).expect("write");

    let rule_json = sample_rule_json("r1");

    let ns = BTreeMap::new();
    let source = fs::read_to_string(&path).expect("read");
    let added = apply_patches_to_text(
        &source,
        &[PatchOp::AddSwrlRule {
            ontology_iri: ontology_iri.to_string(),
            rule_json: rule_json.clone(),
        }],
        false,
        &ns,
    )
    .expect("add swrl");
    assert!(added.applied);
    let after_add = added.preview_text.expect("patched text");
    fs::write(&path, &after_add).expect("write added");

    let rules = rules_from_turtle_document(&after_add);
    assert_eq!(rules.len(), 1, "rule should be present after AddSwrlRule");
    assert_eq!(rules[0].id.as_deref(), Some("r1"));

    // Idempotent add must not duplicate (no text change → no preview).
    let again = apply_patches_to_text(
        &after_add,
        &[PatchOp::AddSwrlRule {
            ontology_iri: ontology_iri.to_string(),
            rule_json: rule_json.clone(),
        }],
        false,
        &ns,
    )
    .expect("idempotent add");
    assert!(!again.applied, "second add should be a no-op");
    let after_again = again.preview_text.as_deref().unwrap_or(&after_add);
    assert_eq!(
        rules_from_turtle_document(after_again).len(),
        1,
        "duplicate add must be idempotent"
    );

    let removed = apply_patches_to_text(
        &after_add,
        &[PatchOp::RemoveSwrlRule {
            ontology_iri: ontology_iri.to_string(),
            rule_json: rule_json.clone(),
        }],
        false,
        &ns,
    )
    .expect("remove swrl");
    assert!(removed.applied);
    let after_remove = removed.preview_text.expect("removed text");
    let rules_after = rules_from_turtle_document(&after_remove);
    assert!(rules_after.is_empty(), "rule should be gone after RemoveSwrlRule");
}

#[test]
fn replace_swrl_rule_round_trip() {
    let ontology_iri = "http://example.org/swrl";
    let initial = format!(
        r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix ex: <http://example.org/swrl#> .

<{ontology_iri}> a owl:Ontology .

ex:Person a owl:Class .
ex:Human a owl:Class .
ex:Agent a owl:Class .
"#
    );
    let old = sample_rule_json("r1");
    let new = sample_rule_json("r2").replace("Human", "Agent");

    let ns = BTreeMap::new();
    let added = apply_patches_to_text(
        &initial,
        &[PatchOp::AddSwrlRule { ontology_iri: ontology_iri.to_string(), rule_json: old.clone() }],
        false,
        &ns,
    )
    .expect("add");
    let after_add = added.preview_text.expect("text");

    let replaced = apply_patches_to_text(
        &after_add,
        &[PatchOp::ReplaceSwrlRule {
            ontology_iri: ontology_iri.to_string(),
            old_rule_json: old,
            new_rule_json: new,
        }],
        false,
        &ns,
    )
    .expect("replace");
    let after = replaced.preview_text.expect("text");
    let rules = rules_from_turtle_document(&after);
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].id.as_deref(), Some("r2"));
}

#[test]
fn add_swrl_rejects_unsafe_ontology_iri() {
    let ns = BTreeMap::new();
    let bad = apply_patches_to_text(
        "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
        &[PatchOp::AddSwrlRule {
            ontology_iri: "http://ex.com/a> . <http://evil#x> a owl:Class".into(),
            rule_json: sample_rule_json("r1"),
        }],
        false,
        &ns,
    )
    .expect("returns soft failure");
    assert!(!bad.applied, "IRI injection must be rejected");
    assert!(
        bad.diagnostics.iter().any(|d| d.severity == "error"),
        "expected error diagnostic, got {:?}",
        bad.diagnostics
    );
}

#[test]
fn add_swrl_does_not_false_positive_on_payload_elsewhere() {
    let ontology_iri = "http://example.org/swrl";
    let rule_json = sample_rule_json("r1");
    let compact =
        serde_json::to_string(&serde_json::from_str::<serde_json::Value>(&rule_json).unwrap())
            .unwrap();
    let escaped = compact.replace('\\', "\\\\").replace('"', "\\\"");
    // Old check matched `text.contains(&escaped)` anywhere — a comment must not block add.
    let initial = format!(
        r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix ex: <http://example.org/swrl#> .

<{ontology_iri}> a owl:Ontology ;
  rdfs:comment "{escaped}" .

ex:Person a owl:Class .
ex:Human a owl:Class .
"#
    );
    let ns = BTreeMap::new();
    let added = apply_patches_to_text(
        &initial,
        &[PatchOp::AddSwrlRule { ontology_iri: ontology_iri.to_string(), rule_json }],
        false,
        &ns,
    )
    .expect("add");
    let text = added.preview_text.expect("text");
    assert_eq!(rules_from_turtle_document(&text).len(), 1);
}

#[test]
fn remove_swrl_matches_despite_json_key_order() {
    // #356 — Rule Browser emits struct field order; stored literal may differ.
    let ontology_iri = "http://example.org/swrl";
    let stored = r#"{"enabled":true,"head":[{"kind":"class","class":"http://example.org/swrl#Human","arg":{"variable":"x"}}],"body":[{"kind":"class","class":"http://example.org/swrl#Person","arg":{"variable":"x"}}],"id":"r1"}"#;
    let browser = r#"{"id":"r1","body":[{"kind":"class","class":"http://example.org/swrl#Person","arg":{"variable":"x"}}],"head":[{"kind":"class","class":"http://example.org/swrl#Human","arg":{"variable":"x"}}],"enabled":true}"#;
    let compact =
        serde_json::to_string(&serde_json::from_str::<serde_json::Value>(stored).unwrap()).unwrap();
    let escaped = compact.replace('\\', "\\\\").replace('"', "\\\"");
    let initial = format!(
        r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix ex: <http://example.org/swrl#> .

<{ontology_iri}> a owl:Ontology .
<{ontology_iri}> <http://ontocode.dev/ns#swrlRule> "{escaped}" .

ex:Person a owl:Class .
ex:Human a owl:Class .
"#
    );
    let ns = BTreeMap::new();
    let removed = apply_patches_to_text(
        &initial,
        &[PatchOp::RemoveSwrlRule {
            ontology_iri: ontology_iri.to_string(),
            rule_json: browser.to_string(),
        }],
        false,
        &ns,
    )
    .expect("remove with reordered keys");
    assert!(removed.applied, "semantic JSON match should remove the rule");
    let after = removed.preview_text.expect("text");
    assert!(
        rules_from_turtle_document(&after).is_empty(),
        "rule should be gone after semantic remove"
    );
}
