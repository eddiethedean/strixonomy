//! Semantic diff between ontology catalogs and Git refs.

mod compare;
mod format;
mod git;
mod model;
mod refs;

pub use compare::{apply_unsat_diff, diff_catalogs, diff_directories, DiffError, Result};
pub use format::{
    format_diff_json, format_diff_markdown, format_diff_pr_summary, format_diff_text,
};
pub use git::{
    catalog_at_git_ref, catalog_at_worktree, diff_git_refs, diff_git_refs_with_catalogs,
    discover_repo_root, parse_git_range, GitDiffSpec,
};
pub use model::{
    AnnotationChange, AxiomChange, BreakingChange, BreakingReason, DiffResult, DiffSummaryCounts,
    EntityChange, EntityChangeKind, ImportChange, InferenceChange,
};
pub use refs::{is_indexed_catalog_ref, is_worktree_ref};
