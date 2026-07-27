//! Portable presentation/util helpers (Protégé Wave 3 ports).
//!
//! Behavioral ports of `StringAbbreviator`, `ISO8601Formatter`,
//! `LiteralLexicalValueReplacer`, `MarkdownRenderer`, and
//! `AnnotationPropertyComparator` default ordering.
//!
//! Keep the annotation-property order in sync with
//! `extension/webview-ui/src/utils/annotationOrder.ts`.

use crate::render::unescape_manchester_rendering;
use regex::Regex;
use std::cmp::Ordering;

/// Protégé `StringAbbreviator.ELLIPSIS` (U+2026).
pub const ELLIPSIS: &str = "\u{2026}";

/// Abbreviate `s` to at most `max_len` characters, appending an ellipsis when truncated.
///
/// Matches Protégé `StringAbbreviator.abbreviateString` edge cases:
/// - empty input → empty
/// - `max_len <= 0` → ellipsis only
pub fn abbreviate_string(s: &str, max_len: usize) -> String {
    if s.is_empty() {
        return String::new();
    }
    if max_len == 0 {
        return ELLIPSIS.to_string();
    }
    // Count Unicode scalar values so ellipsis behaviour matches Java `String.length`
    // for BMP text used in Protégé tests (not grapheme clusters).
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_len {
        return s.to_string();
    }
    chars.into_iter().take(max_len).collect::<String>() + ELLIPSIS
}

/// Format a UTC datetime as `yyyy-MM-dd'T'HH:mm:ss'Z'` (Protégé ISO8601Formatter).
pub fn format_iso8601_utc(
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> String {
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

/// Result of a lexical value replacement (lang XOR datatype preserved).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexicalLiteral {
    pub lexical: String,
    pub lang: Option<String>,
    pub datatype: Option<String>,
}

/// Replace the lexical form via regex, preserving language tag or datatype IRI.
pub fn replace_lexical_value(
    lexical: &str,
    lang: Option<&str>,
    datatype: Option<&str>,
    pattern: &Regex,
    replacement: &str,
) -> LexicalLiteral {
    let new_lexical = pattern.replace_all(lexical, replacement).into_owned();
    LexicalLiteral {
        lexical: new_lexical,
        lang: lang.map(str::to_string),
        datatype: if lang.is_some() { None } else { datatype.map(str::to_string) },
    }
}

/// Whole-string lexical replacement (Protégé `replaceLexicalValue` without pattern).
pub fn replace_lexical_value_whole(
    lexical: &str,
    lang: Option<&str>,
    datatype: Option<&str>,
    replacement: &str,
) -> LexicalLiteral {
    let re = Regex::new(r"(?s)^.*$").expect("whole-string pattern");
    replace_lexical_value(lexical, lang, datatype, &re, replacement)
}

/// Markdown entity link: `[unescaped display](iri)` (Protégé `MarkdownRenderer`).
pub fn render_entity_markdown(display_name: &str, iri: &str) -> String {
    let name = unescape_manchester_rendering(display_name);
    format!("[{name}]({iri})")
}

/// Default annotation-property IRI ordering (Protégé `AnnotationPropertyComparator`).
pub const DEFAULT_ANNOTATION_PROPERTY_ORDER: &[&str] = &[
    // Labels
    "http://www.w3.org/2000/01/rdf-schema#label",
    "http://www.w3.org/2004/02/skos/core#prefLabel",
    "http://purl.org/dc/elements/1.1/title",
    // OBO identifiers / namespace
    "http://www.geneontology.org/formats/oboInOwl#id",
    "http://www.geneontology.org/formats/oboInOwl#hasAlternativeId",
    "http://www.geneontology.org/formats/oboInOwl#hasOBONamespace",
    // Definitions
    "http://purl.obolibrary.org/obo/IAO_0000115",
    "http://www.w3.org/2004/02/skos/core#definition",
    "http://www.w3.org/2004/02/skos/core#note",
    "http://purl.org/dc/elements/1.1/description",
    // License / provenance
    "http://purl.org/dc/elements/1.1/rights",
    "http://purl.org/dc/terms/license",
    "http://purl.org/dc/elements/1.1/publisher",
    "http://purl.org/dc/elements/1.1/creator",
    "http://purl.org/dc/elements/1.1/contributor",
    "http://www.w3.org/2000/01/rdf-schema#comment",
    // Other labels / seeAlso
    "http://www.w3.org/2004/02/skos/core#altLabel",
    "http://www.w3.org/2000/01/rdf-schema#seeAlso",
    "http://www.w3.org/2000/01/rdf-schema#isDefinedBy",
    // Synonyms
    "http://www.geneontology.org/formats/oboInOwl#hasExactSynonym",
    "http://www.geneontology.org/formats/oboInOwl#hasRelatedSynonym",
    "http://www.geneontology.org/formats/oboInOwl#hasBroadSynonym",
    "http://www.geneontology.org/formats/oboInOwl#hasNarrowSynonym",
];

fn annotation_order_index(iri: &str) -> Option<usize> {
    DEFAULT_ANNOTATION_PROPERTY_ORDER.iter().position(|&known| known == iri)
}

/// Compare annotation-property IRIs using the Protégé default ordering.
/// Unknown IRIs sort after known ones, then lexicographically.
pub fn cmp_annotation_property_iri(a: &str, b: &str) -> Ordering {
    match (annotation_order_index(a), annotation_order_index(b)) {
        (Some(i), Some(j)) => i.cmp(&j).then_with(|| a.cmp(b)),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => a.cmp(b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abbreviate_edges() {
        assert_eq!(abbreviate_string("", 5), "");
        assert_eq!(abbreviate_string("abc", 0), ELLIPSIS);
        assert_eq!(abbreviate_string("hello", 5), "hello");
        assert_eq!(abbreviate_string("hello", 3), format!("hel{ELLIPSIS}"));
    }

    #[test]
    fn iso8601_format() {
        assert_eq!(format_iso8601_utc(2015, 11, 20, 12, 0, 0), "2015-11-20T12:00:00Z");
    }

    #[test]
    fn lexical_replace_preserves_lang() {
        let re = Regex::new("world").unwrap();
        let lit = replace_lexical_value("hello world", Some("en"), None, &re, "there");
        assert_eq!(lit.lexical, "hello there");
        assert_eq!(lit.lang.as_deref(), Some("en"));
        assert!(lit.datatype.is_none());
    }

    #[test]
    fn markdown_unescapes_manchester() {
        assert_eq!(
            render_entity_markdown("'has part'", "http://ex.org#hp"),
            "[has part](http://ex.org#hp)"
        );
    }

    #[test]
    fn annotation_order_labels_before_comments() {
        assert_eq!(
            cmp_annotation_property_iri(
                "http://www.w3.org/2000/01/rdf-schema#label",
                "http://www.w3.org/2000/01/rdf-schema#comment"
            ),
            Ordering::Less
        );
    }
}
