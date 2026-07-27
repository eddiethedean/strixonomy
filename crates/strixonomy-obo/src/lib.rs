//! OBO Format 1.4 patch write-back (ADR-0019).

mod apply;
mod error;
pub mod idpolicy;
pub mod obofoundry;
mod patch;
mod remap;

pub use apply::{
    apply_patches, apply_patches_to_text, atomic_write, ApplyPatchResult, PatchDiagnostic,
};
pub use error::{OboError, Result};
pub use idpolicy::{
    parse_id_policy, parse_id_policy_file, parse_id_policy_from_catalog, IdPolicy, IdPolicyError,
    IdRange, IRI_ALLOCATED_TO, IRI_IDS_FOR, IRI_ID_DIGITS, IRI_ID_PREFIX,
};
pub use obofoundry::{
    parse_registry_json, OboFoundryContact, OboFoundryEntry, OboFoundryError, OboFoundryLicense,
    OboFoundryRegistry,
};
pub use patch::OboPatchOp;
pub use remap::{merge_obo_id_in_text, remap_obo_id_in_text, replace_obo_id_refs_in_text};
