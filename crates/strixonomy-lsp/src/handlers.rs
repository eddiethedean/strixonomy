use crate::index_worker::IndexWorker;
use crate::positions::{byte_col_to_utf16, utf16_offset_to_byte};
use crate::protocol::{
    ApplyAxiomPatchParams, ApplyAxiomPatchResult, ApplyRefactorParams, ApplyRefactorResult,
    CatalogSnapshot, CheckInstanceParams, CheckInstanceResult, DiagnosticSummary, DlQueryParams,
    DlQueryResult, FindUsagesParams, FindUsagesResult, GetEntityParams, GetEntityResult,
    GetExplanationParams, GetExplanationResult, GetGraphResult, IndexWorkspaceParams,
    IndexWorkspaceResult, ListPluginsResult, ListSqlSchemaResult, ListSwrlRulesResult,
    LspErrorPayload, ManchesterCompletions, ParseManchesterParams, ParseManchesterResult,
    ParseSwrlRuleParams, ParseSwrlRuleResult, PreviewRefactorParams, PreviewRefactorResult,
    QueryParams, RunPluginParams, RunPluginResult, RunReasonerParams, RunReasonerResult,
    RunRobotParams, RunRobotResult, SearchParams, SearchResult, SemanticDiffParams,
    SemanticDiffResult, SparqlParams, SwrlRuleListItem, TabularQueryResult, UsageSummary,
    ValidateSwrlRuleParams, ValidateSwrlRuleResult,
};
use crate::state::{path_to_uri, resolve_workspace_for_index, ServerState};
use crossbeam_channel::Receiver;
use lsp_server::{Message, RequestId, ResponseError};
use lsp_types::{
    CodeActionProviderCapability, CompletionOptions, DocumentChanges, DocumentSymbol,
    DocumentSymbolParams, DocumentSymbolResponse, GotoDefinitionParams, GotoDefinitionResponse,
    Hover, HoverContents, HoverParams, HoverProviderCapability, InitializeParams, InitializeResult,
    Location, MarkupContent, MarkupKind, OneOf, Position, PrepareRenameResponse, Range,
    ReferenceParams, RenameOptions, RenameParams, SemanticTokensParams,
    SemanticTokensServerCapabilities, ServerCapabilities, SymbolInformation, SymbolKind,
    TextDocumentEdit, TextDocumentPositionParams, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextEdit, Uri, WorkspaceEdit, WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
    WorkspaceSymbolParams, WorkspaceSymbolResponse,
};
use serde::Deserialize;
use serde_json::Value;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use strixonomy_catalog::{GraphBuilder, GraphRequest, IndexBuilder, OntologyCatalog};
use strixonomy_core::{validate_workspace_scope_any, EntityKind, OntologyFormat};
use strixonomy_diff::{
    apply_unsat_diff, catalog_at_git_ref, catalog_at_worktree, diff_catalogs,
    diff_git_refs_with_catalogs, discover_repo_root, format_diff_pr_summary, DiffResult,
};
use strixonomy_reasoner::{
    classify, explain_alternatives, run_dl_query, DlQueryMode, ExplanationRequest, ReasonerId,
    ReasonerSnapshot, WorkspaceInputLoader,
};
use strixonomy_refactor::{
    apply_refactor_plan_checked_with_overrides, find_usages_with_overrides, plans_equivalent,
    preview_refactor, preview_rename_iri,
};

#[allow(deprecated)]
pub fn handle_initialize(state: &ServerState, params: InitializeParams) -> InitializeResult {
    let mut roots: Vec<std::path::PathBuf> = Vec::new();
    if let Some(folders) = params.workspace_folders {
        for folder in folders {
            match resolve_workspace_for_index(folder.uri.as_str(), None) {
                Ok(path) => roots.push(path),
                Err(err) => eprintln!("strixonomy-lsp: failed to resolve workspace folder: {err}"),
            }
        }
    } else if let Some(uri) = params.root_uri {
        match resolve_workspace_for_index(uri.as_str(), None) {
            Ok(path) => roots.push(path),
            Err(err) => eprintln!("strixonomy-lsp: failed to resolve workspace root: {err}"),
        }
    }

    if !roots.is_empty() {
        if let Err(err) = state.set_workspace_roots(roots) {
            eprintln!("strixonomy-lsp: failed to set workspace roots: {err}");
        }
    }

    InitializeResult {
        capabilities: ServerCapabilities {
            definition_provider: Some(OneOf::Left(true)),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            workspace_symbol_provider: Some(OneOf::Left(true)),
            references_provider: Some(OneOf::Left(true)),
            rename_provider: Some(OneOf::Right(RenameOptions {
                prepare_provider: Some(true),
                work_done_progress_options: Default::default(),
            })),
            completion_provider: Some(CompletionOptions {
                trigger_characters: Some(vec![":".to_string(), "<".to_string(), "@".to_string()]),
                ..Default::default()
            }),
            code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            semantic_tokens_provider: Some(
                SemanticTokensServerCapabilities::SemanticTokensOptions(
                    lsp_types::SemanticTokensOptions {
                        legend: crate::semantic_tokens::legend(),
                        full: Some(lsp_types::SemanticTokensFullOptions::Bool(true)),
                        ..Default::default()
                    },
                ),
            ),
            workspace: Some(WorkspaceServerCapabilities {
                workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                    supported: Some(true),
                    change_notifications: Some(OneOf::Left(true)),
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
        server_info: Some(lsp_types::ServerInfo {
            name: "strixonomy-lsp".to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }),
    }
}

pub fn handle_index_workspace(
    state: &ServerState,
    index_worker: &IndexWorker,
    params: IndexWorkspaceParams,
) -> Result<IndexWorkspaceResult, LspErrorPayload> {
    let roots = state.workspace_roots();
    let workspace = match params.workspace_uri.as_deref() {
        Some(uri) => {
            let path = strixonomy_core::workspace_uri_to_path(uri)
                .map_err(LspErrorPayload::index_failed)?;
            if roots.is_empty() {
                return Err(LspErrorPayload::index_failed("workspace not initialized".to_string()));
            }
            validate_workspace_scope_any(&path, &roots).map_err(LspErrorPayload::index_failed)?
        }
        None => state.effective_index_root().ok_or_else(|| {
            LspErrorPayload::index_failed("no workspace URI provided".to_string())
        })?,
    };

    state.set_index_disk_cache(params.disk_cache);

    let (stats, indexed_at) =
        index_worker.enqueue_sync(workspace).map_err(LspErrorPayload::index_failed)?;

    Ok(IndexWorkspaceResult { stats, indexed_at })
}

pub fn build_catalog_snapshot(
    catalog: &strixonomy_catalog::OntologyCatalog,
    extra_diagnostics: &[strixonomy_core::Diagnostic],
) -> CatalogSnapshot {
    let mut diagnostics: Vec<DiagnosticSummary> =
        catalog.data().diagnostics.iter().map(DiagnosticSummary::from).collect();
    diagnostics.extend(extra_diagnostics.iter().map(DiagnosticSummary::from));
    CatalogSnapshot {
        documents: catalog.data().documents.clone(),
        entities: catalog.data().entities.clone(),
        hierarchy: catalog.class_hierarchy(),
        diagnostics,
        reasoner: None,
        stats: Some(catalog.data().stats()),
        active_ontology_id: None,
    }
}

pub fn build_catalog_snapshot_with_reasoner(
    catalog: &strixonomy_catalog::OntologyCatalog,
    reasoner: Option<strixonomy_reasoner::ReasonerSnapshot>,
    extra_diagnostics: &[strixonomy_core::Diagnostic],
) -> CatalogSnapshot {
    let mut snapshot = build_catalog_snapshot(catalog, extra_diagnostics);
    snapshot.reasoner = reasoner;
    snapshot
}

pub fn handle_get_catalog_snapshot(
    state: &ServerState,
) -> Result<CatalogSnapshot, LspErrorPayload> {
    let plugin_diags = state.plugin_diagnostics();
    let mut snapshot = state
        .with_catalog_and_reasoner(|catalog, reasoner| {
            build_catalog_snapshot_with_reasoner(catalog, reasoner.cloned(), &plugin_diags)
        })
        .ok_or_else(LspErrorPayload::not_indexed)?;
    snapshot.active_ontology_id = state.active_ontology_id();
    Ok(snapshot)
}

fn builtin_command_descriptors() -> Vec<crate::protocol::CommandDescriptor> {
    use crate::protocol::{CommandDescriptor, CommandEnablement};
    let e = |id: &str,
             title: &str,
             category: &str,
             enablement: Vec<CommandEnablement>,
             undo: Option<&str>,
             dialog: Option<&str>| {
        CommandDescriptor {
            id: id.to_string(),
            title: title.to_string(),
            category: category.to_string(),
            enablement,
            undo_label: undo.map(str::to_string),
            dialog_id: dialog.map(str::to_string),
        }
    };
    vec![
        e(
            "strixonomy.newOntology",
            "New Ontology",
            "File",
            vec![CommandEnablement::Always],
            None,
            Some("new_ontology"),
        ),
        e(
            "strixonomy.openOntology",
            "Open Ontology",
            "File",
            vec![CommandEnablement::Always],
            None,
            None,
        ),
        e(
            "strixonomy.saveAs",
            "Save Ontology As…",
            "File",
            vec![CommandEnablement::HasOntology],
            None,
            Some("save_as"),
        ),
        e(
            "strixonomy.exportOntology",
            "Export Ontology…",
            "File",
            vec![CommandEnablement::HasOntology],
            None,
            Some("export_ontology"),
        ),
        e(
            "strixonomy.manageImports",
            "Manage Imports",
            "File",
            vec![CommandEnablement::HasOntology],
            None,
            Some("import"),
        ),
        e(
            "strixonomy.reloadImports",
            "Reload Imports",
            "File",
            vec![CommandEnablement::HasOntology],
            None,
            None,
        ),
        e(
            "strixonomy.searchEntities",
            "Search Entities",
            "Edit",
            vec![CommandEnablement::HasOntology],
            None,
            Some("search"),
        ),
        e(
            "strixonomy.openPreferences",
            "Preferences",
            "Edit",
            vec![CommandEnablement::Always],
            None,
            Some("preferences"),
        ),
        e(
            "strixonomy.deleteEntity",
            "Delete Entity",
            "Edit",
            vec![CommandEnablement::HasSelection, CommandEnablement::CanEditSelection],
            Some("Delete entity"),
            Some("delete_entity"),
        ),
        e(
            "strixonomy.copyEntityIri",
            "Copy Entity IRI",
            "Edit",
            vec![CommandEnablement::HasSelection],
            None,
            None,
        ),
        e(
            "strixonomy.setActiveOntology",
            "Set Active Ontology",
            "Active Ontology",
            vec![CommandEnablement::HasOntology],
            None,
            None,
        ),
        e(
            "strixonomy.editOntologyMetadata",
            "Ontology Metadata",
            "Active Ontology",
            vec![CommandEnablement::HasOntology],
            Some("Edit ontology metadata"),
            Some("ontology_metadata"),
        ),
        e(
            "strixonomy.managePrefixes",
            "Manage Prefixes",
            "Active Ontology",
            vec![CommandEnablement::HasOntology],
            Some("Edit prefixes"),
            Some("prefix_manager"),
        ),
        e(
            "strixonomy.showMetrics",
            "Ontology Metrics",
            "Active Ontology",
            vec![CommandEnablement::HasOntology],
            None,
            Some("metrics"),
        ),
        e(
            "strixonomy.renameEntityIri",
            "Rename Entity IRI",
            "Refactor",
            vec![CommandEnablement::HasSelection],
            Some("Rename entity"),
            Some("rename"),
        ),
        e(
            "strixonomy.mergeEntities",
            "Merge Entities",
            "Refactor",
            vec![CommandEnablement::HasSelection],
            Some("Merge entities"),
            None,
        ),
        e(
            "strixonomy.replaceEntity",
            "Replace Entity",
            "Refactor",
            vec![CommandEnablement::HasSelection],
            Some("Replace entity"),
            None,
        ),
        e(
            "strixonomy.startReasoner",
            "Start Reasoner",
            "Reasoner",
            vec![CommandEnablement::HasOntology, CommandEnablement::ReasonerIdle],
            None,
            None,
        ),
        e(
            "strixonomy.stopReasoner",
            "Stop Reasoner",
            "Reasoner",
            vec![CommandEnablement::ReasonerRunning],
            None,
            None,
        ),
        e(
            "strixonomy.synchronizeReasoner",
            "Synchronize Reasoner",
            "Reasoner",
            vec![CommandEnablement::HasOntology],
            None,
            None,
        ),
        e(
            "strixonomy.classifyOntology",
            "Classify Ontology",
            "Reasoner",
            vec![CommandEnablement::HasOntology],
            None,
            None,
        ),
        e(
            "strixonomy.checkConsistency",
            "Check Consistency",
            "Reasoner",
            vec![CommandEnablement::HasOntology],
            None,
            None,
        ),
        e(
            "strixonomy.configureReasoner",
            "Configure Reasoner",
            "Reasoner",
            vec![CommandEnablement::Always],
            None,
            Some("reasoner_settings"),
        ),
        e(
            "strixonomy.validateWorkspace",
            "Validate Workspace",
            "Tools",
            vec![CommandEnablement::HasOntology],
            None,
            None,
        ),
        e(
            "strixonomy.runBatchTools",
            "Run Batch Tools",
            "Tools",
            vec![CommandEnablement::HasOntology],
            None,
            None,
        ),
        e(
            "strixonomy.resetLayout",
            "Reset Layout",
            "Window",
            vec![CommandEnablement::Always],
            None,
            None,
        ),
        e(
            "strixonomy.switchPerspective",
            "Switch Perspective",
            "Window",
            vec![CommandEnablement::Always],
            None,
            None,
        ),
        e(
            "strixonomy.savePerspective",
            "Save Perspective",
            "Window",
            vec![CommandEnablement::Always],
            None,
            None,
        ),
        e(
            "strixonomy.showAbout",
            "About Strixonomy",
            "Help",
            vec![CommandEnablement::Always],
            None,
            None,
        ),
        e(
            "strixonomy.showPluginInfo",
            "Plugin Information",
            "Help",
            vec![CommandEnablement::Always],
            None,
            None,
        ),
        e(
            "strixonomy.openErrorLog",
            "Error Log",
            "Help",
            vec![CommandEnablement::Always],
            None,
            None,
        ),
        e(
            "strixonomy.exportDiagnostics",
            "Export Diagnostics",
            "Help",
            vec![CommandEnablement::HasOntology],
            None,
            None,
        ),
        e(
            "strixonomy.openDocumentation",
            "Documentation",
            "Help",
            vec![CommandEnablement::Always],
            None,
            None,
        ),
    ]
}

pub fn handle_list_commands() -> crate::protocol::ListCommandsResult {
    crate::protocol::ListCommandsResult { commands: builtin_command_descriptors() }
}

pub fn handle_get_workspace_ui_state(
    state: &ServerState,
    params: crate::protocol::WorkspaceUiStateParams,
) -> Result<crate::protocol::WorkspaceUiState, LspErrorPayload> {
    let (has_ontology, ontology_count, stats, selection_editable) = state
        .with_catalog(|catalog| {
            let stats = catalog.data().stats();
            let editable = params
                .selection_iri
                .as_ref()
                .map(|iri| {
                    catalog.find_entity(iri).is_some()
                        && catalog
                            .entity_document(iri)
                            .map(|d| {
                                matches!(
                                    d.format,
                                    OntologyFormat::Turtle
                                        | OntologyFormat::Obo
                                        | OntologyFormat::Owl
                                        | OntologyFormat::RdfXml
                                        | OntologyFormat::OwlXml
                                )
                            })
                            .unwrap_or(false)
                })
                .unwrap_or(false);
            (stats.ontology_count > 0, stats.ontology_count, Some(stats), editable)
        })
        .unwrap_or((false, 0, None, false));
    let reasoner = state.with_catalog_and_reasoner(|_, r| r.cloned()).flatten();
    let registry_dirty = params.ontology_registry.iter().any(|e| e.dirty);
    let dirty = params.dirty_document_count > 0 || registry_dirty;
    Ok(crate::protocol::WorkspaceUiState {
        has_ontology,
        ontology_count,
        is_dirty: dirty,
        has_selection: params.selection_iri.is_some(),
        selection_iri: params.selection_iri.clone(),
        selection_editable,
        reasoner_running: false,
        reasoner_dirty: false,
        reasoner_consistent: reasoner.as_ref().map(|r| r.consistent),
        active_ontology_id: params.active_ontology_id.or_else(|| state.active_ontology_id()),
        stats,
        ontology_registry: params.ontology_registry,
    })
}

pub fn handle_get_dialog_schema(
    params: crate::protocol::GetDialogSchemaParams,
) -> Result<crate::protocol::GetDialogSchemaResult, LspErrorPayload> {
    use crate::protocol::{DialogFieldSchema, DialogSchema};
    let field = |id: &str, label: &str, field_type: &str, required: bool, validation: &[&str]| {
        DialogFieldSchema {
            id: id.to_string(),
            label: label.to_string(),
            field_type: field_type.to_string(),
            required,
            placeholder: None,
            validation: validation.iter().map(|s| (*s).to_string()).collect(),
        }
    };
    let schema = match params.dialog_id.as_str() {
        "new_ontology" => DialogSchema {
            id: "new_ontology".into(),
            title: "New Ontology".into(),
            primary_action: "Create".into(),
            fields: vec![
                field("path", "File path", "path", true, &["required"]),
                field("ontology_iri", "Ontology IRI", "iri", true, &["required", "iri"]),
                field("version_iri", "Version IRI", "iri", false, &["iri"]),
                field("format", "Format", "enum", true, &[]),
            ],
        },
        "export_ontology" | "save_as" => DialogSchema {
            id: params.dialog_id.clone(),
            title: if params.dialog_id == "save_as" {
                "Save As".into()
            } else {
                "Export Ontology".into()
            },
            primary_action: "Export".into(),
            fields: vec![
                field("source_path", "Source", "path", true, &["required"]),
                field("output_path", "Output path", "path", true, &["required"]),
                field("format", "Format", "enum", false, &[]),
            ],
        },
        "prefix_manager" => DialogSchema {
            id: "prefix_manager".into(),
            title: "Prefix Manager".into(),
            primary_action: "Apply".into(),
            fields: vec![
                field("prefix", "Prefix", "string", true, &["required", "prefix"]),
                field("namespace_iri", "Namespace IRI", "iri", true, &["required", "iri"]),
            ],
        },
        "ontology_metadata" => DialogSchema {
            id: "ontology_metadata".into(),
            title: "Ontology Metadata".into(),
            primary_action: "Save".into(),
            fields: vec![
                field("ontology_iri", "Ontology IRI", "iri", true, &["required", "iri"]),
                field("version_iri", "Version IRI", "iri", false, &["iri"]),
                field("label", "Label", "string", false, &[]),
                field("comment", "Comment", "string", false, &[]),
            ],
        },
        "search" => DialogSchema {
            id: "search".into(),
            title: "Search Entities".into(),
            primary_action: "Search".into(),
            fields: vec![field("query", "Query", "string", true, &["required"])],
        },
        "metrics" => DialogSchema {
            id: "metrics".into(),
            title: "Ontology Metrics".into(),
            primary_action: "Close".into(),
            fields: vec![],
        },
        "delete_entity" => DialogSchema {
            id: "delete_entity".into(),
            title: "Delete Entity".into(),
            primary_action: "Delete".into(),
            fields: vec![field("entity_iri", "Entity IRI", "iri", true, &["required", "iri"])],
        },
        "reasoner_settings" => DialogSchema {
            id: "reasoner_settings".into(),
            title: "Reasoner Settings".into(),
            primary_action: "Save".into(),
            fields: vec![
                field("profile", "Profile", "enum", true, &[]),
                field("auto_profile", "Auto profile warnings", "boolean", false, &[]),
            ],
        },
        "preferences" => DialogSchema {
            id: "preferences".into(),
            title: "Preferences".into(),
            primary_action: "Close".into(),
            fields: vec![],
        },
        "import" => DialogSchema {
            id: "import".into(),
            title: "Import Ontology".into(),
            primary_action: "Add Import".into(),
            fields: vec![
                field("ontology_iri", "Ontology IRI", "iri", true, &["required", "iri"]),
                field("import_iri", "Import IRI", "iri", true, &["required", "iri"]),
            ],
        },
        "rename" => DialogSchema {
            id: "rename".into(),
            title: "Rename Entity".into(),
            primary_action: "Rename".into(),
            fields: vec![
                field("from_iri", "Current IRI", "iri", true, &["required", "iri"]),
                field("to_iri", "New IRI", "iri", true, &["required", "iri"]),
            ],
        },
        other => {
            return Err(LspErrorPayload::invalid_params(format!("unknown dialog: {other}")));
        }
    };
    Ok(crate::protocol::GetDialogSchemaResult { schema })
}

pub fn handle_create_ontology(
    state: &ServerState,
    params: crate::protocol::CreateOntologyParams,
) -> Result<crate::protocol::CreateOntologyResult, LspErrorPayload> {
    use std::io::Write;
    let roots = state.workspace_roots();
    if roots.is_empty() {
        return Err(LspErrorPayload::invalid_params("workspace not initialized".into()));
    }
    let path = validate_workspace_scope_any(std::path::Path::new(&params.path), &roots)
        .map_err(LspErrorPayload::invalid_params)?;
    if path.exists() {
        return Err(LspErrorPayload::invalid_params(format!(
            "file already exists: {}",
            path.display()
        )));
    }
    if !strixonomy_owl::is_safe_iri(&params.ontology_iri) {
        return Err(LspErrorPayload::invalid_params(format!(
            "ontology IRI contains characters that cannot be safely written: {:?}",
            params.ontology_iri
        )));
    }
    if let Some(version_iri) = &params.version_iri {
        if !strixonomy_owl::is_safe_iri(version_iri) {
            return Err(LspErrorPayload::invalid_params(format!(
                "version IRI contains characters that cannot be safely written: {version_iri:?}"
            )));
        }
    }
    if let Some(prefixes) = &params.prefixes {
        for (prefix, iri) in prefixes {
            strixonomy_owl::validate_prefix(prefix, iri)
                .map_err(|e| LspErrorPayload::invalid_params(e.to_string()))?;
        }
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| LspErrorPayload::index_failed(e.to_string()))?;
    }
    // #354: re-validate after mkdir so a TOCTOU parent symlink cannot escape the jail.
    let path = validate_workspace_scope_any(std::path::Path::new(&params.path), &roots)
        .map_err(LspErrorPayload::invalid_params)?;
    if let Ok(meta) = std::fs::symlink_metadata(&path) {
        if meta.file_type().is_symlink() {
            return Err(LspErrorPayload::invalid_params(format!(
                "refusing to write through symlink: {}",
                path.display()
            )));
        }
    }
    let format = params
        .format
        .as_deref()
        .unwrap_or_else(|| path.extension().and_then(|e| e.to_str()).unwrap_or("ttl"));
    let format_key = format.to_ascii_lowercase();
    // Reject explicit format/extension mismatches (e.g. format=ttl under *.owl; #342).
    if let Some(ext) = path.extension().and_then(|e| e.to_str()).map(|s| s.to_ascii_lowercase()) {
        let ext_implies = match ext.as_str() {
            "ttl" | "turtle" => Some("turtle"),
            "obo" => Some("obo"),
            "owl" | "rdf" => Some("rdfxml"),
            "owx" => Some("owx"),
            _ => None,
        };
        let content_kind = match format_key.as_str() {
            "ttl" | "turtle" => "turtle",
            "obo" => "obo",
            other => other,
        };
        if let Some(implied) = ext_implies {
            if implied != content_kind {
                return Err(LspErrorPayload::invalid_params(format!(
                    "createOntology format {format_key:?} does not match path extension .{ext}; \
                     use a matching extension (.ttl/.obo) or a matching format"
                )));
            }
        }
    }
    let content = match format_key.as_str() {
        "obo" => {
            let mut s = format!("format-version: 1.2\nontology: {}\n", params.ontology_iri);
            if let Some(v) = &params.version_iri {
                s.push_str(&format!("data-version: {v}\n"));
            }
            s.push('\n');
            s
        }
        "ttl" | "turtle" => {
            let mut s = String::from(
                "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\
                 @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n\
                 @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
                 @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n\n",
            );
            if let Some(prefixes) = &params.prefixes {
                for (p, iri) in prefixes {
                    s.push_str(&format!("@prefix {p}: <{iri}> .\n"));
                }
                s.push('\n');
            }
            s.push_str(&format!("<{}> a owl:Ontology", params.ontology_iri));
            if let Some(v) = &params.version_iri {
                s.push_str(&format!(" ;\n    owl:versionIRI <{v}>"));
            }
            s.push_str(" .\n");
            s
        }
        other => {
            return Err(LspErrorPayload::invalid_params(format!(
                "unsupported createOntology format {other:?}; supported: ttl, turtle, obo"
            )));
        }
    };
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|e| LspErrorPayload::index_failed(e.to_string()))?;
    file.write_all(content.as_bytes()).map_err(|e| LspErrorPayload::index_failed(e.to_string()))?;
    Ok(crate::protocol::CreateOntologyResult {
        path: path.display().to_string(),
        ontology_iri: params.ontology_iri,
    })
}

pub fn handle_export_ontology(
    state: &ServerState,
    params: crate::protocol::ExportOntologyParams,
) -> Result<crate::protocol::ExportOntologyResult, LspErrorPayload> {
    let roots = state.workspace_roots();
    if roots.is_empty() {
        return Err(LspErrorPayload::invalid_params("workspace not initialized".into()));
    }
    let source = validate_workspace_scope_any(std::path::Path::new(&params.source_path), &roots)
        .map_err(LspErrorPayload::invalid_params)?;
    let output = validate_workspace_scope_any(std::path::Path::new(&params.output_path), &roots)
        .map_err(LspErrorPayload::invalid_params)?;
    match strixonomy_robot::robot_convert(None, &source, &output) {
        Ok(out) => Ok(crate::protocol::ExportOntologyResult {
            output_path: output.display().to_string(),
            success: out.exit_code == 0,
            logs: Some(format!("{}\n{}", out.stdout, out.stderr)),
        }),
        Err(e) => {
            // Fallback: copy Turtle/OBO as-is when ROBOT is unavailable.
            if source.extension() == output.extension() {
                if let Some(parent) = output.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|err| LspErrorPayload::robot_failed(err.to_string()))?;
                }
                // #354: re-validate after mkdir (parent symlink TOCTOU).
                let output =
                    validate_workspace_scope_any(std::path::Path::new(&params.output_path), &roots)
                        .map_err(LspErrorPayload::invalid_params)?;
                // #353: refuse leaf symlink; exclusive/non-following write instead of fs::copy.
                if let Ok(meta) = std::fs::symlink_metadata(&output) {
                    if meta.file_type().is_symlink() {
                        return Err(LspErrorPayload::robot_failed(format!(
                            "refusing to write through symlink: {}",
                            output.display()
                        )));
                    }
                }
                let bytes = std::fs::read(&source)
                    .map_err(|err| LspErrorPayload::robot_failed(err.to_string()))?;
                let mut opts = std::fs::OpenOptions::new();
                opts.write(true);
                if output.exists() {
                    opts.truncate(true);
                } else {
                    opts.create_new(true);
                }
                let mut file = opts
                    .open(&output)
                    .map_err(|err| LspErrorPayload::robot_failed(err.to_string()))?;
                use std::io::Write;
                file.write_all(&bytes)
                    .map_err(|err| LspErrorPayload::robot_failed(err.to_string()))?;
                Ok(crate::protocol::ExportOntologyResult {
                    output_path: output.display().to_string(),
                    success: true,
                    logs: Some(format!("ROBOT unavailable ({e}); copied source to output")),
                })
            } else {
                Err(LspErrorPayload::robot_failed(e.to_string()))
            }
        }
    }
}

