use crossbeam_channel::Sender;
use lsp_server::Message;
use lsp_types::notification::Notification as _;
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Uri};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::str::FromStr;

use crate::positions::byte_col_to_utf16;
use crate::state::{path_to_uri, ServerState};

/// Publish diagnostics from catalog data held in [`ServerState`].
pub fn publish_diagnostics_for_state(sender: &Sender<Message>, state: &ServerState) {
    let snapshot = match state.catalog_diagnostic_snapshot() {
        Some(s) => s,
        None => return,
    };
    let text_fn = |path: &Path| state.document_text(path);
    publish_catalog_diagnostics(
        sender,
        state,
        &snapshot.documents,
        &snapshot.diagnostics,
        &text_fn,
    );
}

pub fn publish_catalog_diagnostics(
    sender: &Sender<Message>,
    state: &ServerState,
    documents: &[strixonomy_core::OntologyDocument],
    diagnostics: &[strixonomy_core::Diagnostic],
    document_text: &dyn Fn(&Path) -> Option<String>,
) {
    let mut by_file: BTreeMap<&Path, Vec<&strixonomy_core::Diagnostic>> = BTreeMap::new();
    for diag in diagnostics {
        by_file.entry(diag.file.as_path()).or_default().push(diag);
    }

    let mut current_uris = BTreeSet::new();

    for doc in documents {
        let uri = path_to_uri(&doc.path);
        current_uris.insert(uri.clone());
        let file_diags = by_file.remove(doc.path.as_path()).unwrap_or_default();
        send_publish(sender, &uri, file_diags, document_text);
    }

    for (path, diags) in by_file {
        let uri = path_to_uri(path);
        current_uris.insert(uri.clone());
        send_publish(sender, &uri, diags, document_text);
    }

    let stale = state.replace_published_diagnostic_uris(current_uris);
    for uri in stale {
        send_empty_publish(sender, &uri);
    }
}

fn send_publish(
    sender: &Sender<Message>,
    uri: &str,
    diagnostics: Vec<&strixonomy_core::Diagnostic>,
    document_text: &dyn Fn(&Path) -> Option<String>,
) {
    let Ok(lsp_uri) = Uri::from_str(uri) else {
        eprintln!("strixonomy-lsp: skip diagnostics for invalid URI: {uri}");
        return;
    };
    let params = lsp_types::PublishDiagnosticsParams {
        uri: lsp_uri,
        diagnostics: diagnostics.into_iter().map(|d| to_lsp_diagnostic(d, document_text)).collect(),
        version: None,
    };
    let notif = lsp_server::Notification {
        method: PublishDiagnostics::METHOD.to_string(),
        params: serde_json::to_value(params).unwrap_or_default(),
    };
    if sender.send(Message::Notification(notif)).is_err() {
        eprintln!("strixonomy-lsp: failed to send publishDiagnostics");
    }
}

fn send_empty_publish(sender: &Sender<Message>, uri: &str) {
    let Ok(lsp_uri) = Uri::from_str(uri) else {
        return;
    };
    let params =
        lsp_types::PublishDiagnosticsParams { uri: lsp_uri, diagnostics: vec![], version: None };
    let notif = lsp_server::Notification {
        method: PublishDiagnostics::METHOD.to_string(),
        params: serde_json::to_value(params).unwrap_or_default(),
    };
    let _ = sender.send(Message::Notification(notif));
}

/// Clear previously published diagnostics for the given URIs (e.g. after workspace wipe).
pub fn publish_empty_diagnostics(sender: &Sender<Message>, uris: &BTreeSet<String>) {
    for uri in uris {
        send_empty_publish(sender, uri);
    }
}

