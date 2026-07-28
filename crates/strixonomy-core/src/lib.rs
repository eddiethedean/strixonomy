//! Core types, workspace scanning, path sandboxing, and resource limits for Strixonomy.
//!
//! Published as [`strixonomy-core`](https://crates.io/crates/strixonomy-core).
//!
//! # API stability
//!
//! **Pre-1.0:** public types and constants may change between minor releases until
//! [v1.0 stable core](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/v1.0_BACKLOG.md)
//! is complete. See [workspace limits](https://github.com/eddiethedean/strixonomy/blob/main/docs/workspace-limits.md).

pub mod document_lookup;
pub mod error;
pub mod io;
pub mod limits;
pub mod model;
pub mod path_jail;
pub mod paths;
pub mod quick_fix;
pub mod rdf_literals;
pub mod scanner;

pub use document_lookup::{
    document_for_entity, document_for_ontology_id, document_matches_entity,
    document_matches_ontology_id, file_uri_for_path, normalize_iri,
};
pub use error::{Result, StrixonomyError};

/// Deprecated alias for [`StrixonomyError`] (OntoCore name).
#[deprecated(since = "0.27.0", note = "renamed to StrixonomyError")]
pub type OntoCoreError = StrixonomyError;
pub use io::{atomic_write, read_file_capped, read_to_string_capped, replace_file};
pub use limits::{
    MAX_ENTITIES, MAX_FILE_BYTES, MAX_OPEN_DOCUMENTS, MAX_QUERY_BYTES, MAX_SCAN_FILES,
    MAX_SCAN_WALK_ENTRIES, MAX_SPARQL_RESULT_ROWS, MAX_SQL_RESULT_ROWS, MAX_TOTAL_TRIPLES,
    MAX_TRIPLES_PER_FILE,
};
pub use model::{
    Annotation, Axiom, AxiomAnnotation, Diagnostic, DiagnosticCode, DiagnosticSeverity, Entity,
    EntityKind, Import, Namespace, OntologyDocument, OntologyFormat, ParseStatus,
    PropertyCharacteristics, SourceLocation, AXIOM_KIND_CLASS_ASSERTION,
    AXIOM_KIND_DATATYPE_DEFINITION, AXIOM_KIND_DATA_PROPERTY_ASSERTION,
    AXIOM_KIND_DIFFERENT_INDIVIDUALS, AXIOM_KIND_DISJOINT_CLASS,
    AXIOM_KIND_DISJOINT_DATA_PROPERTIES, AXIOM_KIND_DISJOINT_OBJECT_PROPERTIES,
    AXIOM_KIND_DISJOINT_UNION, AXIOM_KIND_DOMAIN, AXIOM_KIND_EQUIVALENT_CLASS,
    AXIOM_KIND_EQUIVALENT_DATA_PROPERTIES, AXIOM_KIND_EQUIVALENT_OBJECT_PROPERTIES,
    AXIOM_KIND_HAS_KEY, AXIOM_KIND_INVERSE_OBJECT_PROPERTIES,
    AXIOM_KIND_NEGATIVE_DATA_PROPERTY_ASSERTION, AXIOM_KIND_NEGATIVE_OBJECT_PROPERTY_ASSERTION,
    AXIOM_KIND_OBJECT_PROPERTY_ASSERTION, AXIOM_KIND_PROPERTY_CHAIN, AXIOM_KIND_RANGE,
    AXIOM_KIND_SAME_INDIVIDUAL, AXIOM_KIND_SUB_CLASS_OF, AXIOM_KIND_SUB_DATA_PROPERTY_OF,
    AXIOM_KIND_SUB_OBJECT_PROPERTY_OF,
};
pub use path_jail::{
    canonical_workspace_root, discover_git_repo_root, ensure_extract_path_within, file_uri_to_path,
    is_path_within, is_path_within_any, paths_refer_to_same, resolve_document_path,
    resolve_lsp_document_path, resolve_lsp_document_path_any, validate_workspace_scope,
    validate_workspace_scope_any, workspace_uri_to_path,
};
pub use paths::{
    cache_dir, diagnostics_config_path, plugin_search_dirs, plugins_dir, resolve_config_path,
    resolve_dotted_config_path, CACHE_SUBDIR, CONFIG_DIR, DIAGNOSTICS_FILE, LEGACY_CONFIG_DIR,
    LEGACY_PLUGIN_DISABLED_REL, LEGACY_PLUGIN_OUT_REL, PLUGINS_SUBDIR, PLUGIN_DISABLED_REL,
    PLUGIN_OUT_REL,
};
pub use quick_fix::QuickFix;
pub use rdf_literals::parse_boolean_literal;
pub use scanner::{OntologyFile, WorkspaceScanner};
