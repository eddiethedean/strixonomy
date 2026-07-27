//! `textDocument/completion` for Turtle ontology files.

use crate::state::ServerState;
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse, InsertTextFormat,
};
use std::collections::BTreeSet;
use strixonomy_core::EntityKind;

const MAX_COMPLETION_ITEMS: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompletionContext {
    PrefixDeclaration,
    QNamePrefix,
    QNameLocal,
    IriBracket,
    None,
}

pub fn handle_completion(
    state: &ServerState,
    params: CompletionParams,
) -> Option<CompletionResponse> {
    let path = state
        .resolve_lsp_document_uri(params.text_document_position.text_document.uri.as_str())
        .ok()?;
    let content = state.document_text(&path)?;
    let position = params.text_document_position.position;
    let line_idx = position.line as usize;
    let line = content.lines().nth(line_idx)?;
    let byte_col = utf16_col_to_byte(line, position.character);
    let prefix = line.get(..byte_col).unwrap_or(line);
    let context = detect_context(line, byte_col, prefix);

    let mut items = match context {
        CompletionContext::PrefixDeclaration => prefix_declaration_items(prefix, state),
        CompletionContext::QNamePrefix => qname_prefix_items(state),
        CompletionContext::QNameLocal => {
            qname_local_items(prefix, state, &content, line_idx, byte_col)
        }
        CompletionContext::IriBracket => iri_bracket_items(prefix, state),
        CompletionContext::None => Vec::new(),
    };

    if items.len() > MAX_COMPLETION_ITEMS {
        items.truncate(MAX_COMPLETION_ITEMS);
    }

    if items.is_empty() {
        return None;
    }

    Some(CompletionResponse::Array(items))
}

fn utf16_col_to_byte(line: &str, utf16_col: u32) -> usize {
    let mut utf16 = 0u32;
    let mut byte = 0usize;
    for ch in line.chars() {
        if utf16 >= utf16_col {
            break;
        }
        utf16 += ch.len_utf16() as u32;
        byte += ch.len_utf8();
    }
    byte
}

fn detect_context(line: &str, byte_col: usize, prefix: &str) -> CompletionContext {
    let trimmed = line.trim_start();
    if trimmed.starts_with("@prefix") || trimmed.starts_with("@PREFIX") {
        return CompletionContext::PrefixDeclaration;
    }
    if prefix.contains('<') && !prefix.rsplit('<').next().is_some_and(|s| s.contains('>')) {
        return CompletionContext::IriBracket;
    }
    if let Some(colon) = prefix.rfind(':') {
        let before = &prefix[..colon];
        if !before.contains(|c: char| c.is_whitespace() || c == ';' || c == ',') {
            let after_colon = &prefix[colon + 1..];
            if !after_colon
                .contains(|c: char| c.is_whitespace() || c == ';' || c == '.' || c == ',')
            {
                return CompletionContext::QNameLocal;
            }
            return CompletionContext::QNamePrefix;
        }
    }
    let _ = byte_col;
    CompletionContext::None
}

fn prefix_declaration_items(prefix: &str, state: &ServerState) -> Vec<CompletionItem> {
    let mut seen = BTreeSet::new();
    let mut items = Vec::new();

    if prefix.trim_end().ends_with('@') || prefix.contains("@prefix") || prefix.contains("@PREFIX")
    {
        for ns in catalog_namespace_iris(state) {
            if seen.insert(ns.clone()) {
                items.push(completion_text(&ns, &format!("<{ns}> ."), CompletionItemKind::VALUE));
            }
        }
    }
    items
}

fn qname_prefix_items(state: &ServerState) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    let mut seen = BTreeSet::new();
    for prefix in catalog_prefixes(state) {
        if seen.insert(prefix.clone()) {
            items.push(completion_text(
                &format!("{prefix}:"),
                &format!("{prefix}:"),
                CompletionItemKind::VARIABLE,
            ));
        }
    }
    items
}

