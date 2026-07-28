//! Language server for Strixonomy (stdio). Custom methods under `strixonomy/*`.
//!
//! See [docs/lsp-api.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/lsp-api.md).
//!
//! # API stability
//!
//! The LSP wire format and custom `strixonomy/*` methods are **pre-1.0** and may change
//! between minor releases. See the repository README for semver policy.

pub(crate) mod code_actions;
pub(crate) mod completion;
pub(crate) mod diagnostics;
pub(crate) mod handlers;
pub(crate) mod index_worker;
pub(crate) mod positions;
pub mod protocol;
pub(crate) mod semantic_tokens;
pub(crate) mod state;

use handlers::build_catalog_snapshot;
use strixonomy_catalog::OntologyCatalog;

/// Serialize the LSP `strixonomy/getCatalogSnapshot` payload for a built catalog.
pub fn catalog_snapshot_json(
    catalog: &OntologyCatalog,
) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(build_catalog_snapshot(catalog, &[]))
}

#[cfg(test)]
mod handlers_test;

use crate::positions::utf16_offset_to_byte;
use crate::protocol::RunReasonerParams;
use crossbeam_channel::{Receiver, Sender};
use diagnostics::{publish_diagnostics_for_state, publish_empty_diagnostics};
use handlers::{
    handle_custom_request, handle_initialize, handle_run_reasoner_lsp, handle_standard_request,
    StandardRequestOutcome,
};
use index_worker::IndexWorker;
use lsp_server::{Connection, Message, Notification, Request, RequestId, Response, ResponseError};
use lsp_types::{
    notification::Notification as _,
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Exit, Initialized,
        ShowMessage,
    },
    InitializeParams, MessageType, ShowMessageParams,
};
use serde_json::Value;
use state::{
    canonical_roots_match, resolve_workspace_folder_add, resolve_workspace_folder_uri, ServerState,
};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct PendingReindex {
    workspace: std::path::PathBuf,
    scheduled_at: Instant,
}

#[derive(serde::Deserialize)]
struct CancelNotificationParams {
    id: RequestId,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (connection, io_threads) = Connection::stdio();
    let state = ServerState::new();
    let pending_reindex: Arc<Mutex<Option<PendingReindex>>> = Arc::new(Mutex::new(None));
    let pending_diagnostic_publish = Arc::new(AtomicBool::new(false));

    let index_worker = IndexWorker::spawn(state.clone(), connection.sender.clone());

    let timer_worker = index_worker.clone();
    let timer_pending = Arc::clone(&pending_reindex);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));
        if let Some(workspace) = take_due_reindex_workspace(&timer_pending) {
            timer_worker.enqueue(workspace);
        }
    });

    // Messages parked while `strixonomy/runReasoner` polls for cancel (see #268).
    let mut deferred: VecDeque<Message> = VecDeque::new();
    loop {
        let msg = match deferred.pop_front() {
            Some(msg) => msg,
            None => match connection.receiver.recv() {
                Ok(msg) => msg,
                Err(_) => break,
            },
        };
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    break;
                }
                if let Some(resp) = handle_lsp_request(
                    &state,
                    &index_worker,
                    &pending_diagnostic_publish,
                    &connection.receiver,
                    &mut deferred,
                    req,
                ) {
                    connection.sender.send(Message::Response(resp))?;
                }
                publish_pending_diagnostics(
                    &connection.sender,
                    &state,
                    &pending_diagnostic_publish,
                );
            }
            Message::Notification(notif) => {
                if notif.method == Exit::METHOD {
                    break;
                }
                handle_notification(&state, &pending_reindex, &connection.sender, notif);
                publish_pending_diagnostics(
                    &connection.sender,
                    &state,
                    &pending_diagnostic_publish,
                );
            }
            Message::Response(_) => {}
        }
    }

    drop(connection);
    io_threads.join()?;
    Ok(())
}

