use crate::input::DiagnosticInput;
use crate::location::{entity_needles, find_in_source};
use std::collections::BTreeSet;
use std::path::Path;
use strixonomy_core::{
    document_for_entity, Diagnostic, DiagnosticCode, DiagnosticSeverity, EntityKind,
    AXIOM_KIND_SUB_CLASS_OF,
};

const BUILTIN_ROOTS: &[&str] = &[
    "http://www.w3.org/2002/07/owl#Thing",
    "http://www.w3.org/1999/02/22-rdf-syntax-ns#Resource",
    "http://www.w3.org/2000/01/rdf-schema#Resource",
];

/// True when `object` looks like a Manchester/restriction expression rather than a bare IRI.
fn is_complex_class_expression(object: &str) -> bool {
    let trimmed = object.trim();
    if trimmed.is_empty() {
        return false;
    }
    // Absolute / angle-bracket IRIs and plain CURIEs are named parents.
    if trimmed.starts_with('<') && trimmed.ends_with('>') {
        return false;
    }
    if trimmed.contains("://") {
        return false;
    }
    // Restriction / Manchester operators (and whitespace-separated expressions).
    let lower = trimmed.to_ascii_lowercase();
    if lower.contains(" some ")
        || lower.contains(" only ")
        || lower.contains(" value ")
        || lower.contains(" min ")
        || lower.contains(" max ")
        || lower.contains(" exactly ")
        || lower.contains(" and ")
        || lower.contains(" or ")
        || lower.contains(" not ")
        || lower.starts_with("not ")
    {
        return true;
    }
    // Parentheses or commas typically mark complex expressions, not bare IRIs/CURIEs.
    if trimmed.contains('(') || trimmed.contains(')') {
        return true;
    }
    // Bare CURIE / local name without operators → named (possibly unresolved) parent.
    false
}

pub fn orphan_classes(
    data: &DiagnosticInput<'_>,
    source: &dyn Fn(&Path) -> String,
) -> Vec<Diagnostic> {
    let classes: Vec<_> = data.entities.iter().filter(|e| e.kind == EntityKind::Class).collect();
    let entity_iris: BTreeSet<&str> = data.entities.iter().map(|e| e.iri.as_str()).collect();
    let child_set: BTreeSet<&str> = data
        .axioms
        .iter()
        .filter(|a| a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF)
        .map(|a| a.subject.as_str())
        .collect();
    let parent_object_set: BTreeSet<&str> = data
        .axioms
        .iter()
        .filter(|a| a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF)
        .map(|a| a.object.as_str())
        .collect();

    let mut diagnostics = Vec::new();
    for class in classes {
        if BUILTIN_ROOTS.contains(&class.iri.as_str()) {
            continue;
        }
        // Taxonomy roots: other classes subclass this entity in the workspace.
        if parent_object_set.contains(class.iri.as_str()) {
            continue;
        }
        let is_orphan = if !child_set.contains(class.iri.as_str()) {
            true
        } else {
            let parents: Vec<_> = data
                .axioms
                .iter()
                .filter(|a| a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF && a.subject == class.iri)
                .map(|a| a.object.as_str())
                .collect();
            // Named catalog/builtin parents OR complex/Manchester parents count
            // as taxonomic structure (#401).
            !parents.iter().any(|p| {
                entity_iris.contains(p)
                    || BUILTIN_ROOTS.contains(p)
                    || is_complex_class_expression(p)
            })
        };
        if !is_orphan {
            continue;
        }
        let Some(doc) = document_for_entity(data.documents, class) else {
            continue;
        };
        let file = doc.path.clone();
        let namespaces = doc.namespaces.clone();
        let text = source(&file);
        let needles = entity_needles(&class.iri, &class.short_name, &namespaces);
        let range = find_in_source(&text, &needles);
        diagnostics.push(Diagnostic {
            code: DiagnosticCode::OrphanClass,
            severity: DiagnosticSeverity::Warning,
            message: "class has no parent in workspace catalog".to_string(),
            file,
            range,
            entity_iri: Some(class.iri.clone()),
            quick_fix: None,
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
    use strixonomy_core::{Entity, OntologyDocument, OntologyFormat, ParseStatus};

    fn empty_source(_: &Path) -> String {
        String::new()
    }

    #[test]
    fn orphan_class_detected_without_catalog_parent() {
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
            iri: "http://ex/Orphan".to_string(),
            short_name: "Orphan".to_string(),
            kind: EntityKind::Class,
            ontology_id: "doc-1".to_string(),
            source_location: Default::default(),
            labels: vec!["\"Orphan\"".to_string()],
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
        let diags = orphan_classes(&input, &empty_source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, DiagnosticCode::OrphanClass);
    }

    #[test]
    fn root_class_with_children_not_orphan() {
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
        let entities = vec![
            Entity {
                iri: "http://ex/Thing".to_string(),
                short_name: "Thing".to_string(),
                kind: EntityKind::Class,
                ontology_id: "http://ex/".to_string(),
                source_location: Default::default(),
                labels: vec![],
                comments: vec![],
                deprecated: false,
                obo_id: None,
                characteristics: Default::default(),
            },
            Entity {
                iri: "http://ex/Person".to_string(),
                short_name: "Person".to_string(),
                kind: EntityKind::Class,
                ontology_id: "http://ex/".to_string(),
                source_location: Default::default(),
                labels: vec![],
                comments: vec![],
                deprecated: false,
                obo_id: None,
                characteristics: Default::default(),
            },
        ];
        let axioms = vec![strixonomy_core::Axiom {
            id: "a1".to_string(),
            ontology_id: "http://ex/".to_string(),
            subject: "http://ex/Person".to_string(),
            predicate: "subClassOf".to_string(),
            object: "http://ex/Thing".to_string(),
            axiom_kind: AXIOM_KIND_SUB_CLASS_OF.to_string(),
            source_location: Default::default(),
            annotations: Vec::new(),
        }];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &entities,
            annotations: &[],
            axioms: &axioms,
            namespaces: &[],
            imports: &[],
        };
        let diags = orphan_classes(&input, &empty_source);
        assert!(
            !diags.iter().any(|d| d.entity_iri.as_deref() == Some("http://ex/Thing")),
            "root with children should not be orphan"
        );
    }

    #[test]
    fn class_with_only_restriction_parent_is_not_orphan() {
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
            iri: "http://ex/A".to_string(),
            short_name: "A".to_string(),
            kind: EntityKind::Class,
            ontology_id: "doc-1".to_string(),
            source_location: Default::default(),
            labels: vec![],
            comments: vec![],
            deprecated: false,
            obo_id: None,
            characteristics: Default::default(),
        }];
        let axioms = vec![strixonomy_core::Axiom {
            id: "a1".to_string(),
            ontology_id: "doc-1".to_string(),
            subject: "http://ex/A".to_string(),
            predicate: "subClassOf".to_string(),
            object: "ex:p some ex:B".to_string(),
            axiom_kind: AXIOM_KIND_SUB_CLASS_OF.to_string(),
            source_location: Default::default(),
            annotations: Vec::new(),
        }];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &entities,
            annotations: &[],
            axioms: &axioms,
            namespaces: &[],
            imports: &[],
        };
        let diags = orphan_classes(&input, &empty_source);
        assert!(diags.is_empty(), "restriction parents should not flag orphan_class: {diags:?}");
    }
}
