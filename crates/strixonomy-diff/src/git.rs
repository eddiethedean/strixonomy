use crate::compare::{diff_catalogs, DiffError, Result};
use crate::model::DiffResult;
use git2::{ObjectType, Oid, Repository, TreeWalkMode};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use strixonomy_catalog::{IndexBuilder, OntologyCatalog};
use strixonomy_core::{
    discover_git_repo_root, ensure_extract_path_within, limits::MAX_FILE_BYTES,
    limits::MAX_SCAN_FILES, path_jail::path_has_parent_escape, WorkspaceScanner,
};

const ONTOLOGY_EXTENSIONS: &[&str] =
    &["ttl", "rdf", "owl", "jsonld", "json-ld", "nt", "nq", "trig", "obo"];

const CHECKOUT_CACHE_MAX: usize = 4;

#[derive(Debug, Clone)]
pub struct GitDiffSpec {
    pub repo_path: PathBuf,
    pub left_ref: String,
    pub right_ref: String,
}

struct CachedCheckout {
    key: String,
    root: PathBuf,
    /// Keeps the temp directory alive while any caller indexes from `root`.
    _temp: Arc<tempfile::TempDir>,
}

static CHECKOUT_CACHE: Mutex<Vec<CachedCheckout>> = Mutex::new(Vec::new());

/// Parse `main..feature`, `main...feature` (merge-base), or single ref vs WORKTREE.
pub fn parse_git_range(spec: &str) -> Result<(String, String)> {
    let spec = spec.trim();
    if spec.is_empty() {
        return Err(DiffError::Message("empty git range".to_string()));
    }
    if let Some(idx) = spec.find("...") {
        let left = spec[..idx].trim();
        let right = spec[idx + 3..].trim();
        if left.is_empty() || right.is_empty() {
            return Err(DiffError::Message("invalid triple-dot git range".to_string()));
        }
        return Ok((format!("{left}...{right}"), "TRIPLE_DOT".to_string()));
    }
    if let Some((left, right)) = spec.split_once("..") {
        let left = left.trim();
        let right = right.trim();
        if left.is_empty() || right.is_empty() {
            return Err(DiffError::Message("invalid two-dot git range".to_string()));
        }
        return Ok((left.to_string(), right.to_string()));
    }
    Ok((spec.to_string(), "WORKTREE".to_string()))
}

pub fn discover_repo_root(paths: &[PathBuf]) -> Option<PathBuf> {
    discover_git_repo_root(paths)
}

pub fn diff_git_refs(repo_path: &Path, left_ref: &str, right_ref: &str) -> Result<DiffResult> {
    Ok(diff_git_refs_with_catalogs(repo_path, left_ref, right_ref)?.0)
}

/// Like [`diff_git_refs`], but also returns the left and right catalogs used for the diff.
pub fn diff_git_refs_with_catalogs(
    repo_path: &Path,
    left_ref: &str,
    right_ref: &str,
) -> Result<(DiffResult, OntologyCatalog, OntologyCatalog)> {
    if right_ref == "TRIPLE_DOT" || left_ref.contains("...") {
        let (left, right) = parse_merge_base_refs(left_ref)?;
        let repo = Repository::open(repo_path)
            .map_err(|e| DiffError::Message(format!("git open failed: {e}")))?;
        let left_obj = repo
            .revparse_single(&left)
            .map_err(|e| DiffError::Message(format!("git revparse {left}: {e}")))?;
        let right_obj = repo
            .revparse_single(&right)
            .map_err(|e| DiffError::Message(format!("git revparse {right}: {e}")))?;
        let merge_base = repo
            .merge_base(left_obj.id(), right_obj.id())
            .map_err(|e| DiffError::Message(format!("git merge-base: {e}")))?;
        let base = checkout_catalog_at_oid(&repo, merge_base)?;
        let head = catalog_at_git_ref(repo_path, &right)?;
        let diff = diff_catalogs(&base, &head);
        return Ok((diff, base, head));
    }
    let left = catalog_at_git_ref(repo_path, left_ref)?;
    let right = if right_ref == "WORKTREE" || right_ref.is_empty() {
        catalog_at_worktree(repo_path)?
    } else {
        catalog_at_git_ref(repo_path, right_ref)?
    };
    Ok((diff_catalogs(&left, &right), left, right))
}