fn to_lsp_diagnostic(
    diag: &strixonomy_core::Diagnostic,
    document_text: &dyn Fn(&Path) -> Option<String>,
) -> Diagnostic {
    let line_idx = diag.range.line.unwrap_or(1).saturating_sub(1) as u32;
    let byte_col = diag.range.column.unwrap_or(0) as usize;
    let line_text = document_text(&diag.file)
        .or_else(|| {
            strixonomy_core::read_to_string_capped(&diag.file, strixonomy_core::MAX_FILE_BYTES).ok()
        })
        .and_then(|text| text.lines().nth(line_idx as usize).map(|s| s.to_string()));
    let character =
        line_text.as_deref().map(|l| byte_col_to_utf16(l, byte_col)).unwrap_or(byte_col as u32);
    Diagnostic {
        range: Range {
            start: Position { line: line_idx, character },
            end: Position { line: line_idx, character: character.saturating_add(1) },
        },
        severity: Some(match diag.severity {
            strixonomy_core::DiagnosticSeverity::Error => DiagnosticSeverity::ERROR,
            strixonomy_core::DiagnosticSeverity::Warning => DiagnosticSeverity::WARNING,
            strixonomy_core::DiagnosticSeverity::Info => DiagnosticSeverity::INFORMATION,
        }),
        code: Some(NumberOrString::String(diag.display_code())),
        source: Some(if let Some(pid) = &diag.plugin_id {
            format!("strixonomy-plugin:{pid}")
        } else {
            "strixonomy".to_string()
        }),
        message: diag.message.clone(),
        data: diag.quick_fix.as_ref().map(|s| serde_json::Value::String(s.clone())),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use strixonomy_core::{DiagnosticCode, DiagnosticSeverity, SourceLocation};

    #[test]
    fn maps_diagnostic_code_to_lsp() {
        let diag = strixonomy_core::Diagnostic {
            code: DiagnosticCode::BrokenImport,
            severity: DiagnosticSeverity::Error,
            message: "test".to_string(),
            file: PathBuf::from("a.ttl"),
            range: SourceLocation::at_line_col(2, 4),
            entity_iri: None,
            quick_fix: None,
            plugin_id: None,
            plugin_code: None,
        };
        let lsp = to_lsp_diagnostic(&diag, &|_| None);
        assert_eq!(lsp.range.start.line, 1);
        assert_eq!(lsp.range.start.character, 4);
        assert_eq!(lsp.code, Some(NumberOrString::String("broken_import".to_string())));
    }

    #[test]
    fn converts_byte_col_from_disk_when_buffer_missing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("emoji.ttl");
        // Emoji is 4 UTF-8 bytes / 2 UTF-16 units. Mid-sequence byte column 5
        // must convert via the line text (→ 4), not cast the byte offset (→ 5).
        std::fs::write(&path, "# 😀\nex:A a owl:Class .\n").unwrap();
        let diag = strixonomy_core::Diagnostic {
            code: DiagnosticCode::MissingLabel,
            severity: DiagnosticSeverity::Warning,
            message: "test".to_string(),
            file: path.clone(),
            range: SourceLocation::at_line_col(1, 5),
            entity_iri: None,
            quick_fix: None,
            plugin_id: None,
            plugin_code: None,
        };
        let lsp = to_lsp_diagnostic(&diag, &|_| None);
        assert_eq!(lsp.range.start.line, 0);
        assert_eq!(lsp.range.start.character, 4);
    }

    #[test]
    fn publish_diagnostics_clears_stale_uris() {
        use crossbeam_channel::unbounded;
        use lsp_server::Message;
        use lsp_types::notification::PublishDiagnostics;
        use std::collections::BTreeMap;
        use std::time::Duration;
        use strixonomy_core::{OntologyDocument, OntologyFormat, ParseStatus, SourceLocation};

        let dir = tempfile::tempdir().unwrap();
        let path_a = dir.path().join("a.ttl");
        let path_b = dir.path().join("b.ttl");
        std::fs::write(&path_a, "@prefix ex: <http://ex/> .\n").unwrap();
        std::fs::write(&path_b, "@prefix ex: <http://ex/> .\n").unwrap();

        let doc = |path: &std::path::Path| OntologyDocument {
            id: "doc-1".to_string(),
            path: path.to_path_buf(),
            format: OntologyFormat::Turtle,
            base_iri: None,
            version_iri: None,
            imports: vec![],
            namespaces: BTreeMap::new(),
            parse_status: ParseStatus::Ok,
            content_hash: "h".to_string(),
            modified_time: 0,
            parse_message: None,
            parse_error_location: None,
        };

        let diagnostic = |path: &std::path::Path| strixonomy_core::Diagnostic {
            code: DiagnosticCode::UndefinedPrefix,
            severity: DiagnosticSeverity::Error,
            message: "undefined prefix: un:".to_string(),
            file: path.to_path_buf(),
            range: SourceLocation::at_line_col(1, 0),
            entity_iri: None,
            quick_fix: None,
            plugin_id: None,
            plugin_code: None,
        };

        let state = ServerState::new();
        let (tx, rx) = unbounded();
        publish_catalog_diagnostics(
            &tx,
            &state,
            &[doc(&path_a), doc(&path_b)],
            &[diagnostic(&path_a), diagnostic(&path_b)],
            &|_| None,
        );

        let mut publish_count = 0usize;
        while let Ok(Message::Notification(notif)) = rx.recv_timeout(Duration::from_millis(100)) {
            if notif.method == PublishDiagnostics::METHOD {
                publish_count += 1;
            }
        }
        assert_eq!(publish_count, 2);

        publish_catalog_diagnostics(&tx, &state, &[doc(&path_a)], &[diagnostic(&path_a)], &|_| {
            None
        });

        let mut final_notifications = Vec::new();
        while let Ok(Message::Notification(notif)) = rx.recv_timeout(Duration::from_millis(100)) {
            if notif.method == PublishDiagnostics::METHOD {
                final_notifications.push(notif);
            }
        }
        assert_eq!(final_notifications.len(), 2);
        let cleared = final_notifications.iter().any(|n| {
            n.params.get("diagnostics").and_then(|d| d.as_array()).is_some_and(|a| a.is_empty())
        });
        assert!(cleared, "expected empty diagnostics publish for stale URI");
    }
}
