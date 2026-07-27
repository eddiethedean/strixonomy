use std::path::{Component, Path, PathBuf};

/// Canonicalize a workspace root directory.
pub fn canonical_workspace_root(path: &Path) -> Result<PathBuf, String> {
    path.canonicalize().map_err(|e| format!("workspace path invalid: {e}"))
}

/// Returns true if `path` is under any of the workspace roots.
pub fn is_path_within_any(roots: &[PathBuf], path: &Path) -> bool {
    roots.iter().any(|root| is_path_within(root, path))
}

/// Ensure `path` is under at least one workspace root.
pub fn validate_workspace_scope_any(
    requested: &Path,
    workspace_roots: &[PathBuf],
) -> Result<PathBuf, String> {
    for root in workspace_roots {
        if let Ok(resolved) = validate_workspace_scope(requested, root) {
            return Ok(resolved);
        }
    }
    Err("workspace URI is outside allowed workspace roots".to_string())
}

/// Resolve an LSP document URI under any registered workspace root.
pub fn resolve_lsp_document_path_any(
    uri: &str,
    workspace_roots: &[PathBuf],
) -> Result<PathBuf, String> {
    for root in workspace_roots {
        if let Ok(path) = resolve_lsp_document_path(uri, root) {
            return Ok(path);
        }
    }
    Err("document path is outside the indexed workspace".to_string())
}

/// Resolve a `file://` workspace URI to a canonical directory path.
pub fn workspace_uri_to_path(uri: &str) -> Result<PathBuf, String> {
    file_uri_to_path(uri).and_then(|p| canonical_workspace_root(&p))
}

/// Resolve a `file://` URI to a path (not necessarily canonical).
pub fn file_uri_to_path(uri: &str) -> Result<PathBuf, String> {
    let url = url::Url::parse(uri).map_err(|e| e.to_string())?;
    if url.scheme() != "file" {
        return Err(format!("only file:// URIs are supported, got {}", url.scheme()));
    }
    url.to_file_path().map_err(|_| format!("invalid file URI: {uri}"))
}

/// Resolve a document URI and ensure it lies under `workspace_root` (both canonical).
pub fn resolve_document_path(uri: &str, workspace_root: &Path) -> Result<PathBuf, String> {
    let path = file_uri_to_path(uri)?;
    let canonical =
        path.canonicalize().map_err(|e| format!("cannot resolve document path: {e}"))?;
    if !is_path_within(workspace_root, &canonical) {
        return Err("document path is outside the indexed workspace".to_string());
    }
    Ok(canonical)
}

/// Resolve an LSP document URI under `workspace_root` without requiring the file to exist.
///
/// Canonicalizes when the path exists on disk; otherwise resolves via the longest existing
/// path prefix so symlink parents cannot escape the workspace.
pub fn resolve_lsp_document_path(uri: &str, workspace_root: &Path) -> Result<PathBuf, String> {
    let path = file_uri_to_path(uri)?;
    resolve_path_in_workspace(
        &path,
        workspace_root,
        "document path is outside the indexed workspace",
    )
}

/// Ensure `path` is the workspace root or a subdirectory of it (after canonicalization).
pub fn validate_workspace_scope(
    requested: &Path,
    workspace_root: &Path,
) -> Result<PathBuf, String> {
    resolve_path_in_workspace(
        requested,
        workspace_root,
        "workspace URI is outside the allowed workspace root",
    )
}

/// Resolve `path` under `workspace_root`, including non-existent paths.
///
/// Walks upward to the longest existing prefix, canonicalizes that prefix, and joins only
/// the missing suffix. If any existing prefix resolves outside the root, rejects.
///
/// Also rejects a dangling (or out-of-root) **leaf/intermediate symlink** under the
/// workspace: those would otherwise look like a plain non-existent path whose existing
/// prefix is the parent directory.
fn resolve_path_in_workspace(
    path: &Path,
    workspace_root: &Path,
    outside_msg: &str,
) -> Result<PathBuf, String> {
    if path_has_parent_escape(path) {
        return Err("path escapes workspace via ..".to_string());
    }
    let root = canonical_workspace_root(workspace_root)?;

    if let Ok(canonical) = path.canonicalize() {
        if path_is_under(&root, &canonical) {
            return Ok(canonical);
        }
        return Err(outside_msg.to_string());
    }

    let absolute = if path.is_absolute() { path.to_path_buf() } else { root.join(path) };
    let absolute = normalize_lexical(&absolute);

    let (existing_prefix, missing_suffix) = split_existing_prefix(&absolute);
    let canonical_prefix =
        existing_prefix.canonicalize().map_err(|e| format!("cannot resolve path prefix: {e}"))?;

    if !path_is_under(&root, &canonical_prefix) {
        return Err(outside_msg.to_string());
    }

    reject_symlink_escape_from(&canonical_prefix, &missing_suffix, &root)?;

    let candidate = if missing_suffix.as_os_str().is_empty() {
        canonical_prefix
    } else {
        canonical_prefix.join(&missing_suffix)
    };

    if path_is_under(&root, &candidate) || is_path_within_lexical(&root, &candidate) {
        Ok(candidate)
    } else {
        Err(outside_msg.to_string())
    }
}