fn parse_merge_base_refs(spec: &str) -> Result<(String, String)> {
    let idx = spec
        .find("...")
        .ok_or_else(|| DiffError::Message("expected triple-dot range".to_string()))?;
    let left = spec[..idx].trim().to_string();
    let right = spec[idx + 3..].trim().to_string();
    if left.is_empty() || right.is_empty() {
        return Err(DiffError::Message("invalid triple-dot git range".to_string()));
    }
    Ok((left, right))
}

pub fn catalog_at_git_ref(repo_path: &Path, git_ref: &str) -> Result<OntologyCatalog> {
    let repo = Repository::open(repo_path)
        .map_err(|e| DiffError::Message(format!("git open failed: {e}")))?;
    checkout_catalog(&repo, git_ref)
}

/// Build catalog from ontology files in the working tree: git-indexed paths plus
/// untracked workspace files (respecting path jail and scan limits).
pub fn catalog_at_worktree(repo_path: &Path) -> Result<OntologyCatalog> {
    let paths = worktree_ontology_paths(repo_path)?;
    let mut builder = IndexBuilder::new().workspace(repo_path);
    if !paths.is_empty() {
        builder = builder.only_paths(paths);
    }
    builder.build().map_err(DiffError::from)
}

fn worktree_ontology_paths(repo_path: &Path) -> Result<Vec<PathBuf>> {
    let mut seen = HashSet::new();
    let mut paths = Vec::new();

    for path in list_git_tracked_ontology_paths(repo_path)? {
        push_worktree_path(&mut seen, &mut paths, path)?;
    }

    let scanned =
        WorkspaceScanner::new(repo_path).scan().map_err(|e| DiffError::Message(e.to_string()))?;
    for file in scanned {
        push_worktree_path(&mut seen, &mut paths, file.path)?;
    }

    Ok(paths)
}

fn push_worktree_path(
    seen: &mut HashSet<PathBuf>,
    paths: &mut Vec<PathBuf>,
    path: PathBuf,
) -> Result<()> {
    let key = path.canonicalize().unwrap_or_else(|_| path.clone());
    if seen.insert(key) {
        paths.push(path);
        if paths.len() > MAX_SCAN_FILES {
            return Err(DiffError::Message(format!(
                "worktree ontology files exceed maximum of {MAX_SCAN_FILES}"
            )));
        }
    }
    Ok(())
}

fn list_git_tracked_ontology_paths(repo_path: &Path) -> Result<Vec<PathBuf>> {
    let repo = Repository::open(repo_path)
        .map_err(|e| DiffError::Message(format!("git open failed: {e}")))?;
    let mut paths = Vec::new();
    let mut index = repo.index().map_err(|e| DiffError::Message(format!("git index: {e}")))?;
    index.read(true).map_err(|e| DiffError::Message(format!("git index read: {e}")))?;
    for entry in index.iter() {
        let path_str =
            std::str::from_utf8(&entry.path).map_err(|e| DiffError::Message(e.to_string()))?;
        if !is_ontology_file(path_str) {
            continue;
        }
        let full = repo_path.join(path_str);
        if path_has_parent_escape(full.as_path()) {
            continue;
        }
        paths.push(full);
        if paths.len() > MAX_SCAN_FILES {
            return Err(DiffError::Message(format!(
                "git tracked files exceed maximum of {MAX_SCAN_FILES}"
            )));
        }
    }
    Ok(paths)
}

fn checkout_catalog(repo: &Repository, git_ref: &str) -> Result<OntologyCatalog> {
    let obj = repo
        .revparse_single(git_ref)
        .map_err(|e| DiffError::Message(format!("git revparse {git_ref}: {e}")))?;
    let commit =
        obj.peel_to_commit().map_err(|e| DiffError::Message(format!("git peel commit: {e}")))?;
    checkout_catalog_at_oid(repo, commit.id())
}

