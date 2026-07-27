//! Entity / Manchester rendering helpers (Protégé Wave 2 ports).
//!
//! Behavioral ports of `OWLEntityRendererImpl`, `RenderingEscapeUtils`,
//! `IriSplitter`, and prefix expand/match/render helpers.

use crate::patch::best_namespace_match;
use crate::span::short_name_from_iri;
use std::collections::BTreeMap;

const OWL_NS: &str = "http://www.w3.org/2002/07/owl#";
const RDFS_NS: &str = "http://www.w3.org/2000/01/rdf-schema#";
const RDF_NS: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
const XSD_NS: &str = "http://www.w3.org/2001/XMLSchema#";

/// Split an IRI into `(namespace, local)` using `#` then `/` (Protégé IriSplitter-style).
pub fn split_iri(iri: &str) -> (String, String) {
    if let Some((ns, local)) = iri.rsplit_once('#') {
        return (format!("{ns}#"), local.to_string());
    }
    if let Some((ns, local)) = iri.rsplit_once('/') {
        return (format!("{ns}/"), local.to_string());
    }
    (String::new(), iri.to_string())
}

/// Render an entity IRI for display: prefer longest prefix CURIE, else short name, else bracketed IRI.
pub fn render_entity_iri(iri: &str, namespaces: &BTreeMap<String, String>) -> String {
    if let Some(curie) = render_as_curie(iri, namespaces) {
        return curie;
    }
    let short = short_name_from_iri(iri);
    if !short.is_empty() && short != iri {
        return short;
    }
    format!("<{iri}>")
}

/// Prefer built-in vocabulary CURIEs (`owl:`, `rdfs:`, `rdf:`, `xsd:`) then workspace prefixes.
pub fn render_as_curie(iri: &str, namespaces: &BTreeMap<String, String>) -> Option<String> {
    let mut ns = namespaces.clone();
    ns.entry("owl".into()).or_insert_with(|| OWL_NS.to_string());
    ns.entry("rdfs".into()).or_insert_with(|| RDFS_NS.to_string());
    ns.entry("rdf".into()).or_insert_with(|| RDF_NS.to_string());
    ns.entry("xsd".into()).or_insert_with(|| XSD_NS.to_string());
    best_namespace_match(iri, &ns).map(|(prefix, base)| {
        let local = &iri[base.len()..];
        format!("{prefix}:{local}")
    })
}

/// Expand `prefix:local` using `namespaces`. Bare IRIs and `<iri>` pass through.
pub fn expand_prefixed_iri(token: &str, namespaces: &BTreeMap<String, String>) -> Option<String> {
    let token = token.trim();
    if token.is_empty() {
        return None;
    }
    if token.starts_with('<') && token.ends_with('>') && token.len() >= 2 {
        return Some(token[1..token.len() - 1].to_string());
    }
    if token.contains("://") {
        return Some(token.to_string());
    }
    let (prefix, local) = token.split_once(':')?;
    let base = namespaces.get(prefix)?;
    Some(format!("{base}{local}"))
}

/// Longest-prefix match: returns `(prefix, namespace_iri)` or `None`.
pub fn match_prefix<'a>(
    iri: &str,
    namespaces: &'a BTreeMap<String, String>,
) -> Option<(&'a str, &'a str)> {
    best_namespace_match(iri, namespaces)
}

/// Manchester-style escaped rendering (Protégé `RenderingEscapeUtils.getEscapedRendering`).
pub fn escape_manchester_rendering(original: &str) -> String {
    let mut rendering = original.replace('\\', "\\\\");
    rendering = rendering.replace('\'', "\\'");
    rendering = rendering.replace('"', "\\\"");
    let needs_quotes = original.chars().any(|c| {
        matches!(
            c,
            ' ' | '\\' | ',' | '<' | '>' | '=' | '^' | '@' | '{' | '}' | '[' | ']' | '(' | ')'
        )
    });
    if needs_quotes {
        format!("'{rendering}'")
    } else {
        rendering
    }
}

/// Inverse of [`escape_manchester_rendering`] (Protégé `RenderingEscapeUtils.unescape`).
pub fn unescape_manchester_rendering(rendering: &str) -> String {
    let mut s = rendering.to_string();
    if s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2 {
        s = s[1..s.len() - 1].to_string();
    }
    s = s.replace("\\\"", "\"");
    s = s.replace("\\'", "'");
    s = s.replace("\\\\", "\\");
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ns() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("ex".to_string(), "http://example.org/#".to_string()),
            ("owl".to_string(), OWL_NS.to_string()),
        ])
    }

    #[test]
    fn split_hash_and_slash() {
        assert_eq!(
            split_iri("http://example.org#Person"),
            ("http://example.org#".into(), "Person".into())
        );
        assert_eq!(
            split_iri("http://example.org/obo/GO_1"),
            ("http://example.org/obo/".into(), "GO_1".into())
        );
    }

    #[test]
    fn render_curie_and_builtin() {
        assert_eq!(render_entity_iri("http://example.org/#Person", &ns()), "ex:Person");
        assert_eq!(
            render_entity_iri("http://www.w3.org/2002/07/owl#Thing", &BTreeMap::new()),
            "owl:Thing"
        );
    }

    #[test]
    fn expand_and_match_prefix() {
        let namespaces = ns();
        assert_eq!(
            expand_prefixed_iri("ex:Person", &namespaces).as_deref(),
            Some("http://example.org/#Person")
        );
        let (p, base) = match_prefix("http://example.org/#Person", &namespaces).unwrap();
        assert_eq!(p, "ex");
        assert_eq!(base, "http://example.org/#");
    }

    #[test]
    fn manchester_escape_roundtrip() {
        assert_eq!(escape_manchester_rendering("Person"), "Person");
        assert_eq!(escape_manchester_rendering("has part"), "'has part'");
        assert_eq!(unescape_manchester_rendering(&escape_manchester_rendering("a'b")), "a'b");
    }
}
