use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use strixonomy_catalog::IndexBuilder;
use strixonomy_lsp::catalog_snapshot_json;

pub fn fixture_workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

fn normalize_snapshot_paths(mut value: Value) -> Value {
    let obj = value.as_object_mut().expect("snapshot object");

    if let Some(docs) = obj.get_mut("documents").and_then(|d| d.as_array_mut()) {
        for doc in docs {
            if let Some(obj) = doc.as_object_mut() {
                if let Some(path) = obj.get_mut("path") {
                    *path = Value::String(relative_fixture_path(path.as_str().unwrap_or("")));
                }
                obj.remove("modified_time");
                obj.remove("content_hash");
            }
        }
    }

    if let Some(diags) = obj.get_mut("diagnostics").and_then(|d| d.as_array_mut()) {
        for diag in diags {
            if let Some(file) = diag.get_mut("file") {
                *file = Value::String(relative_fixture_path(file.as_str().unwrap_or("")));
            }
        }
    }

    value
}

fn relative_fixture_path(path: &str) -> String {
    let name = Path::new(path).file_name().and_then(|n| n.to_str()).unwrap_or(path);
    format!("fixtures/{name}")
}

pub fn extension_fixture_snapshot_json() -> String {
    let catalog =
        IndexBuilder::new().workspace(fixture_workspace()).build().expect("index fixtures");
    let value =
        normalize_snapshot_paths(catalog_snapshot_json(&catalog).expect("serialize snapshot"));
    serde_json::to_string_pretty(&value).expect("serialize snapshot")
}

#[test]
fn extension_fixture_catalog_snapshot_matches_committed_file() {
    let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("extension/src/test/fixture-catalog.snapshot.json");

    let actual = extension_fixture_snapshot_json();

    if std::env::var("STRIXONOMY_UPDATE_FIXTURE_SNAPSHOT").is_ok() {
        fs::write(&snapshot_path, format!("{actual}\n")).expect("write snapshot");
        return;
    }

    let expected = fs::read_to_string(&snapshot_path)
        .unwrap_or_else(|_| panic!("missing snapshot file: {}", snapshot_path.display()));
    assert_eq!(
        expected.trim_end(),
        actual,
        "extension fixture snapshot out of date; run STRIXONOMY_UPDATE_FIXTURE_SNAPSHOT=1 cargo test extension_fixture_catalog_snapshot_matches_committed_file"
    );
}
