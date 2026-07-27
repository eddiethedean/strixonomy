use lsp_server::RequestId;
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use strixonomy_catalog::{IndexBuilder, OntologyCatalog};
use strixonomy_core::{
    canonical_workspace_root, is_path_within_any,
    limits::{MAX_FILE_BYTES, MAX_OPEN_DOCUMENTS},
    validate_workspace_scope, validate_workspace_scope_any, Diagnostic, OntologyDocument,
};
use strixonomy_reasoner::{ReasonerCacheStore, ReasonerSnapshot};

/// Test-only: sleep this many ms in [`handle_run_reasoner`] after releasing `ops_lock`.
#[cfg(test)]
pub(crate) static TEST_REASONER_CLASSIFY_PAUSE_MS: AtomicU64 = AtomicU64::new(0);

/// Test-only: set while reasoner classify pause is in progress.
#[cfg(test)]
pub(crate) static TEST_REASONER_CLASSIFY_IN_PAUSE: AtomicBool = AtomicBool::new(false);

/// Serializes tests that mutate [`TEST_REASONER_CLASSIFY_PAUSE_MS`] / `_IN_PAUSE`.
#[cfg(test)]
pub(crate) static TEST_REASONER_CLASSIFY_LOCK: Mutex<()> = Mutex::new(());

/// Test-only: after snapshotting the previous catalog (lock released), sleep this many ms
/// before `build_incremental` so concurrent writers can prove they are not blocked.
#[cfg(test)]
pub(crate) static TEST_INCREMENTAL_BUILD_PAUSE_MS: AtomicU64 = AtomicU64::new(0);

/// Test-only: set while [`TEST_INCREMENTAL_BUILD_PAUSE_MS`] sleep is in progress.
#[cfg(test)]
pub(crate) static TEST_INCREMENTAL_BUILD_IN_PAUSE: AtomicBool = AtomicBool::new(false);

#[derive(Clone)]
pub struct ServerState {
    inner: Arc<RwLock<InnerState>>,
    /// Serializes catalog rebuilds and reasoner runs against the same workspace state.
    ops_lock: Arc<Mutex<()>>,
    /// Bumped when a reasoner run starts or is cancelled; stale runs must not update snapshot.
    reasoner_run_generation: Arc<AtomicU64>,
    /// LSP request id for the in-flight `strixonomy/runReasoner`, if any.
    active_reasoner_request_id: Arc<Mutex<Option<RequestId>>>,
    /// Engine-level cancel flag for the in-flight Ontologos classify/realize thread.
    reasoner_cancel_flag: Arc<Mutex<Option<Arc<AtomicBool>>>>,
    /// Workspace edits since last successful reasoner sync (incremental sync contract).
    reasoner_dirty: Arc<AtomicBool>,
}

struct InnerState {
    catalog: Option<Arc<OntologyCatalog>>,
    /// All LSP workspace folder roots (multi-root).
    workspace_roots: Vec<PathBuf>,
    /// Primary workspace root (first folder) for backward-compatible APIs.
    workspace: Option<PathBuf>,
    /// Last successfully indexed directory (may be a subdirectory of [`InnerState::workspace`]).
    indexed_workspace: Option<PathBuf>,
    indexed_at: Option<u64>,
    /// Open document text keyed by canonical file path (unsaved buffer overrides disk).
    open_documents: HashMap<PathBuf, String>,
    /// LSP textDocument versions for open buffers (for versioned WorkspaceEdits).
    document_versions: HashMap<PathBuf, i32>,
    reasoner_cache: ReasonerCacheStore,
    reasoner_snapshot: Option<ReasonerSnapshot>,
    explanation_cache: HashMap<(String, String, String), strixonomy_reasoner::ExplanationResult>,
    /// Insertion order for explanation cache eviction (oldest first).
    explanation_cache_order: Vec<(String, String, String)>,
    /// Diagnostics from workspace plugins (merged at publish time).
    plugin_diagnostics: Vec<Diagnostic>,
    /// URIs that received `publishDiagnostics` (for stale clears).
    published_diagnostic_uris: BTreeSet<String>,
    /// Persist `.strixonomy/cache/` during indexing when enabled via `strixonomy/indexWorkspace`.
    index_disk_cache: bool,
    /// Active ontology document id (path or ontology IRI) for new-axiom targeting.
    active_ontology_id: Option<String>,
}

