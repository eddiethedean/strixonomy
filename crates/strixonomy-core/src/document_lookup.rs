use crate::{Entity, OntologyDocument};

/// Normalize an ontology IRI for prefix/suffix comparison.
pub fn normalize_iri(iri: &str) -> String {
    iri.trim_end_matches('#').trim_end_matches('/').to_string()
}

/// Canonical `file://` URI for workspace document matching and diagnostics.
///
/// Uses the same encoding rules as LSP `path_to_uri` (`url::Url::from_file_path`) so
/// Windows paths become `file:///C:/…` instead of invalid `file://C:\\…` (#218).
pub fn file_uri_for_path(path: &std::path::Path) -> String {
    let abs = path.canonicalize().unwrap_or_else(|_| {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_default().join(path)
        }
    });
    url::Url::from_file_path(&abs).map(|u| u.to_string()).unwrap_or_else(|_| {
        let display = abs.to_string_lossy().replace('\\', "/");
        if display.starts_with('/') {
            format!("file://{display}")
        } else {
            format!("file:///{display}")
        }
    })
}

/// Whether `entity` is owned by `doc` via exact ontology id / ontology IRI equality.
///
/// Does **not** use entity-IRI `starts_with(base_iri)` — that is ambiguous when multiple
/// documents have nested bases. Use [`document_for_entity`] to resolve ownership among a set.
pub fn document_matches_entity(entity: &Entity, doc: &OntologyDocument) -> bool {
    if entity.ontology_id == doc.id {
        return true;
    }
    if let Some(base) = &doc.base_iri {
        if normalize_iri(base) == normalize_iri(&entity.ontology_id) {
            return true;
        }
    }
    false
}

fn base_iri_prefix_len(entity: &Entity, doc: &OntologyDocument) -> Option<usize> {
    let base = doc.base_iri.as_ref()?;
    if entity.iri.starts_with(base.as_str()) {
        Some(base.len())
    } else {
        None
    }
}

/// Whether an import ontology id matches a document.
pub fn document_matches_ontology_id(ontology_id: &str, doc: &OntologyDocument) -> bool {
    if ontology_id == doc.id {
        return true;
    }
    if let Some(base) = &doc.base_iri {
        if normalize_iri(base) == normalize_iri(ontology_id) {
            return true;
        }
    }
    false
}

/// Find the document that owns an entity.
///
/// Preference order:
/// 1. Exact `entity.ontology_id == doc.id`
/// 2. Normalized ontology IRI equality with `doc.base_iri`
/// 3. Longest `doc.base_iri` that is a string prefix of `entity.iri` (nested-base fallback)
pub fn document_for_entity<'a>(
    documents: &'a [OntologyDocument],
    entity: &Entity,
) -> Option<&'a OntologyDocument> {
    if let Some(doc) = documents.iter().find(|d| entity.ontology_id == d.id) {
        return Some(doc);
    }
    if let Some(doc) = documents.iter().find(|d| {
        d.base_iri
            .as_ref()
            .is_some_and(|base| normalize_iri(base) == normalize_iri(&entity.ontology_id))
    }) {
        return Some(doc);
    }
    documents
        .iter()
        .filter_map(|d| base_iri_prefix_len(entity, d).map(|len| (d, len)))
        .max_by_key(|(_, len)| *len)
        .map(|(d, _)| d)
}

/// Find the document for an import's owning ontology id.
pub fn document_for_ontology_id<'a>(
    documents: &'a [OntologyDocument],
    ontology_id: &str,
) -> Option<&'a OntologyDocument> {
    documents.iter().find(|d| document_matches_ontology_id(ontology_id, d))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EntityKind, OntologyFormat, ParseStatus};
    use std::collections::BTreeMap;
    use std::path::Path;

    fn doc(id: &str, base_iri: Option<&str>) -> OntologyDocument {
        OntologyDocument {
            id: id.to_string(),
            path: Path::new(&format!("{id}.ttl")).to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: base_iri.map(str::to_string),
            version_iri: None,
            imports: vec![],
            namespaces: BTreeMap::new(),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        }
    }

    fn entity(iri: &str, ontology_id: &str) -> Entity {
        Entity {
            iri: iri.to_string(),
            short_name: iri.rsplit(['#', '/']).next().unwrap_or(iri).to_string(),
            kind: EntityKind::Class,
            ontology_id: ontology_id.to_string(),
            source_location: Default::default(),
            labels: vec![],
            comments: vec![],
            deprecated: false,
            obo_id: None,
            characteristics: Default::default(),
        }
    }

    #[test]
    fn file_uri_for_path_uses_url_encoding() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("sample.ttl");
        std::fs::write(&file, "").unwrap();
        let uri = file_uri_for_path(&file);
        assert!(uri.starts_with("file://"), "got {uri}");
        assert!(!uri.contains('\\'), "Windows separators must not appear raw: {uri}");
        // Drive-letter URIs are file:///C:/… (three slashes after file:).
        #[cfg(windows)]
        {
            assert!(
                uri.starts_with("file:///"),
                "Windows file URI should use three slashes: {uri}"
            );
            assert!(uri.contains(":/") || uri.contains("%3A"), "got {uri}");
        }
        #[cfg(unix)]
        {
            assert!(uri.starts_with("file:///"), "got {uri}");
        }
    }

    #[test]
    fn matches_entity_by_ontology_iri() {
        let document = doc("doc-1", Some("http://example.org/people"));
        let ent = entity("http://example.org/people#Person", "http://example.org/people");
        assert!(document_matches_entity(&ent, &document));
    }

    #[test]
    fn broader_base_iri_does_not_match_when_ontology_id_differs() {
        let broad = doc("doc-A", Some("http://example.org/"));
        let ent = entity("http://example.org/people#Person", "doc-B");
        assert!(!document_matches_entity(&ent, &broad));
    }

    #[test]
    fn document_for_entity_prefers_exact_ontology_id_over_broader_base() {
        let broad = doc("doc-A", Some("http://example.org/"));
        let specific = doc("doc-B", Some("http://example.org/people#"));
        let ent = entity("http://example.org/people#Person", "doc-B");
        // Broader document listed first — must still resolve to the owning doc.
        let documents = vec![broad, specific];
        let found = document_for_entity(&documents, &ent).expect("owner");
        assert_eq!(found.id, "doc-B");
    }

    #[test]
    fn document_for_entity_longest_base_iri_wins_without_ontology_id_match() {
        let broad = doc("doc-A", Some("http://example.org/"));
        let specific = doc("doc-B", Some("http://example.org/people#"));
        // ontology_id matches neither doc id / base equality — fall back to longest prefix.
        let ent = entity("http://example.org/people#Person", "unknown");
        let documents = vec![broad, specific];
        let found = document_for_entity(&documents, &ent).expect("owner");
        assert_eq!(found.id, "doc-B");
    }

    #[test]
    fn document_for_entity_longest_base_iri_wins_when_broad_is_second() {
        let specific = doc("doc-B", Some("http://example.org/people#"));
        let broad = doc("doc-A", Some("http://example.org/"));
        let ent = entity("http://example.org/people#Person", "unknown");
        let documents = vec![specific, broad];
        let found = document_for_entity(&documents, &ent).expect("owner");
        assert_eq!(found.id, "doc-B");
    }
}