/// Walk `prefix` + each component of `suffix`, rejecting dangling symlinks or symlink
/// targets outside `root`. `prefix` must already be canonical and inside `root`.
fn reject_symlink_escape_from(prefix: &Path, suffix: &Path, root: &Path) -> Result<(), String> {
    let mut current = prefix.to_path_buf();
    for component in suffix.components() {
        current.push(component);
        let meta = match std::fs::symlink_metadata(&current) {
            Ok(meta) => meta,
            Err(_) => continue,
        };
        if !meta.file_type().is_symlink() {
            continue;
        }
        match current.canonicalize() {
            Ok(target) => {
                if !path_is_under(root, &target) {
                    return Err("path escapes workspace via symlink".to_string());
                }
            }
            Err(_) => {
                return Err("path escapes workspace via dangling symlink".to_string());
            }
        }
    }
    Ok(())
}

/// Split `path` into (longest existing prefix, remaining relative suffix).
fn split_existing_prefix(path: &Path) -> (PathBuf, PathBuf) {
    let mut prefix = path.to_path_buf();
    let mut missing = PathBuf::new();
    loop {
        if prefix.exists() {
            let mut suffix = PathBuf::new();
            for component in missing.components().rev() {
                suffix.push(component);
            }
            return (prefix, suffix);
        }
        match prefix.file_name() {
            Some(name) => {
                missing.push(name);
                if !prefix.pop() {
                    break;
                }
            }
            None => break,
        }
    }
    // Nothing exists; treat as relative to current dir (caller already made absolute).
    (PathBuf::from("."), path.to_path_buf())
}

fn normalize_lexical(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                components.pop();
            }
            Component::CurDir => {}
            c => components.push(c),
        }
    }
    components.iter().collect()
}

fn is_path_within_lexical(root: &Path, path: &Path) -> bool {
    let root = normalize_lexical(root);
    let path = normalize_lexical(path);
    path_is_under(&root, &path)
}

/// Component-aware containment for already-canonical (or known-absolute) paths.
///
/// Rejects sibling-directory prefix traps (e.g. `/tmp/ws` must not contain `/tmp/ws_extra`).
fn path_is_under(root: &Path, path: &Path) -> bool {
    path == root || path.strip_prefix(root).is_ok()
}

/// True when two paths refer to the same filesystem location.
///
/// Uses exact equality, then both-sides canonicalize. Dual canonicalize failures
/// do **not** count as a match (`None == None` would wrongly agree).
pub fn paths_refer_to_same(a: &Path, b: &Path) -> bool {
    a == b || a.canonicalize().ok().zip(b.canonicalize().ok()).is_some_and(|(x, y)| x == y)
}

/// Returns true if `path` is `workspace_root` or nested under it.
pub fn is_path_within(workspace_root: &Path, path: &Path) -> bool {
    let Ok(root) = workspace_root.canonicalize() else {
        return false;
    };
    if let Ok(path) = path.canonicalize() {
        return path_is_under(&root, &path);
    }
    // For non-existent paths, resolve via longest existing prefix — never trust lexical alone
    // when a real prefix exists (symlink escape).
    let absolute = if path.is_absolute() {
        normalize_lexical(path)
    } else {
        normalize_lexical(&root.join(path))
    };
    let (existing_prefix, missing_suffix) = split_existing_prefix(&absolute);
    let Ok(canonical_prefix) = existing_prefix.canonicalize() else {
        return is_path_within_lexical(&root, &absolute);
    };
    if !path_is_under(&root, &canonical_prefix) {
        return false;
    }
    if reject_symlink_escape_from(&canonical_prefix, &missing_suffix, &root).is_err() {
        return false;
    }
    let candidate = if missing_suffix.as_os_str().is_empty() {
        canonical_prefix
    } else {
        canonical_prefix.join(missing_suffix)
    };
    path_is_under(&root, &candidate) || is_path_within_lexical(&root, &candidate)
}

