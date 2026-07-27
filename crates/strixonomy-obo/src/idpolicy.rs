//! OBO idranges policy parse (Protégé `IdRangesPolicyParser` behavioral port).
//!
//! Parses IAO ontology annotations and allocated datatype ranges from GO-style
//! Manchester OWL idranges documents (and a thin Turtle equivalent).

use std::path::Path;
use strixonomy_core::OntologyDocument;
use thiserror::Error;

/// IAO_0000599 — id prefix (e.g. `http://purl.obolibrary.org/obo/GO_`).
pub const IRI_ID_PREFIX: &str = "http://purl.obolibrary.org/obo/IAO_0000599";
/// IAO_0000596 — number of digits in local ids.
pub const IRI_ID_DIGITS: &str = "http://purl.obolibrary.org/obo/IAO_0000596";
/// IAO_0000598 — idspace / "ids for" (e.g. `GO`).
pub const IRI_IDS_FOR: &str = "http://purl.obolibrary.org/obo/IAO_0000598";
/// IAO_0000597 — allocated-to annotation on range datatypes.
pub const IRI_ALLOCATED_TO: &str = "http://purl.obolibrary.org/obo/IAO_0000597";

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdPolicyError {
    #[error("{0}")]
    Message(String),
    #[error("io: {0}")]
    Io(String),
}

impl IdPolicyError {
    fn msg(s: impl Into<String>) -> Self {
        Self::Message(s.into())
    }
}

/// One user-allocated numeric id range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdRange {
    pub name: String,
    pub allocated_to: String,
    pub min: i64,
    pub max: i64,
}

/// Parsed GO / OBO idranges policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdPolicy {
    pub ids_for: String,
    pub id_prefix: String,
    pub id_digits: u32,
    pub ranges: Vec<IdRange>,
}

/// Parse id policy Manchester OWL (or Turtle with expanded IAO IRIs) from text.
pub fn parse_id_policy(text: &str) -> Result<IdPolicy, IdPolicyError> {
    let prefixes = collect_manchester_prefixes(text);
    let expand = |local: &str| -> String {
        if let Some((pfx, rest)) = local.split_once(':') {
            if let Some(base) = prefixes.get(pfx) {
                return format!("{base}{rest}");
            }
        }
        local.to_string()
    };

    let ids_for =
        ontology_annotation_lexical(text, &prefixes, "idsfor", IRI_IDS_FOR).ok_or_else(|| {
            IdPolicyError::msg(format!(
                "'Id policy for' ({IRI_IDS_FOR}) ontology annotation not found"
            ))
        })?;
    let id_prefix = ontology_annotation_lexical(text, &prefixes, "idprefix", IRI_ID_PREFIX)
        .ok_or_else(|| {
            IdPolicyError::msg(format!(
                "'Id prefix' ({IRI_ID_PREFIX}) ontology annotation not found"
            ))
        })?;
    let id_digits_lex = ontology_annotation_lexical(text, &prefixes, "iddigits", IRI_ID_DIGITS)
        .ok_or_else(|| {
            IdPolicyError::msg(format!(
                "'Id digit count' ({IRI_ID_DIGITS}) ontology annotation not found"
            ))
        })?;
    let id_digits: u32 = id_digits_lex.parse().map_err(|_| {
        IdPolicyError::msg(format!(
            "Invalid value for digit count ({id_digits_lex}).  Expected integer."
        ))
    })?;

    let ranges = parse_user_ranges(text, &expand)?;
    Ok(IdPolicy { ids_for, id_prefix, id_digits, ranges })
}

/// Read a file and parse as id policy.
pub fn parse_id_policy_file(path: &Path) -> Result<IdPolicy, IdPolicyError> {
    let text = std::fs::read_to_string(path).map_err(|e| IdPolicyError::Io(e.to_string()))?;
    parse_id_policy(&text)
}

/// Look up `doc_id` (exact id, path suffix, or ontology IRI) in documents and parse its file.
pub fn parse_id_policy_from_catalog(
    documents: &[OntologyDocument],
    doc_id: &str,
) -> Result<IdPolicy, IdPolicyError> {
    let doc = documents
        .iter()
        .find(|d| {
            d.id == doc_id
                || d.path.to_string_lossy() == doc_id
                || d.path.ends_with(doc_id)
                || d.base_iri.as_deref() == Some(doc_id)
        })
        .ok_or_else(|| IdPolicyError::msg(format!("document not found: {doc_id}")))?;
    parse_id_policy_file(&doc.path)
}

fn collect_manchester_prefixes(text: &str) -> std::collections::BTreeMap<String, String> {
    let mut map = std::collections::BTreeMap::new();
    // Prefix: idsfor: <http://...>
    // @prefix idsfor: <http://...> .
    for line in text.lines() {
        let t = line.trim();
        let rest = if let Some(r) = t.strip_prefix("Prefix:") {
            r.trim()
        } else if let Some(r) = t.strip_prefix("@prefix").or_else(|| t.strip_prefix("@PREFIX")) {
            r.trim()
        } else {
            continue;
        };
        let Some((name_part, iri_part)) = rest.split_once('<') else {
            continue;
        };
        let name = name_part.trim().trim_end_matches(':').trim();
        let Some(iri) = iri_part.split('>').next() else {
            continue;
        };
        if !name.is_empty() {
            map.insert(name.to_string(), iri.to_string());
        }
    }
    map
}

