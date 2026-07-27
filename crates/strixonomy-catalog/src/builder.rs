use crate::disk_cache::DiskCache;
use crate::incremental::{
    config_fingerprint, content_hash_text, effective_content_hash, paths_equal, DocumentSnapshot,
    IncrementalStats,
};
use crate::OntologyCatalogData;
use oxigraph::model::{BlankNode, GraphName, Quad, Subject, Term, Triple};
use oxigraph::store::Store;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use strixonomy_core::{
    limits::{MAX_ENTITIES, MAX_TOTAL_TRIPLES, MAX_TRIPLES_PER_FILE},
    read_to_string_capped, Annotation, Axiom, Diagnostic, DiagnosticCode, DiagnosticSeverity,
    Entity, EntityKind, Import, Namespace, OntologyDocument, OntologyFormat, ParseStatus,
    SourceLocation, WorkspaceScanner, MAX_FILE_BYTES,
};
use strixonomy_diagnostics::{collect_diagnostics_with_config, find_config, DiagnosticInput};
use strixonomy_owl::{load_owx_text, load_turtle_text, supports_horned_load};
use strixonomy_parser::{parse_ontology_file, parse_ontology_text, ParsedOntology};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CatalogError {
    #[error("core error: {0}")]
    Core(#[from] strixonomy_core::StrixonomyError),

    #[error("parse error in {path}: {message}")]
    Parse { path: PathBuf, message: String },

    #[error("store error: {0}")]
    Store(String),
}

pub type Result<T> = std::result::Result<T, CatalogError>;

pub struct IndexBuilder {
    workspace: PathBuf,
    scan_roots: Vec<PathBuf>,
    document_overrides: HashMap<PathBuf, String>,
    disk_cache: bool,
    only_paths: Option<Vec<PathBuf>>,
}

impl IndexBuilder {
    pub fn new() -> Self {
        Self {
            workspace: PathBuf::from("."),
            scan_roots: Vec::new(),
            document_overrides: HashMap::new(),
            disk_cache: false,
            only_paths: None,
        }
    }

    pub fn workspace(mut self, path: impl Into<PathBuf>) -> Self {
        self.workspace = path.into();
        self
    }

    /// Additional workspace roots to scan and merge into one catalog (multi-root).
    /// The primary [`workspace`](Self::workspace) root is always included.
    pub fn scan_roots(mut self, roots: Vec<PathBuf>) -> Self {
        self.scan_roots = roots;
        self
    }

    /// Roots scanned by [`Self::build`] — primary workspace plus any [`Self::scan_roots`].
    pub fn effective_scan_roots(workspace: &Path, extra_roots: &[PathBuf]) -> Vec<PathBuf> {
        merge_scan_roots(workspace, extra_roots)
    }

    /// Use in-memory text instead of disk for specific paths (LSP open buffers).
    pub fn document_overrides(mut self, overrides: HashMap<PathBuf, String>) -> Self {
        self.document_overrides = overrides;
        self
    }

    /// Enable persistent `.strixonomy/cache/` snapshots keyed by content hash.
    pub fn disk_cache(mut self, enabled: bool) -> Self {
        self.disk_cache = enabled;
        self
    }

    /// Restrict scanning to these paths (e.g. git-tracked files for diff worktree side).
    pub fn only_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.only_paths = Some(paths);
        self
    }

    fn document_override_text(&self, path: &Path) -> Option<&String> {
        self.document_overrides
            .get(path)
            .or_else(|| path.canonicalize().ok().and_then(|p| self.document_overrides.get(&p)))
    }

    fn config_fingerprint(&self) -> String {
        let scan_roots = merge_scan_roots(&self.workspace, &self.scan_roots);
        config_fingerprint(&scan_roots, self.disk_cache, &self.document_overrides)
    }

    pub fn build(self) -> Result<OntologyCatalog> {
        self.build_with_snapshots(None, false).map(|(catalog, _)| catalog)
    }

    /// Rebuild the catalog reusing unchanged documents from `previous` when content hashes match.
    pub fn build_incremental(self, previous: &OntologyCatalog) -> Result<OntologyCatalog> {
        if !paths_equal(&previous.workspace, &self.workspace) {
            return self.build();
        }
        if previous.config_fingerprint != self.config_fingerprint() {
            return self.build();
        }
        self.build_with_snapshots(Some(&previous.document_snapshots), true)
            .map(|(catalog, _)| catalog)
    }

    fn build_with_snapshots(
        self,
        previous_snapshots: Option<&HashMap<PathBuf, DocumentSnapshot>>,
        incremental: bool,
    ) -> Result<(OntologyCatalog, IncrementalStats)> {
        let scan_roots = merge_scan_roots(&self.workspace, &self.scan_roots);
        let mut files = Vec::new();
        let mut seen = std::collections::HashSet::new();
        if let Some(ref only) = self.only_paths {
            for path in only {
                if !path.is_file() {
                    continue;
                }
                let key = path.canonicalize().unwrap_or_else(|_| path.clone());
                if seen.insert(key.clone()) {
                    files.push(WorkspaceScanner::new(&self.workspace).describe_path(&key)?);
                }
            }
        } else {
            for root in scan_roots {
                for file in WorkspaceScanner::new(&root).scan()? {
                    let key = file.path.canonicalize().unwrap_or_else(|_| file.path.clone());
                    if seen.insert(key) {
                        files.push(file);
                    }
                }
            }
        }

        let mut documents = Vec::new();
        let mut entities: Vec<Entity> = Vec::new();
        let mut entity_index: HashMap<String, usize> = HashMap::new();
        let mut entity_to_document: HashMap<String, usize> = HashMap::new();
        let mut document_entity_iris: Vec<Vec<String>> = Vec::new();
        let mut annotations = Vec::new();
        let mut axioms = Vec::new();
        let mut namespaces = Vec::new();
        let mut imports = Vec::new();
        let mut triple_count = 0usize;
        let store = Store::new().map_err(|e| CatalogError::Store(e.to_string()))?;

        let mut bridge_diagnostics = Vec::new();
        let mut document_snapshots = HashMap::new();
        let mut reused = 0usize;
        let mut reparsed = 0usize;

        let disk_cache = DiskCache::enabled(self.disk_cache, &self.workspace);
        let mut merged_previous = previous_snapshots.cloned().unwrap_or_default();
        if let Some(ref cache) = disk_cache {
            for file in &files {
                let override_text = self.document_override_text(&file.path);
                let effective_hash =
                    effective_content_hash(&file.content_hash, override_text.map(String::as_str));
                let lookup_path = file.path.canonicalize().unwrap_or_else(|_| file.path.clone());
                cache.hydrate_previous(
                    &mut merged_previous,
                    &lookup_path,
                    &effective_hash,
                    file.modified_time,
                );
            }
        }
        let previous_snapshots =
            if merged_previous.is_empty() { None } else { Some(&merged_previous) };

        for (idx, file) in files.iter().enumerate() {
            let doc_id = format!("doc-{}", idx + 1);
            let override_text = self.document_override_text(&file.path);
            let effective_hash =
                effective_content_hash(&file.content_hash, override_text.map(String::as_str));
            let lookup_path = file.path.canonicalize().unwrap_or_else(|_| file.path.clone());

            if let Some(prev) = previous_snapshots.and_then(|m| m.get(&lookup_path)) {
                if prev.content_hash == effective_hash {
                    if let Some((reuse_path, modified_time)) = verified_snapshot_source(
                        &self.workspace,
                        &file.path,
                        override_text.map(String::as_str),
                        &effective_hash,
                    ) {
                        let snap = prev.with_reuse_context(&doc_id, &reuse_path, modified_time);
                        apply_document_snapshot(
                            &snap,
                            &doc_id,
                            &mut documents,
                            &mut entities,
                            &mut entity_index,
                            &mut entity_to_document,
                            &mut document_entity_iris,
                            &mut annotations,
                            &mut axioms,
                            &mut namespaces,
                            &mut imports,
                            &mut triple_count,
                            &store,
                            &mut bridge_diagnostics,
                        )?;
                        document_snapshots.insert(lookup_path, snap);
                        reused += 1;
                        continue;
                    }
                }
            }

            reparsed += 1;
            let parsed = if let Some(text) = override_text {
                parse_ontology_text(&file.path, file.format, &doc_id, text, text.as_bytes())
                    .map_err(|e| CatalogError::Parse {
                        path: file.path.clone(),
                        message: e.to_string(),
                    })?
            } else {
                parse_ontology_file(
                    &file.path,
                    file.format,
                    &doc_id,
                    &file.content_hash,
                    file.modified_time,
                )
                .map_err(|e| CatalogError::Parse {
                    path: file.path.clone(),
                    message: e.to_string(),
                })?
            };

            // Load quads even when parse_status is Error (partial recovery after a trailing fault).
            if !parsed.quads().is_empty() {
                load_quads_into_store(&store, parsed.quads(), &mut triple_count, &doc_id)?;
            }

            documents.push(OntologyDocument {
                id: doc_id.clone(),
                path: file.path.clone(),
                format: file.format,
                base_iri: parsed.base_iri.clone(),
                version_iri: parsed.version_iri.clone(),
                imports: parsed.imports.clone(),
                namespaces: parsed.namespaces.clone(),
                parse_status: parsed.parse_status,
                content_hash: file.content_hash.clone(),
                modified_time: file.modified_time,
                parse_message: parsed.parse_message.clone(),
                parse_error_location: parsed.parse_error_location.clone(),
            });

            let doc_idx = documents.len() - 1;
            let mut doc_entity_iris = Vec::new();

            let semantics = semantics_for_document(
                &file.path,
                file.format,
                &doc_id,
                &parsed,
                self.document_override_text(&file.path),
            )?;

            // OWL/XML produces no parser quads; load Horned RDF projection for SPARQL (#75).
            let snapshot_quads = if !parsed.quads().is_empty() {
                parsed.quads().to_vec()
            } else if !semantics.rdf_quads.is_empty() {
                load_quads_into_store(&store, &semantics.rdf_quads, &mut triple_count, &doc_id)?;
                semantics.rdf_quads.clone()
            } else {
                Vec::new()
            };

            let stored_hash = if override_text.is_some() {
                effective_hash.clone()
            } else {
                file.content_hash.clone()
            };
            let snapshot = DocumentSnapshot {
                content_hash: stored_hash.clone(),
                document: documents.last().cloned().expect("document"),
                entities: semantics.entities.clone(),
                annotations: semantics.annotations.clone(),
                axioms: semantics.axioms.clone(),
                namespace_rows: semantics.namespace_rows.clone(),
                imports: semantics.imports.clone(),
                quads: snapshot_quads.clone(),
                triple_count: snapshot_quads.len(),
                bridge_warning: semantics.bridge_warning.clone(),
            };

            if let Some(diag) = semantics.bridge_warning {
                bridge_diagnostics.push(diag);
            }

            for entity in semantics.entities {
                if entities.len() >= MAX_ENTITIES && !entity_index.contains_key(&entity.iri) {
                    return Err(CatalogError::Core(strixonomy_core::StrixonomyError::Scanner(
                        format!("workspace exceeds maximum of {MAX_ENTITIES} entities"),
                    )));
                }
                if let Some(&prev_doc_idx) = entity_to_document.get(&entity.iri) {
                    if prev_doc_idx != doc_idx {
                        document_entity_iris[prev_doc_idx].retain(|iri| iri != &entity.iri);
                    }
                }
                entity_to_document.insert(entity.iri.clone(), doc_idx);
                doc_entity_iris.push(entity.iri.clone());
                if let Some(&existing_idx) = entity_index.get(&entity.iri) {
                    merge_entity(&mut entities[existing_idx], &entity);
                } else {
                    let idx = entities.len();
                    entity_index.insert(entity.iri.clone(), idx);
                    entities.push(entity);
                }
            }
            document_entity_iris.push(doc_entity_iris);
            annotations.extend(semantics.annotations);
            axioms.extend(semantics.axioms);
            namespaces.extend(semantics.namespace_rows);
            imports.extend(semantics.imports);

            if entities.len() > MAX_ENTITIES {
                return Err(CatalogError::Core(strixonomy_core::StrixonomyError::Scanner(
                    format!("workspace exceeds maximum of {MAX_ENTITIES} entities"),
                )));
            }

            if let Some(doc) = documents.last_mut() {
                doc.content_hash = stored_hash.clone();
            }
            if let Some(ref cache) = disk_cache {
                if let Err(e) = cache.store(&snapshot) {
                    bridge_diagnostics.push(Diagnostic {
                        code: DiagnosticCode::IoReadError,
                        severity: DiagnosticSeverity::Warning,
                        message: format!("disk cache write failed: {e}"),
                        file: lookup_path.clone(),
                        range: SourceLocation::default(),
                        entity_iri: None,
                        quick_fix: None,
                        plugin_id: None,
                        plugin_code: None,
                    });
                }
            }
            document_snapshots.insert(lookup_path, snapshot);
        }

        let previous_paths: HashMap<PathBuf, ()> = previous_snapshots
            .map(|m| m.keys().cloned().map(|p| (p, ())).collect())
            .unwrap_or_default();
        let mut removed = 0usize;
        for path in previous_paths.keys() {
            if !document_snapshots.contains_key(path) {
                removed += 1;
            }
        }

        for (override_path, override_text) in &self.document_overrides {
            if files.iter().any(|f| paths_equal(&f.path, override_path)) {
                continue;
            }
            let format = OntologyFormat::from_extension(
                override_path.extension().and_then(|e| e.to_str()).unwrap_or("ttl"),
            );
            if matches!(format, OntologyFormat::Unknown) {
                continue;
            }
            let doc_id = format!("doc-{}", documents.len() + 1);
            let lookup_path =
                override_path.canonicalize().unwrap_or_else(|_| override_path.clone());
            let effective_hash = content_hash_text(override_text);

            if let Some(prev) = previous_snapshots.and_then(|m| m.get(&lookup_path)) {
                if prev.content_hash == effective_hash {
                    if let Some((reuse_path, modified_time)) = verified_snapshot_source(
                        &self.workspace,
                        override_path,
                        Some(override_text.as_str()),
                        &effective_hash,
                    ) {
                        let snap = prev.with_reuse_context(&doc_id, &reuse_path, modified_time);
                        apply_document_snapshot(
                            &snap,
                            &doc_id,
                            &mut documents,
                            &mut entities,
                            &mut entity_index,
                            &mut entity_to_document,
                            &mut document_entity_iris,
                            &mut annotations,
                            &mut axioms,
                            &mut namespaces,
                            &mut imports,
                            &mut triple_count,
                            &store,
                            &mut bridge_diagnostics,
                        )?;
                        document_snapshots.insert(lookup_path, snap);
                        reused += 1;
                        continue;
                    }
                }
            }

            reparsed += 1;
            let parsed = parse_ontology_text(
                override_path,
                format,
                &doc_id,
                override_text,
                override_text.as_bytes(),
            )
            .map_err(|e| CatalogError::Parse {
                path: override_path.clone(),
                message: e.to_string(),
            })?;

            // Load quads even when parse_status is Error (partial recovery after a trailing fault).
            if !parsed.quads().is_empty() {
                load_quads_into_store(&store, parsed.quads(), &mut triple_count, &doc_id)?;
            }

            documents.push(OntologyDocument {
                id: doc_id.clone(),
                path: override_path.clone(),
                format,
                base_iri: parsed.base_iri.clone(),
                version_iri: parsed.version_iri.clone(),
                imports: parsed.imports.clone(),
                namespaces: parsed.namespaces.clone(),
                parse_status: parsed.parse_status,
                content_hash: effective_hash.clone(),
                modified_time: 0,
                parse_message: parsed.parse_message.clone(),
                parse_error_location: parsed.parse_error_location.clone(),
            });

            let doc_idx = documents.len() - 1;
            let mut doc_entity_iris = Vec::new();

            let semantics = semantics_for_document(
                override_path,
                format,
                &doc_id,
                &parsed,
                Some(override_text),
            )?;

            let snapshot_quads = if !parsed.quads().is_empty() {
                parsed.quads().to_vec()
            } else if !semantics.rdf_quads.is_empty() {
                load_quads_into_store(&store, &semantics.rdf_quads, &mut triple_count, &doc_id)?;
                semantics.rdf_quads.clone()
            } else {
                Vec::new()
            };

            let snapshot = DocumentSnapshot {
                content_hash: effective_hash.clone(),
                document: documents.last().cloned().expect("document"),
                entities: semantics.entities.clone(),
                annotations: semantics.annotations.clone(),
                axioms: semantics.axioms.clone(),
                namespace_rows: semantics.namespace_rows.clone(),
                imports: semantics.imports.clone(),
                quads: snapshot_quads.clone(),
                triple_count: snapshot_quads.len(),
                bridge_warning: semantics.bridge_warning.clone(),
            };

            if let Some(diag) = semantics.bridge_warning {
                bridge_diagnostics.push(diag);
            }

            for entity in semantics.entities {
                if entities.len() >= MAX_ENTITIES && !entity_index.contains_key(&entity.iri) {
                    return Err(CatalogError::Core(strixonomy_core::StrixonomyError::Scanner(
                        format!("workspace exceeds maximum of {MAX_ENTITIES} entities"),
                    )));
                }
                if let Some(&prev_doc_idx) = entity_to_document.get(&entity.iri) {
                    if prev_doc_idx != doc_idx {
                        document_entity_iris[prev_doc_idx].retain(|iri| iri != &entity.iri);
                    }
                }
                entity_to_document.insert(entity.iri.clone(), doc_idx);
                doc_entity_iris.push(entity.iri.clone());
                if let Some(&existing_idx) = entity_index.get(&entity.iri) {
                    merge_entity(&mut entities[existing_idx], &entity);
                } else {
                    let idx = entities.len();
                    entity_index.insert(entity.iri.clone(), idx);
                    entities.push(entity);
                }
            }
            document_entity_iris.push(doc_entity_iris);
            annotations.extend(semantics.annotations);
            axioms.extend(semantics.axioms);
            namespaces.extend(semantics.namespace_rows);
            imports.extend(semantics.imports);

            if entities.len() > MAX_ENTITIES {
                return Err(CatalogError::Core(strixonomy_core::StrixonomyError::Scanner(
                    format!("workspace exceeds maximum of {MAX_ENTITIES} entities"),
                )));
            }

            document_snapshots.insert(lookup_path.clone(), snapshot);
            if let Some(ref cache) = disk_cache {
                if let Err(e) = cache.store(document_snapshots.get(&lookup_path).expect("snapshot"))
                {
                    bridge_diagnostics.push(Diagnostic {
                        code: DiagnosticCode::IoReadError,
                        severity: DiagnosticSeverity::Warning,
                        message: format!("disk cache write failed: {e}"),
                        file: lookup_path.clone(),
                        range: SourceLocation::default(),
                        entity_iri: None,
                        quick_fix: None,
                        plugin_id: None,
                        plugin_code: None,
                    });
                }
            }
        }

        let stats = if incremental || reused > 0 || removed > 0 {
            IncrementalStats::Incremental { reused, reparsed, removed }
        } else {
            IncrementalStats::FullBuild
        };

        let config_fingerprint = self.config_fingerprint();
        let mut data = OntologyCatalogData {
            documents,
            entities,
            annotations,
            axioms,
            namespaces,
            imports,
            triple_count,
            diagnostics: Vec::new(),
        };
        let lint_input = DiagnosticInput {
            documents: &data.documents,
            entities: &data.entities,
            annotations: &data.annotations,
            axioms: &data.axioms,
            namespaces: &data.namespaces,
            imports: &data.imports,
        };
        let diag_config = find_config(&self.workspace);
        data.diagnostics = collect_diagnostics_with_config(
            &lint_input,
            &self.document_overrides,
            diag_config.as_ref(),
        );
        data.diagnostics.extend(bridge_diagnostics);

        if let Some(ref cache) = disk_cache {
            let live_hashes: std::collections::HashSet<String> =
                document_snapshots.values().map(|s| s.content_hash.clone()).collect();
            if let Err(e) = cache.prune(&live_hashes) {
                data.diagnostics.push(Diagnostic {
                    code: DiagnosticCode::IoReadError,
                    severity: DiagnosticSeverity::Warning,
                    message: format!("disk cache prune failed: {e}"),
                    file: strixonomy_core::cache_dir(&self.workspace).join("snapshots"),
                    range: SourceLocation::default(),
                    entity_iri: None,
                    quick_fix: None,
                    plugin_id: None,
                    plugin_code: None,
                });
            }
        }

        Ok((
            OntologyCatalog {
                workspace: self.workspace,
                config_fingerprint,
                data,
                store,
                entity_to_document,
                document_entity_iris,
                document_snapshots,
            },
            stats,
        ))
    }
}