fn checkout_catalog_at_oid(repo: &Repository, oid: Oid) -> Result<OntologyCatalog> {
    let repo_path =
        repo.path().parent().map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from("."));
    let key = format!("{}:{oid}", repo_path.display());
    if let Some((root, _keepalive)) = cache_get(&key) {
        return IndexBuilder::new().workspace(&root).build().map_err(DiffError::from);
    }

    let commit =
        repo.find_commit(oid).map_err(|e| DiffError::Message(format!("git find commit: {e}")))?;
    let tree = commit.tree().map_err(|e| DiffError::Message(format!("git tree: {e}")))?;

    let tmp = tempfile::Builder::new()
        .prefix("strixonomy-diff-")
        .tempdir()
        .map_err(|e| DiffError::Message(e.to_string()))?;
    let temp = Arc::new(tmp);
    let root = temp.path().to_path_buf();
    let mut walk_error: Option<String> = None;
    let mut file_count = 0usize;

    tree.walk(TreeWalkMode::PreOrder, |dir, entry| {
        if walk_error.is_some() {
            return 1;
        }
        if entry.kind() != Some(ObjectType::Blob) {
            return 0;
        }
        let name = entry.name().unwrap_or("");
        if !is_ontology_file(name) {
            return 0;
        }
        file_count += 1;
        if file_count > MAX_SCAN_FILES {
            walk_error =
                Some(format!("git tree exceeds maximum of {MAX_SCAN_FILES} ontology files"));
            return 1;
        }
        let rel = if dir.is_empty() { name.to_string() } else { format!("{dir}{name}") };
        let out_path = match ensure_extract_path_within(&root, &rel) {
            Ok(p) => p,
            Err(e) => {
                walk_error = Some(e);
                return 1;
            }
        };
        let blob = match repo.find_blob(entry.id()) {
            Ok(b) => b,
            Err(e) => {
                walk_error = Some(format!("find blob: {e}"));
                return 1;
            }
        };
        let size = blob.content().len() as u64;
        if size > MAX_FILE_BYTES {
            walk_error =
                Some(format!("blob exceeds size limit ({size} bytes > {MAX_FILE_BYTES}): {rel}",));
            return 1;
        }
        if let Some(parent) = out_path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                walk_error = Some(format!("create dir {}: {e}", parent.display()));
                return 1;
            }
        }
        if let Err(e) = std::fs::write(&out_path, blob.content()) {
            walk_error = Some(format!("write {}: {e}", out_path.display()));
            return 1;
        }
        0
    })
    .map_err(|e| DiffError::Message(format!("git tree walk: {e}")))?;

    if let Some(msg) = walk_error {
        return Err(DiffError::Message(msg));
    }

    cache_insert(key, root.clone(), temp);
    IndexBuilder::new().workspace(&root).build().map_err(DiffError::from)
}

fn cache_get(key: &str) -> Option<(PathBuf, Arc<tempfile::TempDir>)> {
    let guard = CHECKOUT_CACHE.lock().ok()?;
    guard.iter().find(|e| e.key == key).map(|e| (e.root.clone(), Arc::clone(&e._temp)))
}

fn cache_insert(key: String, root: PathBuf, temp: Arc<tempfile::TempDir>) {
    if let Ok(mut guard) = CHECKOUT_CACHE.lock() {
        if guard.iter().any(|e| e.key == key) {
            return;
        }
        while guard.len() >= CHECKOUT_CACHE_MAX {
            guard.remove(0);
        }
        guard.push(CachedCheckout { key, root, _temp: temp });
    }
}

