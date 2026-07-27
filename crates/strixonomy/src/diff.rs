//! Semantic diff between ontology catalogs.

pub use strixonomy_diff::{
    apply_unsat_diff, catalog_at_git_ref, diff_catalogs, diff_directories, diff_git_refs,
    format_diff_json, format_diff_markdown, format_diff_text, parse_git_range, AnnotationChange,
    AxiomChange, BreakingChange, BreakingReason, DiffResult, DiffSummaryCounts, EntityChange,
    EntityChangeKind, GitDiffSpec, ImportChange, InferenceChange,
};