/// Reject paths that escape upward via `..` before canonicalize (symlink-free check).
pub fn path_has_parent_escape(path: &Path) -> bool {
    path.components().any(|c| matches!(c, Component::ParentDir))
}

/// Resolve a relative path for extraction under `extract_root` (e.g. git tree checkout).
///
/// Rejects `..`, absolute components, and paths that would escape `extract_root`.
pub fn ensure_extract_path_within(extract_root: &Path, rel: &str) -> Result<PathBuf, String> {
    if rel.is_empty() {
        return Err("empty relative path".to_string());
    }
    let rel_path = Path::new(rel);
    if path_has_parent_escape(rel_path) {
        return Err("extract path escapes via ..".to_string());
    }
    for component in rel_path.components() {
        match component {
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err("invalid path component in extract".to_string());
            }
            _ => {}
        }
    }
    let root = canonical_workspace_root(extract_root)?;
    let candidate = normalize_lexical(&root.join(rel_path));
    if !is_path_within_lexical(&root, &candidate) {
        return Err("extract path outside extraction root".to_string());
    }
    Ok(candidate)
}

/// Discover a git repository root from any of the given workspace paths.
pub fn discover_git_repo_root(paths: &[PathBuf]) -> Option<PathBuf> {
    for path in paths {
        let mut current = if path.is_dir() {
            path.clone()
        } else {
            match path.parent() {
                Some(p) => p.to_path_buf(),
                None => continue,
            }
        };
        loop {
            if current.join(".git").exists() {
                return current.canonicalize().ok().or_else(|| Some(current.clone()));
            }
            match current.parent() {
                Some(p) => current = p.to_path_buf(),
                None => break,
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn validate_scope_allows_nonexistent_file_under_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let root = canonical_workspace_root(dir.path()).unwrap();
        let new_file = dir.path().join("module.ttl");
        let resolved =
            validate_workspace_scope(&new_file, &root).expect("new file under workspace");
        assert!(is_path_within(&root, &resolved));
        assert_eq!(resolved.file_name(), new_file.file_name());
    }

    #[test]
    fn paths_refer_to_same_rejects_dual_canonicalize_failure() {
        let missing_a = PathBuf::from("/definitely/missing/strixonomy-a-xyz.ttl");
        let missing_b = PathBuf::from("/definitely/missing/strixonomy-b-xyz.ttl");
        assert!(!paths_refer_to_same(&missing_a, &missing_b));
        assert!(paths_refer_to_same(&missing_a, &missing_a));
    }

    #[test]
    fn lsp_document_path_allows_nonexistent_file_under_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let root = canonical_workspace_root(dir.path()).unwrap();
        let new_file = dir.path().join("new.ttl");
        let uri = url::Url::from_file_path(&new_file).unwrap().to_string();
        let resolved = resolve_lsp_document_path(&uri, &root).expect("new file under workspace");
        assert!(is_path_within(&root, &resolved));
        assert_eq!(resolved.file_name(), new_file.file_name());
    }

    #[test]
    fn document_must_be_under_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("ontologies");
        fs::create_dir_all(&sub).unwrap();
        let ttl = sub.join("ex.ttl");
        fs::write(&ttl, "@prefix ex: <http://ex/> .").unwrap();

        let root = canonical_workspace_root(dir.path()).unwrap();
        let uri = url::Url::from_file_path(&ttl).unwrap().to_string();
        let resolved = resolve_document_path(&uri, &root).expect("under workspace");
        assert_eq!(resolved, ttl.canonicalize().unwrap());
    }

    #[test]
    fn rejects_path_outside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let ttl = outside.path().join("secret.ttl");
        fs::write(&ttl, "@prefix ex: <http://ex/> .").unwrap();

        let root = canonical_workspace_root(dir.path()).unwrap();
        let uri = url::Url::from_file_path(&ttl).unwrap().to_string();
        let err = resolve_document_path(&uri, &root).expect_err("must reject outside path");
        assert!(err.contains("outside the indexed workspace"), "unexpected error: {err}");
    }

    #[test]
    #[cfg(unix)]
    fn rejects_nonexistent_file_under_symlink_outside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let link = dir.path().join("vendor");
        std::os::unix::fs::symlink(outside.path(), &link).unwrap();

        let root = canonical_workspace_root(dir.path()).unwrap();
        let target = link.join("pwn.ttl");
        let uri = url::Url::from_file_path(&target).unwrap().to_string();
        let err = resolve_lsp_document_path(&uri, &root)
            .expect_err("must reject create-through-symlink escape");
        assert!(
            err.contains("outside") || err.contains("escapes") || err.contains("symlink"),
            "unexpected error: {err}"
        );
        let scope_err = validate_workspace_scope(&target, &root).expect_err("scope");
        assert!(
            scope_err.contains("outside") || scope_err.contains("escapes"),
            "unexpected scope error: {scope_err}"
        );
        assert!(!is_path_within(&root, &target));
    }

    #[test]
    #[cfg(unix)]
    fn rejects_nested_missing_path_through_symlink_prefix() {
        let dir = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let link = dir.path().join("vendor");
        std::os::unix::fs::symlink(outside.path(), &link).unwrap();

        let root = canonical_workspace_root(dir.path()).unwrap();
        let target = link.join("nested").join("pwn.ttl");
        let err = validate_workspace_scope(&target, &root).expect_err("must reject");
        assert!(err.contains("outside") || err.contains("escapes"), "unexpected error: {err}");
    }

    #[test]
    #[cfg(unix)]
    fn rejects_dangling_leaf_symlink_create_escape() {
        let dir = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let root = canonical_workspace_root(dir.path()).unwrap();
        let leaf = dir.path().join("new.ttl");
        let outside_target = outside.path().join("pwn.ttl");
        std::os::unix::fs::symlink(&outside_target, &leaf).unwrap();

        let err = validate_workspace_scope(&leaf, &root)
            .expect_err("must reject dangling leaf symlink escape");
        assert!(
            err.contains("dangling symlink") || err.contains("symlink") || err.contains("outside"),
            "unexpected error: {err}"
        );
        assert!(!is_path_within(&root, &leaf));

        let uri = url::Url::from_file_path(&leaf).unwrap().to_string();
        let lsp_err = resolve_lsp_document_path(&uri, &root).expect_err("lsp path");
        assert!(
            lsp_err.contains("dangling symlink")
                || lsp_err.contains("symlink")
                || lsp_err.contains("outside"),
            "unexpected lsp error: {lsp_err}"
        );
        assert!(!outside_target.exists(), "must not create outside file during checks");
    }

    #[test]
    #[cfg(unix)]
    fn allows_symlink_leaf_that_resolves_inside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let root = canonical_workspace_root(dir.path()).unwrap();
        let real = dir.path().join("real.ttl");
        fs::write(&real, "@prefix ex: <http://ex/> .").unwrap();
        let link = dir.path().join("alias.ttl");
        std::os::unix::fs::symlink(&real, &link).unwrap();

        let resolved = validate_workspace_scope(&link, &root).expect("in-workspace symlink");
        assert_eq!(resolved, real.canonicalize().unwrap());
    }

    #[test]
    fn rejects_parent_escape_components() {
        let dir = tempfile::tempdir().unwrap();
        let root = canonical_workspace_root(dir.path()).unwrap();
        let escaped = dir.path().join("sub").join("..").join("..").join("etc").join("passwd");
        let err = validate_workspace_scope(&escaped, &root).expect_err("must reject .. escape");
        assert!(
            err.contains("escapes workspace via ..") || err.contains("outside"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn rejects_sibling_directory_prefix_trap() {
        let dir = tempfile::tempdir().unwrap();
        let sibling = dir.path().join("ws_extra");
        fs::create_dir_all(&sibling).unwrap();
        let secret = sibling.join("secret.ttl");
        fs::write(&secret, "@prefix ex: <http://ex/> .").unwrap();

        let root = dir.path().join("ws");
        fs::create_dir_all(&root).unwrap();
        let root = canonical_workspace_root(&root).unwrap();

        assert!(
            !is_path_within(&root, &secret.canonicalize().unwrap()),
            "sibling directory must not match as workspace child"
        );
        let uri = url::Url::from_file_path(&secret).unwrap().to_string();
        assert!(resolve_document_path(&uri, &root).is_err());
    }

    #[test]
    fn validate_scope_any_accepts_paths_under_either_root() {
        let a = tempfile::tempdir().unwrap();
        let b = tempfile::tempdir().unwrap();
        let root_a = canonical_workspace_root(a.path()).unwrap();
        let root_b = canonical_workspace_root(b.path()).unwrap();
        let file_in_b = b.path().join("module.ttl");
        std::fs::write(&file_in_b, "@prefix ex: <http://ex/> .").unwrap();

        let resolved = validate_workspace_scope_any(&file_in_b, &[root_a.clone(), root_b.clone()])
            .expect("under second root");
        assert!(is_path_within_any(&[root_a.clone(), root_b.clone()], &resolved));
        assert!(
            resolved.ends_with("module.ttl"),
            "must return the scoped file path, not an empty default: {resolved:?}"
        );
        assert!(is_path_within(&root_b, &resolved));
    }

    #[test]
    fn is_path_within_any_rejects_outside_all_roots() {
        let a = tempfile::tempdir().unwrap();
        let b = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let root_a = canonical_workspace_root(a.path()).unwrap();
        let root_b = canonical_workspace_root(b.path()).unwrap();
        let secret = outside.path().join("secret.ttl");
        std::fs::write(&secret, "@prefix ex: <http://ex/> .").unwrap();
        let secret = secret.canonicalize().unwrap();

        assert!(
            !is_path_within_any(&[root_a.clone(), root_b.clone()], &secret),
            "path outside every root must be rejected"
        );
        let err = validate_workspace_scope_any(&secret, &[root_a, root_b]).expect_err("outside");
        assert!(err.contains("outside") || err.contains("escapes"), "unexpected error: {err}");
    }

    #[test]
    fn resolve_lsp_document_path_any_rejects_outside() {
        let a = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let root_a = canonical_workspace_root(a.path()).unwrap();
        let secret = outside.path().join("secret.ttl");
        std::fs::write(&secret, "@prefix ex: <http://ex/> .").unwrap();
        let uri = url::Url::from_file_path(&secret).unwrap().to_string();
        let err = resolve_lsp_document_path_any(&uri, &[root_a]).expect_err("outside");
        assert!(err.contains("outside"), "unexpected error: {err}");
    }

    #[test]
    fn workspace_uri_to_path_rejects_non_file_scheme() {
        let err = workspace_uri_to_path("https://example.org/ws").expect_err("scheme");
        assert!(!err.is_empty());
    }

    #[test]
    fn path_has_parent_escape_detects_dotdot() {
        assert!(path_has_parent_escape(Path::new("a/../b")));
        assert!(!path_has_parent_escape(Path::new("a/b")));
    }

    #[test]
    fn ensure_extract_path_within_accepts_nested_relative() {
        let dir = tempfile::tempdir().unwrap();
        let nested = ensure_extract_path_within(dir.path(), "ontologies/ex.ttl").expect("nested");
        assert_eq!(nested.file_name().and_then(|s| s.to_str()), Some("ex.ttl"));
        assert!(is_path_within_lexical(&canonical_workspace_root(dir.path()).unwrap(), &nested));
    }

    #[test]
    fn ensure_extract_path_within_rejects_parent_escape() {
        let dir = tempfile::tempdir().unwrap();
        let err = ensure_extract_path_within(dir.path(), "../outside.ttl").expect_err("..");
        assert!(
            err.contains("escapes via ..") || err.contains("invalid path"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn ensure_extract_path_within_rejects_absolute() {
        let dir = tempfile::tempdir().unwrap();
        let err = ensure_extract_path_within(dir.path(), "/etc/passwd").expect_err("absolute");
        assert!(
            err.contains("invalid path component") || err.contains("outside"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn ensure_extract_path_within_rejects_empty() {
        let dir = tempfile::tempdir().unwrap();
        let err = ensure_extract_path_within(dir.path(), "").expect_err("empty");
        assert!(err.contains("empty"), "unexpected error: {err}");
    }

    #[test]
    fn discover_git_repo_root_finds_nested_workspace() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".git")).unwrap();
        let nested = dir.path().join("ontologies");
        fs::create_dir_all(&nested).unwrap();
        let found = discover_git_repo_root(&[nested]).expect("git root");
        assert_eq!(found, dir.path().canonicalize().unwrap());
    }

    #[test]
    fn discover_git_repo_root_none_without_git() {
        let dir = tempfile::tempdir().unwrap();
        assert!(discover_git_repo_root(&[dir.path().to_path_buf()]).is_none());
    }
}