fn ontology_annotation_lexical(
    text: &str,
    prefixes: &std::collections::BTreeMap<String, String>,
    short_name: &str,
    full_iri: &str,
) -> Option<String> {
    // Manchester Ontology Annotations: block — authoritative when present
    if let Some(block) = manchester_annotations_block(text) {
        if let Some(v) = annotation_value_in_block(&block, short_name) {
            return Some(v);
        }
        // Do not fall through to whole-file Turtle scan: Prefix: lines embed the
        // same IAO IRIs and would pick up the next quoted literal by accident.
        let _ = (prefixes, full_iri);
        return None;
    }
    turtle_ontology_annotation(text, full_iri)
}

fn manchester_annotations_block(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    let idx = lower.find("annotations:")?;
    let after = &text[idx + "Annotations:".len()..];
    // End at next top-level declaration
    let end_markers = [
        "\nAnnotationProperty:",
        "\nDatatype:",
        "\nClass:",
        "\nObjectProperty:",
        "\nDataProperty:",
        "\nIndividual:",
    ];
    let mut end = after.len();
    for m in end_markers {
        if let Some(p) = after.find(m) {
            end = end.min(p);
        }
    }
    Some(after[..end].to_string())
}

fn annotation_value_in_block(block: &str, short_name: &str) -> Option<String> {
    // idsfor: "GO"  or iddigits: 7  or idprefix: "http://..."
    for line in block.lines() {
        let t = line.trim().trim_end_matches(',');
        let Some(idx) = t.find(':') else {
            continue;
        };
        let name = t[..idx].trim();
        if !name.eq_ignore_ascii_case(short_name) {
            continue;
        }
        let rest = t[idx + 1..].trim().trim_end_matches(',');
        if let Some(s) = strip_quoted(rest) {
            return Some(s);
        }
        if !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()) {
            return Some(rest.to_string());
        }
    }
    None
}

fn strip_quoted(s: &str) -> Option<String> {
    let s = s.trim();
    if let Some(inner) = s.strip_prefix('"').and_then(|r| r.strip_suffix('"')) {
        return Some(inner.to_string());
    }
    if let Some(inner) = s.strip_prefix('\'').and_then(|r| r.strip_suffix('\'')) {
        return Some(inner.to_string());
    }
    None
}

fn turtle_ontology_annotation(text: &str, property_iri: &str) -> Option<String> {
    // Match <…IAO_…> "literal" or property compact form already expanded in text
    let needle = property_iri;
    let mut search = text;
    while let Some(pos) = search.find(needle) {
        let after = &search[pos + needle.len()..];
        let after = after.trim_start();
        if let Some(s) = strip_quoted(after.split_whitespace().next().unwrap_or("")) {
            return Some(s);
        }
        // "7"^^xsd:integer
        if let Some(q) = after.find('"') {
            let rest = &after[q..];
            if let Some(end) = rest[1..].find('"') {
                return Some(rest[1..1 + end].to_string());
            }
        }
        search = &search[pos + 1..];
    }
    None
}

fn parse_user_ranges(
    text: &str,
    expand: &dyn Fn(&str) -> String,
) -> Result<Vec<IdRange>, IdPolicyError> {
    let mut ranges = Vec::new();
    let mut rest = text;
    while let Some(idx) = find_ci(rest, "Datatype:") {
        let after = &rest[idx + "Datatype:".len()..];
        let next = find_ci(after, "Datatype:").unwrap_or(after.len());
        let block = &after[..next];
        rest = &after[next..];

        let name_line = block.lines().next().unwrap_or("").trim();
        let name_token = name_line.split_whitespace().next().unwrap_or("").trim_end_matches(':');
        if name_token.is_empty() {
            continue;
        }
        let name = expand(name_token);

        // Only count ranges with allocatedto (matches Protégé hasAllocatedToAnnotation)
        let Some(allocated) = datatype_allocated_to(block) else {
            continue;
        };
        let (min, max) = datatype_bounds(block)?;
        ranges.push(IdRange { name, allocated_to: allocated, min, max });
    }
    Ok(ranges)
}

fn datatype_allocated_to(block: &str) -> Option<String> {
    // Annotations: allocatedto: "David Hill" (same line or following)
    for line in block.lines() {
        let lower = line.to_ascii_lowercase();
        let Some(pos) = lower.find("allocatedto:") else {
            continue;
        };
        let after = line[pos + "allocatedto:".len()..].trim().trim_end_matches(',');
        if let Some(s) = strip_quoted(after) {
            return Some(s);
        }
        if !after.is_empty() {
            return Some(after.to_string());
        }
    }
    turtle_ontology_annotation(block, IRI_ALLOCATED_TO)
}