/// Cap explanation results similarly to the reasoner classification cache.
const MAX_EXPLANATION_CACHE_ENTRIES: usize = 8;

impl ServerState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(InnerState {
                catalog: None,
                workspace_roots: Vec::new(),
                workspace: None,
                indexed_workspace: None,
                indexed_at: None,
                open_documents: HashMap::new(),
                document_versions: HashMap::new(),
                reasoner_cache: ReasonerCacheStore::new(),
                reasoner_snapshot: None,
                explanation_cache: HashMap::new(),
                explanation_cache_order: Vec::new(),
                plugin_diagnostics: Vec::new(),
                published_diagnostic_uris: BTreeSet::new(),
                index_disk_cache: false,
                active_ontology_id: None,
            })),
            ops_lock: Arc::new(Mutex::new(())),
            reasoner_run_generation: Arc::new(AtomicU64::new(0)),
            active_reasoner_request_id: Arc::new(Mutex::new(None)),
            reasoner_cancel_flag: Arc::new(Mutex::new(None)),
            reasoner_dirty: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start a new reasoner run; returns a generation token for stale-result checks.
    pub fn begin_reasoner_run(&self, request_id: RequestId) -> u64 {
        let gen = self.reasoner_run_generation.fetch_add(1, Ordering::SeqCst) + 1;
        if let Ok(mut guard) = self.active_reasoner_request_id.lock() {
            *guard = Some(request_id);
        }
        gen
    }

    pub fn clear_active_reasoner_request(&self) {
        if let Ok(mut guard) = self.active_reasoner_request_id.lock() {
            *guard = None;
        }
    }

    /// Invalidate the active reasoner run when the client sends `$/cancelRequest`.
    pub fn cancel_reasoner_request(&self, request_id: RequestId) -> bool {
        let should_cancel = self
            .active_reasoner_request_id
            .lock()
            .ok()
            .and_then(|g| g.as_ref().map(|active| active == &request_id))
            .unwrap_or(false);
        if should_cancel {
            self.reasoner_run_generation.fetch_add(1, Ordering::SeqCst);
            if let Ok(guard) = self.reasoner_cancel_flag.lock() {
                if let Some(flag) = guard.as_ref() {
                    flag.store(true, Ordering::Release);
                }
            }
        }
        should_cancel
    }

    pub fn set_reasoner_cancel_flag(&self, flag: Option<Arc<AtomicBool>>) {
        if let Ok(mut guard) = self.reasoner_cancel_flag.lock() {
            *guard = flag;
        }
    }

    pub fn mark_reasoner_dirty(&self) {
        self.reasoner_dirty.store(true, Ordering::SeqCst);
    }

    pub fn clear_reasoner_dirty(&self) {
        self.reasoner_dirty.store(false, Ordering::SeqCst);
    }

    #[allow(dead_code)]
    pub fn reasoner_is_dirty(&self) -> bool {
        self.reasoner_dirty.load(Ordering::SeqCst)
    }

    pub fn reasoner_run_is_current(&self, generation: u64) -> bool {
        self.reasoner_run_generation.load(Ordering::SeqCst) == generation
    }

    /// Bump the reasoner generation so in-flight classify cannot publish a stale snapshot.
    pub fn invalidate_in_flight_reasoner_runs(&self) {
        self.reasoner_run_generation.fetch_add(1, Ordering::SeqCst);
        self.mark_reasoner_dirty();
        if let Ok(guard) = self.reasoner_cancel_flag.lock() {
            if let Some(flag) = guard.as_ref() {
                flag.store(true, Ordering::Release);
            }
        }
    }

    pub fn active_ontology_id(&self) -> Option<String> {
        self.inner.read().ok()?.active_ontology_id.clone()
    }

    pub fn set_active_ontology_id(&self, id: Option<String>) {
        if let Ok(mut guard) = self.inner.write() {
            guard.active_ontology_id = id;
        }
    }

    pub fn ops_lock(&self) -> Arc<Mutex<()>> {
        Arc::clone(&self.ops_lock)
    }

    pub fn set_workspace_roots(&self, roots: Vec<PathBuf>) -> Result<(), String> {
        let _ops = self.ops_lock.lock().map_err(|e| e.to_string())?;
        let mut canonical = Vec::new();
        for root in roots {
            canonical.push(canonical_workspace_root(&root)?);
        }
        let mut guard = self.inner.write().map_err(|e| e.to_string())?;
        if canonical.is_empty() {
            let _ = clear_workspace_state_inner(&mut guard);
            drop(guard);
            self.invalidate_in_flight_reasoner_runs();
            return Ok(());
        }
        let primary = canonical[0].clone();
        on_workspace_roots_changed_inner(&mut guard, &canonical);
        guard.workspace_roots = canonical;
        guard.workspace = Some(primary);
        drop(guard);
        self.invalidate_in_flight_reasoner_runs();
        Ok(())
    }

    /// Drop all indexed state and open buffers (e.g. when every workspace folder is removed).
    /// Returns previously published diagnostic URIs so the caller can clear the Problems panel.
    pub fn clear_workspace_state(&self) -> BTreeSet<String> {
        let _ops = self.ops_lock.lock().ok();
        let uris = if let Ok(mut guard) = self.inner.write() {
            clear_workspace_state_inner(&mut guard)
        } else {
            BTreeSet::new()
        };
        self.invalidate_in_flight_reasoner_runs();
        uris
    }

    /// Invalidate catalog/reasoner and prune buffers outside the current roots.
    #[allow(dead_code)]
    pub fn on_workspace_roots_changed(&self) {
        let _ops = self.ops_lock.lock().ok();
        if let Ok(mut guard) = self.inner.write() {
            let roots = guard.workspace_roots.clone();
            if roots.is_empty() {
                let _ = clear_workspace_state_inner(&mut guard);
            } else {
                on_workspace_roots_changed_inner(&mut guard, &roots);
            }
        }
        self.invalidate_in_flight_reasoner_runs();
    }

    pub fn workspace_roots(&self) -> Vec<PathBuf> {
        self.inner.read().ok().map(|g| g.workspace_roots.clone()).unwrap_or_default()
    }

    pub fn set_index_disk_cache(&self, enabled: bool) {
        if let Ok(mut guard) = self.inner.write() {
            guard.index_disk_cache = enabled;
        }
    }

    pub fn index_workspace(
        &self,
        workspace: PathBuf,
    ) -> Result<(strixonomy_catalog::CatalogStats, u64), String> {
        let _guard = self.ops_lock.lock().map_err(|e| e.to_string())?;
        let workspace = canonical_workspace_root(&workspace)?;

        let roots = self.workspace_roots();
        if roots.is_empty() {
            return Err("workspace not initialized".to_string());
        }
        validate_workspace_scope_any(&workspace, &roots)?;

        let disk_cache = self.inner.read().map(|g| g.index_disk_cache).unwrap_or(false);
        let previous = {
            let guard = self.inner.read().map_err(|e| e.to_string())?;
            guard.catalog.clone()
        };
        let overrides = self.open_documents_snapshot();
        let builder = IndexBuilder::new()
            .workspace(workspace.clone())
            .scan_roots(roots.clone())
            .document_overrides(overrides)
            .disk_cache(disk_cache);
        // Build without holding `inner`: writers (`set_document_text`, etc.) must not block
        // for the duration of scan/parse. `ops_lock` already serializes index jobs.
        #[cfg(test)]
        if previous.is_some() {
            let pause_ms = TEST_INCREMENTAL_BUILD_PAUSE_MS.load(Ordering::SeqCst);
            if pause_ms > 0 {
                TEST_INCREMENTAL_BUILD_IN_PAUSE.store(true, Ordering::SeqCst);
                std::thread::sleep(std::time::Duration::from_millis(pause_ms));
                TEST_INCREMENTAL_BUILD_IN_PAUSE.store(false, Ordering::SeqCst);
            }
        }
        let catalog = if let Some(prev) = previous.as_deref() {
            builder.build_incremental(prev).map_err(|e| e.to_string())?
        } else {
            builder.build().map_err(|e| e.to_string())?
        };

        let stats = catalog.data().stats();
        let indexed_at = now_epoch_secs();

        let plugin_diags = strixonomy_plugin_builtins::load_plugin_host(&workspace)
            .map(|host| host.run_all_validators(&catalog))
            .unwrap_or_default();

        let mut guard = self.inner.write().map_err(|e| e.to_string())?;
        guard.catalog = Some(Arc::new(catalog));
        guard.indexed_workspace = Some(workspace);
        guard.indexed_at = Some(indexed_at);
        guard.reasoner_cache.invalidate();
        guard.reasoner_snapshot = None;
        guard.explanation_cache.clear();
        guard.explanation_cache_order.clear();
        guard.plugin_diagnostics = plugin_diags;
        drop(guard);
        // Patch/reindex must invalidate in-flight classify so it cannot republish H1 after H2.
        self.invalidate_in_flight_reasoner_runs();

        Ok((stats, indexed_at))
    }

    pub fn get_cached_explanation(
        &self,
        content_hash: &str,
        profile: &str,
        class_iri: &str,
    ) -> Option<strixonomy_reasoner::ExplanationResult> {
        let guard = self.inner.read().ok()?;
        guard
            .explanation_cache
            .get(&(content_hash.to_string(), profile.to_string(), class_iri.to_string()))
            .cloned()
    }

    pub fn put_cached_explanation(
        &self,
        content_hash: &str,
        profile: &str,
        class_iri: &str,
        result: strixonomy_reasoner::ExplanationResult,
    ) {
        if let Ok(mut guard) = self.inner.write() {
            let key = (content_hash.to_string(), profile.to_string(), class_iri.to_string());
            let updating = guard.explanation_cache.contains_key(&key);
            if let Some(pos) = guard.explanation_cache_order.iter().position(|k| k == &key) {
                guard.explanation_cache_order.remove(pos);
            }
            if !updating {
                while guard.explanation_cache.len() >= MAX_EXPLANATION_CACHE_ENTRIES
                    && !guard.explanation_cache_order.is_empty()
                {
                    if let Some(old) = guard.explanation_cache_order.first().cloned() {
                        guard.explanation_cache_order.remove(0);
                        guard.explanation_cache.remove(&old);
                    }
                }
            }
            guard.explanation_cache_order.push(key.clone());
            guard.explanation_cache.insert(key, result);
        }
    }

    pub fn indexed_at(&self) -> Option<u64> {
        self.inner.read().ok().and_then(|g| g.indexed_at)
    }

    pub fn set_reasoner_snapshot(&self, snapshot: ReasonerSnapshot) {
        if let Ok(mut guard) = self.inner.write() {
            guard.reasoner_snapshot = Some(snapshot);
        }
    }

    /// Read catalog and reasoner snapshot under one lock (avoids TOCTOU mismatch).
    pub fn with_catalog_and_reasoner<T>(
        &self,
        f: impl FnOnce(&OntologyCatalog, Option<&ReasonerSnapshot>) -> T,
    ) -> Option<T> {
        let guard = self.inner.read().ok()?;
        let catalog = guard.catalog.as_deref()?;
        Some(f(catalog, guard.reasoner_snapshot.as_ref()))
    }

    /// Read catalog and open-document overrides under one lock (avoids TOCTOU mismatch).
    pub fn with_catalog_and_overrides<T>(
        &self,
        f: impl FnOnce(&OntologyCatalog, &HashMap<PathBuf, String>) -> T,
    ) -> Option<T> {
        let guard = self.inner.read().ok()?;
        let catalog = guard.catalog.as_deref()?;
        Some(f(catalog, &guard.open_documents))
    }

    pub fn reasoner_cache_mut<R>(&self, f: impl FnOnce(&mut ReasonerCacheStore) -> R) -> Option<R> {
        let mut guard = self.inner.write().ok()?;
        Some(f(&mut guard.reasoner_cache))
    }

    pub fn with_reasoner_cache<R>(&self, f: impl FnOnce(&ReasonerCacheStore) -> R) -> Option<R> {
        let guard = self.inner.read().ok()?;
        Some(f(&guard.reasoner_cache))
    }

    pub fn open_documents_for_reasoner(&self) -> HashMap<PathBuf, String> {
        self.open_documents_snapshot()
    }

    pub fn plugin_diagnostics(&self) -> Vec<Diagnostic> {
        self.inner.read().ok().map(|g| g.plugin_diagnostics.clone()).unwrap_or_default()
    }

    /// Clone catalog documents and diagnostics for publishing outside the read lock.
    pub fn catalog_diagnostic_snapshot(&self) -> Option<CatalogDiagnosticSnapshot> {
        let guard = self.inner.read().ok()?;
        let catalog = guard.catalog.as_deref()?;
        Some(CatalogDiagnosticSnapshot {
            documents: catalog.data().documents.clone(),
            diagnostics: {
                let mut all = catalog.data().diagnostics.clone();
                all.extend(guard.plugin_diagnostics.clone());
                all
            },
        })
    }

    pub fn indexed_workspace(&self) -> Option<PathBuf> {
        self.inner.read().ok()?.indexed_workspace.clone()
    }

    /// Directory used for reindex and reasoner input: last indexed path, else workspace root.
    pub fn effective_index_root(&self) -> Option<PathBuf> {
        let guard = self.inner.read().ok()?;
        let roots = &guard.workspace_roots;
        if let Some(ref indexed) = guard.indexed_workspace {
            if is_path_within_any(roots, indexed) {
                return Some(indexed.clone());
            }
        }
        guard.workspace.clone()
    }

    fn open_documents_snapshot(&self) -> HashMap<PathBuf, String> {
        self.inner.read().ok().map(|g| g.open_documents.clone()).unwrap_or_default()
    }

    pub fn open_document_overrides(&self) -> HashMap<PathBuf, String> {
        self.open_documents_snapshot()
    }

    pub fn with_catalog<T>(&self, f: impl FnOnce(&OntologyCatalog) -> T) -> Option<T> {
        let guard = match self.inner.read() {
            Ok(g) => g,
            Err(e) => {
                eprintln!("strixonomy-lsp: catalog read lock poisoned: {e}");
                return None;
            }
        };
        guard.catalog.as_deref().map(f)
    }

    pub fn workspace_root(&self) -> Option<PathBuf> {
        self.inner.read().ok()?.workspace.clone()
    }

    pub fn resolve_lsp_document_uri(&self, uri: &str) -> Result<PathBuf, String> {
        let roots = self.workspace_roots();
        if roots.is_empty() {
            return Err("workspace not initialized".to_string());
        }
        strixonomy_core::resolve_lsp_document_path_any(uri, &roots)
    }

    /// Store unsaved buffer text for a workspace document.
    ///
    /// Returns an error when the buffer exceeds [`MAX_FILE_BYTES`] or when
    /// [`MAX_OPEN_DOCUMENTS`] would be exceeded for a newly opened path.
    pub fn set_document_text(&self, path: PathBuf, text: String) -> Result<(), String> {
        if text.len() as u64 > MAX_FILE_BYTES {
            return Err(format!("document exceeds maximum size of {MAX_FILE_BYTES} bytes"));
        }
        let roots = self.workspace_roots();
        if roots.is_empty() {
            return Err("workspace not initialized".to_string());
        }
        let path = validate_workspace_scope_any(&path, &roots)?;
        let mut guard = self.inner.write().map_err(|e| e.to_string())?;
        if !guard.open_documents.contains_key(&path)
            && guard.open_documents.len() >= MAX_OPEN_DOCUMENTS
        {
            return Err(format!("open document limit of {MAX_OPEN_DOCUMENTS} reached"));
        }
        guard.open_documents.insert(path, text);
        Ok(())
    }

    pub fn remove_document(&self, path: &Path) {
        let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        match self.inner.write() {
            Ok(mut guard) => {
                guard.open_documents.remove(&path);
                guard.document_versions.remove(&path);
            }
            Err(e) => eprintln!("strixonomy-lsp: failed to remove open document: {e}"),
        }
    }

    pub fn set_document_version(&self, path: PathBuf, version: i32) {
        let path = path.canonicalize().unwrap_or(path);
        if let Ok(mut guard) = self.inner.write() {
            guard.document_versions.insert(path, version);
        }
    }

    pub fn document_version(&self, path: &Path) -> Option<i32> {
        let guard = self.inner.read().ok()?;
        if let Some(v) = guard.document_versions.get(path) {
            return Some(*v);
        }
        path.canonicalize().ok().and_then(|c| guard.document_versions.get(&c).copied())
    }

    /// Atomically replace published diagnostic URIs and return URIs that are now stale.
    pub fn replace_published_diagnostic_uris(&self, current: BTreeSet<String>) -> BTreeSet<String> {
        let Ok(mut guard) = self.inner.write() else {
            return BTreeSet::new();
        };
        let stale: BTreeSet<String> =
            guard.published_diagnostic_uris.difference(&current).cloned().collect();
        guard.published_diagnostic_uris = current;
        stale
    }

    /// True when the path has an unsaved LSP buffer (not merely on disk).
    pub fn is_document_open(&self, path: &Path) -> bool {
        let guard = match self.inner.read() {
            Ok(g) => g,
            Err(_) => return false,
        };
        if guard.open_documents.contains_key(path) {
            return true;
        }
        if let Ok(canonical) = path.canonicalize() {
            return guard.open_documents.contains_key(&canonical);
        }
        false
    }

    /// Prefer unsaved LSP buffer text; fall back to disk.
    pub fn document_text(&self, path: &Path) -> Option<String> {
        let roots = {
            let guard = self.inner.read().ok()?;
            if let Some(text) = guard.open_documents.get(path) {
                return Some(text.clone());
            }
            if let Ok(canonical) = path.canonicalize() {
                if let Some(text) = guard.open_documents.get(&canonical) {
                    return Some(text.clone());
                }
            }
            guard.workspace_roots.clone()
        };
        if roots.is_empty() {
            return None;
        }
        validate_workspace_scope_any(path, &roots).ok()?;
        strixonomy_core::read_to_string_capped(path, MAX_FILE_BYTES).ok()
    }
}