fn handle_lsp_request(
    state: &ServerState,
    index_worker: &IndexWorker,
    pending_diagnostic_publish: &Arc<AtomicBool>,
    message_rx: &Receiver<Message>,
    deferred: &mut VecDeque<Message>,
    req: Request,
) -> Option<Response> {
    let id = req.id.clone();

    if req.method == "initialize" {
        let params: InitializeParams = match parse_params(Some(req.params)) {
            Ok(p) => p,
            Err(e) => return Some(error_response(id, e)),
        };
        let result = handle_initialize(state, params);
        if state.with_catalog(|_| ()).is_some() {
            pending_diagnostic_publish.store(true, Ordering::SeqCst);
        }
        return Some(ok_response(id, result));
    }

    if req.method == "shutdown" {
        return Some(ok_response(id, Value::Null));
    }

    if req.method == "strixonomy/runReasoner" {
        let params: RunReasonerParams = match parse_params(Some(req.params)) {
            Ok(p) => p,
            Err(e) => return Some(error_response(id, e)),
        };
        let run_generation = state.begin_reasoner_run(req.id.clone());
        let result = handle_run_reasoner_lsp(state, params, message_rx, deferred, run_generation);
        state.clear_active_reasoner_request();
        return match result {
            Ok(value) => match serde_json::to_value(value) {
                Ok(result) => Some(ok_response(id, result)),
                Err(e) => Some(strixonomy_error_response(
                    id,
                    crate::protocol::LspErrorPayload::reasoner_failed(e.to_string()),
                )),
            },
            Err(err) => Some(strixonomy_error_response(id, err)),
        };
    }

    if crate::handlers::normalize_custom_method(&req.method).is_some() {
        return match handle_custom_request(state, index_worker, &req.method, Some(req.params)) {
            Ok(result) => {
                if crate::handlers::normalize_custom_method(&req.method) == Some("indexWorkspace") {
                    pending_diagnostic_publish.store(true, Ordering::SeqCst);
                }
                Some(ok_response(id, result))
            }
            Err(err) => Some(strixonomy_error_response(id, err)),
        };
    }

    match handle_standard_request(state, &req.method, Some(req.params)) {
        StandardRequestOutcome::Ok(result) => Some(ok_response(id, result)),
        StandardRequestOutcome::MethodNotFound => Some(error_response(
            id,
            ResponseError {
                code: -32601,
                message: format!("Method not found: {}", req.method),
                data: None,
            },
        )),
        StandardRequestOutcome::InvalidParams(err) => Some(error_response(id, err)),
        StandardRequestOutcome::LspError(err) => Some(strixonomy_error_response(id, err)),
    }
}

