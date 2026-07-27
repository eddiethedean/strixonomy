mod support;

#[test]
fn new_rejects_adversarial_ontology_iri() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("evil.ttl");
    let evil = "http://ex.org/x> a owl:Class . <http://ex.org/y";
    let output = support::strixonomy_cmd()
        .args(["new", path.to_str().expect("path"), "--ontology-iri", evil])
        .output()
        .expect("spawn strixonomy new");

    assert!(
        !output.status.success(),
        "expected failure for unsafe IRI, stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cannot be safely written") || stderr.contains("ontology IRI"),
        "expected safety error, got: {stderr}"
    );
    assert!(!path.exists(), "unsafe create must not write a file");
}

#[test]
fn new_refuses_overwrite_without_force() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("existing.ttl");
    std::fs::write(&path, "KEEP\n").unwrap();

    let output = support::strixonomy_cmd()
        .args(["new", path.to_str().expect("path"), "--ontology-iri", "http://example.org/ont"])
        .output()
        .expect("spawn strixonomy new");

    assert!(
        !output.status.success(),
        "expected failure when file exists, stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("already exists") || stderr.contains("--force"),
        "expected overwrite guard, got: {stderr}"
    );
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "KEEP\n");
}

#[test]
fn new_force_overwrites_existing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("existing.ttl");
    std::fs::write(&path, "KEEP\n").unwrap();

    let output = support::strixonomy_cmd()
        .args([
            "new",
            path.to_str().expect("path"),
            "--ontology-iri",
            "http://example.org/ont",
            "--force",
        ])
        .output()
        .expect("spawn strixonomy new");

    assert!(
        output.status.success(),
        "expected success with --force, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let contents = std::fs::read_to_string(&path).unwrap();
    assert!(contents.contains("<http://example.org/ont> a owl:Ontology"));
    assert!(!contents.contains("KEEP"));
}

#[test]
fn new_writes_safe_turtle() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("fresh.ttl");
    let output = support::strixonomy_cmd()
        .args([
            "new",
            path.to_str().expect("path"),
            "--ontology-iri",
            "http://example.org/ont",
            "--version-iri",
            "http://example.org/ont/1",
        ])
        .output()
        .expect("spawn strixonomy new");

    assert!(
        output.status.success(),
        "expected success, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let contents = std::fs::read_to_string(&path).unwrap();
    assert!(contents.contains("<http://example.org/ont> a owl:Ontology"));
    assert!(contents.contains("owl:versionIRI <http://example.org/ont/1>"));
}
