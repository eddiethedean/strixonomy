//! Protégé Wave 2 port: entity render, Manchester escape, prefix expand, IRI split.
//! Upstream: OWLEntityRendererImpl, RenderingEscapeUtils, Prefix*, IRIExpander, IriSplitter.

use std::collections::BTreeMap;
use strixonomy_owl::{
    escape_manchester_rendering, expand_prefixed_iri, match_prefix, render_as_curie,
    render_entity_iri, split_iri, unescape_manchester_rendering,
};

fn ns() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("ex".to_string(), "http://example.org/people#".to_string()),
        ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
    ])
}

#[test]
fn render_prefers_longest_prefix_curie() {
    let namespaces = ns();
    assert_eq!(render_entity_iri("http://example.org/people#Person", &namespaces), "ex:Person");
}

#[test]
fn render_builtin_owl_and_rdfs_without_workspace_prefix() {
    assert_eq!(
        render_as_curie("http://www.w3.org/2002/07/owl#Thing", &BTreeMap::new()).as_deref(),
        Some("owl:Thing")
    );
    assert_eq!(
        render_entity_iri("http://www.w3.org/2000/01/rdf-schema#label", &BTreeMap::new()),
        "rdfs:label"
    );
}

#[test]
fn render_falls_back_to_short_name() {
    assert_eq!(render_entity_iri("http://orphan.example/ns#Widget", &BTreeMap::new()), "Widget");
}

#[test]
fn split_iri_hash_and_path() {
    assert_eq!(
        split_iri("http://example.org/people#Person"),
        ("http://example.org/people#".to_string(), "Person".to_string())
    );
    assert_eq!(
        split_iri("http://purl.obolibrary.org/obo/GO_0000001"),
        ("http://purl.obolibrary.org/obo/".to_string(), "GO_0000001".to_string())
    );
}

#[test]
fn expand_prefixed_iri_and_angle_brackets() {
    let namespaces = ns();
    assert_eq!(
        expand_prefixed_iri("ex:Person", &namespaces).as_deref(),
        Some("http://example.org/people#Person")
    );
    assert_eq!(
        expand_prefixed_iri("<http://example.org/x>", &namespaces).as_deref(),
        Some("http://example.org/x")
    );
    assert!(expand_prefixed_iri("missing:Thing", &namespaces).is_none());
}

#[test]
fn match_prefix_longest_wins() {
    let mut namespaces = ns();
    namespaces.insert("people".to_string(), "http://example.org/people#".to_string());
    // Both match; longer or either with equal length — people and ex share same ns length equal.
    let (prefix, _) = match_prefix("http://example.org/people#Person", &namespaces).expect("match");
    assert!(prefix == "ex" || prefix == "people");
}

#[test]
fn manchester_escape_spaces_and_delimiters() {
    assert_eq!(escape_manchester_rendering("Person"), "Person");
    assert_eq!(escape_manchester_rendering("has part"), "'has part'");
    assert_eq!(escape_manchester_rendering("a,b"), "'a,b'");
    assert_eq!(escape_manchester_rendering("x(y)"), "'x(y)'");
}

#[test]
fn manchester_unescape_roundtrip() {
    let original = r#"say "hi" and 'bye'"#;
    let escaped = escape_manchester_rendering(original);
    assert_eq!(unescape_manchester_rendering(&escaped), original);
}
