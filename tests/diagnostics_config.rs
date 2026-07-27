//! v0.13 configurable diagnostics via `.strixonomy/diagnostics.toml`.

mod support;

use std::path::PathBuf;

use strixonomy_catalog::IndexBuilder;
use strixonomy_core::DiagnosticCode;
use strixonomy_query::query_catalog;

fn diagnostics_fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/diagnostics")
}

#[test]
fn disabled_rule_filters_diagnostics_from_index() {
    let dir = tempfile::tempdir().unwrap();
    for name in ["lint-broken-import.ttl", "lint-orphan.ttl"] {
        std::fs::copy(diagnostics_fixture_dir().join(name), dir.path().join(name)).unwrap();
    }
    std::fs::create_dir_all(dir.path().join(".strixonomy")).unwrap();
    std::fs::write(
        dir.path().join(".strixonomy/diagnostics.toml"),
        r#"
[rules.broken_import]
enabled = false
"#,
    )
    .unwrap();

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("index");
    let codes: Vec<_> = catalog.data().diagnostics.iter().map(|d| d.code).collect();
    assert!(
        !codes.contains(&DiagnosticCode::BrokenImport),
        "broken_import should be filtered when disabled, got: {codes:?}"
    );
    assert!(
        codes.contains(&DiagnosticCode::OrphanClass),
        "other rules should still run, got: {codes:?}"
    );
}

#[test]
fn severity_override_promotes_missing_label_to_error() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::copy(
        diagnostics_fixture_dir().join("lint-duplicate-labels.ttl"),
        dir.path().join("dup.ttl"),
    )
    .unwrap();
    std::fs::create_dir_all(dir.path().join(".strixonomy")).unwrap();
    std::fs::write(
        dir.path().join(".strixonomy/diagnostics.toml"),
        r#"
[rules.missing_label]
severity = "error"
"#,
    )
    .unwrap();

    let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("index");
    let missing: Vec<_> = catalog
        .data()
        .diagnostics
        .iter()
        .filter(|d| d.code == DiagnosticCode::MissingLabel)
        .collect();
    assert!(!missing.is_empty(), "expected missing_label diagnostics");
    assert!(
        missing.iter().all(|d| d.severity == strixonomy_core::DiagnosticSeverity::Error),
        "missing_label should be promoted to error"
    );

    let result = query_catalog(
        &catalog,
        "SELECT code, severity FROM diagnostics WHERE code = 'missing_label'",
    )
    .expect("diagnostics sql");
    assert!(
        result.rows.iter().all(|r| r.get("severity") == Some(&"error".to_string())),
        "sql diagnostics table should reflect severity override: {:?}",
        result.rows
    );
}