fn clear_workspace_state_inner(guard: &mut InnerState) -> BTreeSet<String> {
    guard.catalog = None;
    guard.workspace_roots.clear();
    guard.workspace = None;
    guard.indexed_workspace = None;
    guard.indexed_at = None;
    guard.open_documents.clear();
    guard.document_versions.clear();
    guard.reasoner_cache.invalidate();
    guard.reasoner_snapshot = None;
    guard.plugin_diagnostics.clear();
    guard.explanation_cache.clear();
    guard.explanation_cache_order.clear();
    guard.active_ontology_id = None;
    std::mem::take(&mut guard.published_diagnostic_uris)
}

fn on_workspace_roots_changed_inner(guard: &mut InnerState, new_roots: &[PathBuf]) {
    guard.catalog = None;
    guard.indexed_at = None;
    guard.reasoner_cache.invalidate();
    guard.reasoner_snapshot = None;
    guard.plugin_diagnostics.clear();
    guard.explanation_cache.clear();
    guard.explanation_cache_order.clear();
    guard.active_ontology_id = None;
    if let Some(ref indexed) = guard.indexed_workspace {
        if !is_path_within_any(new_roots, indexed) {
            guard.indexed_workspace = None;
        }
    }
    guard.open_documents.retain(|path, _| is_path_within_any(new_roots, path));
    guard.document_versions.retain(|path, _| is_path_within_any(new_roots, path));
}

