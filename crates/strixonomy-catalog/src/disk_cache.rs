//! Persistent per-document parse cache under `.strixonomy/cache/`.

use crate::incremental::DocumentSnapshot;
use oxigraph::model::vocab::xsd;
use oxigraph::model::{
    GraphName, GraphNameRef, Literal, LiteralRef, NamedNode, Quad, Subject, SubjectRef, Term,
    TermRef,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use strixonomy_core::paths::cache_dir;
use strixonomy_core::Diagnostic;

const SNAPSHOTS_DIR: &str = "snapshots";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredQuad {
    subject: String,
    predicate: String,
    object: String,
    graph: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredSnapshot {
    content_hash: String,
    document: strixonomy_core::OntologyDocument,
    entities: Vec<strixonomy_core::Entity>,
    annotations: Vec<strixonomy_core::Annotation>,
    axioms: Vec<strixonomy_core::Axiom>,
    namespace_rows: Vec<strixonomy_core::Namespace>,
    imports: Vec<strixonomy_core::Import>,
    quads: Vec<StoredQuad>,
    triple_count: usize,
    bridge_warning: Option<Diagnostic>,
}

/// On-disk cache keyed by content hash (optional workspace acceleration).
#[derive(Debug, Clone)]
pub(crate) struct DiskCache {
    root: PathBuf,
}

impl DiskCache {
    pub fn for_workspace(workspace: &Path) -> Self {
        Self { root: cache_dir(workspace) }
    }

    pub fn enabled(enabled: bool, workspace: &Path) -> Option<Self> {
        if enabled {
            Some(Self::for_workspace(workspace))
        } else {
            None
        }
    }

    pub(crate) fn load(&self, content_hash: &str) -> Option<DocumentSnapshot> {
        let path = self.snapshot_path(content_hash).with_extension("json");
        let bytes =
            strixonomy_core::read_file_capped(&path, strixonomy_core::MAX_FILE_BYTES).ok()?;
        let stored: StoredSnapshot = serde_json::from_slice(&bytes).ok()?;
        if stored.content_hash != content_hash {
            return None;
        }
        if stored.triple_count != stored.quads.len() {
            return None;
        }
        let quads: Vec<Quad> = stored.quads.iter().filter_map(restore_quad).collect();
        if quads.len() != stored.quads.len() || quads.len() != stored.triple_count {
            return None;
        }
        Some(DocumentSnapshot {
            content_hash: stored.content_hash,
            document: stored.document,
            entities: stored.entities,
            annotations: stored.annotations,
            axioms: stored.axioms,
            namespace_rows: stored.namespace_rows,
            imports: stored.imports,
            quads,
            triple_count: stored.triple_count,
            bridge_warning: stored.bridge_warning,
        })
    }

    pub(crate) fn store(&self, snap: &DocumentSnapshot) -> std::io::Result<()> {
        if snap.quads.iter().any(quad_has_quoted_triple) {
            return Ok(());
        }
        fs::create_dir_all(self.root.join(SNAPSHOTS_DIR))?;
        let stored = StoredSnapshot::from(snap);
        let bytes = serde_json::to_vec(&stored)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        let path = self.snapshot_path(&snap.content_hash).with_extension("json");
        let temp_path = path.with_extension("json.tmp");
        {
            use std::io::Write;
            let mut file = fs::File::create(&temp_path)?;
            file.write_all(&bytes)?;
            file.sync_all()?;
        }
        fs::rename(temp_path, path)
    }

    /// Merge disk hits into `previous` when scanner hash matches.
    pub(crate) fn hydrate_previous(
        &self,
        previous: &mut HashMap<PathBuf, DocumentSnapshot>,
        path: &Path,
        content_hash: &str,
        modified_time: u64,
    ) {
        if previous.contains_key(path) {
            return;
        }
        if let Some(mut snap) = self.load(content_hash) {
            snap.document.path = path.to_path_buf();
            snap.document.modified_time = modified_time;
            previous.insert(path.to_path_buf(), snap);
        }
    }

    /// Delete snapshot files whose content hash is not in the live catalog (#166).
    ///
    /// Also removes leftover `*.json.tmp` files from interrupted writes.
    /// Returns the number of snapshot files removed.
    pub(crate) fn prune(&self, live_hashes: &HashSet<String>) -> std::io::Result<usize> {
        let dir = self.root.join(SNAPSHOTS_DIR);
        if !dir.is_dir() {
            return Ok(0);
        }
        let mut removed = 0usize;
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if name.ends_with(".json.tmp") {
                let _ = fs::remove_file(&path);
                continue;
            }
            if !name.ends_with(".json") {
                continue;
            }
            let hash = name.trim_end_matches(".json");
            if !live_hashes.contains(hash) {
                fs::remove_file(&path)?;
                removed += 1;
            }
        }
        Ok(removed)
    }

    fn snapshot_path(&self, content_hash: &str) -> PathBuf {
        self.root.join(SNAPSHOTS_DIR).join(content_hash)
    }
}

impl From<&DocumentSnapshot> for StoredSnapshot {
    fn from(snap: &DocumentSnapshot) -> Self {
        Self {
            content_hash: snap.content_hash.clone(),
            document: snap.document.clone(),
            entities: snap.entities.clone(),
            annotations: snap.annotations.clone(),
            axioms: snap.axioms.clone(),
            namespace_rows: snap.namespace_rows.clone(),
            imports: snap.imports.clone(),
            quads: snap.quads.iter().map(stored_quad).collect(),
            triple_count: snap.triple_count,
            bridge_warning: snap.bridge_warning.clone(),
        }
    }
}

impl From<StoredSnapshot> for DocumentSnapshot {
    fn from(stored: StoredSnapshot) -> Self {
        let quads: Vec<Quad> = stored.quads.iter().filter_map(restore_quad).collect();
        Self {
            content_hash: stored.content_hash,
            document: stored.document,
            entities: stored.entities,
            annotations: stored.annotations,
            axioms: stored.axioms,
            namespace_rows: stored.namespace_rows,
            imports: stored.imports,
            quads,
            triple_count: stored.triple_count,
            bridge_warning: stored.bridge_warning,
        }
    }
}

fn quad_has_quoted_triple(quad: &Quad) -> bool {
    matches!(quad.subject.as_ref(), SubjectRef::Triple(_))
        || matches!(quad.object.as_ref(), TermRef::Triple(_))
}

fn stored_quad(quad: &Quad) -> StoredQuad {
    StoredQuad {
        subject: subject_to_string(quad.subject.as_ref()),
        predicate: quad.predicate.as_str().to_string(),
        object: term_to_string(quad.object.as_ref()),
        graph: match quad.graph_name.as_ref() {
            GraphNameRef::DefaultGraph => None,
            GraphNameRef::NamedNode(n) => Some(n.as_str().to_string()),
            GraphNameRef::BlankNode(b) => Some(format!("_:{b}")),
        },
    }
}

fn restore_quad(stored: &StoredQuad) -> Option<Quad> {
    let subject = parse_subject(&stored.subject)?;
    let predicate = NamedNode::new(stored.predicate.as_str()).ok()?;
    let object = parse_term(&stored.object)?;
    let graph = match stored.graph.as_deref() {
        None | Some("") => GraphName::DefaultGraph,
        Some(g) if g.starts_with("_:") => {
            GraphName::BlankNode(oxigraph::model::BlankNode::new_unchecked(&g[2..]))
        }
        Some(g) => GraphName::NamedNode(NamedNode::new(g).ok()?),
    };
    Some(Quad { subject, predicate, object, graph_name: graph })
}

fn subject_to_string(subject: SubjectRef<'_>) -> String {
    match subject {
        SubjectRef::NamedNode(n) => format!("<{}>", n.as_str()),
        SubjectRef::BlankNode(b) => format!("_:{b}"),
        SubjectRef::Triple(_) => "\"<<triple>>\"".to_string(),
    }
}

fn term_to_string(term: TermRef<'_>) -> String {
    match term {
        TermRef::NamedNode(n) => format!("<{}>", n.as_str()),
        TermRef::BlankNode(b) => format!("_:{b}"),
        TermRef::Literal(l) => literal_to_string(l),
        TermRef::Triple(_) => "\"<<triple>>\"".to_string(),
    }
}

fn literal_to_string(l: LiteralRef<'_>) -> String {
    if let Some(lang) = l.language() {
        format!("\"{}\"@{}", l.value(), lang)
    } else if l.datatype() == xsd::STRING {
        format!("\"{}\"", l.value())
    } else {
        format!("\"{}\"^^<{}>", l.value(), l.datatype().as_str())
    }
}

fn parse_subject(raw: &str) -> Option<Subject> {
    if raw.starts_with("<") && raw.ends_with('>') {
        return NamedNode::new(&raw[1..raw.len() - 1]).ok().map(Subject::NamedNode);
    }
    if let Some(stripped) = raw.strip_prefix("_:") {
        return Some(Subject::BlankNode(oxigraph::model::BlankNode::new_unchecked(stripped)));
    }
    NamedNode::new(raw).ok().map(Subject::NamedNode)
}

fn parse_term(raw: &str) -> Option<Term> {
    if raw.starts_with("<") && raw.ends_with('>') {
        return NamedNode::new(&raw[1..raw.len() - 1]).ok().map(Term::NamedNode);
    }
    if let Some(stripped) = raw.strip_prefix("_:") {
        return Some(Term::BlankNode(oxigraph::model::BlankNode::new_unchecked(stripped)));
    }
    if raw.starts_with('"') {
        if let Some(at) = raw.rfind('"') {
            let inner = &raw[1..at];
            let tail = &raw[at + 1..];
            if let Some(lang) = tail.strip_prefix('@') {
                return Some(Term::Literal(Literal::new_language_tagged_literal_unchecked(
                    inner, lang,
                )));
            }
            // Accept canonical `^^<iri>` and legacy single-caret `^<iri>` from older caches.
            if let Some(dt) = tail
                .strip_prefix("^^<")
                .or_else(|| tail.strip_prefix("^<"))
                .and_then(|s| s.strip_suffix('>'))
            {
                return NamedNode::new(dt)
                    .ok()
                    .map(|dt| Term::Literal(Literal::new_typed_literal(inner, dt)));
            }
            return Some(Term::Literal(Literal::new_simple_literal(inner)));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use strixonomy_core::{
        Entity, EntityKind, OntologyDocument, OntologyFormat, ParseStatus, SourceLocation,
    };

    #[test]
    fn roundtrip_snapshot_through_disk_cache() {
        let dir = tempfile::tempdir().unwrap();
        let cache = DiskCache::for_workspace(dir.path());
        let snap = DocumentSnapshot {
            content_hash: "deadbeef".to_string(),
            document: OntologyDocument {
                id: "doc-1".to_string(),
                path: dir.path().join("a.ttl"),
                format: OntologyFormat::Turtle,
                base_iri: None,
                version_iri: None,
                imports: vec![],
                namespaces: Default::default(),
                parse_status: ParseStatus::Ok,
                content_hash: "deadbeef".to_string(),
                modified_time: 0,
                parse_message: None,
                parse_error_location: None,
            },
            entities: vec![Entity {
                iri: "http://ex#A".to_string(),
                kind: EntityKind::Class,
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
        cache.store(&snap).expect("store");
        let loaded = cache.load("deadbeef").expect("load");
        assert_eq!(loaded.entities[0].iri, "http://ex#A");
    }

    #[test]
    fn load_returns_none_for_corrupt_snapshot_json() {
        let dir = tempfile::tempdir().unwrap();
        let cache = DiskCache::for_workspace(dir.path());
        let snap_dir = dir.path().join(".strixonomy/cache/snapshots");
        std::fs::create_dir_all(&snap_dir).unwrap();
        std::fs::write(snap_dir.join("corrupt.json"), b"{not-json").unwrap();
        assert!(cache.load("corrupt").is_none());
    }

    #[test]
    fn roundtrip_preserves_typed_and_language_literals() {
        let integer = NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#integer");
        let typed = Literal::new_typed_literal("42", integer.clone());
        let lang = Literal::new_language_tagged_literal_unchecked("hello", "en");
        let plain = Literal::new_simple_literal("plain");

        assert_eq!(
            literal_to_string(typed.as_ref()),
            "\"42\"^^<http://www.w3.org/2001/XMLSchema#integer>"
        );
        assert_eq!(literal_to_string(lang.as_ref()), "\"hello\"@en");
        assert_eq!(literal_to_string(plain.as_ref()), "\"plain\"");

        let typed_term = parse_term(&literal_to_string(typed.as_ref())).expect("typed");
        let Term::Literal(restored_typed) = typed_term else {
            panic!("expected literal");
        };
        assert_eq!(restored_typed.value(), "42");
        assert_eq!(restored_typed.datatype(), integer.as_ref());
        assert!(restored_typed.language().is_none());

        let lang_term = parse_term(&literal_to_string(lang.as_ref())).expect("lang");
        let Term::Literal(restored_lang) = lang_term else {
            panic!("expected literal");
        };
        assert_eq!(restored_lang.value(), "hello");
        assert_eq!(restored_lang.language(), Some("en"));

        // Legacy single-caret snapshots from older Strixonomy builds.
        let legacy =
            parse_term("\"7\"^<http://www.w3.org/2001/XMLSchema#integer>").expect("legacy");
        let Term::Literal(restored_legacy) = legacy else {
            panic!("expected literal");
        };
        assert_eq!(restored_legacy.value(), "7");
        assert_eq!(restored_legacy.datatype(), integer.as_ref());
    }

    #[test]
    fn disk_cache_roundtrip_keeps_typed_quad_datatype() {
        let dir = tempfile::tempdir().unwrap();
        let cache = DiskCache::for_workspace(dir.path());
        let integer = NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#integer");
        let quad = Quad {
            subject: NamedNode::new_unchecked("http://ex#A").into(),
            predicate: NamedNode::new_unchecked("http://ex#count"),
            object: Literal::new_typed_literal("42", integer.clone()).into(),
            graph_name: GraphName::DefaultGraph,
        };
        let snap = DocumentSnapshot {
            content_hash: "typedquad".to_string(),
            document: OntologyDocument {
                id: "doc-1".to_string(),
                path: dir.path().join("a.ttl"),
                format: OntologyFormat::Turtle,
                base_iri: None,
                version_iri: None,
                imports: vec![],
                namespaces: Default::default(),
                parse_status: ParseStatus::Ok,
                content_hash: "typedquad".to_string(),
                modified_time: 0,
                parse_message: None,
                parse_error_location: None,
            },
            entities: vec![],
            annotations: vec![],
            axioms: vec![],
            namespace_rows: vec![],
            imports: vec![],
            quads: vec![quad.clone()],
            triple_count: 1,
            bridge_warning: None,
        };
        cache.store(&snap).expect("store");
        let loaded = cache.load("typedquad").expect("load");
        assert_eq!(loaded.quads.len(), 1);
        let Term::Literal(lit) = &loaded.quads[0].object else {
            panic!("expected literal object");
        };
        assert_eq!(lit.value(), "42");
        assert_eq!(lit.datatype(), integer.as_ref());
    }

    #[test]
    fn prune_removes_orphan_snapshots_and_keeps_live() {
        let dir = tempfile::tempdir().unwrap();
        let cache = DiskCache::for_workspace(dir.path());

        let make_snap = |hash: &str| DocumentSnapshot {
            content_hash: hash.to_string(),
            document: OntologyDocument {
                id: "doc-1".to_string(),
                path: dir.path().join("a.ttl"),
                format: OntologyFormat::Turtle,
                base_iri: None,
                version_iri: None,
                imports: vec![],
                namespaces: Default::default(),
                parse_status: ParseStatus::Ok,
                content_hash: hash.to_string(),
                modified_time: 0,
                parse_message: None,
                parse_error_location: None,
            },
            entities: vec![],
            annotations: vec![],
            axioms: vec![],
            namespace_rows: vec![],
            imports: vec![],
            quads: vec![],
            triple_count: 0,
            bridge_warning: None,
        };

        cache.store(&make_snap("livehash")).expect("store live");
        cache.store(&make_snap("stalehash")).expect("store stale");
        let snap_dir = dir.path().join(".strixonomy/cache/snapshots");
        std::fs::write(snap_dir.join("orphan.json.tmp"), b"partial").unwrap();

        let live = HashSet::from(["livehash".to_string()]);
        let removed = cache.prune(&live).expect("prune");
        assert_eq!(removed, 1);
        assert!(cache.load("livehash").is_some());
        assert!(cache.load("stalehash").is_none());
        assert!(!snap_dir.join("orphan.json.tmp").exists());
        assert!(!snap_dir.join("stalehash.json").exists());
        assert!(snap_dir.join("livehash.json").exists());
    }

    #[test]
    fn prune_is_noop_when_snapshots_dir_missing() {
        let dir = tempfile::tempdir().unwrap();
        let cache = DiskCache::for_workspace(dir.path());
        let removed = cache.prune(&HashSet::new()).expect("prune empty");
        assert_eq!(removed, 0);
    }

    #[test]
    fn index_builder_prunes_stale_disk_cache_after_reindex() {
        use crate::IndexBuilder;

        let dir = tempfile::tempdir().unwrap();
        let ttl = dir.path().join("a.ttl");
        std::fs::write(
            &ttl,
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n@prefix ex: <http://ex/> .\nex:A a owl:Class .\n",
        )
        .unwrap();

        IndexBuilder::new().workspace(dir.path()).disk_cache(true).build().expect("first build");

        let snap_dir = dir.path().join(".strixonomy/cache/snapshots");
        assert!(snap_dir.is_dir());
        let after_first: Vec<_> = std::fs::read_dir(&snap_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("json"))
            .collect();
        assert_eq!(after_first.len(), 1);
        let first_hash =
            after_first[0].path().file_stem().and_then(|s| s.to_str()).unwrap().to_string();

        // Plant an orphan snapshot that should be removed on the next index.
        std::fs::write(snap_dir.join("orphaneddeadbeef.json"), b"{}").unwrap();

        std::fs::write(
            &ttl,
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n@prefix ex: <http://ex/> .\nex:A a owl:Class .\nex:B a owl:Class .\n",
        )
        .unwrap();

        IndexBuilder::new().workspace(dir.path()).disk_cache(true).build().expect("second build");

        let after_second: Vec<_> = std::fs::read_dir(&snap_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("json"))
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert!(
            !after_second.iter().any(|n| n == "orphaneddeadbeef.json"),
            "orphan snapshot should be pruned"
        );
        assert!(
            !after_second.iter().any(|n| n.as_str() == format!("{first_hash}.json")),
            "stale content-hash snapshot should be pruned after edit"
        );
        assert_eq!(after_second.len(), 1, "only the live hash should remain");
    }
}
