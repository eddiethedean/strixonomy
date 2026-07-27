//! Protégé Wave 3 port: abbreviate, ISO8601, lexical replace, markdown, annotation order.
//! Upstream: StringAbbreviator, ISO8601Formatter, LiteralLexicalValueReplacer,
//! MarkdownRenderer, AnnotationPropertyComparator.

use regex::Regex;
use std::cmp::Ordering;
use strixonomy_owl::{
    abbreviate_string, cmp_annotation_property_iri, format_iso8601_utc, render_entity_markdown,
    replace_lexical_value, replace_lexical_value_whole, ELLIPSIS,
};

#[test]
fn utils_abbreviate_string() {
    assert_eq!(abbreviate_string("OntoCode", 4), format!("Onto{ELLIPSIS}"));
    assert_eq!(abbreviate_string("abc", 10), "abc");
    assert_eq!(abbreviate_string("x", 0), ELLIPSIS);
}

#[test]
fn utils_iso8601_utc() {
    assert_eq!(format_iso8601_utc(2022, 3, 25, 9, 8, 7), "2022-03-25T09:08:07Z");
}

#[test]
fn utils_lexical_replace_pattern_and_whole() {
    let re = Regex::new(r"\d+").unwrap();
    let typed = replace_lexical_value(
        "v1",
        None,
        Some("http://www.w3.org/2001/XMLSchema#string"),
        &re,
        "2",
    );
    assert_eq!(typed.lexical, "v2");
    assert_eq!(typed.datatype.as_deref(), Some("http://www.w3.org/2001/XMLSchema#string"));

    let whole = replace_lexical_value_whole("old", Some("en"), None, "new");
    assert_eq!(whole.lexical, "new");
    assert_eq!(whole.lang.as_deref(), Some("en"));
}

#[test]
fn utils_render_entity_markdown() {
    assert_eq!(
        render_entity_markdown("Person", "http://example.org#Person"),
        "[Person](http://example.org#Person)"
    );
    assert_eq!(
        render_entity_markdown("'has part'", "http://example.org#has_part"),
        "[has part](http://example.org#has_part)"
    );
}

#[test]
fn utils_annotation_property_default_order() {
    let label = "http://www.w3.org/2000/01/rdf-schema#label";
    let comment = "http://www.w3.org/2000/01/rdf-schema#comment";
    let synonym = "http://www.geneontology.org/formats/oboInOwl#hasExactSynonym";
    let unknown = "http://example.org/customAnn";

    assert_eq!(cmp_annotation_property_iri(label, comment), Ordering::Less);
    assert_eq!(cmp_annotation_property_iri(comment, synonym), Ordering::Less);
    assert_eq!(cmp_annotation_property_iri(label, unknown), Ordering::Less);
    assert_eq!(cmp_annotation_property_iri(unknown, label), Ordering::Greater);

    let mut props = vec![comment, synonym, unknown, label];
    props.sort_by(|a, b| cmp_annotation_property_iri(a, b));
    assert_eq!(props, vec![label, comment, synonym, unknown]);
}
