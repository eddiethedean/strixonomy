//! OBO patch round-trip integration test.

use std::path::PathBuf;
use strixonomy_obo::{apply_patches_to_text, OboPatchOp};

fn obo_fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/obo-workflow/demo.obo")
}

#[test]
fn obo_patch_add_synonym_roundtrip() {
    let path = obo_fixture();
    assert!(path.exists(), "missing OBO fixture {}; test must not silently skip", path.display());
    let source = std::fs::read_to_string(&path).expect("read obo");
    let term_id = "DEMO:0000001";
    let result = apply_patches_to_text(
        &source,
        &[OboPatchOp::AddSynonym {
            term_id: term_id.to_string(),
            value: "test synonym".to_string(),
            scope: "EXACT".to_string(),
        }],
        true,
    )
    .expect("preview patch");
    let preview = result.preview_text.expect("preview text");
    assert!(
        preview.contains("synonym:") && preview.contains("test synonym"),
        "synonym must appear in patched OBO: {preview}"
    );
    let validated = fastobo::from_str(&preview);
    assert!(validated.is_ok(), "patched OBO validates: {validated:?}");

    // Semantic check: reparsed document still contains the term id.
    let doc = validated.expect("parse");
    let text = doc.to_string();
    assert!(text.contains("DEMO:0000001"));
    assert!(text.contains("test synonym"));
}
