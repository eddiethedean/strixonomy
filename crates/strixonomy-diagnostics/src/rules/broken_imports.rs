use crate::input::DiagnosticInput;
use crate::location::find_import_in_source;
use std::collections::BTreeSet;
use std::path::Path;
use strixonomy_core::{
    document_for_ontology_id, file_uri_for_path, normalize_iri, Diagnostic, DiagnosticCode,
    DiagnosticSeverity, QuickFix,
};

pub fn broken_imports(
    data: &DiagnosticInput<'_>,
    source: &dyn Fn(&Path) -> String,
) -> Vec<Diagnostic> {
    let mut known_normalized = BTreeSet::new();
    for doc in data.documents {
        if let Some(base) = &doc.base_iri {
            known_normalized.insert(normalize_iri(base));
        }
        known_normalized.insert(normalize_iri(&file_uri_for_path(&doc.path)));
    }

    let mut diagnostics = Vec::new();
    for imp in data.imports {
        if known_normalized.contains(&normalize_iri(&imp.import_iri)) {
            continue;
        }
        let Some(doc) = document_for_ontology_id(data.documents, &imp.ontology_id) else {
            continue;
        };
        let file = doc.path.clone();

        let text = source(&file);
        let range = find_import_in_source(&text, &imp.import_iri);

        let quick_fix = range.line.and_then(|line| {
            QuickFix::RemoveLine {
                label: format!("Remove broken import <{}>", imp.import_iri),
                line: line as usize,
            }
            .encode()
        });

        diagnostics.push(Diagnostic {
            code: DiagnosticCode::BrokenImport,
            severity: DiagnosticSeverity::Error,
            message: format!("import not found in workspace: {}", imp.import_iri),
            file,
            range,
            entity_iri: Some(imp.import_iri.clone()),
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
    use strixonomy_core::{Import, OntologyDocument, OntologyFormat, ParseStatus};

    fn empty_source(_: &Path) -> String {
        String::new()
    }

    #[test]
    fn trailing_slash_import_resolves() {
        let documents = vec![OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("a.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: Some("http://example.org/people".to_string()),
            version_iri: None,
            imports: vec![],
            namespaces: BTreeMap::new(),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        }];
        let imports = vec![Import {
            ontology_id: "http://example.org/org".to_string(),
            import_iri: "http://example.org/people/".to_string(),
        }];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &[],
            annotations: &[],
            axioms: &[],
            namespaces: &[],
            imports: &imports,
        };
        let diags = broken_imports(&input, &empty_source);
        assert!(diags.is_empty(), "expected trailing slash import to resolve");
    }
}
