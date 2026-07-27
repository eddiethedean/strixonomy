//! High-level workspace API for Strixonomy (stable since v0.10).
//!
//! Prefer [`Workspace`] for embedding Strixonomy in applications. Use
//! [`crate::catalog::IndexBuilder`] when you need buffer overrides or custom
//! incremental rebuild logic.
//!
//! # Example
//!
//! ```
//! use strixonomy::Workspace;
//!
//! # fn demo() -> Result<(), Box<dyn std::error::Error>> {
//! let ws = Workspace::open(".")?;
//! let stats = ws.stats();
//! assert!(stats.class_count >= 0);
//! # Ok(())
//! # }
//! ```

use crate::catalog::{
    CatalogError, CatalogStats, ClassHierarchy, EntityDetail, GraphBuilder, GraphError,
    GraphFilters, GraphKind, GraphPayload, GraphRequest, IndexBuilder, OntologyCatalog,
};
use crate::diff::DiffResult;
use crate::docs::{export_workspace, ExportError, ExportOptions};
use crate::query::{query_catalog, sparql_catalog, QueryError, QueryResult, SparqlResult};
use crate::reasoner::{
    classify, explain, ClassificationResult, ExplanationRequest, ExplanationResult, ReasonerError,
    ReasonerId, ReasonerInput, WorkspaceInputLoader,
};
use std::path::{Path, PathBuf};
use strixonomy_core::Diagnostic;

/// Options for [`Workspace::open_with_options`].
///
/// Use [`WorkspaceOptions::single`] for a single root, or add scan roots for
/// multi-folder workspaces (same behavior as v0.10 LSP multi-root indexing).
#[derive(Debug, Clone)]
pub struct WorkspaceOptions {
    /// Primary workspace root (indexed catalog workspace path).
    pub root: PathBuf,
    /// Additional roots merged into one catalog (multi-root). Primary [`WorkspaceOptions::root`] is always scanned.
    pub scan_roots: Vec<PathBuf>,
    /// Persist parse snapshots under `.strixonomy/cache/`.
    pub disk_cache: bool,
}

impl WorkspaceOptions {
    pub fn single(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into(), scan_roots: Vec::new(), disk_cache: false }
    }

    pub fn with_disk_cache(mut self, enabled: bool) -> Self {
        self.disk_cache = enabled;
        self
    }

    pub fn with_scan_roots(mut self, roots: Vec<PathBuf>) -> Self {
        self.scan_roots = roots;
        self
    }
}

/// An indexed ontology workspace.
///
/// Open with [`Workspace::open`] or [`Workspace::open_with_options`], then query
/// via [`Self::query`], [`Self::sparql`], or inspect via [`Self::catalog`].
pub struct Workspace {
    options: WorkspaceOptions,
    catalog: OntologyCatalog,
    class_hierarchy: ClassHierarchy,
    /// Cached scan roots (primary + additional) used for indexing.
    effective_scan_roots: Vec<PathBuf>,
}

