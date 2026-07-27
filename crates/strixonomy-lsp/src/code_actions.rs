//! `textDocument/codeAction` from diagnostic quick fixes.

use crate::state::{path_to_uri, ServerState};
use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams, Range, TextEdit, Uri,
    WorkspaceEdit,
};
use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;
use strixonomy_core::QuickFix;
use strixonomy_owl::patch::{apply_patches_to_text, PatchOp};

pub fn handle_code_action(
    state: &ServerState,
    params: CodeActionParams,
) -> Option<Vec<CodeActionOrCommand>> {
    let path = state.resolve_lsp_document_uri(params.text_document.uri.as_str()).ok()?;
    let content = state.document_text(&path)?;
    let namespaces = namespaces_for_path(state, &path);
    let mut actions = Vec::new();

    for diag in params.context.diagnostics {
        let fix = diag.data.as_ref().and_then(|v| {
            if let Some(s) = v.as_str() {
                QuickFix::decode(s)
            } else {
                serde_json::from_value::<String>(v.clone()).ok().and_then(|s| QuickFix::decode(&s))
            }
        });

        let Some(fix) = fix else { continue };

        if let Some(action) = quick_fix_to_action(&fix, &path, &content, &namespaces) {
            actions.push(CodeActionOrCommand::CodeAction(action));
        }
    }

    if actions.is_empty() {
        None
    } else {
        Some(actions)
    }
}

fn namespaces_for_path(state: &ServerState, path: &Path) -> BTreeMap<String, String> {
    state
        .with_catalog(|catalog| {
            catalog
                .data()
                .documents
                .iter()
                .find(|d| paths_equal(&d.path, path))
                .map(|d| d.namespaces.clone())
                .unwrap_or_default()
        })
        .unwrap_or_default()
}

fn paths_equal(a: &Path, b: &Path) -> bool {
    a == b || a.canonicalize().ok().zip(b.canonicalize().ok()).is_some_and(|(x, y)| x == y)
}

fn quick_fix_to_action(
    fix: &QuickFix,
    path: &Path,
    content: &str,
    namespaces: &BTreeMap<String, String>,
) -> Option<CodeAction> {
    match fix {
        QuickFix::InsertText { label, line, column, text } => {
            let line_idx = line.saturating_sub(1) as u32;
            let line_str = content.lines().nth(line_idx as usize)?;
            let character = crate::positions::byte_col_to_utf16(line_str, column.saturating_sub(1));
            let edit = TextEdit {
                range: Range {
                    start: lsp_types::Position { line: line_idx, character },
                    end: lsp_types::Position { line: line_idx, character },
                },
                new_text: text.clone(),
            };
            Some(code_action_with_edit(label, path, vec![edit]))
        }
        QuickFix::RemoveLine { label, line } => {
            let new_text = remove_line_preserving_endings(content, *line)?;
            Some(code_action_with_edit(label, path, vec![full_document_edit(content, &new_text)]))
        }
        QuickFix::ApplyPatch { label, document_path: _, patches } => {
            let patch_ops: Vec<PatchOp> = patches
                .iter()
                .map(|v| serde_json::from_value(v.clone()))
                .collect::<Result<Vec<_>, _>>()
                .ok()?;
            if patch_ops.is_empty() {
                return None;
            }
            // Always target the open/jailed document used for patch computation — never trust
            // diagnostic `document_path`, which can redirect the WorkspaceEdit to another file.
            let result = apply_patches_to_text(content, &patch_ops, true, namespaces).ok()?;
            if !result.diagnostics.is_empty() {
                return None;
            }
            let new_text = result.preview_text?;
            Some(code_action_with_edit(label, path, vec![full_document_edit(content, &new_text)]))
        }
    }
}

/// Remove a 1-based line while preserving CRLF vs LF endings and trailing newline shape.
fn remove_line_preserving_endings(content: &str, line_1based: usize) -> Option<String> {
    let target = line_1based.saturating_sub(1);
    let bytes = content.as_bytes();
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'\r' && bytes.get(i + 1) == Some(&b'\n') {
            ranges.push((start, i + 2));
            i += 2;
            start = i;
        } else if bytes[i] == b'\n' {
            ranges.push((start, i + 1));
            i += 1;
            start = i;
        } else {
            i += 1;
        }
    }
    if start < bytes.len() {
        ranges.push((start, bytes.len()));
    } else if content.is_empty() {
        ranges.push((0, 0));
    }
    if target >= ranges.len() {
        return None;
    }
    let mut out = String::with_capacity(content.len());
    for (idx, (s, e)) in ranges.iter().enumerate() {
        if idx != target {
            out.push_str(&content[*s..*e]);
        }
    }
    Some(out)
}

fn full_document_edit(old: &str, new_text: &str) -> TextEdit {
    // Prefer u32::MAX end so trailing newlines are included (str::lines() drops them).
    let end = if old.is_empty() {
        lsp_types::Position { line: 0, character: 0 }
    } else {
        lsp_types::Position { line: u32::MAX, character: 0 }
    };
    TextEdit {
        range: Range { start: lsp_types::Position { line: 0, character: 0 }, end },
        new_text: new_text.to_string(),
    }
}

