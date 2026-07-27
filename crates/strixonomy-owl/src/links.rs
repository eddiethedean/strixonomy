//! Annotation hyperlink extractors (Protégé Wave 2 `*LinkExtractor` ports).
//!
//! Keep regex tables in sync with
//! `extension/webview-ui/src/utils/annotationLinks.ts`.

use regex::Regex;
use std::sync::OnceLock;

/// A single hyperlink match inside annotation text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationLink {
    pub kind: String,
    pub matched_text: String,
    pub url: String,
    pub start: usize,
    pub end: usize,
}

struct Extractor {
    name: &'static str,
    pattern: Regex,
    /// Replacement template: `$1` is replaced with capture group 1.
    url_template: &'static str,
}

fn extractors() -> &'static [Extractor] {
    static EXTRACTORS: OnceLock<Vec<Extractor>> = OnceLock::new();
    EXTRACTORS.get_or_init(|| {
        vec![
            Extractor {
                name: "DOI",
                pattern: Regex::new(r"(?i)DOI:\s*([^\s]+)").expect("DOI pattern"),
                url_template: "https://doi.org/$1",
            },
            Extractor {
                name: "PubMedId",
                pattern: Regex::new(r"(?i)PMID:\s*(\d+)").expect("PMID pattern"),
                url_template: "http://www.ncbi.nlm.nih.gov/pubmed/$1",
            },
            Extractor {
                name: "ORCID",
                pattern: Regex::new(r"(?i)ORCID:\s*([^\s]+)").expect("ORCID pattern"),
                url_template: "https://orcid.org/$1",
            },
            Extractor {
                name: "OMIMPS",
                // Longer OMIMPS before OMIM so "OMIMPS:1" is not partial-matched as OMIM.
                pattern: Regex::new(r"(?i)OMIMPS:\s*(\d+)").expect("OMIMPS pattern"),
                url_template: "https://www.omim.org/phenotypicSeries/$1",
            },
            Extractor {
                name: "OMIM",
                pattern: Regex::new(r"(?i)OMIM:\s*(\d+)").expect("OMIM pattern"),
                url_template: "https://omim.org/entry/$1",
            },
            Extractor {
                name: "Orphanet",
                pattern: Regex::new(r"(?i)Orphanet:\s*(\d+)").expect("Orphanet pattern"),
                url_template: "https://www.orpha.net/consor/cgi-bin/OC_Exp.php?Expert=$1",
            },
            Extractor {
                name: "ISBN-10",
                pattern: Regex::new(r"(?i)ISBN:(\d{10})").expect("ISBN pattern"),
                url_template: "http://www.isbnsearch.org/isbn/$1",
            },
            Extractor {
                name: "WikipediaVersioned",
                pattern: Regex::new(r"(?i)WikipediaVersioned:([^\s]+)").expect("WikiVer pattern"),
                url_template: "https://wikipedia.org/wiki/index.php?title=$1",
            },
            Extractor {
                name: "Wikipedia",
                pattern: Regex::new(r"(?i)Wikipedia:([^\s]+)").expect("Wikipedia pattern"),
                url_template: "https://wikipedia.org/wiki/$1",
            },
        ]
    })
}

fn apply_template(template: &str, capture: &str) -> String {
    template.replace("$1", capture)
}

/// Extract all annotation hyperlinks from `text` (non-overlapping, left-to-right).
pub fn extract_links(text: &str) -> Vec<AnnotationLink> {
    let mut found: Vec<AnnotationLink> = Vec::new();
    for ex in extractors() {
        for caps in ex.pattern.captures_iter(text) {
            let Some(m) = caps.get(0) else { continue };
            let capture = caps.get(1).map(|c| c.as_str()).unwrap_or("");
            found.push(AnnotationLink {
                kind: ex.name.to_string(),
                matched_text: m.as_str().to_string(),
                url: apply_template(ex.url_template, capture),
                start: m.start(),
                end: m.end(),
            });
        }
    }
    found.sort_by_key(|l| (l.start, l.end));
    // Drop overlapping matches (prefer earlier / longer already ordered).
    let mut out = Vec::new();
    let mut cursor = 0usize;
    for link in found {
        if link.start < cursor {
            continue;
        }
        cursor = link.end;
        out.push(link);
    }
    out
}

/// First matching link URL for `text`, if any (Protégé `extractLinkLiteral` style).
pub fn extract_first_link_url(text: &str) -> Option<String> {
    extract_links(text).into_iter().next().map(|l| l.url)
}

/// Build a regex-based extractor for custom patterns (Protégé `RegExBasedLinkExtractor`).
pub fn extract_with_pattern(
    name: &str,
    pattern: &Regex,
    url_template: &str,
    text: &str,
) -> Option<AnnotationLink> {
    let caps = pattern.captures(text)?;
    let m = caps.get(0)?;
    let capture = caps.get(1).map(|c| c.as_str()).unwrap_or("");
    Some(AnnotationLink {
        kind: name.to_string(),
        matched_text: m.as_str().to_string(),
        url: apply_template(url_template, capture),
        start: m.start(),
        end: m.end(),
    })
}

/// Escape markdown specials, then wrap extracted link spans as `[text](url)`.
pub fn linkify_markdown_text(text: &str) -> String {
    let links = extract_links(text);
    if links.is_empty() {
        return escape_md_plain(text);
    }
    let mut out = String::new();
    let mut idx = 0;
    for link in &links {
        if link.start > idx {
            out.push_str(&escape_md_plain(&text[idx..link.start]));
        }
        out.push('[');
        out.push_str(&escape_md_plain(&link.matched_text));
        out.push_str("](");
        out.push_str(&link.url);
        out.push(')');
        idx = link.end;
    }
    if idx < text.len() {
        out.push_str(&escape_md_plain(&text[idx..]));
    }
    out
}

fn escape_md_plain(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .replace('<', "\\<")
        .replace('>', "\\>")
        .replace('`', "\\`")
        .replace('*', "\\*")
        .replace('_', "\\_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doi_and_pmid() {
        let links = extract_links("See DOI: 10.1000/xyz and PMID: 12345");
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].kind, "DOI");
        assert_eq!(links[0].url, "https://doi.org/10.1000/xyz");
        assert_eq!(links[1].kind, "PubMedId");
        assert_eq!(links[1].url, "http://www.ncbi.nlm.nih.gov/pubmed/12345");
    }

    #[test]
    fn omimps_before_omim() {
        let links = extract_links("OMIMPS: 99");
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].kind, "OMIMPS");
    }

    #[test]
    fn generic_regex_extractor() {
        let re = Regex::new(r"(?i)CUSTOM:\s*(\w+)").unwrap();
        let link =
            extract_with_pattern("Custom", &re, "https://example.org/$1", "CUSTOM: abc").unwrap();
        assert_eq!(link.url, "https://example.org/abc");
    }

    #[test]
    fn linkify_embeds_markdown() {
        let md = linkify_markdown_text("PMID: 1");
        assert!(md.contains("[PMID: 1](http://www.ncbi.nlm.nih.gov/pubmed/1)"));
    }
}
