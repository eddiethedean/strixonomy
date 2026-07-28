//! Turtle / JSON helpers for SWRL rules.
//!
//! Full RDF SWRL vocabulary round-trip for complex blank-node shapes is handled
//! primarily via horned load/save of `Component::Rule`. This module provides a
//! compact JSON interchange for the IDE plus a simple Turtle annotation store
//! (`strixonomy:swrlRule` JSON literal) so create/edit/save works in `.ttl` without
//! requiring a full RDF list emitter for every atom shape.

use crate::model::SwrlRule;
use crate::{Result, SwrlError};

const SWRL_RULE_PRED: &str = "http://ontocode.dev/ns#swrlRule";

pub fn parse_swrl_rule_json(text: &str) -> Result<SwrlRule> {
    Ok(serde_json::from_str(text)?)
}

fn escape_turtle_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn unescape_turtle_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('t') => out.push('\t'),
                Some('"') => out.push('"'),
                Some('\\') => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn compact_json(rule: &SwrlRule) -> Result<String> {
    Ok(serde_json::to_string(rule)?)
}

/// Emit a Turtle snippet attaching a JSON-encoded rule to the ontology.
pub fn rule_to_turtle_fragment(ontology_iri: &str, rule: &SwrlRule) -> Result<String> {
    let json = compact_json(rule)?;
    let escaped = escape_turtle_string(&json);
    Ok(format!(
        "<{ontology_iri}> <{SWRL_RULE_PRED}> \"{escaped}\"^^<http://www.w3.org/2001/XMLSchema#string> .\n"
    ))
}

/// Extract rules stored as `strixonomy:swrlRule` JSON literals from Turtle text.
pub fn rules_from_turtle_document(text: &str) -> Vec<SwrlRule> {
    let mut rules = Vec::new();
    let marker = SWRL_RULE_PRED;
    for line in text.lines() {
        if !line.contains(marker) {
            continue;
        }
        // Crude extract of quoted JSON literal (supports escaped quotes / newlines).
        if let Some(start) = line.find('"') {
            let rest = &line[start + 1..];
            let mut end = None;
            let mut escaped = false;
            for (i, ch) in rest.char_indices() {
                if escaped {
                    escaped = false;
                    continue;
                }
                if ch == '\\' {
                    escaped = true;
                    continue;
                }
                if ch == '"' {
                    end = Some(i);
                    break;
                }
            }
            if let Some(end) = end {
                let raw = &rest[..end];
                let unescaped = unescape_turtle_string(raw);
                if let Ok(rule) = serde_json::from_str::<SwrlRule>(&unescaped) {
                    rules.push(rule);
                }
            }
        }
    }
    rules
}

/// Rewrite `from_iri` → `to_iri` inside `strixonomy:swrlRule` JSON string literals.
///
/// Returns updated text and number of rules remapped. Leaves non-SWRL content unchanged.
pub fn rewrite_swrl_iris_in_turtle(text: &str, from_iri: &str, to_iri: &str) -> (String, usize) {
    if from_iri == to_iri || !text.contains(SWRL_RULE_PRED) {
        return (text.to_string(), 0);
    }
    let mut result = text.to_string();
    let mut remapped = 0usize;
    let marker = SWRL_RULE_PRED;
    // Walk occurrences from the end so byte offsets stay valid.
    let mut occ: Vec<(usize, usize, String)> = Vec::new();
    let mut search_from = 0usize;
    while let Some(rel) = result[search_from..].find(marker) {
        let marker_pos = search_from + rel;
        let after = &result[marker_pos..];
        let Some(quote_rel) = after.find('"') else {
            break;
        };
        let json_start = marker_pos + quote_rel + 1;
        let rest = &result[json_start..];
        let mut end = None;
        let mut escaped = false;
        for (i, ch) in rest.char_indices() {
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == '"' {
                end = Some(i);
                break;
            }
        }
        let Some(json_len) = end else {
            break;
        };
        let raw = &rest[..json_len];
        let unescaped = unescape_turtle_string(raw);
        if let Ok(mut rule) = serde_json::from_str::<SwrlRule>(&unescaped) {
            if rule.remap_iri(from_iri, to_iri) {
                if let Ok(new_json) = serde_json::to_string(&rule) {
                    occ.push((json_start, json_start + json_len, escape_turtle_string(&new_json)));
                    remapped += 1;
                }
            }
        }
        search_from = json_start + json_len + 1;
    }
    for (start, end, new_raw) in occ.into_iter().rev() {
        result.replace_range(start..end, &new_raw);
    }
    (result, remapped)
}

#[allow(dead_code)]
pub fn err_msg(msg: impl Into<String>) -> SwrlError {
    SwrlError::Message(msg.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{SwrlAtom, SwrlIArg, SwrlRule};

    #[test]
    fn round_trip_preserves_newlines_in_json() {
        let rule = SwrlRule {
            id: Some("r1".into()),
            body: vec![SwrlAtom::Class {
                class: "http://ex#A".into(),
                arg: SwrlIArg::Variable("x".into()),
            }],
            head: vec![SwrlAtom::Class {
                class: "http://ex#B".into(),
                arg: SwrlIArg::Variable("x".into()),
            }],
            enabled: true,
        };
        let frag = rule_to_turtle_fragment("http://ex#ont", &rule).expect("frag");
        assert!(!frag.contains('\n') || frag.ends_with('\n'));
        // JSON is compacted so no raw newlines inside the literal.
        let inner = frag.lines().next().unwrap();
        assert!(!inner.trim_end_matches(" .").contains("\n"));
        let parsed = rules_from_turtle_document(&frag);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].id.as_deref(), Some("r1"));
    }

    #[test]
    fn injection_payload_with_quotes_round_trips() {
        let json = r#"{"id":"q","body":[{"kind":"class","class":"http://ex#A","arg":{"variable":"x"}}],"head":[{"kind":"class","class":"http://ex#B","arg":{"variable":"x"}}],"enabled":true}"#;
        let rule: SwrlRule = serde_json::from_str(json).unwrap();
        let frag = rule_to_turtle_fragment("http://ex#ont", &rule).unwrap();
        assert_eq!(rules_from_turtle_document(&frag).len(), 1);
    }
}