fn datatype_bounds(block: &str) -> Result<(i64, i64), IdPolicyError> {
    // EquivalentTo: xsd:integer[>= 0060001 , <= 0065000]
    let lower = block.to_ascii_lowercase();
    let Some(eq) = find_ci(block, "EquivalentTo:") else {
        return Err(IdPolicyError::msg(format!(
            "Expected datatype restriction definition, but not found ({})",
            block.lines().next().unwrap_or("")
        )));
    };
    let restriction = &block[eq..];
    let min =
        facet_bound(restriction, ">=").or_else(|| facet_bound(restriction, ">").map(|v| v + 1));
    let max =
        facet_bound(restriction, "<=").or_else(|| facet_bound(restriction, "<").map(|v| v - 1));
    let min = min.ok_or_else(|| {
        IdPolicyError::msg(format!(
            "Expected min inclusive facet to specify lower bound of data range, but not found ({restriction})"
        ))
    })?;
    let max = max.ok_or_else(|| {
        IdPolicyError::msg(format!(
            "Expected max inclusive facet to specify upper bound of data range, but not found ({restriction})"
        ))
    })?;
    let _ = lower;
    Ok((min, max))
}

fn facet_bound(text: &str, op: &str) -> Option<i64> {
    let mut search = text;
    while let Some(pos) = search.find(op) {
        let after = search[pos + op.len()..].trim_start();
        let num: String = after.chars().take_while(|c| c.is_ascii_digit() || *c == '-').collect();
        if !num.is_empty() {
            if let Ok(v) = num.parse::<i64>() {
                return Some(v);
            }
        }
        search = &search[pos + 1..];
    }
    // Turtle: xsd:minInclusive "60001"
    let key = match op {
        ">=" => "minInclusive",
        "<=" => "maxInclusive",
        ">" => "minExclusive",
        "<" => "maxExclusive",
        _ => return None,
    };
    if let Some(pos) = find_ci(text, key) {
        let after = &text[pos + key.len()..];
        if let Some(q) = after.find('"') {
            let rest = &after[q + 1..];
            if let Some(end) = rest.find('"') {
                return rest[..end].trim().parse().ok();
            }
        }
    }
    None
}

fn find_ci(hay: &str, needle: &str) -> Option<usize> {
    hay.to_ascii_lowercase().find(&needle.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_ranges_policy() {
        let text = r#"
Prefix: idsfor: <http://purl.obolibrary.org/obo/IAO_0000598>
Prefix: idprefix: <http://purl.obolibrary.org/obo/IAO_0000599>
Prefix: iddigits: <http://purl.obolibrary.org/obo/IAO_0000596>
Prefix: allocatedto: <http://purl.obolibrary.org/obo/IAO_0000597>
Prefix: owl: <http://www.w3.org/2002/07/owl#>

Ontology: <http://purl.obolibrary.org/obo/go/go-idranges.owl>

Annotations:
    idsfor: "GO",
    idprefix: "http://purl.obolibrary.org/obo/GO_",
    iddigits: 7

AnnotationProperty: idprefix:
"#;
        let p = parse_id_policy(text).expect("parse");
        assert_eq!(p.ids_for, "GO");
        assert_eq!(p.id_digits, 7);
        assert!(p.ranges.is_empty());
    }

    #[test]
    fn missing_prefix_fails() {
        let text = r#"
Prefix: idsfor: <http://purl.obolibrary.org/obo/IAO_0000598>
Prefix: iddigits: <http://purl.obolibrary.org/obo/IAO_0000596>
Ontology: <http://example.org/x>
Annotations:
    idsfor: "GO",
    iddigits: 7
"#;
        let err = parse_id_policy(text).unwrap_err();
        assert!(err.to_string().contains("Id prefix"), "{err}");
    }

    #[test]
    fn one_range() {
        let text = r#"
Prefix: idsfor: <http://purl.obolibrary.org/obo/IAO_0000598>
Prefix: idprefix: <http://purl.obolibrary.org/obo/IAO_0000599>
Prefix: iddigits: <http://purl.obolibrary.org/obo/IAO_0000596>
Prefix: allocatedto: <http://purl.obolibrary.org/obo/IAO_0000597>
Prefix: idrange: <http://purl.obolibrary.org/obo/ro/idrange/>
Prefix: xsd: <http://www.w3.org/2001/XMLSchema#>

Ontology: <http://example.org/x>
Annotations:
    idsfor: "GO",
    idprefix: "http://purl.obolibrary.org/obo/GO_",
    iddigits: 7

Datatype: idrange:2
Annotations: allocatedto: "David Hill"
EquivalentTo: xsd:integer[>= 0060001 , <= 0065000]
"#;
        let p = parse_id_policy(text).unwrap();
        assert_eq!(p.ranges.len(), 1);
        assert_eq!(p.ranges[0].allocated_to, "David Hill");
        assert_eq!(p.ranges[0].min, 60001);
        assert_eq!(p.ranges[0].max, 65000);
    }
}
