//! Helpers for Protégé Wave 1 behavioral ports (`tests/protege_port_*.rs`).

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use strixonomy_catalog::{IndexBuilder, OntologyCatalog};
use strixonomy_owl::PatchOp;
use tempfile::TempDir;

/// Directory of synthetic Protégé-ported fixtures.
pub fn ported_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/protege-roundtrip/ported")
}

/// Copy a single file from `ported/` into a new tempfile workspace.
pub fn copy_ported_workspace(relative: &str) -> (TempDir, PathBuf) {
    let dir = TempDir::new().expect("tempdir");
    let src = ported_dir().join(relative);
    let file_name = src.file_name().expect("file name");
    let dest = dir.path().join(file_name);
    std::fs::copy(&src, &dest).unwrap_or_else(|e| panic!("copy {relative}: {e}"));
    (dir, dest)
}

/// Copy an entire subdirectory (e.g. `imports_home`) into a tempfile workspace.
pub fn copy_ported_tree(relative: &str) -> TempDir {
    let dir = TempDir::new().expect("tempdir");
    let src = ported_dir().join(relative);
    copy_dir_recursive(&src, dir.path()).expect("copy tree");
    dir
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&entry.path(), &to)?;
        } else {
            std::fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}

pub fn index_workspace(workspace: &Path) -> OntologyCatalog {
    IndexBuilder::new().workspace(workspace).build().expect("index workspace")
}

pub fn standard_ns() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("ex".to_string(), "http://example.org/tree#".to_string()),
        ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ("rdf".to_string(), "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()),
        ("xsd".to_string(), "http://www.w3.org/2001/XMLSchema#".to_string()),
    ])
}

pub fn apply_patches_reindex(
    workspace: &Path,
    path: &Path,
    patches: &[PatchOp],
    namespaces: &BTreeMap<String, String>,
) -> OntologyCatalog {
    let result = strixonomy_owl::apply_patches(path, patches, false, namespaces).expect("apply");
    assert!(result.applied, "expected patches applied; diagnostics={:?}", result.diagnostics);
    index_workspace(workspace)
}

pub fn assert_parent_of(catalog: &OntologyCatalog, child: &str, parent: &str) {
    let h = catalog.class_hierarchy();
    assert!(
        h.parents.get(child).is_some_and(|parents| parents.iter().any(|p| p == parent)),
        "expected {child} parent {parent}; parents={:?}",
        h.parents.get(child)
    );
}

pub fn assert_not_parent_of(catalog: &OntologyCatalog, child: &str, parent: &str) {
    let h = catalog.class_hierarchy();
    let still = h.parents.get(child).is_some_and(|parents| parents.iter().any(|p| p == parent));
    assert!(!still, "did not expect {child} parent {parent}");
}

pub fn property_parents(catalog: &OntologyCatalog, prop: &str, axiom_kind: &str) -> Vec<String> {
    catalog
        .data()
        .axioms
        .iter()
        .filter(|a| a.axiom_kind == axiom_kind && a.subject == prop)
        .map(|a| a.object.clone())
        .collect()
}

/// Parse tab-indented hierarchy lines into (child_local, parent_local) edges.
/// Root lines (no leading tabs) have no parent edge.
pub fn parse_tabbed_hierarchy(text: &str) -> Vec<(String, String)> {
    let mut stack: Vec<(usize, String)> = Vec::new();
    let mut edges = Vec::new();
    for raw in text.lines() {
        if raw.trim().is_empty() {
            continue;
        }
        let depth = raw.chars().take_while(|c| *c == '\t').count();
        let name = raw.trim().to_string();
        while stack.last().is_some_and(|(d, _)| *d >= depth) {
            stack.pop();
        }
        if let Some((_, parent)) = stack.last() {
            edges.push((name.clone(), parent.clone()));
        }
        stack.push((depth, name));
    }
    edges
}
