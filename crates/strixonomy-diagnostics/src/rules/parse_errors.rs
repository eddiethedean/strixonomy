use crate::input::DiagnosticInput;
use std::path::Path;
use strixonomy_core::{
    Diagnostic, DiagnosticCode, DiagnosticSeverity, ParseStatus, SourceLocation,
};

pub fn parse_errors(
    data: &DiagnosticInput<'_>,
    _source: &dyn Fn(&Path) -> String,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for doc in data.documents {
        if doc.parse_status != ParseStatus::Error {
            continue;
        }
        let message = doc.parse_message.clone().unwrap_or_else(|| "parse error".to_string());
        let range = doc.parse_error_location.clone().unwrap_or(SourceLocation::at_line_col(1, 0));
        diagnostics.push(Diagnostic {
            code: DiagnosticCode::ParseError,
            severity: DiagnosticSeverity::Error,
            message,
            file: doc.path.clone(),
            range,
            entity_iri: None,
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
    use strixonomy_core::{
        DiagnosticCode, DiagnosticSeverity, OntologyDocument, OntologyFormat, ParseStatus,
        SourceLocation,
    };

    fn empty_source(_: &Path) -> String {
        String::new()
    }

    #[test]
    fn maps_parse_error_document_to_diagnostic() {
        let documents = vec![OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("bad.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: None,
            version_iri: None,
            imports: vec![],
            namespaces: BTreeMap::new(),
            parse_status: ParseStatus::Error,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: Some("unexpected token".to_string()),
            parse_error_location: Some(SourceLocation::at_line_col(3, 5)),
        }];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &[],
            annotations: &[],
            axioms: &[],
            namespaces: &[],
            imports: &[],
        };
        let diags = parse_errors(&input, &empty_source);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, DiagnosticCode::ParseError);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Error);
        assert_eq!(diags[0].message, "unexpected token");
        assert_eq!(diags[0].range.line, Some(3));
    }
}