fn handle_notification(
    state: &ServerState,
    pending_reindex: &Arc<Mutex<Option<PendingReindex>>>,
    sender: &Sender<Message>,
    notif: Notification,
) {
    match notif.method.as_str() {
        "$/cancelRequest" => {
            if let Ok(params) = serde_json::from_value::<CancelNotificationParams>(notif.params) {
                state.cancel_reasoner_request(params.id);
            }
        }
        Initialized::METHOD => {
            if let Some(workspace) = state.effective_index_root() {
                schedule_reindex(pending_reindex, workspace, Duration::from_millis(500));
            }
        }
        "workspace/didChangeWatchedFiles" => {
            if let Some(workspace) = state.effective_index_root() {
                schedule_reindex(pending_reindex, workspace, Duration::from_millis(500));
            }
        }
        DidOpenTextDocument::METHOD => {
            if let Ok(params) =
                serde_json::from_value::<lsp_types::DidOpenTextDocumentParams>(notif.params)
            {
                if let Ok(path) = document_path_from_uri(state, &params.text_document.uri) {
                    if let Err(err) =
                        state.set_document_text(path.clone(), params.text_document.text)
                    {
                        eprintln!("strixonomy-lsp: rejected open document: {err}");
                    } else {
                        state.set_document_version(path, params.text_document.version);
                    }
                }
            }
            if let Some(workspace) = state.effective_index_root() {
                schedule_reindex(pending_reindex, workspace, Duration::from_millis(750));
            }
        }
        DidChangeTextDocument::METHOD => {
            if let Ok(params) =
                serde_json::from_value::<lsp_types::DidChangeTextDocumentParams>(notif.params)
            {
                if let Ok(path) = document_path_from_uri(state, &params.text_document.uri) {
                    if let Some(text) = merged_document_text(state, &path, &params) {
                        if let Err(err) = state.set_document_text(path.clone(), text) {
                            eprintln!("strixonomy-lsp: rejected document change: {err}");
                        } else {
                            state.set_document_version(path, params.text_document.version);
                            if let Some(workspace) = state.effective_index_root() {
                                schedule_reindex(
                                    pending_reindex,
                                    workspace,
                                    Duration::from_millis(750),
                                );
                            }
                        }
                    } else {
                        // Invalid incremental edit — drop the stale buffer and ask the user to
                        // reopen so client/server text cannot silently diverge (#90).
                        eprintln!(
                            "strixonomy-lsp: rejected document change: invalid edit range for {}",
                            params.text_document.uri.as_str()
                        );
                        state.remove_document(&path);
                        let params = ShowMessageParams {
                            typ: MessageType::ERROR,
                            message: format!(
                                "Strixonomy: document sync failed for {}; reopen the file to resync the language server buffer",
                                path.display()
                            ),
                        };
                        let _ = sender.send(Message::Notification(Notification {
                            method: ShowMessage::METHOD.to_string(),
                            params: serde_json::to_value(params).unwrap_or_default(),
                        }));
                    }
                }
            }
        }
        DidCloseTextDocument::METHOD => {
            if let Ok(params) =
                serde_json::from_value::<lsp_types::DidCloseTextDocumentParams>(notif.params)
            {
                if let Ok(path) = document_path_from_uri(state, &params.text_document.uri) {
                    state.remove_document(&path);
                }
            }
            if let Some(workspace) = state.effective_index_root() {
                schedule_reindex(pending_reindex, workspace, Duration::from_millis(750));
            }
        }
        "workspace/didChangeWorkspaceFolders" => {
            if let Ok(params) =
                serde_json::from_value::<lsp_types::DidChangeWorkspaceFoldersParams>(notif.params)
            {
                let mut roots = state.workspace_roots();
                for removed in params.event.removed {
                    if let Ok(path) = resolve_workspace_folder_uri(removed.uri.as_str(), &roots) {
                        roots.retain(|r| !canonical_roots_match(r, &path));
                    }
                }
                for added in params.event.added {
                    if let Ok(path) = resolve_workspace_folder_add(added.uri.as_str()) {
                        if !roots.iter().any(|r| canonical_roots_match(r, &path)) {
                            roots.push(path);
                        }
                    }
                }
                if roots.is_empty() {
                    let stale = state.clear_workspace_state();
                    publish_empty_diagnostics(sender, &stale);
                    eprintln!("strixonomy-lsp: all workspace folders removed");
                } else if let Err(err) = state.set_workspace_roots(roots) {
                    eprintln!("strixonomy-lsp: failed to update workspace roots: {err}");
                } else if let Some(workspace) = state.effective_index_root() {
                    schedule_reindex(pending_reindex, workspace, Duration::from_millis(500));
                }
            }
        }
        _ => {}
    }
}

fn publish_pending_diagnostics(
    sender: &Sender<Message>,
    state: &ServerState,
    pending: &Arc<AtomicBool>,
) {
    if !pending.swap(false, Ordering::SeqCst) {
        return;
    }
    publish_diagnostics_for_state(sender, state);
}

fn document_path_from_uri(
    state: &ServerState,
    uri: &lsp_types::Uri,
) -> Result<std::path::PathBuf, String> {
    state.resolve_lsp_document_uri(uri.as_str())
}

fn merged_document_text(
    state: &ServerState,
    path: &std::path::Path,
    params: &lsp_types::DidChangeTextDocumentParams,
) -> Option<String> {
    let mut text = state.document_text(path)?;
    for change in &params.content_changes {
        if let Some(range) = &change.range {
            text = apply_text_change(&text, range, &change.text)?;
        } else {
            text = change.text.clone();
        }
    }
    Some(text)
}