impl Workspace {
    /// Scan and index an ontology workspace on disk.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, CatalogError> {
        Self::open_with_options(WorkspaceOptions::single(path.as_ref().to_path_buf()))
    }

    /// Open with explicit roots and optional disk cache.
    pub fn open_with_options(options: WorkspaceOptions) -> Result<Self, CatalogError> {
        let effective_scan_roots =
            IndexBuilder::effective_scan_roots(&options.root, &options.scan_roots);
        let mut builder =
            IndexBuilder::new().workspace(options.root.clone()).disk_cache(options.disk_cache);
        if !options.scan_roots.is_empty() {
            builder = builder.scan_roots(options.scan_roots.clone());
        }
        let catalog = builder.build()?;
        let class_hierarchy = catalog.class_hierarchy();
        Ok(Self { options, catalog, class_hierarchy, effective_scan_roots })
    }

    /// Full rebuild from disk.
    pub fn reindex(&mut self) -> Result<CatalogStats, CatalogError> {
        let catalog = self.build_fresh()?;
        self.apply_catalog(catalog);
        Ok(self.stats())
    }

    /// Rebuild reusing unchanged documents when content hashes match.
    pub fn reindex_incremental(&mut self) -> Result<CatalogStats, CatalogError> {
        let previous = &self.catalog;
        let catalog = self.build_fresh_incremental(previous)?;
        self.apply_catalog(catalog);
        Ok(self.stats())
    }

    /// Primary workspace root.
    pub fn root(&self) -> &Path {
        &self.options.root
    }

    /// All scan roots (primary plus any configured additional roots).
    pub fn scan_roots(&self) -> &[PathBuf] {
        &self.effective_scan_roots
    }

    /// Indexed catalog (SQL, SPARQL, entity API).
    pub fn catalog(&self) -> &OntologyCatalog {
        &self.catalog
    }

    /// Catalog statistics without reaching into internal data structures.
    pub fn stats(&self) -> CatalogStats {
        self.catalog.data().stats()
    }

    /// Import dependency graph (default: import kind, depth 2).
    pub fn import_graph(&self) -> Result<GraphPayload, GraphError> {
        self.import_graph_with(&GraphRequest {
            graph_kind: GraphKind::Import.as_str().to_string(),
            root_iri: None,
            root_iris: vec![],
            depth: 2,
            include_inferred: false,
            filters: GraphFilters::default(),
        })
    }

    /// Import or other graph export with explicit [`GraphRequest`] parameters.
    pub fn import_graph_with(&self, request: &GraphRequest) -> Result<GraphPayload, GraphError> {
        GraphBuilder::new(&self.catalog).build(request)
    }

    /// Lint and parse diagnostics collected during indexing.
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        self.catalog.data().diagnostics.clone()
    }

    /// Run a SQL query against virtual ontology tables.
    pub fn query(&self, sql: &str) -> Result<QueryResult, QueryError> {
        query_catalog(&self.catalog, sql)
    }

    /// Run SPARQL against the indexed triple store.
    pub fn sparql(&self, sparql: &str) -> Result<SparqlResult, QueryError> {
        sparql_catalog(&self.catalog, sparql)
    }

    /// Semantic diff against another indexed workspace.
    pub fn diff(&self, other: &Workspace) -> DiffResult {
        crate::diff::diff_catalogs(self.catalog(), other.catalog())
    }

    /// Classify the workspace with an OntoLogos reasoner profile.
    pub fn classify(&self, profile: ReasonerId) -> Result<ClassificationResult, ReasonerError> {
        let input = self.reasoner_input()?;
        classify(profile, &input, profile == ReasonerId::Auto)
    }

    /// Explain an unsatisfiable class (or related axiom) with the given profile.
    pub fn explain(
        &self,
        profile: ReasonerId,
        request: &ExplanationRequest,
    ) -> Result<ExplanationResult, ReasonerError> {
        let input = self.reasoner_input()?;
        explain(profile, &input, request)
    }

    /// Export workspace documentation to Markdown or HTML.
    pub fn export_docs(&self, options: ExportOptions) -> Result<(), ExportError> {
        export_workspace(&self.catalog, options)
    }

    /// Build reasoner input from the indexed catalog and workspace roots.
    pub fn reasoner_input(&self) -> Result<ReasonerInput, ReasonerError> {
        WorkspaceInputLoader::new(self.options.root.clone())
            .scan_roots(self.options.scan_roots.clone())
            .load()
    }

    /// Discover plugin manifests under `.strixonomy/plugins/` (requires feature `plugins`).
    #[cfg(feature = "plugins")]
    pub fn discover_plugins(
        &self,
    ) -> Result<Vec<crate::plugin::DiscoveredPlugin>, crate::plugin::PluginDiscoveryError> {
        crate::plugin::discover_plugins(self.root())
    }

    /// Index `other` on disk and diff against this workspace.
    pub fn diff_against_path(&self, other: impl AsRef<Path>) -> Result<DiffResult, CatalogError> {
        let other_ws = Workspace::open(other)?;
        Ok(self.diff(&other_ws))
    }

    /// Search entities by IRI fragment, short name, or label (case-insensitive).
    pub fn search(&self, term: &str) -> Vec<EntityDetail> {
        let needle = term.to_ascii_lowercase();
        if needle.is_empty() {
            return Vec::new();
        }

        let mut matches: Vec<EntityDetail> = self
            .catalog
            .data()
            .entities
            .iter()
            .filter(|entity| entity_matches_term(entity, &needle))
            .filter_map(|entity| {
                self.catalog.entity_detail_with_hierarchy(&entity.iri, &self.class_hierarchy)
            })
            .collect();

        matches.sort_by(|a, b| a.entity.short_name.cmp(&b.entity.short_name));
        matches.dedup_by(|a, b| a.entity.iri == b.entity.iri);
        matches
    }

    fn build_fresh(&self) -> Result<OntologyCatalog, CatalogError> {
        let mut builder = IndexBuilder::new()
            .workspace(self.options.root.clone())
            .disk_cache(self.options.disk_cache);
        if !self.options.scan_roots.is_empty() {
            builder = builder.scan_roots(self.options.scan_roots.clone());
        }
        builder.build()
    }

    fn build_fresh_incremental(
        &self,
        previous: &OntologyCatalog,
    ) -> Result<OntologyCatalog, CatalogError> {
        let mut builder = IndexBuilder::new()
            .workspace(self.options.root.clone())
            .disk_cache(self.options.disk_cache);
        if !self.options.scan_roots.is_empty() {
            builder = builder.scan_roots(self.options.scan_roots.clone());
        }
        builder.build_incremental(previous)
    }

    fn apply_catalog(&mut self, catalog: OntologyCatalog) {
        self.class_hierarchy = catalog.class_hierarchy();
        self.catalog = catalog;
    }
}

fn entity_matches_term(entity: &strixonomy_core::Entity, needle: &str) -> bool {
    if entity.iri.to_ascii_lowercase().contains(needle) {
        return true;
    }
    if entity.short_name.to_ascii_lowercase().contains(needle) {
        return true;
    }
    if entity.obo_id.as_ref().is_some_and(|id| id.to_ascii_lowercase().contains(needle)) {
        return true;
    }
    entity.labels.iter().any(|label| label.to_ascii_lowercase().contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixtures_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
    }

    #[test]
    fn workspace_open_fixtures() {
        let ws = Workspace::open(fixtures_path()).expect("open fixtures");
        assert!(ws.stats().class_count > 0);
    }

    #[test]
    fn workspace_query_classes() {
        let ws = Workspace::open(fixtures_path()).expect("open fixtures");
        let result = ws.query("SELECT short_name FROM classes").expect("query");
        assert!(!result.rows.is_empty());
    }

    #[test]
    fn workspace_search_person() {
        let ws = Workspace::open(fixtures_path()).expect("open fixtures");
        let hits = ws.search("Person");
        assert!(!hits.is_empty());
    }

    #[test]
    fn workspace_import_graph_non_empty() {
        let ws = Workspace::open(fixtures_path()).expect("open fixtures");
        let graph = ws.import_graph().expect("import graph");
        assert!(!graph.nodes.is_empty());
    }
}
