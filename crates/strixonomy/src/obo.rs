//! OBO Format patch write-back (re-export).

pub use strixonomy_obo::{
    apply_patches, apply_patches as apply_obo_patches, apply_patches_to_text,
    apply_patches_to_text as apply_obo_patches_to_text, atomic_write, ApplyPatchResult, OboError,
    OboPatchOp, PatchDiagnostic,
};
