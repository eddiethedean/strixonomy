//! Horned-OWL facade for Strixonomy: load, catalog bridge, and patch write-back.
//!
//! Published as [`strixonomy-owl`](https://crates.io/crates/strixonomy-owl).

mod apply_xml;
mod bridge;
pub mod compare;
mod error;
pub mod links;
mod load;
pub mod manchester;
mod mutate;
pub mod patch;
mod remap;
pub mod render;
mod serialize;
mod span;
mod turtle_lex;
pub mod util;

pub use apply_xml::{apply_xml_patches, apply_xml_patches_to_text};
pub use bridge::{bridge_ontology, OwlBridgeResult};
pub use compare::{compare_bridges, compare_ontologies, SemanticDiff};
pub use error::{OwlError, Result};
pub use links::{
    extract_first_link_url, extract_links, extract_with_pattern, linkify_markdown_text,
    AnnotationLink,
};
pub use load::{load_from_quads, load_owx_text, load_turtle_text, supports_horned_load};
pub use manchester::{
    class_expression_to_manchester, class_expression_to_turtle_fragment,
    class_expression_to_turtle_value, data_range_to_manchester, data_range_to_turtle_term,
    expression_tree_json, parse_class_expression, parse_class_expression_with_datatypes,
    parse_data_range, ManchesterDiagnostic, ManchesterParseOutput,
};
pub use mutate::{apply_patches_to_ontology, apply_patches_to_ontology_with_ns};
pub use patch::{
    apply_patches, apply_patches_to_text, atomic_write, is_safe_iri, validate_prefix,
    ApplyPatchResult, PatchDiagnostic, PatchEntityKind, PatchOp,
};
pub use remap::{
    merge_entity_iri, merge_entity_iri_in_xml_text, remap_entity_iri, remap_entity_iri_in_xml_text,
};
pub use render::{
    escape_manchester_rendering, expand_prefixed_iri, match_prefix, render_as_curie,
    render_entity_iri, split_iri, unescape_manchester_rendering,
};
pub use serialize::{
    load_owl_xml_ontology, load_rdf_xml_ontology, serialize_owl_xml, serialize_rdf_xml,
};
pub use span::{
    all_entity_statement_ranges, entity_block_range, entity_primary_block_range,
    is_in_comment_or_string, namespaces_for_text, prefixes_from_turtle, short_name_from_iri,
    ByteRange,
};
pub use util::{
    abbreviate_string, cmp_annotation_property_iri, format_iso8601_utc, render_entity_markdown,
    replace_lexical_value, replace_lexical_value_whole, LexicalLiteral,
    DEFAULT_ANNOTATION_PROPERTY_ORDER, ELLIPSIS,
};
