use crate::input::DiagnosticInput;
use crate::location::find_prefix_in_source;
use std::collections::BTreeSet;
use std::path::Path;
use strixonomy_core::{
    Diagnostic, DiagnosticCode, DiagnosticSeverity, OntologyFormat, ParseStatus, QuickFix,
};

pub fn undefined_prefixes(
    data: &DiagnosticInput<'_>,
    source: &dyn Fn(&Path) -> String,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for doc in data.documents {
        if doc.parse_status == ParseStatus::Error {
            continue;
        }
        if !matches!(
            doc.format,
            OntologyFormat::Turtle
                | OntologyFormat::TriG
                | OntologyFormat::Owl
                | OntologyFormat::RdfXml
        ) {
            continue;
        }
        if matches!(doc.format, OntologyFormat::RdfXml | OntologyFormat::Owl) {
            continue;
        }
        let text = source(&doc.path);
        let declared: BTreeSet<&str> = doc.namespaces.keys().map(String::as_str).collect();
        let builtins = builtin_prefixes();
        let scan_text = strip_comments_and_strings(&text);

        for cap in QNAME_RE.captures_iter(&scan_text) {
            let full = cap.get(0).map(|m| m.as_str()).unwrap_or("");
            if full.contains("://") {
                continue;
            }
            let prefix = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            if prefix.eq_ignore_ascii_case("urn") {
                continue;
            }
            if prefix.is_empty() || declared.contains(prefix) || builtins.contains(prefix) {
                continue;
            }
            let range = find_prefix_in_source(&text, prefix);
            let message = format!("undefined prefix: {prefix}:");
            if diagnostics.iter().any(|d: &Diagnostic| {
                d.file == doc.path
                    && d.code == DiagnosticCode::UndefinedPrefix
                    && d.message == message
                    && d.range.line == range.line
            }) {
                continue;
            }
            let quick_fix = QuickFix::InsertText {
                label: format!("Declare @prefix {prefix}:"),
                line: 1,
                column: 1,
                text: format!("@prefix {prefix}: <http://example.org/{prefix}#> .\n"),
            }
            .encode();
            diagnostics.push(Diagnostic {
                code: DiagnosticCode::UndefinedPrefix,
                severity: DiagnosticSeverity::Error,
                message,
                file: doc.path.clone(),
                range,
                entity_iri: None,
                quick_fix,
                plugin_id: None,
                plugin_code: None,
            });
        }
    }
    diagnostics
}

