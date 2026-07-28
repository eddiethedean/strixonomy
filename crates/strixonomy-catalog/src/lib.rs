//! Workspace indexing: scan files, parse ontologies, build queryable catalog + Oxigraph store.
//!
//! Entry points: [`IndexBuilder`], [`OntologyCatalog`].
//!
//! # API stability
//!
//! **During v0.x:** catalog shapes and method signatures may change between minor releases.
//! Use [`strixonomy_query::sparql_catalog`] for SPARQL; do not depend on Oxigraph types.

mod builder;
mod disk_cache;
mod entity_api;
mod graph;
mod incremental;
pub mod xml_catalog;

pub use builder::{CatalogError, IndexBuilder, OntologyCatalog};
pub use entity_api::{ClassHierarchy, EntityDetail, SourceHint, SubclassEdge};
pub use graph::{
    GraphBuilder, GraphEdge, GraphError, GraphFilters, GraphKind, GraphNode, GraphPayload,
    GraphRequest,
};
pub use incremental::IncrementalStats;
pub use xml_catalog::{
    discover_workspace_catalogs, load_workspace_xml_catalogs, load_xml_catalog, parse_xml_catalog,
    parse_xml_catalog_in_workspace, XmlCatalog, XmlCatalogError,
};

use serde::Serialize;
use strixonomy_core::{
    Annotation, Axiom, Diagnostic, Entity, Import, Namespace, OntologyDocument, ParseStatus,
};

#[derive(Debug, Clone, Serialize)]
pub struct CatalogStats {
    pub ontology_count: usize,
    pub class_count: usize,
    pub object_property_count: usize,
    pub data_property_count: usize,
    pub annotation_property_count: usize,
    pub individual_count: usize,
    pub axiom_count: usize,
    pub annotation_count: usize,
    pub triple_count: usize,
    pub error_count: usize,
    pub diagnostic_error_count: usize,
    pub diagnostic_warning_count: usize,
    pub diagnostic_info_count: usize,
}

#[derive(Debug, Clone)]
pub struct OntologyCatalogData {
    pub documents: Vec<OntologyDocument>,
    pub entities: Vec<Entity>,
    pub annotations: Vec<Annotation>,
    pub axioms: Vec<Axiom>,
    pub namespaces: Vec<Namespace>,
    pub imports: Vec<Import>,
    pub triple_count: usize,
    pub diagnostics: Vec<Diagnostic>,
}

impl OntologyCatalogData {
    pub fn stats(&self) -> CatalogStats {
        CatalogStats {
            ontology_count: self.documents.len(),
            class_count: self
                .entities
                .iter()
                .filter(|e| e.kind == strixonomy_core::EntityKind::Class)
                .count(),
            object_property_count: self
                .entities
                .iter()
                .filter(|e| e.kind == strixonomy_core::EntityKind::ObjectProperty)
                .count(),
            data_property_count: self
                .entities
                .iter()
                .filter(|e| e.kind == strixonomy_core::EntityKind::DataProperty)
                .count(),
            annotation_property_count: self
                .entities
                .iter()
                .filter(|e| e.kind == strixonomy_core::EntityKind::AnnotationProperty)
                .count(),
            individual_count: self
                .entities
                .iter()
                .filter(|e| e.kind == strixonomy_core::EntityKind::Individual)
                .count(),
            axiom_count: self.axioms.len(),
            annotation_count: self.annotations.len(),
            triple_count: self.triple_count,
            error_count: self
                .documents
                .iter()
                .filter(|d| d.parse_status == ParseStatus::Error)
                .count(),
            diagnostic_error_count: self
                .diagnostics
                .iter()
                .filter(|d| d.severity == strixonomy_core::DiagnosticSeverity::Error)
                .count(),
            diagnostic_warning_count: self
                .diagnostics
                .iter()
                .filter(|d| d.severity == strixonomy_core::DiagnosticSeverity::Warning)
                .count(),
            diagnostic_info_count: self
                .diagnostics
                .iter()
                .filter(|d| d.severity == strixonomy_core::DiagnosticSeverity::Info)
                .count(),
        }
    }
}