/// Snapshot of catalog data needed to publish LSP diagnostics.
pub struct CatalogDiagnosticSnapshot {
    pub documents: Vec<OntologyDocument>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Resolve a workspace URI for indexing (must be a `file://` directory).
pub fn resolve_workspace_for_index(
    uri: &str,
    existing_root: Option<&Path>,
) -> Result<PathBuf, String> {
    let path = strixonomy_core::workspace_uri_to_path(uri)?;
    if let Some(root) = existing_root {
        validate_workspace_scope(&path, root)
    } else {
        Ok(path)
    }
}

/// Resolve a workspace folder URI for multi-root folder **remove** events.
///
/// When `existing_roots` is non-empty, the folder must match or lie under at least one root.
/// Empty roots accept the path as an initial root.
pub fn resolve_workspace_folder_uri(
    uri: &str,
    existing_roots: &[PathBuf],
) -> Result<PathBuf, String> {
    let path = strixonomy_core::workspace_uri_to_path(uri)?;
    if existing_roots.is_empty() {
        return Ok(path);
    }
    validate_workspace_scope_any(&path, existing_roots)
}

/// Resolve a workspace folder URI for multi-root folder **add** events.
///
/// Peer folders (siblings of existing roots) are accepted — the same rule as
/// `initialize` with multiple `workspaceFolders`. Document paths remain jailed
/// via [`validate_workspace_scope_any`].
pub fn resolve_workspace_folder_add(uri: &str) -> Result<PathBuf, String> {
    strixonomy_core::workspace_uri_to_path(uri)
}

pub(crate) fn canonical_roots_match(a: &Path, b: &Path) -> bool {
    canonical_workspace_root(a).ok().as_ref() == canonical_workspace_root(b).ok().as_ref() || a == b
}

pub fn path_to_uri(path: &Path) -> String {
    strixonomy_core::file_uri_for_path(path)
}

fn now_epoch_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use strixonomy_core::{
        is_path_within,
        limits::{MAX_FILE_BYTES, MAX_OPEN_DOCUMENTS},
    };

