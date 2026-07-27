//! Workspace refactoring (rename, migrate, move, extract).

pub use strixonomy_refactor::{
    apply_refactor_plan, apply_refactor_plan_checked, apply_refactor_plan_checked_with_overrides,
    find_usages, find_usages_with_overrides, plan_touches_path, plans_equivalent,
    preview_extract_module, preview_migrate_namespace, preview_move_entity, preview_refactor,
    preview_rename_iri, validate_refactor_plan_paths, validate_refactor_plan_paths_any, FileChange,
    Hunk, RefactorError, RefactorPlan, RefactorRequest, Usage, UsageKind,
};