impl Default for IndexBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OntologyCatalog {
    workspace: PathBuf,
    pub(crate) config_fingerprint: String,
    data: OntologyCatalogData,
    store: Store,
    /// Entity IRI → index in [`OntologyCatalogData::documents`].
    pub(crate) entity_to_document: HashMap<String, usize>,
    /// Entity IRIs declared per document (parallel to `documents`).
    pub(crate) document_entity_iris: Vec<Vec<String>>,
    /// Per-file snapshots for incremental reindex (keyed by canonical path).
    pub(crate) document_snapshots: HashMap<PathBuf, DocumentSnapshot>,
}

impl OntologyCatalog {
    pub fn workspace(&self) -> &Path {
        &self.workspace
    }

    pub fn data(&self) -> &OntologyCatalogData {
        &self.data
    }

    /// Oxigraph triple store for SPARQL — not a stable public API; use [`strixonomy_query::sparql_catalog`].
    #[doc(hidden)]
    pub fn store(&self) -> &Store {
        &self.store
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_document_snapshot(
    snap: &DocumentSnapshot,
    doc_id: &str,
    documents: &mut Vec<OntologyDocument>,
    entities: &mut Vec<Entity>,
    entity_index: &mut HashMap<String, usize>,
    entity_to_document: &mut HashMap<String, usize>,
    document_entity_iris: &mut Vec<Vec<String>>,
    annotations: &mut Vec<Annotation>,
    axioms: &mut Vec<Axiom>,
    namespaces: &mut Vec<Namespace>,
    imports: &mut Vec<Import>,
    triple_count: &mut usize,
    store: &Store,
    bridge_diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    if snap.triple_count != snap.quads.len() {
        return Err(CatalogError::Core(strixonomy_core::StrixonomyError::Scanner(
            "document snapshot triple_count does not match quads length".to_string(),
        )));
    }
    if !snap.quads.is_empty() {
        load_quads_into_store(store, &snap.quads, triple_count, doc_id)?;
    }

    documents.push(snap.document.clone());
    let doc_idx = documents.len() - 1;
    let mut doc_entity_iris = Vec::new();

    if let Some(diag) = &snap.bridge_warning {
        bridge_diagnostics.push(diag.clone());
    }

    for entity in &snap.entities {
        if entities.len() >= MAX_ENTITIES && !entity_index.contains_key(&entity.iri) {
            return Err(CatalogError::Core(strixonomy_core::StrixonomyError::Scanner(format!(
                "workspace exceeds maximum of {MAX_ENTITIES} entities"
            ))));
        }
        if let Some(&prev_doc_idx) = entity_to_document.get(&entity.iri) {
            if prev_doc_idx != doc_idx {
                document_entity_iris[prev_doc_idx].retain(|iri| iri != &entity.iri);
            }
        }
        entity_to_document.insert(entity.iri.clone(), doc_idx);
        doc_entity_iris.push(entity.iri.clone());
        if let Some(&existing_idx) = entity_index.get(&entity.iri) {
            merge_entity(&mut entities[existing_idx], entity);
        } else {
            let idx = entities.len();
            entity_index.insert(entity.iri.clone(), idx);
            entities.push(entity.clone());
        }
    }
    document_entity_iris.push(doc_entity_iris);
    annotations.extend(snap.annotations.clone());
    axioms.extend(snap.axioms.clone());
    namespaces.extend(snap.namespace_rows.clone());
    imports.extend(snap.imports.clone());

    let _ = doc_id;
    Ok(())
}

fn merge_entity(existing: &mut Entity, incoming: &Entity) {
    for label in &incoming.labels {
        if !existing.labels.contains(label) {
            existing.labels.push(label.clone());
        }
    }
    for comment in &incoming.comments {
        if !existing.comments.contains(comment) {
            existing.comments.push(comment.clone());
        }
    }
    existing.deprecated |= incoming.deprecated;
    existing.ontology_id = incoming.ontology_id.clone();
    if existing.short_name.is_empty() {
        existing.short_name = incoming.short_name.clone();
    }
    // Preserve OBO term ids across multi-document IRI merges. Prefer an existing
    // value when both sides set one (first-wins); otherwise take the non-empty side.
    if existing.obo_id.is_none() {
        existing.obo_id = incoming.obo_id.clone();
    }
    // OR-merge property characteristics declared across documents.
    existing.characteristics.functional |= incoming.characteristics.functional;
    existing.characteristics.inverse_functional |= incoming.characteristics.inverse_functional;
    existing.characteristics.transitive |= incoming.characteristics.transitive;
    existing.characteristics.symmetric |= incoming.characteristics.symmetric;
    existing.characteristics.asymmetric |= incoming.characteristics.asymmetric;
    existing.characteristics.reflexive |= incoming.characteristics.reflexive;
    existing.characteristics.irreflexive |= incoming.characteristics.irreflexive;
    // Prefer a more specific kind when the existing entry is Other.
    if existing.kind == EntityKind::Other && incoming.kind != EntityKind::Other {
        existing.kind = incoming.kind;
    }
    if existing.source_location.line.is_none() && incoming.source_location.line.is_some() {
        existing.source_location = incoming.source_location.clone();
    }
}

fn incomplete_bridge_warning(path: &Path, message: String) -> Diagnostic {
    Diagnostic {
        code: DiagnosticCode::OwlBridgeFailed,
        severity: DiagnosticSeverity::Warning,
        message,
        file: path.to_path_buf(),
        range: SourceLocation::default(),
        entity_iri: None,
        quick_fix: None,
        plugin_id: None,
        plugin_code: None,
    }
}

struct DocumentSemantics {
    entities: Vec<Entity>,
    annotations: Vec<Annotation>,
    axioms: Vec<Axiom>,
    namespace_rows: Vec<Namespace>,
    imports: Vec<Import>,
    bridge_warning: Option<Diagnostic>,
    /// Extra RDF quads when the parser produced none (OWL/XML → Horned RDF projection).
    rdf_quads: Vec<oxigraph::model::Quad>,
}

fn semantics_for_document(
    path: &Path,
    format: OntologyFormat,
    doc_id: &str,
    parsed: &ParsedOntology,
    override_text: Option<&String>,
) -> Result<DocumentSemantics> {
    if parsed.parse_status == ParseStatus::Error || !supports_horned_load(format) {
        return Ok(DocumentSemantics {
            entities: parsed.entities.clone(),
            annotations: parsed.annotations.clone(),
            axioms: parsed.axioms.clone(),
            namespace_rows: parsed.namespace_rows.clone(),
            imports: parsed.import_rows.clone(),
            bridge_warning: None,
            rdf_quads: Vec::new(),
        });
    }

    let source_text = if let Some(text) = override_text {
        text.clone()
    } else {
        read_to_string_capped(path, MAX_FILE_BYTES).map_err(CatalogError::Core)?
    };

    if format == OntologyFormat::OwlXml {
        return match load_owx_text(path, doc_id, &source_text, &parsed.namespaces) {
            Ok(owl) => Ok(DocumentSemantics {
                entities: owl.bridge.entities,
                annotations: owl.bridge.annotations,
                axioms: owl.bridge.axioms,
                namespace_rows: owl.bridge.namespace_rows,
                imports: owl.bridge.imports,
                bridge_warning: owl
                    .load_warning
                    .map(|message| incomplete_bridge_warning(path, message)),
                rdf_quads: owl.quads,
            }),
            Err(e) => {
                eprintln!(
                    "strixonomy-catalog: Horned-OWL OWX load failed for {}: {e}; using parser entities",
                    path.display()
                );
                Ok(DocumentSemantics {
                    entities: parsed.entities.clone(),
                    annotations: parsed.annotations.clone(),
                    axioms: parsed.axioms.clone(),
                    namespace_rows: parsed.namespace_rows.clone(),
                    imports: parsed.import_rows.clone(),
                    bridge_warning: Some(Diagnostic {
                        code: DiagnosticCode::OwlBridgeFailed,
                        severity: DiagnosticSeverity::Warning,
                        message: format!(
                            "Horned-OWL OWL/XML bridge failed; using parser-only entities and axioms: {e}"
                        ),
                        file: path.to_path_buf(),
                        range: SourceLocation::default(),
                        entity_iri: None,
                        quick_fix: None,
                        plugin_id: None,
                        plugin_code: None,
                    }),
                    rdf_quads: Vec::new(),
                })
            }
        };
    }

    match load_turtle_text(path, doc_id, &source_text, parsed.quads(), &parsed.namespaces) {
        Ok(owl) => Ok(DocumentSemantics {
            entities: owl.bridge.entities,
            annotations: owl.bridge.annotations,
            axioms: owl.bridge.axioms,
            namespace_rows: owl.bridge.namespace_rows,
            imports: owl.bridge.imports,
            bridge_warning: owl
                .load_warning
                .map(|message| incomplete_bridge_warning(path, message)),
            rdf_quads: Vec::new(),
        }),
        Err(e) => {
            eprintln!(
                "strixonomy-catalog: Horned-OWL load failed for {}: {e}; using parser entities",
                path.display()
            );
            Ok(DocumentSemantics {
                entities: parsed.entities.clone(),
                annotations: parsed.annotations.clone(),
                axioms: parsed.axioms.clone(),
                namespace_rows: parsed.namespace_rows.clone(),
                imports: parsed.import_rows.clone(),
                bridge_warning: Some(Diagnostic {
                    code: DiagnosticCode::OwlBridgeFailed,
                    severity: DiagnosticSeverity::Warning,
                    message: format!(
                        "Horned-OWL bridge failed; using parser-only entities and axioms: {e}"
                    ),
                    file: path.to_path_buf(),
                    range: SourceLocation::default(),
                    entity_iri: None,
                    quick_fix: None,
                    plugin_id: None,
                    plugin_code: None,
                }),
                rdf_quads: Vec::new(),
            })
        }
    }
}

/// Insert document quads into the shared Oxigraph store, remapping blank nodes so
/// parser-local IDs (often `b0`, `b1`, …) cannot collide across files.
fn load_quads_into_store(
    store: &Store,
    quads: &[Quad],
    triple_count: &mut usize,
    doc_id: &str,
) -> Result<()> {
    let mut blank_map = HashMap::new();
    let mut file_triples = 0usize;
    for quad in quads {
        file_triples += 1;
        if file_triples > MAX_TRIPLES_PER_FILE {
            return Err(CatalogError::Core(strixonomy_core::StrixonomyError::Scanner(format!(
                "file exceeds {MAX_TRIPLES_PER_FILE} triples"
            ))));
        }
        *triple_count += 1;
        if *triple_count > MAX_TOTAL_TRIPLES {
            return Err(CatalogError::Core(strixonomy_core::StrixonomyError::Scanner(format!(
                "workspace exceeds maximum of {MAX_TOTAL_TRIPLES} triples"
            ))));
        }
        let remapped = remap_quad(quad, doc_id, &mut blank_map);
        store.insert(&remapped).map_err(|e| CatalogError::Store(e.to_string()))?;
    }
    Ok(())
}

fn blank_id_prefix(doc_id: &str) -> String {
    let sanitized: String =
        doc_id.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    if sanitized.is_empty() {
        "doc".to_string()
    } else {
        sanitized
    }
}

fn remap_blank_node(
    node: &BlankNode,
    doc_id: &str,
    blank_map: &mut HashMap<BlankNode, BlankNode>,
) -> BlankNode {
    blank_map
        .entry(node.clone())
        .or_insert_with(|| {
            let id = format!("{}_{}", blank_id_prefix(doc_id), node.as_str());
            BlankNode::new_unchecked(id)
        })
        .clone()
}

fn remap_quad(quad: &Quad, doc_id: &str, blank_map: &mut HashMap<BlankNode, BlankNode>) -> Quad {
    Quad {
        subject: remap_subject(&quad.subject, doc_id, blank_map),
        predicate: quad.predicate.clone(),
        object: remap_term(&quad.object, doc_id, blank_map),
        graph_name: remap_graph_name(&quad.graph_name, doc_id, blank_map),
    }
}

fn remap_subject(
    subject: &Subject,
    doc_id: &str,
    blank_map: &mut HashMap<BlankNode, BlankNode>,
) -> Subject {
    match subject {
        Subject::NamedNode(n) => Subject::NamedNode(n.clone()),
        Subject::BlankNode(b) => Subject::BlankNode(remap_blank_node(b, doc_id, blank_map)),
        Subject::Triple(t) => Subject::Triple(Box::new(remap_triple(t, doc_id, blank_map))),
    }
}

fn remap_term(term: &Term, doc_id: &str, blank_map: &mut HashMap<BlankNode, BlankNode>) -> Term {
    match term {
        Term::NamedNode(n) => Term::NamedNode(n.clone()),
        Term::BlankNode(b) => Term::BlankNode(remap_blank_node(b, doc_id, blank_map)),
        Term::Literal(l) => Term::Literal(l.clone()),
        Term::Triple(t) => Term::Triple(Box::new(remap_triple(t, doc_id, blank_map))),
    }
}

fn remap_triple(
    triple: &Triple,
    doc_id: &str,
    blank_map: &mut HashMap<BlankNode, BlankNode>,
) -> Triple {
    Triple {
        subject: remap_subject(&triple.subject, doc_id, blank_map),
        predicate: triple.predicate.clone(),
        object: remap_term(&triple.object, doc_id, blank_map),
    }
}

fn remap_graph_name(
    graph: &GraphName,
    doc_id: &str,
    blank_map: &mut HashMap<BlankNode, BlankNode>,
) -> GraphName {
    match graph {
        GraphName::NamedNode(n) => GraphName::NamedNode(n.clone()),
        GraphName::BlankNode(b) => GraphName::BlankNode(remap_blank_node(b, doc_id, blank_map)),
        GraphName::DefaultGraph => GraphName::DefaultGraph,
    }
}

fn verified_snapshot_source(
    workspace: &Path,
    path: &Path,
    override_text: Option<&str>,
    effective_hash: &str,
) -> Option<(PathBuf, u64)> {
    if let Some(text) = override_text {
        if content_hash_text(text) != effective_hash {
            return None;
        }
        let modified_time = std::fs::metadata(path)
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        return Some((path.to_path_buf(), modified_time));
    }
    if !path.exists() {
        return None;
    }
    let file = WorkspaceScanner::new(workspace).describe_path(path).ok()?;
    if file.content_hash != effective_hash {
        return None;
    }
    // Re-verify immediately before reuse to close TOCTOU between hash check and apply.
    let file = WorkspaceScanner::new(workspace).describe_path(path).ok()?;
    if file.content_hash == effective_hash {
        Some((file.path, file.modified_time))
    } else {
        None
    }
}

/// Primary workspace root plus any additional scan roots (deduplicated).
pub(crate) fn merge_scan_roots(workspace: &Path, extra_roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut roots = vec![workspace.to_path_buf()];
    for root in extra_roots {
        if roots.iter().any(|existing| paths_equal(existing, root)) {
            continue;
        }
        roots.push(root.clone());
    }
    roots
}

#[cfg(test)]
mod merge_entity_tests {
    use super::*;
    use strixonomy_core::EntityKind;

    fn entity(obo_id: Option<&str>) -> Entity {
        Entity {
            iri: "http://example.org/A".into(),
            short_name: "A".into(),
            kind: EntityKind::Class,
            ontology_id: "doc".into(),
            obo_id: obo_id.map(str::to_string),
            ..Default::default()
        }
    }

    #[test]
    fn merge_entity_takes_incoming_obo_id_when_missing() {
        let mut existing = entity(None);
        merge_entity(&mut existing, &entity(Some("TEST:0000001")));
        assert_eq!(existing.obo_id.as_deref(), Some("TEST:0000001"));
    }

    #[test]
    fn merge_entity_keeps_existing_obo_id_when_incoming_missing() {
        let mut existing = entity(Some("TEST:0000001"));
        merge_entity(&mut existing, &entity(None));
        assert_eq!(existing.obo_id.as_deref(), Some("TEST:0000001"));
    }

    #[test]
    fn merge_entity_keeps_first_obo_id_on_conflict() {
        let mut existing = entity(Some("TEST:0000001"));
        merge_entity(&mut existing, &entity(Some("TEST:0000002")));
        assert_eq!(existing.obo_id.as_deref(), Some("TEST:0000001"));
    }

    #[test]
    fn merge_entity_or_merges_property_characteristics() {
        let mut existing = Entity {
            iri: "http://example.org/p".into(),
            short_name: "p".into(),
            kind: EntityKind::ObjectProperty,
            ontology_id: "doc-a".into(),
            characteristics: strixonomy_core::PropertyCharacteristics {
                functional: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let incoming = Entity {
            iri: "http://example.org/p".into(),
            short_name: "p".into(),
            kind: EntityKind::ObjectProperty,
            ontology_id: "doc-b".into(),
            characteristics: strixonomy_core::PropertyCharacteristics {
                transitive: true,
                ..Default::default()
            },
            ..Default::default()
        };
        merge_entity(&mut existing, &incoming);
        assert!(existing.characteristics.functional);
        assert!(existing.characteristics.transitive);
        assert!(!existing.characteristics.symmetric);
    }

    #[test]
    fn index_merge_preserves_obo_id_from_later_document() {
        let dir = tempfile::tempdir().unwrap();
        // Turtle declares the IRI first without an OBO id.
        std::fs::write(
            dir.path().join("a.ttl"),
            r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
<http://purl.obolibrary.org/obo/TEST_0000001> a owl:Class ;
    rdfs:label "From Turtle" .
"#,
        )
        .unwrap();
        // OBO document contributes the same IRI with an obo_id.
        std::fs::write(
            dir.path().join("b.obo"),
            "format-version: 1.2\nontology: test\n\n[Term]\nid: TEST:0000001\nname: From OBO\n",
        )
        .unwrap();

        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
        let entity = catalog
            .data()
            .entities
            .iter()
            .find(|e| e.iri.contains("TEST_0000001") || e.obo_id.as_deref() == Some("TEST:0000001"))
            .expect("merged entity");
        assert_eq!(
            entity.obo_id.as_deref(),
            Some("TEST:0000001"),
            "merged catalog entity must retain obo_id from the OBO document"
        );
        assert!(
            entity.labels.iter().any(|l| l == "From Turtle" || l == "From OBO"),
            "labels should still merge: {:?}",
            entity.labels
        );
    }
}

#[cfg(test)]
mod blank_remap_tests {
    use super::*;
    use oxigraph::model::NamedNode;

    #[test]
    fn remap_blank_nodes_are_scoped_per_document() {
        let blank = BlankNode::new_unchecked("b0");
        let mut map_a = HashMap::new();
        let mut map_b = HashMap::new();
        let a = remap_blank_node(&blank, "doc-1", &mut map_a);
        let b = remap_blank_node(&blank, "doc-2", &mut map_b);
        assert_ne!(a.as_str(), b.as_str());
        assert!(a.as_str().starts_with("doc_1_"));
        assert!(b.as_str().starts_with("doc_2_"));
        assert_eq!(remap_blank_node(&blank, "doc-1", &mut map_a).as_str(), a.as_str());
    }

    #[test]
    fn multi_file_restrictions_do_not_fuse_in_store() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("a.ttl"),
            r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix ex: <http://ex.org/a#> .
ex:A a owl:Class ;
  rdfs:subClassOf [ a owl:Restriction ; owl:onProperty ex:p1 ; owl:someValuesFrom ex:B ] .
ex:B a owl:Class .
ex:p1 a owl:ObjectProperty .
"#,
        )
        .unwrap();
        std::fs::write(
            dir.path().join("b.ttl"),
            r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix ex: <http://ex.org/b#> .
ex:X a owl:Class ;
  rdfs:subClassOf [ a owl:Restriction ; owl:onProperty ex:p2 ; owl:someValuesFrom ex:Y ] .
ex:Y a owl:Class .
ex:p2 a owl:ObjectProperty .
"#,
        )
        .unwrap();

        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
        let store = catalog.store();
        let on_property = NamedNode::new_unchecked("http://www.w3.org/2002/07/owl#onProperty");
        let some_values = NamedNode::new_unchecked("http://www.w3.org/2002/07/owl#someValuesFrom");
        let p1 = NamedNode::new_unchecked("http://ex.org/a#p1");
        let filler_b = NamedNode::new_unchecked("http://ex.org/a#B");
        let filler_y = NamedNode::new_unchecked("http://ex.org/b#Y");

        let mut fused = 0usize;
        let mut intact_a = 0usize;
        for quad in store.quads_for_pattern(
            None,
            Some(on_property.as_ref()),
            Some(p1.as_ref().into()),
            None,
        ) {
            let quad = quad.expect("store iterate");
            let fused_quad = Quad {
                subject: quad.subject.clone(),
                predicate: some_values.clone(),
                object: filler_y.clone().into(),
                graph_name: GraphName::DefaultGraph,
            };
            if store.contains(&fused_quad).unwrap_or(false) {
                fused += 1;
            }
            let intact_quad = Quad {
                subject: quad.subject.clone(),
                predicate: some_values.clone(),
                object: filler_b.clone().into(),
                graph_name: GraphName::DefaultGraph,
            };
            if store.contains(&intact_quad).unwrap_or(false) {
                intact_a += 1;
            }
        }
        assert_eq!(
            fused, 0,
            "cross-file blank collision would fuse a#p1 with b#Y on one restriction"
        );
        assert_eq!(intact_a, 1, "file A restriction should remain intact");
    }
}

#[cfg(test)]
mod merge_scan_roots_tests {
    use super::*;

    #[test]
    fn merge_scan_roots_includes_primary_when_extras_set() {
        let dir = tempfile::tempdir().unwrap();
        let primary = dir.path().join("ws");
        let extra = dir.path().join("imports");
        std::fs::create_dir_all(&primary).unwrap();
        std::fs::create_dir_all(&extra).unwrap();
        let merged = merge_scan_roots(&primary, std::slice::from_ref(&extra));
        assert_eq!(merged.len(), 2);
        assert!(merged.iter().any(|p| paths_equal(p, &primary)));
        assert!(merged.iter().any(|p| paths_equal(p, &extra)));
    }
}

#[cfg(test)]
mod incremental_tests {
    use super::*;

    #[test]
    fn incremental_rebuild_preserves_unchanged_documents() {
        let dir = tempfile::tempdir().unwrap();
        let ttl = dir.path().join("a.ttl");
        std::fs::write(
            &ttl,
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n@prefix ex: <http://ex/> .\nex:A a owl:Class .\n",
        )
        .unwrap();

        let first = IndexBuilder::new().workspace(dir.path()).build().expect("first build");
        let entity_count = first.data().entities.len();
        assert!(entity_count > 0);

        let second = IndexBuilder::new()
            .workspace(dir.path())
            .build_incremental(&first)
            .expect("incremental build");
        assert_eq!(second.data().entities.len(), entity_count);
        assert_eq!(second.data().documents.len(), 1);
    }

    #[test]
    fn incremental_rebuild_picks_up_edited_file() {
        let dir = tempfile::tempdir().unwrap();
        let ttl = dir.path().join("a.ttl");
        std::fs::write(
            &ttl,
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n@prefix ex: <http://ex/> .\nex:A a owl:Class .\n",
        )
        .unwrap();
        let first = IndexBuilder::new().workspace(dir.path()).build().expect("first build");

        std::fs::write(
            &ttl,
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n@prefix ex: <http://ex/> .\nex:A a owl:Class .\nex:B a owl:Class .\n",
        )
        .unwrap();

        let second = IndexBuilder::new()
            .workspace(dir.path())
            .build_incremental(&first)
            .expect("incremental build");
        assert!(
            second.data().entities.len() > first.data().entities.len(),
            "edited ontology should add entities"
        );
    }

    #[test]
    fn owx_workspace_populates_sparql_store() {
        let dir = tempfile::tempdir().unwrap();
        let owx = include_str!("../../../examples/protege-roundtrip/example.owx");
        std::fs::write(dir.path().join("example.owx"), owx).unwrap();

        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build owx");
        assert!(catalog.find_entity("http://example.org/org#Department").is_some());
        assert!(
            catalog.data().stats().triple_count > 0,
            "OWL/XML workspace must load RDF quads into the SPARQL store"
        );

        let mut found = false;
        for quad in catalog.store().quads_for_pattern(None, None, None, None) {
            let q = quad.expect("quad");
            if q.subject.to_string().contains("Department")
                || q.object.to_string().contains("Department")
            {
                found = true;
                break;
            }
        }
        assert!(found, "expected Department IRI in Oxigraph store");
    }

    #[test]
    fn owx_horned_load_failure_surfaces_bridge_diagnostic() {
        let dir = tempfile::tempdir().unwrap();
        // Recognized as OWL/XML by extension; Horned OWX reader must fail.
        std::fs::write(
            dir.path().join("broken.owx"),
            "<?xml version=\"1.0\"?>\n<NotAnOntology>garbage</NotAnOntology>\n",
        )
        .unwrap();

        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
        let has_bridge = catalog.data().diagnostics.iter().any(|d| {
            d.code == DiagnosticCode::OwlBridgeFailed
                && d.file.file_name().and_then(|n| n.to_str()) == Some("broken.owx")
        });
        assert!(
            has_bridge,
            "OWX Horned failure must emit OwlBridgeFailed; got {:?}",
            catalog
                .data()
                .diagnostics
                .iter()
                .map(|d| (&d.code, d.message.as_str()))
                .collect::<Vec<_>>()
        );
    }
}