fn qname_local_items(
    prefix_line: &str,
    state: &ServerState,
    _content: &str,
    _line_idx: usize,
    byte_col: usize,
) -> Vec<CompletionItem> {
    let Some(colon) = prefix_line[..byte_col].rfind(':') else {
        return Vec::new();
    };
    let qprefix = &prefix_line[..colon];
    let partial = prefix_line[colon + 1..byte_col].trim_start();

    state
        .with_catalog(|catalog| {
            // Resolve namespace IRIs bound to this prefix across the catalog (#7).
            let mut ns_iris = BTreeSet::new();
            for doc in &catalog.data().documents {
                if let Some(ns) = doc.namespaces.get(qprefix) {
                    ns_iris.insert(ns.clone());
                }
            }
            if ns_iris.is_empty() {
                return Vec::new();
            }

            let mut items = Vec::new();
            let mut seen_locals = BTreeSet::new();
            for entity in &catalog.data().entities {
                let Some(local) = local_name_for_prefix_namespaces(&entity.iri, &ns_iris) else {
                    continue;
                };
                if !partial.is_empty()
                    && !local.to_ascii_lowercase().starts_with(&partial.to_ascii_lowercase())
                {
                    continue;
                }
                if !seen_locals.insert(local.clone()) {
                    continue;
                }
                let insert = format!("{qprefix}:{local}");
                let kind = match entity.kind {
                    EntityKind::Class => CompletionItemKind::CLASS,
                    EntityKind::ObjectProperty
                    | EntityKind::DataProperty
                    | EntityKind::AnnotationProperty => CompletionItemKind::PROPERTY,
                    EntityKind::Individual => CompletionItemKind::VALUE,
                    _ => CompletionItemKind::TEXT,
                };
                items.push(completion_text(
                    &format!("{} ({})", local, entity.kind.as_str()),
                    &insert,
                    kind,
                ));
                if items.len() >= MAX_COMPLETION_ITEMS {
                    break;
                }
            }
            items
        })
        .unwrap_or_default()
}

/// Local name under a typed prefix when the entity IRI expands with one of the prefix namespaces.
fn local_name_for_prefix_namespaces(iri: &str, ns_iris: &BTreeSet<String>) -> Option<String> {
    for ns in ns_iris {
        if let Some(rest) = iri.strip_prefix(ns.as_str()) {
            if rest.is_empty() {
                continue;
            }
            // Reject if the remainder still looks like a path segment beyond PN_LOCAL.
            if rest.contains('/') || rest.contains('#') {
                continue;
            }
            return Some(rest.to_string());
        }
    }
    None
}

fn iri_bracket_items(prefix: &str, state: &ServerState) -> Vec<CompletionItem> {
    let partial = prefix.rsplit('<').next().unwrap_or("").trim_start_matches('<');
    let mut items = Vec::new();
    state.with_catalog(|catalog| {
        for entity in &catalog.data().entities {
            if !partial.is_empty()
                && !entity.iri.to_ascii_lowercase().contains(&partial.to_ascii_lowercase())
            {
                continue;
            }
            items.push(completion_text(
                &entity.iri,
                &format!("{}>", entity.iri),
                CompletionItemKind::TEXT,
            ));
            if items.len() >= MAX_COMPLETION_ITEMS {
                break;
            }
        }
    });
    items
}

fn catalog_prefixes(state: &ServerState) -> Vec<String> {
    state
        .with_catalog(|catalog| {
            let mut prefixes = BTreeSet::new();
            for doc in &catalog.data().documents {
                for prefix in doc.namespaces.keys() {
                    prefixes.insert(prefix.clone());
                }
            }
            prefixes.into_iter().collect()
        })
        .unwrap_or_default()
}

fn catalog_namespace_iris(state: &ServerState) -> Vec<String> {
    state
        .with_catalog(|catalog| {
            let mut iris = BTreeSet::new();
            for doc in &catalog.data().documents {
                for iri in doc.namespaces.values() {
                    iris.insert(iri.clone());
                }
                if let Some(base) = &doc.base_iri {
                    iris.insert(base.clone());
                }
            }
            for entity in &catalog.data().entities {
                iris.insert(entity.iri.clone());
            }
            iris.into_iter().collect()
        })
        .unwrap_or_default()
}

fn completion_text(label: &str, insert: &str, kind: CompletionItemKind) -> CompletionItem {
    CompletionItem {
        label: label.to_string(),
        kind: Some(kind),
        insert_text: Some(insert.to_string()),
        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_prefix_declaration_context() {
        let line = "@prefix ex: ";
        assert_eq!(detect_context(line, line.len(), line), CompletionContext::PrefixDeclaration);
    }

    #[test]
    fn detects_qname_local_context() {
        let line = "ex:Per";
        let prefix = "ex:Per";
        assert_eq!(detect_context(line, prefix.len(), prefix), CompletionContext::QNameLocal);
    }

    #[test]
    fn local_name_requires_prefix_namespace_match() {
        let mut ns = BTreeSet::new();
        ns.insert("http://example.org/people#".to_string());
        assert_eq!(
            local_name_for_prefix_namespaces("http://example.org/people#Person", &ns).as_deref(),
            Some("Person")
        );
        assert_eq!(local_name_for_prefix_namespaces("http://other.org/clinic#Person", &ns), None);
    }
}
