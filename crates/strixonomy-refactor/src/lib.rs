//! Workspace refactoring for Strixonomy (v0.8).
//!
//! Published as [`strixonomy-refactor`](https://crates.io/crates/strixonomy-refactor).

mod apply;
mod error;
mod model;
mod ontology;
mod rename;
mod source;
mod text;
mod usages;

pub use apply::{
    apply_refactor_plan, apply_refactor_plan_checked, apply_refactor_plan_checked_with_overrides,
    plan_touches_path, plans_equivalent, validate_refactor_plan_paths,
    validate_refactor_plan_paths_any,
};
pub use error::{RefactorError, Result};
pub use model::{FileChange, Hunk, RefactorPlan, RefactorRequest, Usage, UsageKind};
pub use ontology::{
    expand_signature_locality, preview_cleanup_imports, preview_flatten_imports,
    preview_merge_ontologies, resolve_import_document,
};
pub use rename::{
    preview_extract_module, preview_merge_entities, preview_migrate_namespace, preview_move_axioms,
    preview_move_entity, preview_refactor, preview_rename_iri, preview_replace_entity,
};
pub use usages::{find_usages, find_usages_with_overrides};
