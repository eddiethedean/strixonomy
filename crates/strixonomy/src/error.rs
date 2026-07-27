//! Unified error type for common Strixonomy façade operations.

use thiserror::Error;

/// Aggregates errors from Strixonomy façade modules for embedders that prefer one enum.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Catalog(#[from] strixonomy_catalog::CatalogError),
    #[error(transparent)]
    Query(#[from] strixonomy_query::QueryError),
    #[error(transparent)]
    Graph(#[from] strixonomy_catalog::GraphError),
    #[error(transparent)]
    Reasoner(#[from] strixonomy_reasoner::ReasonerError),
    #[error(transparent)]
    Export(#[from] strixonomy_docs::ExportError),
    #[error(transparent)]
    Owl(#[from] strixonomy_owl::OwlError),
    #[error(transparent)]
    Obo(#[from] strixonomy_obo::OboError),
}
