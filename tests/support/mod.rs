use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use strixonomy_catalog::{IndexBuilder, OntologyCatalog};
use strixonomy_owl::PatchOp;

pub mod protege_port;

pub fn fixture_workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

#[allow(dead_code)]
pub fn fixture_catalog() -> OntologyCatalog {
    IndexBuilder::new().workspace(fixture_workspace()).build().expect("index fixtures")
}

/// Write `ttl` to a temp workspace, apply patches to disk, reindex via Horned/`IndexBuilder`.
/// Empty `patches` writes the source unchanged and indexes (treats as applied).
#[allow(dead_code)]
pub fn apply_and_reindex(
    ttl: &str,
    patches: &[PatchOp],
    namespaces: &BTreeMap<String, String>,
) -> (tempfile::TempDir, PathBuf, OntologyCatalog) {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("patched.ttl");
    std::fs::write(&path, ttl).expect("write ttl");
    if patches.is_empty() {
        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("reindex");
        return (dir, path, catalog);
    }
    let catalog = reapply_and_reindex(dir.path(), &path, patches, namespaces);
    (dir, path, catalog)
}

/// Apply further patches on an existing workspace file and reindex.
#[allow(dead_code)]
pub fn reapply_and_reindex(
    workspace: &Path,
    path: &Path,
    patches: &[PatchOp],
    namespaces: &BTreeMap<String, String>,
) -> OntologyCatalog {
    let result = strixonomy_owl::apply_patches(path, patches, false, namespaces).expect("apply");
    assert!(result.applied, "expected patches applied; diagnostics={:?}", result.diagnostics);
    IndexBuilder::new().workspace(workspace).build().expect("reindex")
}

#[allow(dead_code)]
pub fn strixonomy_binary() -> PathBuf {
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_strixonomy") {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return candidate;
        }
    }

    let mut target_dirs = Vec::new();
    if let Ok(dir) = std::env::var("CARGO_TARGET_DIR") {
        target_dirs.push(PathBuf::from(dir));
    }
    target_dirs.push(Path::new(env!("CARGO_MANIFEST_DIR")).join("target"));

    for target_dir in &target_dirs {
        for subdir in ["debug", "release"] {
            let candidate = target_dir.join(subdir).join("strixonomy");
            if candidate.exists() {
                return candidate;
            }
        }
    }

    panic!(
        "strixonomy binary not found under {:?} (run `cargo build -p strixonomy-cli` first, or add strixonomy-cli as a dev-dependency)",
        target_dirs
    );
}

/// Spawn the prebuilt `strixonomy` CLI (avoids `cargo run` re-linking on every test invocation).
#[allow(dead_code)]
pub fn strixonomy_cmd() -> Command {
    Command::new(strixonomy_binary())
}
