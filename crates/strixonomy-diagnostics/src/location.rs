use std::collections::BTreeMap;
use strixonomy_core::SourceLocation;

pub fn find_in_source(source_text: &str, needles: &[String]) -> SourceLocation {
    for (line_idx, line) in source_text.lines().enumerate() {
        for needle in needles {
            if let Some(col) = find_token_col(line, needle) {
                return SourceLocation::at_line_col((line_idx + 1) as u64, col as u64);
            }
        }
    }
    SourceLocation::default()
}

/// Locate an import IRI, preferring `owl:imports` lines over earlier comment/string hits.
pub fn find_import_in_source(source_text: &str, import_iri: &str) -> SourceLocation {
    let needles = [import_iri.to_string(), format!("<{import_iri}>")];
    for (line_idx, line) in source_text.lines().enumerate() {
        let lower = line.to_ascii_lowercase();
        if !lower.contains("owl:imports") {
            continue;
        }
        for needle in &needles {
            if let Some(col) = find_token_col(line, needle) {
                return SourceLocation::at_line_col((line_idx + 1) as u64, col as u64);
            }
        }
    }
    find_in_source(source_text, &needles)
}

pub fn find_prefix_in_source(source_text: &str, prefix: &str) -> SourceLocation {
    let needles = vec![
        format!("@prefix {prefix}:"),
        format!("@PREFIX {prefix}:"),
        format!("PREFIX {prefix}:"),
        format!("{prefix}:"),
    ];
    find_in_source(source_text, &needles)
}

pub fn entity_needles(
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut needles = vec![iri.to_string(), format!("<{iri}>")];
    for (prefix, ns) in namespaces {
        if iri.starts_with(ns) && !prefix.is_empty() {
            needles.push(format!("{prefix}:{short_name}"));
        }
    }
    // Bare `:LocalName` last — it is a substring of `{prefix}:{short_name}`.
    needles.push(format!(":{short_name}"));
    needles
}

fn find_token_col(line: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return None;
    }
    let mut start = 0usize;
    while let Some(rel) = line[start..].find(needle) {
        let col = start + rel;
        if is_token_match_at(line, needle, col) {
            return Some(col);
        }
        start = col + 1;
    }
    None
}

/// True when `needle` at `col` in `line` is a standalone Turtle token (not a substring).
fn is_token_match_at(line: &str, needle: &str, col: usize) -> bool {
    if !line[col..].starts_with(needle) {
        return false;
    }
    is_safe_replacement_boundary(line, col, col + needle.len())
}

fn is_iri_continuation(b: u8) -> bool {
    b.is_ascii_alphanumeric()
        || matches!(b, b'_' | b'-' | b'.' | b'~' | b':' | b'#' | b'/' | b'%' | b'?' | b'=' | b'&')
}

fn is_safe_replacement_boundary(text: &str, start: usize, end: usize) -> bool {
    let before = text.as_bytes().get(start.wrapping_sub(1)).copied();
    let after = text.as_bytes().get(end).copied();
    let ok_before = before.is_none_or(|b| !is_iri_continuation(b));
    let ok_after = after.is_none_or(|b| !is_iri_continuation(b));
    ok_before && ok_after
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn entity_needles_prefixed_match_before_bare_local() {
        let mut namespaces = BTreeMap::new();
        namespaces.insert("ex".to_string(), "http://example.org/ex#".to_string());
        let needles = entity_needles("http://example.org/ex#Person", "Person", &namespaces);
        let line = "ex:Person a owl:Class .";
        let loc = find_in_source(line, &needles);
        assert_eq!(loc.line, Some(1));
        assert_eq!(loc.column, Some(0));
    }

    #[test]
    fn entity_needles_bare_local_still_matches_default_prefix() {
        let namespaces = BTreeMap::new();
        let needles = entity_needles("http://example.org/ex#Person", "Person", &namespaces);
        let line = ":Person a owl:Class .";
        let loc = find_in_source(line, &needles);
        assert_eq!(loc.line, Some(1));
        assert_eq!(loc.column, Some(0));
    }

    #[test]
    fn find_in_source_skips_substring_inside_longer_qname() {
        let mut namespaces = BTreeMap::new();
        namespaces.insert("ex".to_string(), "http://example.org/org#".to_string());
        let needles = entity_needles("http://example.org/org#Person", "Person", &namespaces);
        let text = "ex:PersonType a owl:Class .\nex:Person a owl:Class .\n";
        let loc = find_in_source(text, &needles);
        assert_eq!(loc.line, Some(2));
        assert_eq!(loc.column, Some(0));
    }

    #[test]
    fn find_import_prefers_owl_imports_over_earlier_comment() {
        let iri = "http://example.org/missing";
        let text = format!(
            "# see <{iri}>\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n<> owl:imports <{iri}> .\n"
        );
        let loc = find_import_in_source(&text, iri);
        assert_eq!(loc.line, Some(3));
    }
}
