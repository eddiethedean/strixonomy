mod support;

#[test]
fn cli_classify_el_json() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    std::fs::copy(
        support::fixture_workspace().join("reasoner-el.ttl"),
        workspace.join("reasoner-el.ttl"),
    )
    .expect("copy fixture");
    let output = support::strixonomy_cmd()
        .args(["classify", workspace.to_str().unwrap(), "--profile", "el", "--format", "json"])
        .output()
        .expect("run classify");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).expect("json output");
    assert_eq!(json.get("profile_used").and_then(|v| v.as_str()), Some("el"));
    assert_eq!(json.get("consistent").and_then(|v| v.as_bool()), Some(true));
    let edges = json
        .pointer("/inferred/combined/edges")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    assert!(
        edges.iter().any(|e| {
            e.get("child").and_then(|c| c.as_str()).is_some_and(|c| c.ends_with("#Dog"))
                && e.get("parent").and_then(|p| p.as_str()).is_some_and(|p| p.ends_with("#Mammal"))
        }),
        "expected Dog ⊑ Mammal: {edges:?}"
    );
}

#[test]
fn cli_dl_profile_classifies_json() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    std::fs::copy(
        support::fixture_workspace().join("reasoner-el.ttl"),
        workspace.join("reasoner-el.ttl"),
    )
    .expect("copy fixture");
    let output = support::strixonomy_cmd()
        .args(["classify", workspace.to_str().unwrap(), "--profile", "dl", "--format", "json"])
        .output()
        .expect("run classify");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).expect("json output");
    assert_eq!(json.get("profile_used").and_then(|v| v.as_str()), Some("dl"));
    assert_eq!(json.get("consistent").and_then(|v| v.as_bool()), Some(true));
}

#[test]
fn cli_auto_profile_classifies_json() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    std::fs::copy(
        support::fixture_workspace().join("reasoner-el.ttl"),
        workspace.join("reasoner-el.ttl"),
    )
    .expect("copy fixture");
    let output = support::strixonomy_cmd()
        .args(["classify", workspace.to_str().unwrap(), "--profile", "auto", "--format", "json"])
        .output()
        .expect("run classify");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).expect("json output");
    assert_eq!(
        json.get("profile_used").and_then(|v| v.as_str()),
        Some("el"),
        "Auto must report the concrete engine for an EL-only workspace"
    );
    assert_eq!(json.get("consistent").and_then(|v| v.as_bool()), Some(true));
    let edges = json
        .pointer("/inferred/combined/edges")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    assert!(
        edges.iter().any(|e| {
            e.get("child").and_then(|c| c.as_str()).is_some_and(|c| c.ends_with("#Dog"))
                && e.get("parent").and_then(|p| p.as_str()).is_some_and(|p| p.ends_with("#Mammal"))
        }),
        "expected Dog ⊑ Mammal in combined edges: {edges:?}"
    );
}
