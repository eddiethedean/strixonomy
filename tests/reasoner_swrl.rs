//! SWRL classify honesty: profile skip warnings and non-injectable atom skips.

use std::path::PathBuf;
use strixonomy_reasoner::{classify, inject_swrl_from_turtle, ReasonerId, WorkspaceInputLoader};

fn load_swrl_fixture() -> strixonomy_reasoner::ReasonerInput {
    let dir = tempfile::tempdir().expect("tempdir");
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/swrl/basic_class_property.ttl");
    let name = src.file_name().unwrap();
    std::fs::copy(&src, dir.path().join(name)).expect("copy");
    WorkspaceInputLoader::new(dir.path()).load().expect("load")
}

#[test]
fn el_profile_warns_when_swrl_present() {
    let input = load_swrl_fixture();
    assert!(strixonomy_reasoner::input_has_swrl_rules(&input), "fixture should contain SWRL");
    let result = classify(ReasonerId::El, &input, false).expect("classify");
    assert!(
        result.warnings.iter().any(|w| w.code == "swrl_skipped_for_profile"),
        "expected swrl_skipped_for_profile: {:?}",
        result.warnings
    );
    assert_eq!(
        result
            .warnings
            .iter()
            .find(|w| w.code == "swrl_skipped_for_profile")
            .and_then(|w| w.suggested_profile.as_deref()),
        Some("dl")
    );
}

#[test]
fn inject_skips_builtin_rules_with_warning() {
    // Start from a loaded ontology so we have Ontologos Ontology without linking ontologos-core here.
    let input = load_swrl_fixture();
    let mut ontology = input.ontology;
    let rule = serde_json::json!({
        "id": "with-builtin",
        "enabled": true,
        "body": [
            {"kind": "class", "class": "http://example.org/swrl#Person", "arg": {"variable": "x"}},
            {
                "kind": "built_in",
                "predicate": "http://www.w3.org/2003/11/swrlb#equal",
                "args": [{"variable": "x"}, {"variable": "y"}]
            }
        ],
        "head": [
            {"kind": "class", "class": "http://example.org/swrl#Human", "arg": {"variable": "x"}}
        ]
    });
    let compact = serde_json::to_string(&rule).unwrap();
    let escaped = compact.replace('\\', "\\\\").replace('"', "\\\"");
    let text = format!(
        r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
<http://example.org/swrl> a owl:Ontology .
<http://example.org/swrl> <http://ontocode.dev/ns#swrlRule> "{escaped}" .
"#
    );

    let before = ontology.swrl_rules().len();
    let (injected, warnings) = inject_swrl_from_turtle(&mut ontology, &text);
    assert_eq!(injected, 0, "BuiltIn rules must not inject");
    assert_eq!(ontology.swrl_rules().len(), before, "store must not grow on skipped BuiltIn rule");
    assert!(
        warnings.iter().any(|w| w.code == "swrl_rule_skipped"),
        "expected swrl_rule_skipped: {warnings:?}"
    );
}