fn code_action_with_edit(label: &str, path: &Path, edits: Vec<TextEdit>) -> CodeAction {
    let uri = Uri::from_str(&path_to_uri(path))
        .unwrap_or_else(|_| Uri::from_str("file:///").expect("file uri"));
    CodeAction {
        title: label.to_string(),
        kind: Some(CodeActionKind::QUICKFIX),
        edit: Some(WorkspaceEdit {
            changes: Some(std::collections::HashMap::from([(uri, edits)])),
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::path_to_uri;
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use strixonomy_core::QuickFix;

    #[test]
    fn insert_text_quick_fix_becomes_code_action() {
        let path = PathBuf::from("/tmp/live.ttl");
        let content = "ex:Bad a owl:Class .\n";
        let fix = QuickFix::InsertText {
            label: "Declare @prefix ex:".to_string(),
            line: 1,
            column: 1,
            text: "@prefix ex: <http://example.org/ex#> .\n".to_string(),
        };
        let action = quick_fix_to_action(&fix, &path, content, &BTreeMap::new()).expect("action");
        assert_eq!(action.title, "Declare @prefix ex:");
        assert!(action.edit.is_some());
    }

    #[test]
    fn code_action_uri_uses_path_to_uri() {
        let path = PathBuf::from("/tmp/my ontologies/live.ttl");
        let content = "ex:Bad a owl:Class .\n";
        let fix = QuickFix::InsertText {
            label: "fix".to_string(),
            line: 1,
            column: 1,
            text: "@prefix ex: <http://example.org/ex#> .\n".to_string(),
        };
        let action = quick_fix_to_action(&fix, &path, content, &BTreeMap::new()).expect("action");
        let uri = action
            .edit
            .expect("edit")
            .changes
            .expect("changes")
            .keys()
            .next()
            .expect("uri")
            .to_string();
        assert_eq!(uri, path_to_uri(&path));
    }

    #[test]
    fn apply_patch_quick_fix_skipped_when_patch_has_diagnostics() {
        let path = PathBuf::from("/tmp/live.ttl");
        let content = "@prefix ex: <http://example.org/ex#> .\n\nex:Bad a owl:Class .\n";
        let fix = QuickFix::ApplyPatch {
            label: "Add rdfs:label \"Ghost\"".to_string(),
            document_path: path.display().to_string(),
            patches: vec![serde_json::json!({
                "op": "add_label",
                "entity_iri": "http://example.org/ex#Ghost",
                "value": "Ghost",
            })],
        };
        let mut namespaces = BTreeMap::new();
        namespaces.insert("ex".to_string(), "http://example.org/ex#".to_string());
        assert!(
            quick_fix_to_action(&fix, &path, content, &namespaces).is_none(),
            "missing entity patch must not produce a no-op code action"
        );
    }

    #[test]
    fn apply_patch_workspace_edit_targets_open_path_not_diagnostic_document_path() {
        let open_path = PathBuf::from("/tmp/workspace/a.ttl");
        let forged_path = PathBuf::from("/tmp/workspace/b.ttl");
        let content = "@prefix ex: <http://example.org/ex#> .\n@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\nex:Bad a owl:Class .\n";
        let fix = QuickFix::ApplyPatch {
            label: "Add rdfs:label \"Bad\"".to_string(),
            document_path: forged_path.display().to_string(),
            patches: vec![serde_json::json!({
                "op": "add_label",
                "entity_iri": "http://example.org/ex#Bad",
                "value": "Bad",
            })],
        };
        let mut namespaces = BTreeMap::new();
        namespaces.insert("ex".to_string(), "http://example.org/ex#".to_string());
        namespaces.insert("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string());

        let action =
            quick_fix_to_action(&fix, &open_path, content, &namespaces).expect("code action");
        let uri = action
            .edit
            .expect("edit")
            .changes
            .expect("changes")
            .keys()
            .next()
            .expect("uri")
            .to_string();
        assert_eq!(uri, path_to_uri(&open_path));
        assert_ne!(uri, path_to_uri(&forged_path));
    }

    #[test]
    fn apply_patch_quick_fix_skipped_when_any_op_is_malformed() {
        let path = PathBuf::from("/tmp/live.ttl");
        let content = "@prefix ex: <http://example.org/ex#> .\n@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\nex:Bad a owl:Class .\n";
        let fix = QuickFix::ApplyPatch {
            label: "Partial patch".to_string(),
            document_path: path.display().to_string(),
            patches: vec![
                serde_json::json!({
                    "op": "add_label",
                    "entity_iri": "http://example.org/ex#Bad",
                    "value": "Bad",
                }),
                serde_json::json!({
                    "op": "add_label",
                    "entity_iri": "http://example.org/ex#Bad"
                    // missing required "value"
                }),
            ],
        };
        let mut namespaces = BTreeMap::new();
        namespaces.insert("ex".to_string(), "http://example.org/ex#".to_string());
        namespaces.insert("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string());
        assert!(
            quick_fix_to_action(&fix, &path, content, &namespaces).is_none(),
            "malformed ops must not yield a partial ApplyPatch code action"
        );
    }

    #[test]
    fn remove_line_preserves_crlf_endings() {
        let path = PathBuf::from("/tmp/crlf.ttl");
        let content = "line1\r\nline2\r\nline3\r\n";
        let fix = QuickFix::RemoveLine { label: "Remove line2".to_string(), line: 2 };
        let action = quick_fix_to_action(&fix, &path, content, &BTreeMap::new()).expect("action");
        let edit = action
            .edit
            .expect("edit")
            .changes
            .expect("changes")
            .into_values()
            .next()
            .expect("edits")
            .into_iter()
            .next()
            .expect("text edit");
        assert_eq!(edit.new_text, "line1\r\nline3\r\n");
        assert!(!edit.new_text.contains('\n') || edit.new_text.contains("\r\n"));
        assert!(!edit.new_text.replace("\r\n", "").contains('\n'));
        assert_eq!(
            edit.range.end.line,
            u32::MAX,
            "full-document replace must cover trailing newlines"
        );
    }

    #[test]
    fn full_document_edit_covers_trailing_newline() {
        let edit = full_document_edit("a\nb\n", "a\n");
        assert_eq!(edit.range.end.line, u32::MAX);
        assert_eq!(edit.new_text, "a\n");
    }
}