fn is_ontology_file(name: &str) -> bool {
    Path::new(name)
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| ONTOLOGY_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::{Repository, Signature};

    #[test]
    fn parse_two_dot_range_splits_refs() {
        let (left, right) = parse_git_range("main..feature").unwrap();
        assert_eq!(left, "main");
        assert_eq!(right, "feature");
    }

    #[test]
    fn parse_single_ref_defaults_right_to_worktree() {
        let (left, right) = parse_git_range("HEAD").unwrap();
        assert_eq!(left, "HEAD");
        assert_eq!(right, "WORKTREE");
    }

    #[test]
    fn triple_dot_diff_compares_merge_base_to_feature() {
        // History:
        //   base -- main (adds MainOnly)
        //       \-- feature (adds FeatureOnly)
        // main...feature must equal merge-base..feature, so MainOnly must NOT appear
        // as an "added on left / removed on right" relative to feature-only tip.
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let ttl = |name: &str| {
            format!(
                "@prefix ex: <http://example.org#> .\n\
                 @prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
                 ex:{name} a owl:Class .\n"
            )
        };

        let repo = Repository::init(root).expect("git init");
        let sig = Signature::now("Strixonomy Test", "test@example.com").expect("signature");

        std::fs::write(root.join("onto.ttl"), ttl("Base")).unwrap();
        let mut index = repo.index().expect("index");
        index.add_path(Path::new("onto.ttl")).unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let base_oid =
            repo.commit(Some("HEAD"), &sig, &sig, "base", &tree, &[]).expect("base commit");
        let base_commit = repo.find_commit(base_oid).unwrap();

        // main diverges
        std::fs::write(root.join("onto.ttl"), ttl("MainOnly")).unwrap();
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("onto.ttl")).unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "main tip", &tree, &[&base_commit])
            .expect("main commit");
        repo.branch("main", &repo.head().unwrap().peel_to_commit().unwrap(), true).ok();

        // feature from base
        repo.set_head_detached(base_oid).unwrap();
        repo.checkout_head(Some(git2::build::CheckoutBuilder::new().force())).unwrap();
        std::fs::write(root.join("onto.ttl"), ttl("FeatureOnly")).unwrap();
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("onto.ttl")).unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let feature_oid = repo
            .commit(Some("refs/heads/feature"), &sig, &sig, "feature tip", &tree, &[&base_commit])
            .expect("feature commit");
        let _ = feature_oid;

        let (left, right) = parse_git_range("main...feature").unwrap();
        let (diff, left_cat, right_cat) =
            diff_git_refs_with_catalogs(root, &left, &right).expect("triple-dot diff");

        let left_iris: Vec<_> = left_cat.data().entities.iter().map(|e| e.iri.as_str()).collect();
        let right_iris: Vec<_> = right_cat.data().entities.iter().map(|e| e.iri.as_str()).collect();
        assert!(
            left_iris.iter().any(|i| i.contains("Base")),
            "merge-base catalog should contain Base, got {left_iris:?}"
        );
        assert!(
            !left_iris.iter().any(|i| i.contains("MainOnly")),
            "merge-base must not be main tip (MainOnly): {left_iris:?}"
        );
        assert!(
            right_iris.iter().any(|i| i.contains("FeatureOnly")),
            "right catalog should be feature tip: {right_iris:?}"
        );
        assert!(
            diff.entity_changes.iter().any(|c| c.iri.contains("FeatureOnly")),
            "diff should mention FeatureOnly: {:?}",
            diff.entity_changes
        );
    }

    #[test]
    fn reject_escape_path() {
        let dir = tempfile::tempdir().unwrap();
        let err = ensure_extract_path_within(dir.path(), "../outside.ttl").expect_err("escape");
        assert!(
            err.contains("..") || err.contains("escapes") || err.contains("outside"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn worktree_catalog_includes_untracked_ontology_files() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let tracked_ttl = concat!(
            "@prefix ex: <http://example.org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "ex:Tracked a owl:Class .\n"
        );
        let untracked_ttl = concat!(
            "@prefix ex: <http://example.org#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
            "ex:Untracked a owl:Class .\n"
        );
        std::fs::write(root.join("tracked.ttl"), tracked_ttl).unwrap();

        let repo = Repository::init(root).expect("git init");
        let mut index = repo.index().expect("index");
        index.add_path(Path::new("tracked.ttl")).expect("index add tracked");
        index.write().expect("index write");
        let tree_id = index.write_tree().expect("write tree");
        let tree = repo.find_tree(tree_id).expect("find tree");
        let sig = Signature::now("Strixonomy Test", "test@example.com").expect("signature");
        repo.commit(Some("HEAD"), &sig, &sig, "init", &tree, &[]).expect("commit");

        std::fs::write(root.join("new.ttl"), untracked_ttl).unwrap();

        let catalog = catalog_at_worktree(root).expect("worktree catalog");
        let iris: Vec<_> = catalog.data().entities.iter().map(|e| e.iri.as_str()).collect();
        assert!(
            iris.iter().any(|iri| iri.contains("Tracked")),
            "expected tracked entity, got {iris:?}"
        );
        assert!(
            iris.iter().any(|iri| iri.contains("Untracked")),
            "expected untracked entity in WORKTREE catalog, got {iris:?}"
        );
    }
}
