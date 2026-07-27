mod support;

use std::fs;

#[test]
fn patch_complex_subclass_manchester() {
    let workspace = support::fixture_workspace();
    let src = workspace.join("complex-classes.ttl");
    let tmp = tempfile::tempdir().expect("tmpdir");
    let dst = tmp.path().join("complex-classes.ttl");
    fs::copy(&src, &dst).expect("copy fixture");

    let before = fs::read_to_string(&dst).unwrap();

    let patch = serde_json::json!([{
        "op": "add_complex_sub_class_of",
        "entity_iri": "http://example.org/clinic#Staff",
        "manchester": "ex:worksFor some ex:ClinicOrganization"
    }]);

    let patch_file = tmp.path().join("patch.json");
    fs::write(&patch_file, serde_json::to_string(&patch).unwrap()).unwrap();

    let output = support::strixonomy_cmd()
        .args(["patch", dst.to_str().unwrap(), patch_file.to_str().unwrap()])
        .output()
        .expect("run patch");
    assert!(output.status.success(), "patch failed: {}", String::from_utf8_lossy(&output.stderr));

    let after = fs::read_to_string(&dst).unwrap();
    assert_ne!(before, after, "patch should modify file content");
    assert!(
        after.contains("worksFor") && after.contains("ClinicOrganization"),
        "expected new restriction in Turtle output"
    );

    let validate = support::strixonomy_cmd()
        .args(["validate", dst.to_str().unwrap()])
        .output()
        .expect("validate");
    assert!(
        validate.status.success(),
        "patched file should validate: {}",
        String::from_utf8_lossy(&validate.stderr)
    );
}
