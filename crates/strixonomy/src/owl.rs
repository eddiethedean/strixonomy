//! Horned-OWL bridge, patch write-back, and Manchester syntax.

pub use strixonomy_owl::{
    all_entity_statement_ranges, apply_patches, apply_patches as apply_owl_patches,
    apply_patches_to_text, apply_patches_to_text as apply_owl_patches_to_text, atomic_write,
    bridge_ontology, class_expression_to_manchester, class_expression_to_turtle_fragment,
    entity_block_range, entity_primary_block_range, expression_tree_json, load_from_quads,
    load_owx_text, load_turtle_text, namespaces_for_text, parse_class_expression,
    prefixes_from_turtle, short_name_from_iri, supports_horned_load, ApplyPatchResult, ByteRange,
    ManchesterDiagnostic, ManchesterParseOutput, OwlBridgeResult, OwlError, PatchDiagnostic,
    PatchEntityKind, PatchOp,
};