pub fn handle_set_active_ontology(
    state: &ServerState,
    params: crate::protocol::SetActiveOntologyParams,
) -> Result<crate::protocol::SetActiveOntologyResult, LspErrorPayload> {
    let found = state
        .with_catalog(|catalog| {
            catalog.data().documents.iter().any(|d| {
                d.id == params.ontology_id
                    || d.path.as_os_str() == std::ffi::OsStr::new(params.ontology_id.as_str())
                    || d.base_iri.as_deref() == Some(params.ontology_id.as_str())
            })
        })
        .unwrap_or(false);
    if !found {
        return Err(LspErrorPayload::not_found(&params.ontology_id));
    }
    state.set_active_ontology_id(Some(params.ontology_id.clone()));
    Ok(crate::protocol::SetActiveOntologyResult { active_ontology_id: params.ontology_id })
}

pub fn handle_delete_impact(
    state: &ServerState,
    params: crate::protocol::DeleteImpactParams,
) -> Result<crate::protocol::DeleteImpactResult, LspErrorPayload> {
    let usages = state
        .with_catalog_and_overrides(|catalog, overrides| {
            strixonomy_refactor::find_usages_with_overrides(catalog, &params.entity_iri, overrides)
        })
        .ok_or_else(LspErrorPayload::not_indexed)?;
    let mut referencing = std::collections::BTreeSet::new();
    let mut warnings = Vec::new();
    for u in &usages {
        // `referenced_iri` is always the search target; `iri` is the referencer (or the
        // target itself for declarations / subject-side axioms). Collect distinct
        // referencers, excluding the entity's own declaration and self-usages.
        if !matches!(u.kind, strixonomy_refactor::UsageKind::EntityDeclaration)
            && u.iri != params.entity_iri
        {
            referencing.insert(u.iri.clone());
        }
        if matches!(u.kind, strixonomy_refactor::UsageKind::Import) {
            warnings.push("Entity appears in import-related references".into());
        }
    }
    let axiom_count = state
        .with_catalog(|catalog| {
            catalog
                .data()
                .axioms
                .iter()
                .filter(|a| a.subject == params.entity_iri || a.object == params.entity_iri)
                .count()
        })
        .unwrap_or(0);
    Ok(crate::protocol::DeleteImpactResult {
        entity_iri: params.entity_iri,
        usage_count: usages.len(),
        axiom_count,
        referencing_entities: referencing.into_iter().collect(),
        warnings,
    })
}

pub fn handle_list_plugins(state: &ServerState) -> Result<ListPluginsResult, LspErrorPayload> {
    let workspace = state.effective_index_root().ok_or_else(LspErrorPayload::not_indexed)?;
    let host = strixonomy_plugin_builtins::load_plugin_host(&workspace)
        .map_err(|e| LspErrorPayload::index_failed(e.to_string()))?;
    Ok(ListPluginsResult { plugins: host.list_plugins() })
}