/// Map an LSP position to a byte offset, preserving `\n` / `\r\n` / `\r` line endings.
fn position_to_byte(text: &str, pos: lsp_types::Position) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut line = 0u32;
    let mut i = 0usize;
    while line < pos.line {
        if i >= bytes.len() {
            return None;
        }
        if bytes[i] == b'\n' {
            line += 1;
            i += 1;
        } else if bytes[i] == b'\r' {
            line += 1;
            i += 1;
            if i < bytes.len() && bytes[i] == b'\n' {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    let line_start = i;
    let mut line_end = i;
    while line_end < bytes.len() && bytes[line_end] != b'\n' && bytes[line_end] != b'\r' {
        line_end += 1;
    }
    let line_str = text.get(line_start..line_end)?;
    let col = utf16_offset_to_byte(line_str, pos.character).min(line_str.len());
    let offset = line_start + col;
    if !text.is_char_boundary(offset) {
        return None;
    }
    Some(offset)
}

fn apply_text_change(text: &str, range: &lsp_types::Range, new_text: &str) -> Option<String> {
    let start = position_to_byte(text, range.start)?;
    let end = position_to_byte(text, range.end)?;
    if start > end {
        eprintln!("strixonomy-lsp: text change range inverted (start={start}, end={end})");
        return None;
    }
    let mut result = String::with_capacity(text.len() - (end - start) + new_text.len());
    result.push_str(&text[..start]);
    result.push_str(new_text);
    result.push_str(&text[end..]);
    Some(result)
}

fn schedule_reindex(
    pending: &Arc<Mutex<Option<PendingReindex>>>,
    workspace: std::path::PathBuf,
    delay: Duration,
) {
    if let Ok(mut guard) = pending.lock() {
        *guard = Some(PendingReindex { workspace, scheduled_at: Instant::now() + delay });
    }
}

fn take_due_reindex_workspace(
    pending: &Arc<Mutex<Option<PendingReindex>>>,
) -> Option<std::path::PathBuf> {
    let Ok(mut guard) = pending.lock() else {
        return None;
    };
    let entry = guard.as_ref()?;
    if Instant::now() < entry.scheduled_at {
        return None;
    }
    let workspace = entry.workspace.clone();
    *guard = None;
    Some(workspace)
}

fn parse_params<T: serde::de::DeserializeOwned>(params: Option<Value>) -> Result<T, ResponseError> {
    serde_json::from_value(params.unwrap_or(Value::Null)).map_err(|e| ResponseError {
        code: -32602,
        message: format!("invalid params: {e}"),
        data: None,
    })
}

fn ok_response(id: RequestId, result: impl serde::Serialize) -> Response {
    Response { id, result: Some(serde_json::to_value(result).unwrap_or(Value::Null)), error: None }
}

fn error_response(id: RequestId, error: ResponseError) -> Response {
    Response { id, result: None, error: Some(error) }
}

fn strixonomy_error_response(id: RequestId, err: protocol::LspErrorPayload) -> Response {
    Response {
        id,
        result: None,
        error: Some(ResponseError {
            code: -32000,
            message: err.message.clone(),
            data: Some(serde_json::to_value(err).unwrap_or(Value::Null)),
        }),
    }
}

#[cfg(test)]
mod debounce_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn take_due_clears_pending_after_delay() {
        let pending: Arc<Mutex<Option<PendingReindex>>> = Arc::new(Mutex::new(None));
        let ws = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
        schedule_reindex(&pending, ws.clone(), Duration::from_millis(30));
        assert!(pending.lock().unwrap().is_some());
        thread::sleep(Duration::from_millis(60));
        let taken = take_due_reindex_workspace(&pending);
        assert_eq!(taken, Some(ws));
        assert!(pending.lock().unwrap().is_none());
    }

    #[test]
    fn apply_text_change_rejects_inverted_range() {
        let text = "ex:Person a owl:Class .\n";
        let range = lsp_types::Range {
            start: lsp_types::Position { line: 0, character: 10 },
            end: lsp_types::Position { line: 0, character: 2 },
        };
        assert!(apply_text_change(text, &range, "X").is_none());
    }
}