    #[test]
    fn subdirectory_index_does_not_shrink_workspace_root() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("sub");
        std::fs::create_dir_all(&sub).unwrap();
        let root = dir.path().canonicalize().unwrap();

        let state = ServerState::new();
        state.set_workspace_roots(vec![root.clone()]).expect("set workspace");
        state.index_workspace(sub.clone()).expect("index subdirectory");

        assert_eq!(state.workspace_root().as_deref(), Some(root.as_path()));
        let indexed = state.indexed_workspace().expect("indexed workspace");
        assert!(is_path_within(&root, &indexed));
        assert_eq!(indexed.file_name(), sub.file_name());
    }

    #[test]
    fn rejects_oversized_buffer() {
        let state = ServerState::new();
        let path = PathBuf::from("/tmp/oversized.ttl");
        let text = "x".repeat((MAX_FILE_BYTES + 1) as usize);
        let err = state.set_document_text(path, text).unwrap_err();
        assert!(err.contains("exceeds maximum size"));
    }

    #[test]
    fn rejects_when_open_document_limit_reached() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        let state = ServerState::new();
        state.set_workspace_roots(vec![root.clone()]).expect("set workspace");
        for i in 0..MAX_OPEN_DOCUMENTS {
            let path = root.join(format!("doc-{i}.ttl"));
            state.set_document_text(path, "@prefix ex: <http://ex#> .".to_string()).unwrap();
        }
        let err = state.set_document_text(root.join("extra.ttl"), "x".to_string()).unwrap_err();
        assert!(err.contains("open document limit"));
    }

    #[test]
    fn catalog_and_reasoner_unavailable_before_index() {
        let state = ServerState::new();
        assert!(state.with_catalog_and_reasoner(|_, _| ()).is_none());
    }

    #[test]
    fn resolve_workspace_folder_uri_rejects_outside_existing_roots() {
        let root_dir = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let root = root_dir.path().canonicalize().unwrap();
        let outside_uri =
            url::Url::from_file_path(outside.path().canonicalize().unwrap()).unwrap().to_string();

        assert!(resolve_workspace_folder_uri(&outside_uri, &[root]).is_err());
    }

    #[test]
    fn resolve_workspace_folder_add_accepts_peer_root() {
        let root_a = tempfile::tempdir().unwrap();
        let root_b = tempfile::tempdir().unwrap();
        let peer_uri =
            url::Url::from_file_path(root_b.path().canonicalize().unwrap()).unwrap().to_string();

        let resolved = resolve_workspace_folder_add(&peer_uri).expect("peer add");
        assert_eq!(resolved, root_b.path().canonicalize().unwrap());
        // Remove still requires the folder to be under/equal an existing root.
        let root = root_a.path().canonicalize().unwrap();
        assert!(resolve_workspace_folder_uri(&peer_uri, &[root]).is_err());
    }

    #[test]
    fn resolve_workspace_folder_uri_accepts_subdirectory_of_root() {
        let root_dir = tempfile::tempdir().unwrap();
        let sub = root_dir.path().join("ontologies");
        std::fs::create_dir_all(&sub).unwrap();
        let root = root_dir.path().canonicalize().unwrap();
        let sub_uri = url::Url::from_file_path(sub.canonicalize().unwrap()).unwrap().to_string();

        let resolved =
            resolve_workspace_folder_uri(&sub_uri, &[root]).expect("subfolder under root");
        assert!(is_path_within(&root_dir.path().canonicalize().unwrap(), &resolved));
    }

    #[test]
    fn incremental_reindex_does_not_hold_inner_lock_across_build() {
        use std::thread;
        use std::time::{Duration, Instant};

        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        std::fs::write(
            root.join("doc.ttl"),
            "@prefix ex: <http://example.org#> .\nex:C a <http://www.w3.org/2002/07/owl#Class> .\n",
        )
        .unwrap();

        let state = ServerState::new();
        state.set_workspace_roots(vec![root.clone()]).expect("set workspace");
        state.index_workspace(root.clone()).expect("initial index");

        TEST_INCREMENTAL_BUILD_IN_PAUSE.store(false, Ordering::SeqCst);
        TEST_INCREMENTAL_BUILD_PAUSE_MS.store(200, Ordering::SeqCst);
        let state_index = state.clone();
        let root_index = root.clone();
        let indexer = thread::spawn(move || {
            state_index.index_workspace(root_index).expect("incremental reindex");
        });

        let deadline = Instant::now() + Duration::from_secs(5);
        while !TEST_INCREMENTAL_BUILD_IN_PAUSE.load(Ordering::SeqCst) {
            assert!(Instant::now() < deadline, "indexer never entered build pause");
            thread::sleep(Duration::from_millis(1));
        }

        let write_start = Instant::now();
        state
            .set_document_text(
                root.join("doc.ttl"),
                "@prefix ex: <http://example.org#> .\nex:C a <http://www.w3.org/2002/07/owl#Class> .\n"
                    .to_string(),
            )
            .expect("document write during reindex pause");
        let write_elapsed = write_start.elapsed();
        TEST_INCREMENTAL_BUILD_PAUSE_MS.store(0, Ordering::SeqCst);

        indexer.join().expect("indexer thread");
        assert!(
            write_elapsed < Duration::from_millis(100),
            "document write blocked for {write_elapsed:?}; catalog lock likely held across build"
        );
    }

    #[test]
    fn explanation_cache_evicts_oldest_beyond_cap() {
        let state = ServerState::new();
        for i in 0..10 {
            let iri = format!("http://example.org/C{i}");
            state.put_cached_explanation(
                "hash",
                "elk",
                &iri,
                strixonomy_reasoner::ExplanationResult {
                    class_iri: iri.clone(),
                    steps: vec![],
                    text: String::new(),
                },
            );
        }
        assert!(state.get_cached_explanation("hash", "elk", "http://example.org/C0").is_none());
        assert!(state.get_cached_explanation("hash", "elk", "http://example.org/C1").is_none());
        assert!(state.get_cached_explanation("hash", "elk", "http://example.org/C2").is_some());
        assert!(state.get_cached_explanation("hash", "elk", "http://example.org/C9").is_some());
    }

    #[test]
    fn set_workspace_roots_clears_active_ontology_id() {
        let dir_a = tempfile::tempdir().unwrap();
        let dir_b = tempfile::tempdir().unwrap();
        let root_a = dir_a.path().canonicalize().unwrap();
        let root_b = dir_b.path().canonicalize().unwrap();
        let state = ServerState::new();
        state.set_workspace_roots(vec![root_a]).expect("set roots A");
        state.set_active_ontology_id(Some("http://example.org/a".into()));
        assert_eq!(state.active_ontology_id().as_deref(), Some("http://example.org/a"));
        state.set_workspace_roots(vec![root_b]).expect("set roots B");
        assert!(state.active_ontology_id().is_none());
    }

    #[test]
    fn clear_workspace_state_returns_published_uris_and_clears_explanations() {
        let state = ServerState::new();
        let mut uris = BTreeSet::new();
        uris.insert("file:///tmp/stale.ttl".into());
        let _ = state.replace_published_diagnostic_uris(uris);
        state.put_cached_explanation(
            "hash",
            "elk",
            "http://example.org/C",
            strixonomy_reasoner::ExplanationResult {
                class_iri: "http://example.org/C".into(),
                steps: vec![],
                text: String::new(),
            },
        );

        let stale = state.clear_workspace_state();
        assert!(stale.contains("file:///tmp/stale.ttl"));
        assert!(state.get_cached_explanation("hash", "elk", "http://example.org/C").is_none());
        assert!(state.replace_published_diagnostic_uris(BTreeSet::new()).is_empty());
    }
}
