use crate::input::DiagnosticInput;
use crate::location::{entity_needles, find_in_source};
use std::path::Path;
use strixonomy_core::{
    document_for_entity, Diagnostic, DiagnosticCode, DiagnosticSeverity, EntityKind, QuickFix,
};

pub fn missing_labels(
    data: &DiagnosticInput<'_>,
    source: &dyn Fn(&Path) -> String,
) -> Vec<Diagnostic> {
    let kinds = [
        EntityKind::Class,
        EntityKind::ObjectProperty,
        EntityKind::DataProperty,
        EntityKind::AnnotationProperty,
    ];
    let mut diagnostics = Vec::new();
    for entity in data.entities {
        if !kinds.contains(&entity.kind) || !entity.labels.is_empty() {
            continue;
        }
        let Some(doc) = document_for_entity(data.documents, entity) else {
            continue;
        };
        let file = doc.path.clone();
        let namespaces = doc.namespaces.clone();
        let text = source(&file);
        let needles = entity_needles(&entity.iri, &entity.short_name, &namespaces);
        let range = find_in_source(&text, &needles);
        let label_value = entity.short_name.clone();
        let quick_fix = QuickFix::ApplyPatch {
            label: format!("Add rdfs:label \"{label_value}\""),
            document_path: file.display().to_string(),
            patches: vec![serde_json::json!({
                "op": "add_label",
                "entity_iri": entity.iri,
                "value": label_value,
            })],
        }
        .encode();
        diagnostics.push(Diagnostic {
            code: DiagnosticCode::MissingLabel,
            severity: DiagnosticSeverity::Warning,
            message: format!("{} has no rdfs:label", entity.kind.as_str()),
            file,
            range,
            entity_iri: Some(entity.iri.clone()),
            quick_fix,
            plugin_id: None,
            plugin_code: None,
        });
    }
    diagnostics
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
    fn flags_class_without_label() {
        let documents = vec![OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("test.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: Some("http://ex/".to_string()),
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
            iri: "http://ex/Unlabeled".to_string(),
            short_name: "Unlabeled".to_string(),
            kind: EntityKind::Class,
            ontology_id: "http://ex/".to_string(),
            source_location: Default::default(),
            labels: vec![],
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
        let diags = missing_labels(&input, &empty_source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, DiagnosticCode::MissingLabel);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Warning);
        assert!(diags[0].message.contains("rdfs:label"));
    }

    #[test]
    fn missing_label_column_at_prefixed_entity_start() {
        let source_text = concat!(
            "@prefix ex: <http://example.org/ex#> .\n",
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\n",
            "ex:Unlabeled a owl:Class .\n"
        );
        let mut namespaces = BTreeMap::new();
        namespaces.insert("ex".to_string(), "http://example.org/ex#".to_string());
        let documents = vec![OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("test.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: Some("http://example.org/ex#".to_string()),
            version_iri: None,
            imports: vec![],
            namespaces: namespaces.clone(),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        }];
        let entities = vec![Entity {
            iri: "http://example.org/ex#Unlabeled".to_string(),
            short_name: "Unlabeled".to_string(),
            kind: EntityKind::Class,
            ontology_id: "http://example.org/ex#".to_string(),
            source_location: Default::default(),
            labels: vec![],
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
        let diags = missing_labels(&input, &|_| source_text.to_string());
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].range.line, Some(4));
        assert_eq!(diags[0].range.column, Some(0));
    }
}