pub fn handle_run_plugin(
    state: &ServerState,
    params: RunPluginParams,
) -> Result<RunPluginResult, LspErrorPayload> {
    let workspace = state.effective_index_root().ok_or_else(LspErrorPayload::not_indexed)?;
    let mut host = strixonomy_plugin_builtins::load_plugin_host(&workspace)
        .map_err(|e| LspErrorPayload::index_failed(e.to_string()))?;
    state
        .with_catalog(|catalog| {
            host.run_plugin_action(
                &params.plugin_id,
                &params.action,
                Some(catalog),
                None,
                params.step.as_deref(),
                params.view_id.as_deref(),
                params.query.as_deref(),
                params.focus_iri.as_deref(),
            )
            .map(|result| RunPluginResult {
                diagnostics: result.diagnostics.iter().map(DiagnosticSummary::from).collect(),
                output_paths: result.output_paths,
                logs: result.logs,
                view_html: result.view_html,
                success: result.success,
                result: result.result,
                columns: result.columns,
                rows: result.rows,
                unsatisfiable: result.unsatisfiable,
                affected_iris: result.affected_iris,
                root_iris: result.root_iris,
                graph_kind: result.graph_kind,
                hints: result.hints,
                profile: result.profile,
            })
            .map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_get_entity(
    state: &ServerState,
    params: GetEntityParams,
) -> Result<GetEntityResult, LspErrorPayload> {
    state
        .with_catalog(|catalog| {
            catalog
                .entity_detail(&params.iri)
                .map(|detail| GetEntityResult { detail })
                .ok_or_else(|| LspErrorPayload::not_found(&params.iri))
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

/// Entity search over the indexed catalog (same matching as `Workspace::search`).
pub fn handle_search(
    state: &ServerState,
    params: SearchParams,
) -> Result<SearchResult, LspErrorPayload> {
    let needle = params.query.to_ascii_lowercase();
    let limit = params.limit.unwrap_or(100).min(500);
    if needle.is_empty() {
        return Ok(SearchResult { entities: Vec::new() });
    }
    state
        .with_catalog(|catalog| {
            let hierarchy = catalog.class_hierarchy();
            let mut matches: Vec<strixonomy_catalog::EntityDetail> = catalog
                .data()
                .entities
                .iter()
                .filter(|entity| entity_matches_search_term(entity, &needle))
                .filter_map(|entity| catalog.entity_detail_with_hierarchy(&entity.iri, &hierarchy))
                .collect();
            matches.sort_by(|a, b| a.entity.short_name.cmp(&b.entity.short_name));
            matches.dedup_by(|a, b| a.entity.iri == b.entity.iri);
            matches.truncate(limit);
            Ok(SearchResult { entities: matches })
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

fn entity_matches_search_term(entity: &strixonomy_core::Entity, needle: &str) -> bool {
    if entity.iri.to_ascii_lowercase().contains(needle) {
        return true;
    }
    if entity.short_name.to_ascii_lowercase().contains(needle) {
        return true;
    }
    if entity.obo_id.as_ref().is_some_and(|id| id.to_ascii_lowercase().contains(needle)) {
        return true;
    }
    entity.labels.iter().any(|label| label.to_ascii_lowercase().contains(needle))
}

pub fn handle_get_graph(
    state: &ServerState,
    params: GraphRequest,
) -> Result<GetGraphResult, LspErrorPayload> {
    state
        .with_catalog_and_reasoner(|catalog, reasoner| {
            let mut builder = GraphBuilder::new(catalog);
            if params.include_inferred {
                if let Some(snapshot) = reasoner {
                    builder = builder.with_inferred_edges(&snapshot.inferred.edges);
                }
            }
            let graph =
                builder.build(&params).map_err(|e| LspErrorPayload::graph_failed(e.to_string()))?;
            Ok(GetGraphResult { graph })
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_semantic_diff(
    state: &ServerState,
    params: SemanticDiffParams,
) -> Result<SemanticDiffResult, LspErrorPayload> {
    let roots = state.workspace_roots();
    if roots.is_empty() {
        return Err(LspErrorPayload::invalid_params("workspace not initialized".to_string()));
    }

    if let (Some(left), Some(right)) = (&params.left_path, &params.right_path) {
        let left = validate_workspace_scope_any(Path::new(left), &roots)
            .map_err(LspErrorPayload::invalid_params)?;
        let right = validate_workspace_scope_any(Path::new(right), &roots)
            .map_err(LspErrorPayload::invalid_params)?;
        let left_cat = IndexBuilder::new()
            .workspace(&left)
            .build()
            .map_err(|e| LspErrorPayload::invalid_params(e.to_string()))?;
        let right_cat = IndexBuilder::new()
            .workspace(&right)
            .build()
            .map_err(|e| LspErrorPayload::invalid_params(e.to_string()))?;
        let mut diff = diff_catalogs(&left_cat, &right_cat);
        if params.reasoner {
            enrich_diff_with_reasoner(state, &mut diff, &left_cat, &right_cat)?;
        }
        return Ok(semantic_diff_result(diff, params.format.as_deref()));
    }

    let left_ref = params.left_ref.as_deref().unwrap_or("HEAD");
    let right_ref = params.right_ref.as_deref().unwrap_or("WORKTREE");
    let repo_root = discover_repo_root(&roots).ok_or_else(|| {
        LspErrorPayload::invalid_params("no git repository found in workspace".to_string())
    })?;

    if strixonomy_diff::is_indexed_catalog_ref(left_ref)
        && strixonomy_diff::is_worktree_ref(right_ref)
    {
        return state
            .with_catalog(|head| {
                let worktree = build_lsp_worktree_catalog(state)?;
                let mut diff = diff_catalogs(head, &worktree);
                if params.reasoner {
                    enrich_diff_with_reasoner(state, &mut diff, head, &worktree)?;
                }
                Ok(semantic_diff_result(diff, params.format.as_deref()))
            })
            .ok_or_else(LspErrorPayload::not_indexed)?;
    }

    if strixonomy_diff::is_indexed_catalog_ref(left_ref)
        && strixonomy_diff::is_indexed_catalog_ref(right_ref)
    {
        return state
            .with_catalog(|catalog| {
                let mut diff = diff_catalogs(catalog, catalog);
                if params.reasoner {
                    enrich_diff_with_reasoner(state, &mut diff, catalog, catalog)?;
                }
                Ok(semantic_diff_result(diff, params.format.as_deref()))
            })
            .ok_or_else(LspErrorPayload::not_indexed)?;
    }

    if strixonomy_diff::is_indexed_catalog_ref(left_ref) {
        let other = catalog_at_git_ref(&repo_root, right_ref)
            .map_err(|e| LspErrorPayload::invalid_params(e.to_string()))?;
        return state
            .with_catalog(|indexed| {
                let mut diff = diff_catalogs(&other, indexed);
                if params.reasoner {
                    enrich_diff_with_reasoner(state, &mut diff, &other, indexed)?;
                }
                Ok(semantic_diff_result(diff, params.format.as_deref()))
            })
            .ok_or_else(LspErrorPayload::not_indexed)?;
    }

    if strixonomy_diff::is_indexed_catalog_ref(right_ref) {
        let base = catalog_at_git_ref(&repo_root, left_ref)
            .map_err(|e| LspErrorPayload::invalid_params(e.to_string()))?;
        return state
            .with_catalog(|indexed| {
                let mut diff = diff_catalogs(&base, indexed);
                if params.reasoner {
                    enrich_diff_with_reasoner(state, &mut diff, &base, indexed)?;
                }
                Ok(semantic_diff_result(diff, params.format.as_deref()))
            })
            .ok_or_else(LspErrorPayload::not_indexed)?;
    }

    let (mut diff, left_cat, right_cat) = if strixonomy_diff::is_worktree_ref(right_ref) {
        let left = catalog_at_git_ref(&repo_root, left_ref)
            .map_err(|e| LspErrorPayload::invalid_params(e.to_string()))?;
        let right = build_lsp_worktree_catalog(state).or_else(|_| {
            catalog_at_worktree(&repo_root)
                .map_err(|e| LspErrorPayload::invalid_params(e.to_string()))
        })?;
        let diff = diff_catalogs(&left, &right);
        (diff, left, right)
    } else {
        diff_git_refs_with_catalogs(&repo_root, left_ref, right_ref)
            .map_err(|e| LspErrorPayload::invalid_params(e.to_string()))?
    };

    if params.reasoner {
        enrich_diff_with_reasoner(state, &mut diff, &left_cat, &right_cat)?;
    }

    Ok(semantic_diff_result(diff, params.format.as_deref()))
}

fn semantic_diff_result(diff: DiffResult, format: Option<&str>) -> SemanticDiffResult {
    let formatted = format
        .filter(|f| f.eq_ignore_ascii_case("pr-summary"))
        .map(|_| format_diff_pr_summary(&diff));
    SemanticDiffResult { diff, formatted }
}

fn build_lsp_worktree_catalog(state: &ServerState) -> Result<OntologyCatalog, LspErrorPayload> {
    let roots = state.workspace_roots();
    if roots.is_empty() {
        return Err(LspErrorPayload::not_indexed());
    }
    let overrides = state.open_document_overrides();
    IndexBuilder::new()
        .workspace(roots[0].clone())
        .scan_roots(roots)
        .document_overrides(overrides)
        .build()
        .map_err(|e| LspErrorPayload::invalid_params(e.to_string()))
}

fn enrich_diff_with_reasoner(
    state: &ServerState,
    diff: &mut DiffResult,
    base_catalog: &OntologyCatalog,
    head_catalog: &OntologyCatalog,
) -> Result<(), LspErrorPayload> {
    let roots = state.workspace_roots();
    let profile = ReasonerId::El;

    let mut base_loader = WorkspaceInputLoader::new(base_catalog.workspace());
    if catalog_is_live_workspace(base_catalog, &roots) && !roots.is_empty() {
        base_loader = base_loader.scan_roots(roots.clone());
    }

    let mut head_loader = WorkspaceInputLoader::new(head_catalog.workspace());
    if catalog_is_live_workspace(head_catalog, &roots) && !roots.is_empty() {
        head_loader =
            head_loader.scan_roots(roots).document_overrides(state.open_documents_for_reasoner());
    }

    let base_input =
        base_loader.load().map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;
    let head_input =
        head_loader.load().map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;
    let base_cls = classify(profile, &base_input, true)
        .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;
    let head_cls = classify(profile, &head_input, true)
        .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;
    apply_unsat_diff(diff, &base_cls.unsatisfiable, &head_cls.unsatisfiable);
    Ok(())
}

/// True when `catalog` was built from the live workspace (not a git checkout tempdir).
fn catalog_is_live_workspace(catalog: &OntologyCatalog, roots: &[std::path::PathBuf]) -> bool {
    if roots.is_empty() {
        return false;
    }
    let ws = catalog.workspace();
    roots.iter().any(|root| {
        root == ws
            || ws.starts_with(root)
            || root.starts_with(ws)
            || root
                .canonicalize()
                .ok()
                .zip(ws.canonicalize().ok())
                .is_some_and(|(a, b)| a == b || b.starts_with(&a) || a.starts_with(&b))
    })
}

pub fn handle_run_robot(
    state: &ServerState,
    index_worker: &IndexWorker,
    params: RunRobotParams,
) -> Result<RunRobotResult, LspErrorPayload> {
    let roots = state.workspace_roots();
    if roots.is_empty() {
        return Err(LspErrorPayload::robot_failed("workspace not initialized".to_string()));
    }
    let mut args = vec![params.subcommand];
    args.extend(params.args);
    // #346: rewrite path operands to absolute jailed paths (do not rely on process CWD).
    let args = jail_robot_path_args(&roots, &args).map_err(LspErrorPayload::robot_failed)?;
    index_worker.run_robot_sync(params.robot_path, args).map_err(LspErrorPayload::robot_failed)
}

/// Jail ROBOT file operands and rewrite them to absolute paths under the workspace (#346).
///
/// Handles separate `--input path`, `--input=path`, attached short forms (`-i/path`),
/// and multi-operand flags such as `--query <query> <output>`.
fn jail_robot_path_args(
    workspace_roots: &[std::path::PathBuf],
    args: &[String],
) -> Result<Vec<String>, String> {
    /// How many path operands a flag still requires / may accept.
    #[derive(Clone, Copy)]
    struct PathExpect {
        /// Must still consume this many paths before the next flag or end.
        min: usize,
        /// May still consume this many paths (consecutive non-flag args).
        max: usize,
    }

    fn path_expect_for_long_flag(flag: &str) -> Option<PathExpect> {
        match flag {
            "--input" | "--output" | "--report" | "--catalog" | "--ontology" | "--left"
            | "--right" | "--merge-input" | "--output-dir" | "--update" | "--prefixes"
            | "--add-prefixes" | "--inputs" => Some(PathExpect { min: 1, max: 1 }),
            // ROBOT: --query/--select/--construct take query file + output file.
            "--query" | "--select" | "--construct" => Some(PathExpect { min: 1, max: 2 }),
            // ROBOT: --queries takes one or more query files until the next flag.
            "--queries" => Some(PathExpect { min: 1, max: usize::MAX }),
            _ => None,
        }
    }

    fn path_expect_for_short_flag(flag: &str) -> Option<PathExpect> {
        match flag {
            "-i" | "-o" | "-c" | "-u" => Some(PathExpect { min: 1, max: 1 }),
            "-q" => Some(PathExpect { min: 1, max: 2 }),
            _ => None,
        }
    }

    fn jail_path(
        workspace_roots: &[std::path::PathBuf],
        value: &str,
    ) -> Result<std::path::PathBuf, String> {
        strixonomy_core::validate_workspace_scope_any(std::path::Path::new(value), workspace_roots)
    }

    let mut out = args.to_vec();
    let mut expect: Option<PathExpect> = None;
    // Index loop: rewrite `out[idx]` in place while tracking multi-operand path flags.
    #[allow(clippy::needless_range_loop)]
    for idx in 1..out.len() {
        let arg = out[idx].clone();
        if let Some(pending) = expect.as_mut() {
            if arg.starts_with('-') {
                if pending.min > 0 {
                    return Err("ROBOT path flag is missing a path argument".to_string());
                }
                expect = None;
            } else {
                let abs = jail_path(workspace_roots, &arg)?;
                out[idx] = abs.display().to_string();
                pending.min = pending.min.saturating_sub(1);
                pending.max = pending.max.saturating_sub(1);
                if pending.max == 0 {
                    expect = None;
                }
                continue;
            }
        }

        if let Some((flag, value)) = arg.split_once('=') {
            if path_expect_for_long_flag(flag).is_some()
                || path_expect_for_short_flag(flag).is_some()
            {
                // Handles `--input=/abs` and `-i=/abs` (short `=` was previously
                // mis-parsed as attached value `=/abs` and jailed as relative).
                let abs = jail_path(workspace_roots, value)?;
                out[idx] = format!("{flag}={}", abs.display());
                continue;
            }
        }

        if let Some(pending) = path_expect_for_long_flag(&arg) {
            expect = Some(pending);
            continue;
        }
        if let Some(pending) = path_expect_for_short_flag(&arg) {
            expect = Some(pending);
            continue;
        }

        let attached_short = ["-i", "-o", "-c", "-u", "-q"].iter().find_map(|flag| {
            if arg.starts_with(flag)
                && arg.len() > flag.len()
                && !arg[flag.len()..].starts_with('-')
            {
                Some((*flag, &arg[flag.len()..]))
            } else {
                None
            }
        });
        if let Some((flag, value)) = attached_short {
            let abs = jail_path(workspace_roots, value)?;
            out[idx] = format!("{flag}{}", abs.display());
            // Attached short form supplies the first path; optional second path may follow.
            if let Some(mut pending) = path_expect_for_short_flag(flag) {
                pending.min = pending.min.saturating_sub(1);
                pending.max = pending.max.saturating_sub(1);
                if pending.max > 0 {
                    expect = Some(pending);
                }
            }
            continue;
        }

        // Positional path-like args (contain / or end with ontology extensions).
        let looks_like_path = arg.contains('/')
            || arg.contains('\\')
            || arg.ends_with(".ttl")
            || arg.ends_with(".owl")
            || arg.ends_with(".obo")
            || arg.ends_with(".rdf")
            || arg.ends_with(".sparql")
            || arg.ends_with(".rq")
            || arg.ends_with(".ru");
        if looks_like_path && !arg.starts_with('-') {
            let abs = jail_path(workspace_roots, &arg)?;
            out[idx] = abs.display().to_string();
        }
    }
    if expect.is_some_and(|e| e.min > 0) {
        return Err("ROBOT path flag is missing a path argument".to_string());
    }
    Ok(out)
}

pub fn handle_apply_axiom_patch(
    state: &ServerState,
    index_worker: &IndexWorker,
    params: ApplyAxiomPatchParams,
) -> Result<ApplyAxiomPatchResult, LspErrorPayload> {
    state.with_catalog(|_| ()).ok_or_else(LspErrorPayload::not_indexed)?;
    let workspace_root = state
        .workspace_root()
        .ok_or_else(|| LspErrorPayload::patch_invalid("workspace not initialized".to_string()))?;
    let document_path = state
        .resolve_lsp_document_uri(&params.document_uri)
        .map_err(|e| LspErrorPayload::patch_invalid(e.to_string()))?;

    let namespaces = state
        .with_catalog(|catalog| {
            catalog
                .data()
                .documents
                .iter()
                .find(|d| strixonomy_core::paths_refer_to_same(&d.path, &document_path))
                .map(|d| d.namespaces.clone())
        })
        .flatten()
        .unwrap_or_default();

    let format = OntologyFormat::from_extension(
        document_path.extension().and_then(|e| e.to_str()).unwrap_or(""),
    );

    let edit_format = strixonomy_edit::EditFormat::from_ontology_format(format).map_err(|e| {
        LspErrorPayload::unsupported_format(format!(
            "write-back supports Turtle, OBO, RDF/XML, and OWL/XML; {e}"
        ))
    })?;

    // Serialize applies without holding ops_lock across enqueue_sync (index worker needs it).
    let (patch_result, workspace_edit, needs_reindex, entity_iri, undo_patches) = {
        let ops_lock = state.ops_lock();
        let _guard = ops_lock.lock().map_err(|e| LspErrorPayload::patch_invalid(e.to_string()))?;

        let source = state
            .document_text(&document_path)
            .ok_or_else(|| LspErrorPayload::patch_invalid("cannot read document".to_string()))?;

        let (transaction, patch_applied, patch_preview, patch_diagnostics) = if format
            == OntologyFormat::Obo
        {
            let transaction = strixonomy_edit::parse_obo_input(params.patches).map_err(|e| {
                LspErrorPayload::patch_invalid(format!("invalid OBO patch JSON: {e}"))
            })?;
            let result = transaction
                .apply_to_text(&source, params.preview_only, &namespaces)
                .map_err(|e| LspErrorPayload::patch_invalid(e.to_string()))?;
            (transaction, result.applied, result.preview_text, result.diagnostics)
        } else if matches!(
            format,
            OntologyFormat::Turtle
                | OntologyFormat::Owl
                | OntologyFormat::RdfXml
                | OntologyFormat::OwlXml
        ) {
            let transaction = strixonomy_edit::parse_turtle_input(params.patches)
                .map_err(|e| LspErrorPayload::patch_invalid(format!("invalid patch JSON: {e}")))?;
            let result = transaction
                .apply_to_text_as(&source, params.preview_only, &namespaces, edit_format)
                .map_err(|e| match e {
                    strixonomy_edit::EditError::UnsupportedFormat(m) => {
                        LspErrorPayload::unsupported_format(m)
                    }
                    _ => LspErrorPayload::patch_invalid(e.to_string()),
                })?;
            (transaction, result.applied, result.preview_text, result.diagnostics)
        } else {
            return Err(LspErrorPayload::unsupported_format(format!(
                "write-back supports Turtle, OBO, RDF/XML, and OWL/XML; got {}",
                format.as_str()
            )));
        };

        let entity_iri = primary_iri_from_transaction(&transaction);

        let patch_result = strixonomy_owl::ApplyPatchResult {
            applied: patch_applied,
            preview_text: patch_preview.clone(),
            diagnostics: patch_diagnostics,
            document_path: Some(document_path.display().to_string()),
        };

        if !patch_result.applied && !patch_result.diagnostics.is_empty() {
            return Ok(ApplyAxiomPatchResult {
                patch: patch_result,
                entity_detail: None,
                reindex_warning: None,
                workspace_edit: None,
                undo_patches: None,
            });
        }

        let mut workspace_edit = None;
        let mut needs_reindex = false;
        let undo_patches = if patch_result.applied && !params.preview_only {
            transaction.invert().ok().and_then(|inverted| match format {
                OntologyFormat::Turtle
                | OntologyFormat::Owl
                | OntologyFormat::RdfXml
                | OntologyFormat::OwlXml => {
                    inverted.turtle_patches().ok().and_then(|p| serde_json::to_value(p).ok())
                }
                OntologyFormat::Obo => {
                    inverted.obo_patches().ok().and_then(|p| serde_json::to_value(p).ok())
                }
                _ => None,
            })
        } else {
            None
        };
        if patch_result.applied && !params.preview_only {
            if let Some(text) = &patch_result.preview_text {
                if text.len() as u64 > strixonomy_core::MAX_FILE_BYTES {
                    return Err(LspErrorPayload::patch_invalid(format!(
                        "patched document exceeds maximum size of {} bytes",
                        strixonomy_core::MAX_FILE_BYTES
                    )));
                }
                // Disk first so a failed write never leaves a divergent LSP buffer.
                if format == OntologyFormat::Obo {
                    strixonomy_obo::atomic_write(&document_path, text)
                        .map_err(|e| LspErrorPayload::patch_invalid(e.to_string()))?;
                } else {
                    strixonomy_owl::atomic_write(&document_path, text)
                        .map_err(|e| LspErrorPayload::patch_invalid(e.to_string()))?;
                }
                if state.is_document_open(&document_path) {
                    if let Err(e) = state.set_document_text(document_path.clone(), text.clone()) {
                        if let Err(rollback_err) =
                            atomic_write_for_format(format, &document_path, &source)
                        {
                            return Err(LspErrorPayload::patch_invalid(format!(
                                "buffer update failed ({e}); disk rollback also failed: {rollback_err}"
                            )));
                        }
                        return Err(LspErrorPayload::patch_invalid(e));
                    }
                }
                workspace_edit = full_document_workspace_edit(state, &document_path, text);
                needs_reindex = true;
            }
        }
        (patch_result, workspace_edit, needs_reindex, entity_iri, undo_patches)
    };

    let mut reindex_warning = None;
    if needs_reindex {
        let index_root = state.effective_index_root().unwrap_or_else(|| workspace_root.clone());
        if let Err(msg) = index_worker.enqueue_sync(index_root) {
            reindex_warning = Some(format!("patch applied but reindex failed: {msg}"));
        }
    }

    let entity_detail = entity_iri
        .and_then(|iri| state.with_catalog(|catalog| catalog.entity_detail(&iri)))
        .flatten();

    Ok(ApplyAxiomPatchResult {
        patch: patch_result,
        entity_detail,
        reindex_warning,
        workspace_edit,
        undo_patches,
    })
}

fn primary_iri_from_transaction(transaction: &strixonomy_edit::Transaction) -> Option<String> {
    if let Ok(patches) = transaction.turtle_patches() {
        return patches.first().and_then(primary_iri_from_turtle_patch).filter(|s| !s.is_empty());
    }
    if let Ok(patches) = transaction.obo_patches() {
        return patches.first().and_then(primary_iri_from_obo_patch).filter(|s| !s.is_empty());
    }
    None
}

fn primary_iri_from_obo_patch(op: &strixonomy_obo::OboPatchOp) -> Option<String> {
    Some(match op {
        strixonomy_obo::OboPatchOp::SetName { term_id, .. }
        | strixonomy_obo::OboPatchOp::AddSynonym { term_id, .. }
        | strixonomy_obo::OboPatchOp::RemoveSynonym { term_id, .. }
        | strixonomy_obo::OboPatchOp::AddDef { term_id, .. }
        | strixonomy_obo::OboPatchOp::RemoveDef { term_id }
        | strixonomy_obo::OboPatchOp::AddXref { term_id, .. }
        | strixonomy_obo::OboPatchOp::RemoveXref { term_id, .. }
        | strixonomy_obo::OboPatchOp::SetNamespace { term_id, .. }
        | strixonomy_obo::OboPatchOp::SetDeprecated { term_id, .. }
        | strixonomy_obo::OboPatchOp::AddIsA { term_id, .. }
        | strixonomy_obo::OboPatchOp::RemoveIsA { term_id, .. } => term_id.clone(),
    })
}

fn primary_iri_from_turtle_patch(p: &strixonomy_owl::PatchOp) -> Option<String> {
    Some(match p {
        strixonomy_owl::PatchOp::CreateEntity { entity_iri, .. }
        | strixonomy_owl::PatchOp::DeleteEntity { entity_iri }
        | strixonomy_owl::PatchOp::SetLabel { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddLabel { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveLabel { entity_iri, .. }
        | strixonomy_owl::PatchOp::SetComment { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddComment { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveComment { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddSubClassOf { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveSubClassOf { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddComplexSubClassOf { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveComplexSubClassOf { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddEquivalentClass { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveEquivalentClass { entity_iri, .. }
        | strixonomy_owl::PatchOp::SetEquivalentClass { entity_iri, .. }
        | strixonomy_owl::PatchOp::SetDeprecated { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddDisjointClass { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveDisjointClass { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddDomain { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveDomain { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddRange { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveRange { entity_iri, .. }
        | strixonomy_owl::PatchOp::SetFunctional { entity_iri, .. }
        | strixonomy_owl::PatchOp::SetInverseFunctional { entity_iri, .. }
        | strixonomy_owl::PatchOp::SetTransitive { entity_iri, .. }
        | strixonomy_owl::PatchOp::SetSymmetric { entity_iri, .. }
        | strixonomy_owl::PatchOp::SetAsymmetric { entity_iri, .. }
        | strixonomy_owl::PatchOp::SetReflexive { entity_iri, .. }
        | strixonomy_owl::PatchOp::SetIrreflexive { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddPropertyChain { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemovePropertyChain { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddClassAssertion { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveClassAssertion { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddObjectPropertyAssertion { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveObjectPropertyAssertion { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddDataPropertyAssertion { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveDataPropertyAssertion { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddAnnotation { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveAnnotation { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddNegativeObjectPropertyAssertion { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveNegativeObjectPropertyAssertion { entity_iri, .. }
        | strixonomy_owl::PatchOp::AddNegativeDataPropertyAssertion { entity_iri, .. }
        | strixonomy_owl::PatchOp::RemoveNegativeDataPropertyAssertion { entity_iri, .. } => {
            entity_iri.clone()
        }
        strixonomy_owl::PatchOp::AddHasKey { class_iri, .. }
        | strixonomy_owl::PatchOp::RemoveHasKey { class_iri, .. }
        | strixonomy_owl::PatchOp::AddDisjointUnion { class_iri, .. }
        | strixonomy_owl::PatchOp::RemoveDisjointUnion { class_iri, .. } => class_iri.clone(),
        strixonomy_owl::PatchOp::AddInverseObjectProperties { property_iri, .. }
        | strixonomy_owl::PatchOp::RemoveInverseObjectProperties { property_iri, .. }
        | strixonomy_owl::PatchOp::AddSubObjectPropertyOf { property_iri, .. }
        | strixonomy_owl::PatchOp::RemoveSubObjectPropertyOf { property_iri, .. }
        | strixonomy_owl::PatchOp::AddSubDataPropertyOf { property_iri, .. }
        | strixonomy_owl::PatchOp::RemoveSubDataPropertyOf { property_iri, .. } => {
            property_iri.clone()
        }
        strixonomy_owl::PatchOp::AddEquivalentObjectProperties { properties }
        | strixonomy_owl::PatchOp::RemoveEquivalentObjectProperties { properties }
        | strixonomy_owl::PatchOp::AddDisjointObjectProperties { properties }
        | strixonomy_owl::PatchOp::RemoveDisjointObjectProperties { properties }
        | strixonomy_owl::PatchOp::AddEquivalentDataProperties { properties }
        | strixonomy_owl::PatchOp::RemoveEquivalentDataProperties { properties }
        | strixonomy_owl::PatchOp::AddDisjointDataProperties { properties }
        | strixonomy_owl::PatchOp::RemoveDisjointDataProperties { properties } => {
            properties.first().cloned().unwrap_or_default()
        }
        strixonomy_owl::PatchOp::AddSameIndividual { individuals }
        | strixonomy_owl::PatchOp::RemoveSameIndividual { individuals }
        | strixonomy_owl::PatchOp::AddDifferentIndividuals { individuals }
        | strixonomy_owl::PatchOp::RemoveDifferentIndividuals { individuals } => {
            individuals.first().cloned().unwrap_or_default()
        }
        strixonomy_owl::PatchOp::AddDatatypeDefinition { datatype_iri, .. }
        | strixonomy_owl::PatchOp::RemoveDatatypeDefinition { datatype_iri, .. } => {
            datatype_iri.clone()
        }
        strixonomy_owl::PatchOp::AddAxiomAnnotation { subject_iri, .. }
        | strixonomy_owl::PatchOp::RemoveAxiomAnnotation { subject_iri, .. } => subject_iri.clone(),
        strixonomy_owl::PatchOp::AddImport { ontology_iri, .. }
        | strixonomy_owl::PatchOp::RemoveImport { ontology_iri, .. }
        | strixonomy_owl::PatchOp::SetOntologyIri { ontology_iri }
        | strixonomy_owl::PatchOp::SetVersionIri { ontology_iri, .. }
        | strixonomy_owl::PatchOp::AddOntologyAnnotation { ontology_iri, .. }
        | strixonomy_owl::PatchOp::RemoveOntologyAnnotation { ontology_iri, .. }
        | strixonomy_owl::PatchOp::AddSwrlRule { ontology_iri, .. }
        | strixonomy_owl::PatchOp::RemoveSwrlRule { ontology_iri, .. }
        | strixonomy_owl::PatchOp::ReplaceSwrlRule { ontology_iri, .. } => ontology_iri.clone(),
        strixonomy_owl::PatchOp::AddPrefix { .. }
        | strixonomy_owl::PatchOp::RemovePrefix { .. }
        | strixonomy_owl::PatchOp::SetPrefix { .. } => String::new(),
    })
}

pub fn handle_query(
    state: &ServerState,
    params: QueryParams,
) -> Result<TabularQueryResult, LspErrorPayload> {
    state
        .with_catalog(|catalog| {
            let result = strixonomy_query::query_catalog(catalog, &params.sql).map_err(
                |e: strixonomy_query::QueryError| LspErrorPayload::query_failed(e.to_string()),
            )?;
            let truncated = if result.truncated { Some(true) } else { None };
            Ok(TabularQueryResult { columns: result.columns, rows: result.rows, truncated })
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_list_sql_schema(state: &ServerState) -> Result<ListSqlSchemaResult, LspErrorPayload> {
    let tables = state
        .with_catalog(strixonomy_query::list_sql_schema)
        .ok_or_else(LspErrorPayload::not_indexed)?;
    Ok(ListSqlSchemaResult { tables })
}

pub fn handle_sparql(
    state: &ServerState,
    params: SparqlParams,
) -> Result<TabularQueryResult, LspErrorPayload> {
    state
        .with_catalog(|catalog| {
            let result = strixonomy_query::sparql_catalog(catalog, &params.query).map_err(
                |e: strixonomy_query::QueryError| LspErrorPayload::query_failed(e.to_string()),
            )?;
            let truncated = if result.truncated { Some(true) } else { None };
            Ok(TabularQueryResult { columns: result.columns, rows: result.rows, truncated })
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_dl_query(
    state: &ServerState,
    params: DlQueryParams,
) -> Result<DlQueryResult, LspErrorPayload> {
    let ops_lock = state.ops_lock();
    let _guard = ops_lock.lock().map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;
    let profile = ReasonerId::parse(&params.profile)
        .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;
    let mode = match params.mode.to_ascii_lowercase().as_str() {
        "asserted" => DlQueryMode::Asserted,
        _ => DlQueryMode::Inferred,
    };
    let input = load_reasoner_input(state)?;
    let namespaces = resolve_namespaces_for_dl_query(state, params.document_uri.as_deref())?;
    let result = run_dl_query(profile, &input, &params.expression, &namespaces, mode)
        .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;
    Ok(DlQueryResult {
        expression: result.expression,
        normalized: result.normalized,
        query_class_iri: result.query_class_iri,
        subclasses: result.subclasses,
        superclasses: result.superclasses,
        equivalents: result.equivalents,
        instances: result.instances,
        profile: result.profile,
        mode: match result.mode {
            DlQueryMode::Asserted => "asserted".to_string(),
            DlQueryMode::Inferred => "inferred".to_string(),
        },
        duration_ms: result.duration_ms,
        warnings: result.warnings,
        diagnostics: result.diagnostics,
    })
}

pub fn handle_parse_manchester(
    state: &ServerState,
    params: ParseManchesterParams,
) -> Result<ParseManchesterResult, LspErrorPayload> {
    let namespaces = resolve_namespaces_for_manchester(state, &params)?;
    let completions = build_manchester_completions(state);

    let parsed = strixonomy_owl::parse_class_expression(&params.expression, &namespaces)
        .map_err(|e| LspErrorPayload::manchester_invalid(e.to_string()))?;

    let turtle_predicate = match params.axiom_kind.as_str() {
        "equivalent_class" => "owl:equivalentClass",
        "disjoint_class" => {
            return Err(LspErrorPayload::manchester_invalid(
                "disjoint_class axioms use IRI patch ops (add_disjoint_class), not Manchester expressions"
                    .to_string(),
            ));
        }
        _ => "rdfs:subClassOf",
    };

    let turtle_fragment = strixonomy_owl::class_expression_to_turtle_fragment(
        &parsed.expression,
        turtle_predicate,
        &namespaces,
    )
    .map_err(|e| LspErrorPayload::manchester_invalid(e.to_string()))?;

    Ok(ParseManchesterResult {
        normalized: parsed.normalized,
        turtle_fragment,
        tree: parsed.tree,
        diagnostics: parsed
            .diagnostics
            .into_iter()
            .map(|d| strixonomy_owl::PatchDiagnostic {
                severity: "warning".to_string(),
                message: d.message,
            })
            .collect(),
        completions,
    })
}

fn ontology_iri_from_turtle(text: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix('<') {
            if let Some(end) = rest.find('>') {
                let iri = &rest[..end];
                if rest[end + 1..].contains("owl:Ontology")
                    || rest[end + 1..].contains("http://www.w3.org/2002/07/owl#Ontology")
                {
                    return Some(iri.to_string());
                }
            }
        }
    }
    None
}

fn collect_swrl_rules_from_text(
    text: &str,
    document_uri: Option<String>,
    rules: &mut Vec<SwrlRuleListItem>,
    seen: &mut std::collections::BTreeSet<String>,
) {
    let ontology_iri = ontology_iri_from_turtle(text);
    for (index, rule) in strixonomy_swrl::rules_from_turtle_document(text).into_iter().enumerate() {
        let summary = rule.summary(index);
        let rule_json = serde_json::to_string(&rule).ok();
        // Dedupe on document + canonical rule JSON so distinct bodies that share
        // id/summary label are not collapsed (#362).
        let key = format!(
            "{}|{}",
            document_uri.as_deref().unwrap_or(""),
            rule_json.as_deref().unwrap_or(summary.id.as_str())
        );
        if !seen.insert(key) {
            continue;
        }
        rules.push(SwrlRuleListItem {
            id: summary.id,
            label: summary.label,
            body_count: summary.body_count,
            head_count: summary.head_count,
            enabled: summary.enabled,
            rule_json,
            document_uri: document_uri.clone(),
            ontology_iri: ontology_iri.clone(),
        });
    }
}

pub fn handle_list_swrl_rules(state: &ServerState) -> Result<ListSwrlRulesResult, LspErrorPayload> {
    let mut rules = Vec::new();
    let mut seen = std::collections::BTreeSet::new();

    for (path, text) in state.open_documents_for_reasoner() {
        let uri = path_to_uri(&path);
        collect_swrl_rules_from_text(text.as_str(), Some(uri), &mut rules, &mut seen);
    }

    if let Some(docs) = state.with_catalog(|catalog| catalog.data().documents.clone()) {
        let open = state.open_documents_for_reasoner();
        for doc in docs {
            if doc.path.extension().is_none_or(|ext| ext != "ttl") {
                continue;
            }
            if open.contains_key(&doc.path) {
                continue;
            }
            if let Ok(canonical) = doc.path.canonicalize() {
                if open.contains_key(&canonical) {
                    continue;
                }
            }
            let Ok(text) = std::fs::read_to_string(&doc.path) else {
                continue;
            };
            let uri = path_to_uri(&doc.path);
            collect_swrl_rules_from_text(&text, Some(uri), &mut rules, &mut seen);
        }
    }

    Ok(ListSwrlRulesResult { rules })
}

pub fn handle_validate_swrl_rule(
    params: ValidateSwrlRuleParams,
) -> Result<ValidateSwrlRuleResult, LspErrorPayload> {
    let rule = strixonomy_swrl::parse_swrl_rule_json(&params.rule_json)
        .map_err(|e| LspErrorPayload::invalid_params(format!("invalid SWRL rule JSON: {e}")))?;
    Ok(ValidateSwrlRuleResult { diagnostics: strixonomy_swrl::validate_rule(&rule) })
}

pub fn handle_parse_swrl_rule(
    params: ParseSwrlRuleParams,
) -> Result<ParseSwrlRuleResult, LspErrorPayload> {
    let rule = strixonomy_swrl::parse_swrl_rule_json(&params.rule_json)
        .map_err(|e| LspErrorPayload::invalid_params(format!("invalid SWRL rule JSON: {e}")))?;
    let diagnostics = strixonomy_swrl::validate_rule(&rule);
    Ok(ParseSwrlRuleResult { rule, diagnostics })
}

pub fn handle_run_reasoner_lsp(
    state: &ServerState,
    params: RunReasonerParams,
    message_rx: &Receiver<Message>,
    deferred: &mut VecDeque<Message>,
    run_generation: u64,
) -> Result<RunReasonerResult, LspErrorPayload> {
    let profile = ReasonerId::parse(&params.profile)
        .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;

    let (input, cached) = {
        let ops_lock = state.ops_lock();
        let _guard =
            ops_lock.lock().map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;

        let input = load_reasoner_input(state)?;
        let cached = state
            .with_reasoner_cache(|cache| cache.get(&input.content_hash, profile).cloned())
            .flatten();
        (input, cached)
    };

    if let Some(cached) = cached {
        if !state.reasoner_run_is_current(run_generation) {
            return Err(LspErrorPayload::reasoner_failed("Reasoner run cancelled".to_string()));
        }
        let ops_lock = state.ops_lock();
        let _guard =
            ops_lock.lock().map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;
        let snapshot = cached.snapshot.clone();
        state.set_reasoner_snapshot(snapshot.clone());
        return Ok(run_reasoner_result_from_classification(&cached.classification, snapshot));
    }

    let auto_detect = params.auto_detect;
    let input_for_classify = input.clone();
    let cancel_flag = Arc::new(AtomicBool::new(false));
    state.set_reasoner_cancel_flag(Some(Arc::clone(&cancel_flag)));
    let classify_cancel = Arc::clone(&cancel_flag);
    let classify_handle = thread::spawn(move || {
        strixonomy_reasoner::install_cancel_flag(classify_cancel);
        reasoner_classify_test_pause();
        let result = classify(profile, &input_for_classify, auto_detect);
        strixonomy_reasoner::clear_cancel_flag();
        result
    });

    while !classify_handle.is_finished() {
        if !state.reasoner_run_is_current(run_generation) {
            break;
        }
        drain_reasoner_cancel_notifications(message_rx, deferred, state);
        thread::sleep(Duration::from_millis(5));
    }

    let classification = classify_handle
        .join()
        .map_err(|_| LspErrorPayload::reasoner_failed("reasoner thread panicked".to_string()))?
        .map_err(|e| {
            state.set_reasoner_cancel_flag(None);
            if matches!(e, strixonomy_reasoner::ReasonerError::Cancelled) {
                LspErrorPayload::reasoner_failed("Reasoner run cancelled".to_string())
            } else {
                LspErrorPayload::reasoner_failed(e.to_string())
            }
        })?;

    if !state.reasoner_run_is_current(run_generation) || cancel_flag.load(Ordering::Acquire) {
        state.set_reasoner_cancel_flag(None);
        return Err(LspErrorPayload::reasoner_failed("Reasoner run cancelled".to_string()));
    }

    let ops_lock = state.ops_lock();
    let _guard = ops_lock.lock().map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;

    let mut snapshot = state
        .reasoner_cache_mut(|cache| {
            cache.store_classification(input.clone(), profile, classification.clone())
        })
        .unwrap_or_else(|| ReasonerSnapshot::from(classification.clone()));

    // Keep cancel armed through ABox enrich (realize can be O(I×C)).
    strixonomy_reasoner::install_cancel_flag(Arc::clone(&cancel_flag));
    drain_reasoner_cancel_notifications(message_rx, deferred, state);

    let enrich_cancelled = |flag: &AtomicBool, state: &ServerState| {
        flag.load(Ordering::Acquire) || !state.reasoner_run_is_current(run_generation)
    };

    if enrich_cancelled(&cancel_flag, state) {
        strixonomy_reasoner::clear_cancel_flag();
        state.set_reasoner_cancel_flag(None);
        return Err(LspErrorPayload::reasoner_failed("Reasoner run cancelled".to_string()));
    }

    // Enrich with ABox realization / consistency (best-effort; keep TBox snapshot on failure).
    // For SWRL classify, consistency/realize must use the post-materialization ontology (#358).
    let enrich_ontology = if classification.profile_used == "swrl" {
        match strixonomy_reasoner::prepare_swrl_ontology(&input) {
            Ok((ont, _)) => ont,
            Err(_) => input.ontology.clone(),
        }
    } else {
        input.ontology.clone()
    };

    match strixonomy_reasoner::check_full_consistency(
        &enrich_ontology,
        profile,
        &classification.unsatisfiable,
    ) {
        Ok(detail) => {
            snapshot.consistency = Some(detail.clone());
            snapshot.consistent = detail.consistent;
        }
        Err(strixonomy_reasoner::ReasonerError::Cancelled) => {
            strixonomy_reasoner::clear_cancel_flag();
            state.set_reasoner_cancel_flag(None);
            return Err(LspErrorPayload::reasoner_failed("Reasoner run cancelled".to_string()));
        }
        Err(_) => {}
    }

    drain_reasoner_cancel_notifications(message_rx, deferred, state);
    if enrich_cancelled(&cancel_flag, state) {
        strixonomy_reasoner::clear_cancel_flag();
        state.set_reasoner_cancel_flag(None);
        return Err(LspErrorPayload::reasoner_failed("Reasoner run cancelled".to_string()));
    }

    let realization = match strixonomy_reasoner::realize(profile, &{
        let mut enrich_input = input.clone();
        enrich_input.ontology = enrich_ontology.clone();
        enrich_input
    }) {
        Ok(r) => Some(r),
        Err(strixonomy_reasoner::ReasonerError::Cancelled) => {
            strixonomy_reasoner::clear_cancel_flag();
            state.set_reasoner_cancel_flag(None);
            return Err(LspErrorPayload::reasoner_failed("Reasoner run cancelled".to_string()));
        }
        Err(_) => None,
    };
    if let Some(ref realization) = realization {
        snapshot.realization = Some(realization.clone());
        if realization.truncated {
            snapshot.warnings.push(strixonomy_reasoner::ReasonerWarning {
                code: "realization_truncated".into(),
                message: if realization.entailment_errors > 0 {
                    format!(
                        "realization incomplete: truncated and {} entailment check error(s)",
                        realization.entailment_errors
                    )
                } else {
                    "realization incomplete: individual or entailment budget reached".into()
                },
                suggested_profile: None,
            });
        }
        drain_reasoner_cancel_notifications(message_rx, deferred, state);
        if enrich_cancelled(&cancel_flag, state) {
            strixonomy_reasoner::clear_cancel_flag();
            state.set_reasoner_cancel_flag(None);
            return Err(LspErrorPayload::reasoner_failed("Reasoner run cancelled".to_string()));
        }
        match strixonomy_reasoner::inferred_assertions_from_realization(
            &enrich_ontology,
            profile,
            realization,
        ) {
            Ok(assertions) => snapshot.inferred_assertions = Some(assertions),
            Err(strixonomy_reasoner::ReasonerError::Cancelled) => {
                strixonomy_reasoner::clear_cancel_flag();
                state.set_reasoner_cancel_flag(None);
                return Err(LspErrorPayload::reasoner_failed("Reasoner run cancelled".to_string()));
            }
            Err(_) => {}
        }
    }

    strixonomy_reasoner::clear_cancel_flag();
    state.set_reasoner_cancel_flag(None);
    // Persist enriched snapshot so cache hits keep realization/consistency (#355).
    let _ = state.reasoner_cache_mut(|cache| {
        cache.update_snapshot(&input.content_hash, profile, snapshot.clone());
    });
    state.set_reasoner_snapshot(snapshot.clone());
    state.clear_reasoner_dirty();

    Ok(run_reasoner_result_from_classification(&classification, snapshot))
}

#[cfg(test)]
fn reasoner_classify_test_pause() {
    use crate::state::{TEST_REASONER_CLASSIFY_IN_PAUSE, TEST_REASONER_CLASSIFY_PAUSE_MS};
    use std::sync::atomic::Ordering;

    let pause_ms = TEST_REASONER_CLASSIFY_PAUSE_MS.load(Ordering::SeqCst);
    if pause_ms > 0 {
        TEST_REASONER_CLASSIFY_IN_PAUSE.store(true, Ordering::SeqCst);
        thread::sleep(Duration::from_millis(pause_ms));
        TEST_REASONER_CLASSIFY_IN_PAUSE.store(false, Ordering::SeqCst);
    }
}

#[cfg(not(test))]
fn reasoner_classify_test_pause() {}

#[derive(Debug, Deserialize)]
struct CancelParams {
    id: RequestId,
}

/// Poll the LSP inbox for `$/cancelRequest` while classify runs.
///
/// Non-cancel messages must not be dropped: the main loop is blocked in this wait, so
/// anything else is parked in `deferred` and processed when the dispatcher resumes.
fn drain_reasoner_cancel_notifications(
    rx: &Receiver<Message>,
    deferred: &mut VecDeque<Message>,
    state: &ServerState,
) {
    while let Ok(msg) = rx.try_recv() {
        match msg {
            Message::Notification(notif) if notif.method == "$/cancelRequest" => {
                if let Ok(params) = serde_json::from_value::<CancelParams>(notif.params) {
                    state.cancel_reasoner_request(params.id);
                }
            }
            other => deferred.push_back(other),
        }
    }
}

pub fn handle_get_explanation(
    state: &ServerState,
    params: GetExplanationParams,
) -> Result<GetExplanationResult, LspErrorPayload> {
    let ops_lock = state.ops_lock();
    let _guard = ops_lock.lock().map_err(|e| LspErrorPayload::explanation_failed(e.to_string()))?;

    let profile = ReasonerId::parse(&params.profile)
        .map_err(|e| LspErrorPayload::explanation_failed(e.to_string()))?;

    let fresh_input = load_reasoner_input(state)?;
    let input = state
        .with_reasoner_cache(|cache| {
            cache.get(&fresh_input.content_hash, profile).map(|c| c.input.clone())
        })
        .flatten()
        .unwrap_or(fresh_input);

    let profile_key = profile.as_str().to_string();
    if let Some(cached) =
        state.get_cached_explanation(&input.content_hash, &profile_key, &params.class_iri)
    {
        return Ok(GetExplanationResult {
            class_iri: cached.class_iri,
            steps: cached.steps,
            text: cached.text,
            alternatives: Vec::new(),
            indexed_at: state.indexed_at().unwrap_or(0),
            content_hash: input.content_hash,
        });
    }

    let request = ExplanationRequest { class_iri: params.class_iri.clone() };
    let mut alternatives = explain_alternatives(profile, &input, &request, 5)
        .map_err(|e| LspErrorPayload::explanation_failed(e.to_string()))?;
    let primary = alternatives.remove(0);
    state.put_cached_explanation(
        &input.content_hash,
        &profile_key,
        &params.class_iri,
        primary.clone(),
    );

    Ok(GetExplanationResult {
        class_iri: primary.class_iri,
        steps: primary.steps,
        text: primary.text,
        alternatives,
        indexed_at: state.indexed_at().unwrap_or(0),
        content_hash: input.content_hash,
    })
}

pub fn handle_check_instance(
    state: &ServerState,
    params: CheckInstanceParams,
) -> Result<CheckInstanceResult, LspErrorPayload> {
    let ops_lock = state.ops_lock();
    let _guard = ops_lock.lock().map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;
    let profile = ReasonerId::parse(&params.profile)
        .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;
    let input = load_reasoner_input(state)?;
    let result = strixonomy_reasoner::check_instance(
        profile,
        &input,
        &params.individual_iri,
        &params.class_iri,
    )
    .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;
    Ok(CheckInstanceResult {
        individual_iri: result.individual_iri,
        class_iri: result.class_iri,
        entailed: result.entailed,
        profile_used: result.profile_used,
        duration_ms: result.duration_ms,
    })
}

fn run_reasoner_result_from_classification(
    classification: &strixonomy_reasoner::ClassificationResult,
    snapshot: ReasonerSnapshot,
) -> RunReasonerResult {
    RunReasonerResult {
        profile_used: classification.profile_used.clone(),
        consistent: classification.consistent,
        unsatisfiable: classification.unsatisfiable.clone(),
        inferred_edge_count: classification.inferred.edges.len(),
        new_inferences: classification.new_inferences.clone(),
        warnings: classification.warnings.clone(),
        duration_ms: classification.duration_ms,
        snapshot,
    }
}

fn load_reasoner_input(
    state: &ServerState,
) -> Result<strixonomy_reasoner::ReasonerInput, LspErrorPayload> {
    let roots = state.workspace_roots();
    if roots.is_empty() {
        return Err(LspErrorPayload::not_indexed());
    }
    let workspace = state
        .indexed_workspace()
        .or_else(|| state.workspace_root())
        .ok_or_else(LspErrorPayload::not_indexed)?;
    let overrides = state.open_documents_for_reasoner();
    WorkspaceInputLoader::new(&workspace)
        .scan_roots(roots)
        .document_overrides(overrides)
        .load()
        .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))
}

fn resolve_namespaces_for_manchester(
    state: &ServerState,
    params: &ParseManchesterParams,
) -> Result<std::collections::BTreeMap<String, String>, LspErrorPayload> {
    if let Some(uri) = &params.document_uri {
        let path = state
            .resolve_lsp_document_uri(uri)
            .map_err(|e| LspErrorPayload::manchester_invalid(e.to_string()))?;
        if let Some(ns) = state
            .with_catalog(|catalog| {
                catalog
                    .data()
                    .documents
                    .iter()
                    .find(|d| strixonomy_core::paths_refer_to_same(&d.path, &path))
                    .map(|d| d.namespaces.clone())
            })
            .flatten()
        {
            return Ok(ns);
        }
    }
    state
        .with_catalog(|catalog| {
            catalog
                .data()
                .documents
                .iter()
                .find(|d| {
                    params.entity_iri.as_ref().is_some_and(|iri| {
                        catalog.entity_document(iri).is_some_and(|ed| ed.id == d.id)
                    })
                })
                .map(|d| d.namespaces.clone())
        })
        .flatten()
        .ok_or_else(LspErrorPayload::not_indexed)
}

fn resolve_namespaces_for_dl_query(
    state: &ServerState,
    document_uri: Option<&str>,
) -> Result<std::collections::BTreeMap<String, String>, LspErrorPayload> {
    if let Some(uri) = document_uri {
        let path = state
            .resolve_lsp_document_uri(uri)
            .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))?;
        if let Some(ns) = state
            .with_catalog(|catalog| {
                catalog
                    .data()
                    .documents
                    .iter()
                    .find(|d| strixonomy_core::paths_refer_to_same(&d.path, &path))
                    .map(|d| d.namespaces.clone())
            })
            .flatten()
        {
            return Ok(ns);
        }
    }
    state
        .with_catalog(|catalog| {
            let mut namespaces = std::collections::BTreeMap::new();
            for doc in &catalog.data().documents {
                for (prefix, iri) in &doc.namespaces {
                    namespaces.entry(prefix.clone()).or_insert_with(|| iri.clone());
                }
            }
            namespaces
        })
        .ok_or_else(LspErrorPayload::not_indexed)
}

fn build_manchester_completions(state: &ServerState) -> ManchesterCompletions {
    const DATATYPES: &[&str] = &[
        "xsd:string",
        "xsd:integer",
        "xsd:decimal",
        "xsd:boolean",
        "xsd:dateTime",
        "xsd:float",
        "xsd:double",
    ];

    state
        .with_catalog(|catalog| {
            let mut classes = Vec::new();
            let mut object_properties = Vec::new();
            let mut data_properties = Vec::new();
            for entity in &catalog.data().entities {
                match entity.kind {
                    EntityKind::Class => classes.push(entity.iri.clone()),
                    EntityKind::ObjectProperty => object_properties.push(entity.iri.clone()),
                    EntityKind::DataProperty => data_properties.push(entity.iri.clone()),
                    _ => {}
                }
            }
            classes.sort();
            object_properties.sort();
            data_properties.sort();
            ManchesterCompletions {
                classes,
                object_properties,
                data_properties,
                datatypes: DATATYPES.iter().map(|s| s.to_string()).collect(),
            }
        })
        .unwrap_or(ManchesterCompletions {
            classes: Vec::new(),
            object_properties: Vec::new(),
            data_properties: Vec::new(),
            datatypes: DATATYPES.iter().map(|s| s.to_string()).collect(),
        })
}

pub fn handle_hover(state: &ServerState, params: HoverParams) -> Option<Hover> {
    let path = lsp_document_path(state, &params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position;
    let content = state.document_text(&path)?;
    let iri = iri_at_position(&content, position)?;

    state.with_catalog(|catalog| {
        let detail = catalog.entity_detail(&iri)?;
        let mut md = format!(
            "**{}** (`{}`)\n\n",
            escape_markdown(&detail.entity.short_name),
            escape_markdown(detail.entity.kind.as_str())
        );
        if !detail.entity.labels.is_empty() {
            md.push_str(&format!(
                "Labels: {}\n\n",
                detail
                    .entity
                    .labels
                    .iter()
                    .map(|l| strixonomy_owl::linkify_markdown_text(l))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !detail.entity.comments.is_empty() {
            md.push_str(&format!(
                "Comments: {}\n\n",
                detail
                    .entity
                    .comments
                    .iter()
                    .map(|c| strixonomy_owl::linkify_markdown_text(c))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !detail.parents.is_empty() {
            md.push_str(&format!(
                "Parents: {}\n",
                detail.parents.iter().map(|p| escape_markdown(p)).collect::<Vec<_>>().join(", ")
            ));
        }
        if detail.entity.deprecated {
            md.push_str("\n*deprecated*");
        }
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: md,
            }),
            range: None,
        })
    })?
}

#[allow(deprecated)]
pub fn handle_document_symbol(
    state: &ServerState,
    params: DocumentSymbolParams,
) -> Option<DocumentSymbolResponse> {
    let path = lsp_document_path(state, &params.text_document.uri)?;
    // Clone under catalog lock; never call document_text while holding it (non-reentrant RwLock).
    let entities: Vec<strixonomy_core::Entity> = state.with_catalog(|catalog| {
        catalog.entities_in_document(&path).into_iter().cloned().collect()
    })?;
    if entities.is_empty() {
        return None;
    }
    let doc_text = state.document_text(&path);
    let symbols: Vec<DocumentSymbol> = entities
        .into_iter()
        .map(|e| {
            let range = entity_source_range(doc_text.as_deref(), &e);
            DocumentSymbol {
                name: e.short_name.clone(),
                detail: Some(e.iri.clone()),
                kind: entity_kind_to_symbol_kind(e.kind),
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            }
        })
        .collect();
    Some(DocumentSymbolResponse::Nested(symbols))
}

#[allow(deprecated)]
pub fn handle_workspace_symbol(
    state: &ServerState,
    params: WorkspaceSymbolParams,
) -> Option<WorkspaceSymbolResponse> {
    let query = params.query.to_ascii_lowercase();
    let entries: Vec<(strixonomy_core::Entity, std::path::PathBuf)> =
        state.with_catalog(|catalog| {
            catalog
                .data()
                .entities
                .iter()
                .filter(|e| {
                    query.is_empty()
                        || e.short_name.to_ascii_lowercase().contains(&query)
                        || e.iri.to_ascii_lowercase().contains(&query)
                        || e.labels.iter().any(|l| l.to_ascii_lowercase().contains(&query))
                })
                .filter_map(|e| {
                    let doc = catalog.entity_document(&e.iri)?;
                    Some((e.clone(), doc.path.clone()))
                })
                .collect()
        })?;
    let symbols: Vec<SymbolInformation> = entries
        .into_iter()
        .filter_map(|(e, path)| {
            let uri = path_to_lsp_uri(&path)?;
            let doc_text = state.document_text(&path);
            let range = entity_source_range(doc_text.as_deref(), &e);
            Some(SymbolInformation {
                name: e.short_name.clone(),
                kind: entity_kind_to_symbol_kind(e.kind),
                tags: None,
                deprecated: None,
                location: Location { uri, range },
                container_name: None,
            })
        })
        .collect();
    Some(WorkspaceSymbolResponse::Flat(symbols))
}

pub fn handle_goto_definition(
    state: &ServerState,
    params: GotoDefinitionParams,
) -> Option<GotoDefinitionResponse> {
    let path = lsp_document_path(state, &params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position;
    let content = state.document_text(&path)?;
    let iri = iri_at_position(&content, position)?;

    let source = state.with_catalog(|catalog| catalog.find_source_location(&iri))??;
    let uri = path_to_lsp_uri(&source.path)?;
    let line_text = state.document_text(&source.path).and_then(|text| {
        text.lines().nth(source.line.saturating_sub(1) as usize).map(|s| s.to_string())
    });
    let character = line_text
        .as_deref()
        .map(|l| byte_col_to_utf16(l, source.column as usize))
        .unwrap_or(source.column as u32);
    let range = Range {
        start: Position { line: (source.line.saturating_sub(1)) as u32, character },
        end: Position {
            line: (source.line.saturating_sub(1)) as u32,
            character: character.saturating_add(1),
        },
    };
    Some(GotoDefinitionResponse::Scalar(Location { uri, range }))
}

pub fn handle_find_usages(
    state: &ServerState,
    params: FindUsagesParams,
) -> Result<FindUsagesResult, LspErrorPayload> {
    state
        .with_catalog_and_overrides(|catalog, overrides| {
            let usages = find_usages_with_overrides(catalog, &params.iri, overrides);
            Ok(FindUsagesResult { usages: usages.into_iter().map(usage_to_summary).collect() })
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_preview_refactor(
    state: &ServerState,
    params: PreviewRefactorParams,
) -> Result<PreviewRefactorResult, LspErrorPayload> {
    let roots = state.workspace_roots();
    if roots.is_empty() {
        return Err(LspErrorPayload::refactor_failed("workspace not initialized".to_string()));
    }
    state
        .with_catalog_and_overrides(|catalog, overrides| {
            let plan = preview_refactor(catalog, &params.request, overrides, &roots)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))?;
            validate_refactor_plan_any_roots(&roots, &plan)
                .map_err(LspErrorPayload::refactor_failed)?;
            Ok(PreviewRefactorResult { plan })
        })
        .ok_or_else(LspErrorPayload::not_indexed)?
}

pub fn handle_apply_refactor(
    state: &ServerState,
    index_worker: &IndexWorker,
    params: ApplyRefactorParams,
) -> Result<ApplyRefactorResult, LspErrorPayload> {
    let roots = state.workspace_roots();
    if roots.is_empty() {
        return Err(LspErrorPayload::refactor_failed("workspace not initialized".to_string()));
    }

    let (files_written, server_plan) = {
        let ops_lock = state.ops_lock();
        let _guard =
            ops_lock.lock().map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))?;

        let server_plan = state
            .with_catalog_and_overrides(|catalog, overrides| {
                let plan = preview_refactor(catalog, &params.request, overrides, &roots)
                    .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))?;
                validate_refactor_plan_any_roots(&roots, &plan)
                    .map_err(LspErrorPayload::refactor_failed)?;
                Ok(plan)
            })
            .ok_or_else(LspErrorPayload::not_indexed)??;

        if !plans_equivalent(&server_plan, &params.plan) {
            return Err(LspErrorPayload::refactor_failed(
                "submitted plan does not match server preview".to_string(),
            ));
        }

        let overrides = state.open_document_overrides();
        let files_written = apply_refactor_plan_checked_with_overrides(
            &server_plan,
            params.preview_only,
            Some(&roots),
            Some(&overrides),
        )
        .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))?;

        if !params.preview_only {
            let mut buffer_snapshots: Vec<(std::path::PathBuf, String)> = Vec::new();
            for change in &server_plan.changes {
                if change.preview_text != change.original_text
                    && state.is_document_open(&change.path)
                {
                    if let Some(text) = state.document_text(&change.path) {
                        buffer_snapshots.push((change.path.clone(), text));
                    }
                }
            }
            for change in &server_plan.changes {
                if change.preview_text != change.original_text
                    && state.is_document_open(&change.path)
                {
                    if let Err(e) =
                        state.set_document_text(change.path.clone(), change.preview_text.clone())
                    {
                        let mut rollback_errors = Vec::new();
                        for rollback in &server_plan.changes {
                            if rollback.preview_text != rollback.original_text {
                                if let Err(re) =
                                    atomic_write_for_path(&rollback.path, &rollback.original_text)
                                {
                                    rollback_errors
                                        .push(format!("{}: {re}", rollback.path.display()));
                                }
                            }
                        }
                        for (path, text) in buffer_snapshots {
                            let _ = state.set_document_text(path, text);
                        }
                        if !rollback_errors.is_empty() {
                            return Err(LspErrorPayload::refactor_failed(format!(
                                "buffer update failed ({e}); disk rollback also failed: {}",
                                rollback_errors.join("; ")
                            )));
                        }
                        return Err(LspErrorPayload::refactor_failed(e));
                    }
                }
            }
        }
        (files_written, server_plan)
    };

    if params.preview_only {
        return Ok(ApplyRefactorResult {
            files_written: 0,
            reindex_warning: None,
            workspace_edit: None,
        });
    }

    let reindex_warning = match state.effective_index_root() {
        Some(root) => match index_worker.enqueue_sync(root) {
            Ok(_) => None,
            Err(e) => Some(format!("refactor applied but reindex failed: {e}")),
        },
        None => Some("refactor applied but workspace root unknown".to_string()),
    };
    let workspace_edit = plan_to_workspace_edit(state, &server_plan);
    Ok(ApplyRefactorResult { files_written, reindex_warning, workspace_edit })
}

pub fn handle_references(state: &ServerState, params: ReferenceParams) -> Option<Vec<Location>> {
    let path = lsp_document_path(state, &params.text_document_position.text_document.uri)?;
    let position = params.text_document_position.position;
    let content = state.document_text(&path)?;
    let iri = iri_at_position(&content, position)?;
    let usages = state.with_catalog_and_overrides(|catalog, overrides| {
        find_usages_with_overrides(catalog, &iri, overrides)
    })?;
    let locations: Vec<Location> =
        usages.into_iter().filter_map(|u| usage_to_location(state, &u)).collect();
    Some(locations)
}

pub fn handle_rename(
    state: &ServerState,
    params: RenameParams,
) -> Result<Option<WorkspaceEdit>, LspErrorPayload> {
    let path = lsp_document_path(state, &params.text_document_position.text_document.uri)
        .ok_or_else(LspErrorPayload::not_indexed)?;
    let position = params.text_document_position.position;
    let content = state
        .document_text(&path)
        .ok_or_else(|| LspErrorPayload::not_found("document not available"))?;
    let from_iri = iri_at_position(&content, position)
        .ok_or_else(|| LspErrorPayload::not_found("no IRI at cursor"))?;
    let to_iri = derive_renamed_iri(&from_iri, &params.new_name, &content);
    let overrides = state.open_document_overrides();
    let plan = state
        .with_catalog(|catalog| {
            preview_rename_iri(catalog, &from_iri, &to_iri, &overrides)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))
        })
        .ok_or_else(LspErrorPayload::not_indexed)??;
    let edit = plan_to_workspace_edit(state, &plan).ok_or_else(|| {
        LspErrorPayload::refactor_failed("rename produced no file changes".to_string())
    })?;
    Ok(Some(edit))
}

pub fn handle_prepare_rename(
    state: &ServerState,
    params: TextDocumentPositionParams,
) -> Option<PrepareRenameResponse> {
    let path = lsp_document_path(state, &params.text_document.uri)?;
    let content = state.document_text(&path)?;
    let (range, placeholder) = rename_range_at_position(&content, params.position)?;
    Some(PrepareRenameResponse::RangeWithPlaceholder { range, placeholder })
}

fn validate_refactor_plan_any_roots(
    roots: &[PathBuf],
    plan: &strixonomy_refactor::RefactorPlan,
) -> Result<(), String> {
    for change in &plan.changes {
        validate_workspace_scope_any(&change.path, roots)?;
    }
    Ok(())
}

fn usage_to_summary(u: strixonomy_refactor::Usage) -> UsageSummary {
    UsageSummary {
        iri: u.iri,
        referenced_iri: u.referenced_iri,
        file: u.file.display().to_string(),
        line: u.line,
        column: u.column,
        kind: format!("{:?}", u.kind).to_ascii_lowercase(),
        context: u.context,
    }
}

fn derive_renamed_iri(from_iri: &str, new_name: &str, document_content: &str) -> String {
    if new_name.contains("://") {
        return new_name.to_string();
    }
    if let Some(expanded) = expand_iri_token(document_content, new_name) {
        if expanded.starts_with("http://") || expanded.starts_with("https://") {
            return expanded;
        }
    }
    if let Some((base, _)) = from_iri.rsplit_once('#') {
        return format!("{base}#{new_name}");
    }
    if let Some((base, _)) = from_iri.rsplit_once('/') {
        return format!("{base}/{new_name}");
    }
    new_name.to_string()
}

fn usage_to_location(state: &ServerState, usage: &strixonomy_refactor::Usage) -> Option<Location> {
    let uri = path_to_lsp_uri(&usage.file)?;
    let line_idx = usage.line.unwrap_or(1).saturating_sub(1) as u32;
    let line_text = state
        .document_text(&usage.file)
        .and_then(|text| text.lines().nth(line_idx as usize).map(|s| s.to_string()))?;
    let line = line_text.as_str();
    let byte_col = usage.column.unwrap_or(0) as usize;
    let (start_byte, end_byte) = if byte_col < line.len() {
        let (start, end) = token_byte_range_at(line, byte_col);
        (start, end)
    } else {
        (byte_col, byte_col.saturating_add(1))
    };
    let start_char = byte_col_to_utf16(line, start_byte);
    let end_char = byte_col_to_utf16(line, end_byte);
    Some(Location {
        uri,
        range: Range {
            start: Position { line: line_idx, character: start_char },
            end: Position { line: line_idx, character: end_char.max(start_char.saturating_add(1)) },
        },
    })
}

fn token_byte_range_at(line: &str, byte_col: usize) -> (usize, usize) {
    let bytes = line.as_bytes();
    let mut start = byte_col.min(bytes.len());
    let mut end = byte_col.min(bytes.len());
    while start > 0 && is_iri_char(bytes[start - 1]) {
        start -= 1;
    }
    while end < bytes.len() && is_iri_char(bytes[end]) {
        end += 1;
    }
    if start == end && byte_col < bytes.len() {
        end = (byte_col + 1).min(bytes.len());
    }
    (start, end)
}

fn full_document_workspace_edit(
    state: &ServerState,
    path: &std::path::Path,
    new_text: &str,
) -> Option<WorkspaceEdit> {
    full_document_workspace_edit_labeled(state, path, new_text, "Strixonomy semantic edit")
}

/// Format-aware atomic write used by apply and rollback paths.
fn atomic_write_for_format(
    format: OntologyFormat,
    path: &std::path::Path,
    contents: &str,
) -> Result<(), String> {
    if format == OntologyFormat::Obo {
        strixonomy_obo::atomic_write(path, contents).map_err(|e| e.to_string())
    } else {
        strixonomy_owl::atomic_write(path, contents).map_err(|e| e.to_string())
    }
}

fn atomic_write_for_path(path: &std::path::Path, contents: &str) -> Result<(), String> {
    let format =
        OntologyFormat::from_extension(path.extension().and_then(|e| e.to_str()).unwrap_or(""));
    atomic_write_for_format(format, path, contents)
}

fn full_document_workspace_edit_labeled(
    state: &ServerState,
    path: &std::path::Path,
    new_text: &str,
    label: &str,
) -> Option<WorkspaceEdit> {
    use lsp_types::{ChangeAnnotation, ChangeAnnotationIdentifier};
    use std::collections::HashMap;
    let uri = path_to_lsp_uri(path)?;
    let version = state.document_version(path);
    let annotation_id = ChangeAnnotationIdentifier::from("strixonomy.edit");
    let mut annotations = HashMap::new();
    annotations.insert(
        annotation_id.clone(),
        ChangeAnnotation {
            label: label.to_string(),
            needs_confirmation: Some(false),
            description: Some(label.to_string()),
        },
    );
    Some(WorkspaceEdit {
        changes: None,
        document_changes: Some(DocumentChanges::Edits(vec![TextDocumentEdit {
            text_document: lsp_types::OptionalVersionedTextDocumentIdentifier { uri, version },
            edits: vec![OneOf::Right(lsp_types::AnnotatedTextEdit {
                text_edit: TextEdit {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: u32::MAX, character: 0 },
                    },
                    new_text: new_text.to_string(),
                },
                annotation_id,
            })],
        }])),
        change_annotations: Some(annotations),
    })
}

fn plan_to_workspace_edit(
    state: &ServerState,
    plan: &strixonomy_refactor::RefactorPlan,
) -> Option<WorkspaceEdit> {
    use lsp_types::{ChangeAnnotation, ChangeAnnotationIdentifier};
    use std::collections::HashMap;
    let mut document_changes = Vec::new();
    let annotation_id = ChangeAnnotationIdentifier::from("strixonomy.refactor");
    for change in &plan.changes {
        if change.preview_text == change.original_text {
            continue;
        }
        let uri = path_to_lsp_uri(&change.path)?;
        let version = state.document_version(&change.path);
        document_changes.push(TextDocumentEdit {
            text_document: lsp_types::OptionalVersionedTextDocumentIdentifier { uri, version },
            edits: vec![OneOf::Right(lsp_types::AnnotatedTextEdit {
                text_edit: TextEdit {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: u32::MAX, character: 0 },
                    },
                    new_text: change.preview_text.clone(),
                },
                annotation_id: annotation_id.clone(),
            })],
        });
    }
    if document_changes.is_empty() {
        return None;
    }
    let mut annotations = HashMap::new();
    annotations.insert(
        annotation_id,
        ChangeAnnotation {
            label: "Strixonomy refactor".to_string(),
            needs_confirmation: Some(false),
            description: Some("Semantic refactor applied by Strixonomy".to_string()),
        },
    );
    Some(WorkspaceEdit {
        changes: None,
        document_changes: Some(DocumentChanges::Edits(document_changes)),
        change_annotations: Some(annotations),
    })
}

fn parse_custom_params<T: serde::de::DeserializeOwned>(
    params: Option<Value>,
) -> Result<T, LspErrorPayload> {
    serde_json::from_value(params.unwrap_or(Value::Null))
        .map_err(|e| LspErrorPayload::invalid_params(format!("invalid params: {e}")))
}

/// Accept `strixonomy/*` custom method names.
pub fn normalize_custom_method(method: &str) -> Option<&str> {
    method.strip_prefix("strixonomy/")
}

#[cfg(test)]
mod normalize_custom_method_tests {
    use super::normalize_custom_method;

    #[test]
    fn accepts_primary_prefix() {
        assert_eq!(normalize_custom_method("strixonomy/query"), Some("query"));
        assert_eq!(normalize_custom_method("ontocore/query"), None);
        assert_eq!(normalize_custom_method("other/query"), None);
    }
}

pub fn handle_custom_request(
    state: &ServerState,
    index_worker: &IndexWorker,
    method: &str,
    params: Option<Value>,
) -> Result<Value, LspErrorPayload> {
    let method = normalize_custom_method(method).ok_or_else(|| {
        LspErrorPayload::invalid_params(format!("unknown custom method: {method}"))
    })?;
    match method {
        "indexWorkspace" => {
            let params: IndexWorkspaceParams = parse_custom_params(params)?;
            let result = handle_index_workspace(state, index_worker, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "getCatalogSnapshot" => {
            let result = handle_get_catalog_snapshot(state)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "getEntity" => {
            let params: GetEntityParams = parse_custom_params(params)?;
            let result = handle_get_entity(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "getGraph" => {
            let params: GraphRequest = parse_custom_params(params)?;
            let result = handle_get_graph(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "applyAxiomPatch" => {
            let params: ApplyAxiomPatchParams = parse_custom_params(params)?;
            let result = handle_apply_axiom_patch(state, index_worker, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "query" => {
            let params: QueryParams = parse_custom_params(params)?;
            let result = handle_query(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "listSqlSchema" => {
            let result = handle_list_sql_schema(state)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "sparql" => {
            let params: SparqlParams = parse_custom_params(params)?;
            let result = handle_sparql(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "dlQuery" => {
            let params: DlQueryParams = parse_custom_params(params)?;
            let result = handle_dl_query(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "search" => {
            let params: SearchParams = parse_custom_params(params)?;
            let result = handle_search(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "parseManchester" => {
            let params: ParseManchesterParams = parse_custom_params(params)?;
            let result = handle_parse_manchester(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "listSwrlRules" => {
            let result = handle_list_swrl_rules(state)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "validateSwrlRule" => {
            let params: ValidateSwrlRuleParams = parse_custom_params(params)?;
            let result = handle_validate_swrl_rule(params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::invalid_params(e.to_string()))
        }
        "parseSwrlRule" => {
            let params: ParseSwrlRuleParams = parse_custom_params(params)?;
            let result = handle_parse_swrl_rule(params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::invalid_params(e.to_string()))
        }
        "getExplanation" => {
            let params: GetExplanationParams = parse_custom_params(params)?;
            let result = handle_get_explanation(state, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::explanation_failed(e.to_string()))
        }
        "checkInstance" => {
            let params: CheckInstanceParams = parse_custom_params(params)?;
            let result = handle_check_instance(state, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::reasoner_failed(e.to_string()))
        }
        "runRobot" => {
            let params: RunRobotParams = parse_custom_params(params)?;
            let result = handle_run_robot(state, index_worker, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::robot_failed(e.to_string()))
        }
        "findUsages" => {
            let params: FindUsagesParams = parse_custom_params(params)?;
            let result = handle_find_usages(state, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))
        }
        "previewRefactor" => {
            let params: PreviewRefactorParams = parse_custom_params(params)?;
            let result = handle_preview_refactor(state, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))
        }
        "applyRefactor" => {
            let params: ApplyRefactorParams = parse_custom_params(params)?;
            let result = handle_apply_refactor(state, index_worker, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))
        }
        "semanticDiff" | "getSemanticDiff" => {
            let params: SemanticDiffParams = parse_custom_params(params)?;
            let result = handle_semantic_diff(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::invalid_params(e.to_string()))
        }
        "listPlugins" => {
            let result = handle_list_plugins(state)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "runPlugin" => {
            let params: RunPluginParams = parse_custom_params(params)?;
            let result = handle_run_plugin(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "listCommands" => {
            let result = handle_list_commands();
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "getWorkspaceUiState" => {
            let params: crate::protocol::WorkspaceUiStateParams = parse_custom_params(params)?;
            let result = handle_get_workspace_ui_state(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "getDialogSchema" => {
            let params: crate::protocol::GetDialogSchemaParams = parse_custom_params(params)?;
            let result = handle_get_dialog_schema(params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "createOntology" => {
            let params: crate::protocol::CreateOntologyParams = parse_custom_params(params)?;
            let result = handle_create_ontology(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "exportOntology" => {
            let params: crate::protocol::ExportOntologyParams = parse_custom_params(params)?;
            let result = handle_export_ontology(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::robot_failed(e.to_string()))
        }
        "setActiveOntology" => {
            let params: crate::protocol::SetActiveOntologyParams = parse_custom_params(params)?;
            let result = handle_set_active_ontology(state, params)?;
            serde_json::to_value(result).map_err(|e| LspErrorPayload::index_failed(e.to_string()))
        }
        "deleteImpact" => {
            let params: crate::protocol::DeleteImpactParams = parse_custom_params(params)?;
            let result = handle_delete_impact(state, params)?;
            serde_json::to_value(result)
                .map_err(|e| LspErrorPayload::refactor_failed(e.to_string()))
        }
        _ => Err(LspErrorPayload::invalid_params(format!("unknown method: {method}"))),
    }
}

#[derive(Debug)]
pub enum StandardRequestOutcome {
    Ok(Value),
    MethodNotFound,
    InvalidParams(ResponseError),
    LspError(LspErrorPayload),
}

pub fn handle_standard_request(
    state: &ServerState,
    method: &str,
    params: Option<Value>,
) -> StandardRequestOutcome {
    match method {
        "textDocument/hover" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("hover"));
            };
            match handle_hover(state, params) {
                Some(hover) => serde_json::to_value(hover)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Null)),
                None => StandardRequestOutcome::Ok(Value::Null),
            }
        }
        "textDocument/documentSymbol" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("documentSymbol"));
            };
            match handle_document_symbol(state, params) {
                Some(symbols) => serde_json::to_value(symbols)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Null)),
                None => StandardRequestOutcome::Ok(Value::Null),
            }
        }
        "workspace/symbol" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("workspace/symbol"));
            };
            match handle_workspace_symbol(state, params) {
                Some(symbols) => serde_json::to_value(symbols)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Array(vec![]))),
                None => StandardRequestOutcome::Ok(Value::Array(vec![])),
            }
        }
        "textDocument/definition" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("definition"));
            };
            match handle_goto_definition(state, params) {
                Some(def) => serde_json::to_value(def)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Null)),
                None => StandardRequestOutcome::Ok(Value::Null),
            }
        }
        "textDocument/references" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("references"));
            };
            match handle_references(state, params) {
                Some(refs) => serde_json::to_value(refs)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Array(vec![]))),
                None => StandardRequestOutcome::Ok(Value::Array(vec![])),
            }
        }
        "textDocument/rename" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("rename"));
            };
            match handle_rename(state, params) {
                Ok(Some(edit)) => serde_json::to_value(edit)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Null)),
                Ok(None) => StandardRequestOutcome::Ok(Value::Null),
                Err(err) => StandardRequestOutcome::LspError(err),
            }
        }
        "textDocument/prepareRename" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("prepareRename"));
            };
            match handle_prepare_rename(state, params) {
                Some(resp) => serde_json::to_value(resp)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Null)),
                None => StandardRequestOutcome::Ok(Value::Null),
            }
        }
        "textDocument/completion" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("completion"));
            };
            match crate::completion::handle_completion(state, params) {
                Some(list) => serde_json::to_value(list)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Null)),
                None => StandardRequestOutcome::Ok(Value::Array(vec![])),
            }
        }
        "textDocument/codeAction" => {
            let Ok(params) = serde_json::from_value(params.unwrap_or(Value::Null)) else {
                return StandardRequestOutcome::InvalidParams(invalid_params("codeAction"));
            };
            match crate::code_actions::handle_code_action(state, params) {
                Some(actions) => serde_json::to_value(actions)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Array(vec![]))),
                None => StandardRequestOutcome::Ok(Value::Array(vec![])),
            }
        }
        "textDocument/semanticTokens/full" => {
            let Ok(st_params) =
                serde_json::from_value::<SemanticTokensParams>(params.unwrap_or(Value::Null))
            else {
                return StandardRequestOutcome::InvalidParams(invalid_params(
                    "semanticTokens/full",
                ));
            };
            let doc_text =
                strixonomy_core::workspace_uri_to_path(st_params.text_document.uri.as_str())
                    .ok()
                    .and_then(|path| state.document_text(&path));
            match crate::semantic_tokens::handle_semantic_tokens_full(st_params, doc_text) {
                Some(tokens) => serde_json::to_value(tokens)
                    .map(StandardRequestOutcome::Ok)
                    .unwrap_or(StandardRequestOutcome::Ok(Value::Null)),
                None => StandardRequestOutcome::Ok(Value::Null),
            }
        }
        _ => StandardRequestOutcome::MethodNotFound,
    }
}

fn invalid_params(method: &str) -> ResponseError {
    ResponseError { code: -32602, message: format!("invalid params for {method}"), data: None }
}

/// Build an LSP range from entity source metadata and optional document text.
/// Callers must not hold the catalog `RwLock` when loading `doc_text`.
fn entity_source_range(doc_text: Option<&str>, entity: &strixonomy_core::Entity) -> Range {
    let line_idx = entity.source_location.line.unwrap_or(1).saturating_sub(1) as u32;
    let byte_col = entity.source_location.column.unwrap_or(0) as usize;
    let line_text =
        doc_text.and_then(|text| text.lines().nth(line_idx as usize).map(|s| s.to_string()));
    let character =
        line_text.as_deref().map(|l| byte_col_to_utf16(l, byte_col)).unwrap_or(byte_col as u32);
    Range {
        start: Position { line: line_idx, character },
        end: Position { line: line_idx, character: character.saturating_add(1) },
    }
}

fn lsp_document_path(state: &ServerState, uri: &Uri) -> Option<std::path::PathBuf> {
    state.resolve_lsp_document_uri(uri.as_str()).ok()
}

fn path_to_lsp_uri(path: &Path) -> Option<Uri> {
    Uri::from_str(&path_to_uri(path)).ok()
}

fn escape_markdown(value: &str) -> String {
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

fn entity_kind_to_symbol_kind(kind: EntityKind) -> SymbolKind {
    match kind {
        EntityKind::Class => SymbolKind::CLASS,
        EntityKind::ObjectProperty | EntityKind::DataProperty | EntityKind::AnnotationProperty => {
            SymbolKind::PROPERTY
        }
        EntityKind::Individual => SymbolKind::VARIABLE,
        EntityKind::Datatype => SymbolKind::STRUCT,
        EntityKind::Ontology => SymbolKind::NAMESPACE,
        EntityKind::Other => SymbolKind::OBJECT,
    }
}

fn iri_at_position(content: &str, position: Position) -> Option<String> {
    let (_, token) = rename_range_at_position(content, position)?;
    expand_iri_token(content, &token)
}

/// Returns the UTF-16 range of the renameable token under the cursor and its text placeholder.
fn rename_range_at_position(content: &str, position: Position) -> Option<(Range, String)> {
    let lines: Vec<&str> = content.lines().collect();
    let line = lines.get(position.line as usize)?;
    let byte_col = utf16_offset_to_byte(line, position.character);
    // Past end-of-line or not on a token: do not fall back to the first QName on the line (#13).
    if byte_col > line.len() {
        return None;
    }

    let (start, end, token) = extract_token_span_at(line, byte_col);
    if token.is_empty() {
        return None;
    }
    if !(token.contains(':') || token.starts_with("http")) {
        return None;
    }
    let _iri = expand_iri_token(content, &token)?;
    let range = Range {
        start: Position { line: position.line, character: byte_col_to_utf16(line, start) },
        end: Position { line: position.line, character: byte_col_to_utf16(line, end) },
    };
    Some((range, token))
}

fn extract_token_span_at(line: &str, ch: usize) -> (usize, usize, String) {
    let bytes = line.as_bytes();
    let mut start = ch.min(bytes.len());
    let mut end = ch.min(bytes.len());

    while start > 0 && is_iri_char(bytes[start - 1]) {
        start -= 1;
    }
    while end < bytes.len() && is_iri_char(bytes[end]) {
        end += 1;
    }

    (start, end, line[start..end].to_string())
}

fn is_iri_char(b: u8) -> bool {
    // Include `.` and common IRI path/query characters (#397).
    b.is_ascii_alphanumeric()
        || matches!(
            b,
            b':' | b'#'
                | b'/'
                | b'_'
                | b'-'
                | b'.'
                | b'?'
                | b'='
                | b'&'
                | b'%'
                | b'+'
                | b'~'
                | b'@'
        )
}

fn expand_iri_token(content: &str, token: &str) -> Option<String> {
    if token.starts_with("http://") || token.starts_with("https://") {
        return Some(token.to_string());
    }

    if let Some((prefix, local)) = token.split_once(':') {
        let namespaces =
            strixonomy_owl::namespaces_for_text(content, &std::collections::BTreeMap::new());
        let ns = namespaces.get(prefix)?;
        return Some(format!("{ns}{local}"));
    }

    Some(token.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_renamed_iri_expands_prefixed_name() {
        let content = "@prefix ex: <http://example.org/people#> .\nex:Person a owl:Class .";
        let iri = derive_renamed_iri("http://example.org/people#Person", "ex:Human", content);
        assert_eq!(iri, "http://example.org/people#Human");
    }

    #[test]
    fn expand_prefixed_iri() {
        let content = "@prefix ex: <http://example.org/people#> .\nex:Person a owl:Class .";
        let iri = expand_iri_token(content, "ex:Person").expect("expanded");
        assert_eq!(iri, "http://example.org/people#Person");
    }

    #[test]
    fn iri_at_position_returns_none_past_line_end() {
        let content = "ex:Person a owl:Class ;";
        let pos = Position { line: 0, character: 80 };
        assert!(iri_at_position(content, pos).is_none());
    }

    #[test]
    fn iri_at_position_returns_none_on_trailing_whitespace() {
        let content = "ex:Person a owl:Class ;  ";
        let pos = Position { line: 0, character: (content.len() - 1) as u32 };
        assert!(iri_at_position(content, pos).is_none());
    }

    #[test]
    fn iri_at_position_resolves_token_under_cursor() {
        let content = "@prefix ex: <http://example.org/people#> .\nex:Person a owl:Class ;";
        let pos = Position { line: 1, character: 3 };
        let iri = iri_at_position(content, pos).expect("iri");
        assert_eq!(iri, "http://example.org/people#Person");
    }

    #[test]
    fn iri_at_position_resolves_angle_bracket_iri_with_dots() {
        // #397 — `.` must not split absolute IRIs.
        let content = "ex:Person a owl:Class ; rdfs:seeAlso <http://example.org/Person> .";
        let start = content.find("http://example.org/Person").expect("iri");
        let pos = Position { line: 0, character: (start + "http://example".len()) as u32 };
        let iri = iri_at_position(content, pos).expect("full iri");
        assert_eq!(iri, "http://example.org/Person");
    }

    #[test]
    fn prepare_rename_returns_range_on_qname() {
        let content = "@prefix ex: <http://example.org/people#> .\nex:Person a owl:Class ;";
        let pos = Position { line: 1, character: 3 };
        let (range, placeholder) = rename_range_at_position(content, pos).expect("range");
        assert_eq!(placeholder, "ex:Person");
        assert_eq!(range.start.line, 1);
        assert_eq!(range.start.character, 0);
        assert_eq!(range.end.character, "ex:Person".len() as u32);
    }

    #[test]
    fn prepare_rename_rejects_non_iri_token() {
        let content = "@prefix ex: <http://example.org/people#> .\nex:Person a owl:Class ;";
        let pos = Position { line: 1, character: 12 }; // on "owl"
                                                       // "owl:Class" is a QName — move to "a"
        let pos_a = Position { line: 1, character: 10 };
        assert!(rename_range_at_position(content, pos_a).is_none());
        let _ = pos;
    }

    #[test]
    fn expand_iri_token_ignores_prefix_in_comment() {
        let content = "# @prefix ex: <http://evil/> .\n@prefix ex: <http://example.org/people#> .\nex:Person a owl:Class .";
        let iri = expand_iri_token(content, "ex:Person").expect("expanded");
        assert_eq!(iri, "http://example.org/people#Person");
    }

    #[test]
    fn expand_iri_token_rejects_malformed_prefix_line() {
        let content = "@prefix ex: >oops<http://example.org/people#> .\nex:Person a owl:Class .";
        assert!(expand_iri_token(content, "ex:Person").is_none());
    }

    #[test]
    fn escape_markdown_neutralizes_links() {
        let escaped = escape_markdown("[click](https://evil.example)");
        assert!(!escaped.contains("](https://"));
        assert!(escaped.contains("\\["));
    }

    #[test]
    fn jail_robot_rejects_query_equals_outside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let roots = vec![dir.path().to_path_buf()];
        let input = dir.path().join("in.ttl");
        std::fs::write(&input, "").unwrap();
        let args = vec![
            "query".to_string(),
            format!("--input={}", input.display()),
            "--query=/tmp/evil.sparql".to_string(),
        ];
        let err = jail_robot_path_args(&roots, &args).unwrap_err();
        assert!(err.contains("outside") || err.contains("workspace"), "unexpected error: {err}");
    }

    #[test]
    fn jail_robot_rejects_update_equals_outside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let roots = vec![dir.path().to_path_buf()];
        let input = dir.path().join("in.ttl");
        std::fs::write(&input, "").unwrap();
        let args = vec![
            "query".to_string(),
            format!("--input={}", input.display()),
            "--update=/tmp/evil.ru".to_string(),
        ];
        assert!(jail_robot_path_args(&roots, &args).is_err());
    }

    #[test]
    fn jail_robot_rejects_query_pair_outside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let roots = vec![dir.path().to_path_buf()];
        let input = dir.path().join("in.ttl");
        let query = dir.path().join("q.sparql");
        std::fs::write(&input, "").unwrap();
        std::fs::write(&query, "").unwrap();
        let args = vec![
            "query".to_string(),
            format!("--input={}", input.display()),
            "--query".to_string(),
            query.display().to_string(),
            "/tmp/evil-out.csv".to_string(),
        ];
        assert!(jail_robot_path_args(&roots, &args).is_err());
    }

    #[test]
    fn jail_robot_accepts_in_workspace_query_pair() {
        let dir = tempfile::tempdir().unwrap();
        let roots = vec![dir.path().to_path_buf()];
        let input = dir.path().join("in.ttl");
        let query = dir.path().join("q.sparql");
        let output = dir.path().join("out.csv");
        std::fs::write(&input, "").unwrap();
        std::fs::write(&query, "").unwrap();
        let args = vec![
            "query".to_string(),
            format!("--input={}", input.display()),
            "--query".to_string(),
            query.display().to_string(),
            output.display().to_string(),
        ];
        jail_robot_path_args(&roots, &args).expect("in-workspace query pair");
    }

    #[test]
    fn jail_robot_rewrites_relative_paths_to_absolute() {
        // #346 — jail must return absolute operands so ROBOT does not use LSP CWD.
        let dir = tempfile::tempdir().unwrap();
        let roots = vec![dir.path().to_path_buf()];
        std::fs::write(dir.path().join("in.ttl"), "").unwrap();
        let args = vec![
            "convert".to_string(),
            "--input".to_string(),
            "in.ttl".to_string(),
            "--output".to_string(),
            "out.owl".to_string(),
        ];
        let rewritten = jail_robot_path_args(&roots, &args).expect("relative paths in workspace");
        assert!(
            PathBuf::from(&rewritten[2]).is_absolute(),
            "input must be absolute: {}",
            rewritten[2]
        );
        assert!(
            PathBuf::from(&rewritten[4]).is_absolute(),
            "output must be absolute: {}",
            rewritten[4]
        );
        assert!(rewritten[2].contains("in.ttl"), "unexpected rewritten input: {}", rewritten[2]);
        assert!(rewritten[4].contains("out.owl"), "unexpected rewritten output: {}", rewritten[4]);
    }

    #[test]
    fn jail_robot_rejects_output_dir_outside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let roots = vec![dir.path().to_path_buf()];
        let input = dir.path().join("in.ttl");
        std::fs::write(&input, "").unwrap();
        let args = vec![
            "query".to_string(),
            format!("--input={}", input.display()),
            "--output-dir".to_string(),
            "/tmp/evil-results".to_string(),
        ];
        assert!(jail_robot_path_args(&roots, &args).is_err());
    }

    #[test]
    fn jail_robot_rejects_prefixes_equals_outside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let roots = vec![dir.path().to_path_buf()];
        let input = dir.path().join("in.ttl");
        std::fs::write(&input, "").unwrap();
        let args = vec![
            "convert".to_string(),
            format!("--input={}", input.display()),
            "--prefixes=/tmp/evil.jsonld".to_string(),
            format!("--output={}", dir.path().join("out.owl").display()),
        ];
        assert!(jail_robot_path_args(&roots, &args).is_err());
    }

    #[test]
    fn jail_robot_rejects_inputs_and_add_prefixes_outside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let roots = vec![dir.path().to_path_buf()];
        let input = dir.path().join("in.ttl");
        std::fs::write(&input, "").unwrap();
        let args_inputs = vec![
            "merge".to_string(),
            format!("--input={}", input.display()),
            "--inputs=/tmp/outside.owl".to_string(),
            format!("--output={}", dir.path().join("out.owl").display()),
        ];
        assert!(jail_robot_path_args(&roots, &args_inputs).is_err());
        let args_add = vec![
            "convert".to_string(),
            format!("--input={}", input.display()),
            "--add-prefixes=/tmp/evil.jsonld".to_string(),
            format!("--output={}", dir.path().join("out.owl").display()),
        ];
        assert!(jail_robot_path_args(&roots, &args_add).is_err());
    }

    #[test]
    fn jail_robot_rejects_short_input_equals_outside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let roots = vec![dir.path().to_path_buf()];
        let args = vec!["validate".to_string(), "-i=/tmp/evil.owl".to_string()];
        let err = jail_robot_path_args(&roots, &args).unwrap_err();
        assert!(
            err.contains("outside") || err.contains("workspace") || err.contains("path"),
            "expected jail error, got {err}"
        );
    }

    #[test]
    fn jail_robot_rejects_short_output_equals_outside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let roots = vec![dir.path().to_path_buf()];
        let input = dir.path().join("in.ttl");
        std::fs::write(&input, "").unwrap();
        let args = vec![
            "convert".to_string(),
            format!("-i={}", input.display()),
            "-o=/tmp/evil.owl".to_string(),
        ];
        assert!(jail_robot_path_args(&roots, &args).is_err());
    }

    #[test]
    fn atomic_write_for_path_dispatches_obo_and_turtle() {
        let dir = tempfile::tempdir().unwrap();
        let obo = dir.path().join("term.obo");
        let ttl = dir.path().join("ont.ttl");
        std::fs::write(&obo, "format-version: 1.2\nold\n").unwrap();
        std::fs::write(&ttl, "@prefix owl: <http://www.w3.org/2002/07/owl#> .\nold\n").unwrap();

        atomic_write_for_path(&obo, "format-version: 1.2\nontology: new\n").expect("obo write");
        atomic_write_for_path(
            &ttl,
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n<http://ex.org> a owl:Ontology .\n",
        )
        .expect("ttl write");

        assert!(std::fs::read_to_string(&obo).unwrap().contains("ontology: new"));
        assert!(std::fs::read_to_string(&ttl).unwrap().contains("owl:Ontology"));
    }
}
