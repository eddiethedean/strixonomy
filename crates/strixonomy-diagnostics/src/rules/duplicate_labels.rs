use crate::input::DiagnosticInput;
use crate::location::{entity_needles, find_in_source};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use strixonomy_core::{document_for_entity, Diagnostic, DiagnosticCode, DiagnosticSeverity};

pub fn duplicate_labels(
    data: &DiagnosticInput<'_>,
    source: &dyn Fn(&Path) -> String,
) -> Vec<Diagnostic> {
    let mut by_label: BTreeMap<String, Vec<&strixonomy_core::Entity>> = BTreeMap::new();
    for entity in data.entities {
        for label in &entity.labels {
            let key = normalize_label(label);
            if key.is_empty() {
                continue;
            }
            by_label.entry(key).or_default().push(entity);
        }
    }

    let mut diagnostics = Vec::new();
    for (label, entities) in by_label {
        let mut seen = BTreeSet::new();
        let unique: Vec<_> =
            entities.into_iter().filter(|entity| seen.insert(entity.iri.as_str())).collect();
        if unique.len() < 2 {
            continue;
        }
        for entity in unique {
            let Some(doc) = document_for_entity(data.documents, entity) else {
                continue;
            };
            let file = doc.path.clone();
            let namespaces = doc.namespaces.clone();
            let text = source(&file);
            let needles = entity_needles(&entity.iri, &entity.short_name, &namespaces);
            let range = find_in_source(&text, &needles);
            diagnostics.push(Diagnostic {
                code: DiagnosticCode::DuplicateLabel,
                severity: DiagnosticSeverity::Warning,
                message: format!("duplicate label \"{label}\" (also used by other entities)"),
                file,
                range,
                entity_iri: Some(entity.iri.clone()),
                quick_fix: None,
                plugin_id: None,
                plugin_code: None,
            });
        }
    }
    diagnostics
}

fn normalize_label(label: &str) -> String {
    label.trim_matches('"').trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::DiagnosticInput;
    use std::collections::BTreeMap;
    use std::path::Path;
    use strixonomy_core::{
        DiagnosticCode, DiagnosticSeverity, Entity, EntityKind, OntologyDocument, OntologyFormat,
        ParseStatus,
    };

    fn empty_source(_: &Path) -> String {
        String::new()
    }

    #[test]
    fn flags_duplicate_labels_with_warning() {
        let documents = vec![OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("dup.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: Some("http://example.org/dup".to_string()),
            version_iri: None,
            imports: vec![],
            namespaces: BTreeMap::new(),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        }];
        let entities = vec![
            Entity {
                iri: "http://example.org/dup#Alpha".to_string(),
                short_name: "Alpha".to_string(),
                kind: EntityKind::Class,
                ontology_id: "http://example.org/dup".to_string(),
                source_location: Default::default(),
                labels: vec!["\"Shared\"".to_string()],
                comments: vec![],
                deprecated: false,
                obo_id: None,
                characteristics: Default::default(),
            },
            Entity {
                iri: "http://example.org/dup#Beta".to_string(),
                short_name: "Beta".to_string(),
                kind: EntityKind::Class,
                ontology_id: "http://example.org/dup".to_string(),
                source_location: Default::default(),
                labels: vec!["\"Shared\"".to_string()],
                comments: vec![],
                deprecated: false,
                obo_id: None,
                characteristics: Default::default(),
            },
        ];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &entities,
            annotations: &[],
            axioms: &[],
            namespaces: &[],
            imports: &[],
        };
        let diags = duplicate_labels(&input, &empty_source);
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().all(|d| d.code == DiagnosticCode::DuplicateLabel));
        assert!(diags.iter().all(|d| d.severity == DiagnosticSeverity::Warning));
        assert!(diags[0].message.contains("shared"));
    }

    #[test]
    fn ignores_case_variant_labels_on_same_entity() {
        let documents = vec![OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("dup.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: Some("http://example.org/dup".to_string()),
            version_iri: None,
            imports: vec![],
            namespaces: BTreeMap::new(),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        }];
        let entities = vec![Entity {
            iri: "http://example.org/dup#Person".to_string(),
            short_name: "Person".to_string(),
            kind: EntityKind::Class,
            ontology_id: "http://example.org/dup".to_string(),
            source_location: Default::default(),
            labels: vec!["\"Person\"".to_string(), "\"person\"".to_string()],
            comments: vec![],
            deprecated: false,
            obo_id: None,
            characteristics: Default::default(),
        }];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &entities,
            annotations: &[],
            axioms: &[],
            namespaces: &[],
            imports: &[],
        };
        let diags = duplicate_labels(&input, &empty_source);
        assert!(diags.is_empty(), "same-entity case-variant labels must not warn: {diags:?}");
    }
}