fn strip_comments_and_strings(text: &str) -> String {
    // Keep aligned with strixonomy-owl/src/turtle_lex.rs (canonical byte lexer).
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '#' => {
                while chars.next().is_some_and(|ch| ch != '\n') {}
                out.push(' ');
            }
            '<' => {
                // Blank IRIs so QNAME_RE does not match colons inside <…>.
                out.push(' ');
                for ch in chars.by_ref() {
                    out.push(' ');
                    if ch == '>' {
                        break;
                    }
                }
            }
            '"' => {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    if chars.peek() == Some(&'"') {
                        chars.next();
                        let mut escape = false;
                        while let Some(ch) = chars.next() {
                            if escape {
                                escape = false;
                                continue;
                            }
                            if ch == '\\' {
                                escape = true;
                                continue;
                            }
                            if ch == '"' && chars.peek() == Some(&'"') {
                                let _ = chars.next();
                                if chars.peek() == Some(&'"') {
                                    chars.next();
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    #[allow(clippy::while_let_on_iterator)]
                    while let Some(ch) = chars.next() {
                        if ch == '"' {
                            break;
                        }
                        if ch == '\\' {
                            chars.next();
                        }
                    }
                }
                out.push(' ');
            }
            '\'' => {
                if chars.peek() == Some(&'\'') {
                    chars.next();
                    if chars.peek() == Some(&'\'') {
                        chars.next();
                        let mut escape = false;
                        while let Some(ch) = chars.next() {
                            if escape {
                                escape = false;
                                continue;
                            }
                            if ch == '\\' {
                                escape = true;
                                continue;
                            }
                            if ch == '\'' && chars.peek() == Some(&'\'') {
                                let _ = chars.next();
                                if chars.peek() == Some(&'\'') {
                                    chars.next();
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    #[allow(clippy::while_let_on_iterator)]
                    while let Some(ch) = chars.next() {
                        if ch == '\'' {
                            break;
                        }
                        if ch == '\\' {
                            chars.next();
                        }
                    }
                }
                out.push(' ');
            }
            other => out.push(other),
        }
    }
    out
}

fn builtin_prefixes() -> BTreeSet<&'static str> {
    ["rdf", "rdfs", "owl", "xsd", "xml", "xmlns", "sh", "skos", "dc", "dcterms", "foaf", "schema"]
        .into_iter()
        .collect()
}

static QNAME_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"([A-Za-z][\w-]*):[A-Za-z_][\w-]*").expect("qname regex")
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::DiagnosticInput;
    use std::collections::BTreeMap;
    use std::path::Path;
    use strixonomy_core::{
        DiagnosticCode, DiagnosticSeverity, OntologyDocument, OntologyFormat, ParseStatus,
    };

    fn source_for(path: &Path) -> String {
        match path.to_str() {
            Some("live.ttl") => {
                "@prefix ex: <http://example.org/live#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\nex:Ok a owl:Class .\nun:Bad a owl:Class .\n".to_string()
            }
            _ => String::new(),
        }
    }

    #[test]
    fn detects_undefined_prefix_in_turtle() {
        let documents = vec![OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("live.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: Some("http://example.org/live".to_string()),
            version_iri: None,
            imports: vec![],
            namespaces: BTreeMap::from([(
                "ex".to_string(),
                "http://example.org/live#".to_string(),
            )]),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        }];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &[],
            annotations: &[],
            axioms: &[],
            namespaces: &[],
            imports: &[],
        };
        let diags = undefined_prefixes(&input, &source_for);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, DiagnosticCode::UndefinedPrefix);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Error);
        assert!(diags[0].message.contains("un:"));
        assert!(diags[0].quick_fix.is_some(), "undefined prefix should offer quick fix");
    }

    #[test]
    fn ignores_undefined_prefix_inside_comment() {
        let text = "@prefix ex: <http://ex/> .\n# un:Hidden a owl:Class .\nex:Ok a owl:Class .\n";
        let documents = vec![OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("ok.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: Some("http://ex/".to_string()),
            version_iri: None,
            imports: vec![],
            namespaces: BTreeMap::from([("ex".to_string(), "http://ex/".to_string())]),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        }];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &[],
            annotations: &[],
            axioms: &[],
            namespaces: &[],
            imports: &[],
        };
        let diags = undefined_prefixes(&input, &|_| text.to_string());
        assert!(diags.is_empty(), "commented qname should not lint: {diags:?}");
    }

    #[test]
    fn strip_does_not_eat_past_empty_double_quotes() {
        let stripped = strip_comments_and_strings(r#"""ex:Bad a owl:Class ."#);
        assert!(
            stripped.contains("ex:Bad"),
            "empty \"\" must not consume following QName: {stripped:?}"
        );
    }

    #[test]
    fn strip_handles_long_single_quoted_strings() {
        let stripped = strip_comments_and_strings("ex:Ok rdfs:comment '''note about un:Bad''' .");
        assert!(!stripped.contains("un:Bad"), "qname inside ''' must be stripped: {stripped:?}");
        assert!(stripped.contains("ex:Ok"));
    }

    #[test]
    fn ignores_undefined_prefix_inside_long_single_quoted_string() {
        let text = "@prefix ex: <http://ex/> .\nex:Ok rdfs:comment '''note about un:Bad''' .\n";
        let documents = vec![OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("ok.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: Some("http://ex/".to_string()),
            version_iri: None,
            imports: vec![],
            namespaces: BTreeMap::from([("ex".to_string(), "http://ex/".to_string())]),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        }];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &[],
            annotations: &[],
            axioms: &[],
            namespaces: &[],
            imports: &[],
        };
        let diags = undefined_prefixes(&input, &|_| text.to_string());
        assert!(diags.is_empty(), "long single-quoted string should not lint: {diags:?}");
    }

    #[test]
    fn strip_handles_escaped_double_quotes() {
        let stripped =
            strip_comments_and_strings(r#"ex:Ok rdfs:label "a\"b" . un:Bad a owl:Class ."#);
        assert!(!stripped.contains("a\\\"b") && !stripped.contains(r#"a"b"#));
        assert!(
            stripped.contains("un:Bad"),
            "qname after escaped string must remain: {stripped:?}"
        );
    }

    #[test]
    fn strip_handles_escaped_single_quotes() {
        let stripped =
            strip_comments_and_strings(r#"ex:Ok rdfs:label 'a\'b' . un:Bad a owl:Class ."#);
        assert!(stripped.contains("un:Bad"), "qname after escaped ': {stripped:?}");
    }

    #[test]
    fn ignores_qname_inside_empty_and_escaped_strings() {
        let text = r#"@prefix ex: <http://ex/> .
ex:Ok rdfs:label "" .
ex:Ok2 rdfs:label "say \"hi\"" .
ex:Ok3 rdfs:label 'say \'hi\'' .
"#;
        let documents = vec![OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("ok.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: Some("http://ex/".to_string()),
            version_iri: None,
            imports: vec![],
            namespaces: BTreeMap::from([("ex".to_string(), "http://ex/".to_string())]),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        }];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &[],
            annotations: &[],
            axioms: &[],
            namespaces: &[],
            imports: &[],
        };
        let diags = undefined_prefixes(&input, &|_| text.to_string());
        assert!(diags.is_empty(), "string contents should not lint: {diags:?}");
    }

    #[test]
    fn ignores_colons_inside_angle_bracket_iris() {
        let text = r#"@prefix ex: <http://ex/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
ex:Ok a owl:Class ;
    rdfs:seeAlso <http://dbpedia.org/resource/Foo:Bar> .
"#;
        let documents = vec![OntologyDocument {
            id: "doc-1".to_string(),
            path: Path::new("ok.ttl").to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: Some("http://ex/".to_string()),
            version_iri: None,
            imports: vec![],
            namespaces: BTreeMap::from([
                ("ex".to_string(), "http://ex/".to_string()),
                ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
                ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
            ]),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        }];
        let input = DiagnosticInput {
            documents: &documents,
            entities: &[],
            annotations: &[],
            axioms: &[],
            namespaces: &[],
            imports: &[],
        };
        let diags = undefined_prefixes(&input, &|_| text.to_string());
        assert!(diags.is_empty(), "colons inside <IRI> must not lint as prefixes: {diags:?}");
        let stripped = strip_comments_and_strings(text);
        assert!(!stripped.contains("Foo:Bar"), "IRI interior must be blanked: {stripped:?}");
    }
}
