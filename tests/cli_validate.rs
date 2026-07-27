mod support;

#[test]
fn validate_exits_zero_on_clean_fixtures() {
    let fixtures = support::fixture_workspace();
    let output = support::strixonomy_cmd()
        .args(["validate", fixtures.to_str().expect("fixture path")])
        .output()
        .expect("spawn strixonomy validate");

    assert!(
        output.status.success(),
        "expected exit 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OK:"), "expected success message, got: {stdout}");
}

#[test]
fn validate_exits_zero_when_only_warnings() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::copy(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/diagnostics/lint-orphan.ttl"),
        dir.path().join("orphan.ttl"),
    )
    .unwrap();

    let output = support::strixonomy_cmd()
        .args(["validate", dir.path().to_str().expect("temp path")])
        .output()
        .expect("spawn strixonomy validate");

    assert!(
        output.status.success(),
        "expected exit 0 for warnings-only workspace, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("WARN")
            || stderr.contains("orphan_class")
            || stderr.contains("missing_label"),
        "expected warning diagnostics, got: {stderr}"
    );
}

#[test]
fn validate_exits_nonzero_on_diagnostic_errors() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::copy(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/diagnostics/lint-broken-import.ttl"),
        dir.path().join("bad.ttl"),
    )
    .unwrap();

    let output = support::strixonomy_cmd()
        .args(["validate", dir.path().to_str().expect("temp path")])
        .output()
        .expect("spawn strixonomy validate");

    assert!(!output.status.success(), "expected non-zero exit on broken import fixture");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("broken_import"), "expected broken_import in stderr, got: {stderr}");
}
