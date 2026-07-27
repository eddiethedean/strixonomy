//! Semantic diff ref tokens shared by CLI and LSP.

/// Indexed in-memory catalog (LSP last index). Legacy alias: `WORKSPACE`.
pub fn is_indexed_catalog_ref(git_ref: &str) -> bool {
    let upper = git_ref.to_ascii_uppercase();
    upper == "INDEXED" || upper == "CATALOG" || upper == "WORKSPACE"
}

/// Filesystem / git worktree catalog (uncommitted or working tree snapshot).
pub fn is_worktree_ref(git_ref: &str) -> bool {
    git_ref.eq_ignore_ascii_case("WORKTREE")
}
