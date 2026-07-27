//! Per-document snapshots for content-hash incremental reindexing.

use oxigraph::model::Quad;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use strixonomy_core::{Annotation, Axiom, Diagnostic, Entity, Import, Namespace, OntologyDocument};

/// Cached parse + semantics for one ontology file at a specific content hash.
#[derive(Debug, Clone)]
pub(crate) struct DocumentSnapshot {
    pub content_hash: String,
    pub document: OntologyDocument,
    pub entities: Vec<Entity>,
    pub annotations: Vec<Annotation>,
    pub axioms: Vec<Axiom>,
    pub namespace_rows: Vec<Namespace>,
    pub imports: Vec<Import>,
    pub quads: Vec<Quad>,
    pub triple_count: usize,
    pub bridge_warning: Option<Diagnostic>,
}

impl DocumentSnapshot {
    pub fn with_doc_id(&self, doc_id: &str) -> Self {
        let mut snap = self.clone();
        snap.document.id = doc_id.to_string();
        for entity in &mut snap.entities {
            entity.ontology_id = doc_id.to_string();
        }
        for ann in &mut snap.annotations {
            ann.ontology_id = doc_id.to_string();
        }
        for ax in &mut snap.axioms {
            ax.ontology_id = doc_id.to_string();
        }
        for ns in &mut snap.namespace_rows {
            ns.ontology_id = doc_id.to_string();
        }
        for imp in &mut snap.imports {
            imp.ontology_id = doc_id.to_string();
        }
        snap
    }

    pub fn with_reuse_context(&self, doc_id: &str, path: &Path, modified_time: u64) -> Self {
        let mut snap = self.with_doc_id(doc_id);
        snap.document.path = path.to_path_buf();
        snap.document.modified_time = modified_time;
        snap
    }
}

pub(crate) fn effective_content_hash(disk_hash: &str, override_text: Option<&str>) -> String {
    if let Some(text) = override_text {
        content_hash_text(text)
    } else {
        disk_hash.to_string()
    }
}

pub(crate) fn content_hash_text(text: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hex::encode(hasher.finalize())
}

pub(crate) fn paths_equal(a: &Path, b: &Path) -> bool {
    a == b || a.canonicalize().ok().zip(b.canonicalize().ok()).is_some_and(|(x, y)| x == y)
}

/// Fingerprint of incremental-relevant builder config (scan roots, disk cache, override paths).
pub(crate) fn config_fingerprint(
    scan_roots: &[PathBuf],
    disk_cache: bool,
    document_overrides: &HashMap<PathBuf, String>,
) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"v1\0scan_roots\0");
    for root in scan_roots {
        hasher.update(root.to_string_lossy().as_bytes());
        hasher.update([0]);
    }
    hasher.update([if disk_cache { 1 } else { 0 }]);
    hasher.update(b"\0overrides\0");
    let mut keys: Vec<_> = document_overrides.keys().collect();
    keys.sort_by(|a, b| a.as_os_str().cmp(b.as_os_str()));
    for key in keys {
        hasher.update(key.to_string_lossy().as_bytes());
        hasher.update([0]);
    }
    hex::encode(hasher.finalize())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncrementalStats {
    FullBuild,
    Incremental { reused: usize, reparsed: usize, removed: usize },
}

impl IncrementalStats {
    pub fn reused_documents(&self) -> usize {
        match self {
            Self::FullBuild => 0,
            Self::Incremental { reused, .. } => *reused,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use strixonomy_core::{OntologyFormat, ParseStatus, SourceLocation};

    #[test]
    fn content_hash_is_stable() {
        let h1 = content_hash_text("hello");
        let h2 = content_hash_text("hello");
        assert_eq!(h1, h2);
        assert_ne!(h1, content_hash_text("world"));
    }

    #[test]
    fn remap_doc_id_updates_ontology_ids() {
        let snap = DocumentSnapshot {
            content_hash: "abc".to_string(),
            document: OntologyDocument {
                id: "doc-1".to_string(),
                path: PathBuf::from("a.ttl"),
                format: OntologyFormat::Turtle,
                base_iri: None,
                version_iri: None,
                imports: vec![],
                namespaces: BTreeMap::new(),
                parse_status: ParseStatus::Ok,
                content_hash: "abc".to_string(),
                modified_time: 0,
                parse_message: None,
                parse_error_location: None,
            },
            entities: vec![Entity {
                iri: "http://ex#A".to_string(),
                kind: strixonomy_core::EntityKind::Class,
                short_name: "A".to_string(),
                ontology_id: "doc-1".to_string(),
                source_location: SourceLocation::default(),
                labels: vec![],
                comments: vec![],
                deprecated: false,
                obo_id: None,
                characteristics: Default::default(),
            }],
            annotations: vec![],
            axioms: vec![],
            namespace_rows: vec![],
            imports: vec![],
            quads: vec![],
            triple_count: 0,
            bridge_warning: None,
        };
        let remapped = snap.with_doc_id("doc-2");
        assert_eq!(remapped.document.id, "doc-2");
        assert_eq!(remapped.entities[0].ontology_id, "doc-2");
    }
}
