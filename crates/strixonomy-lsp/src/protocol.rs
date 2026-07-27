use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use strixonomy_catalog::{CatalogStats, ClassHierarchy, EntityDetail, SubclassEdge};
use strixonomy_core::{Diagnostic, Entity, OntologyDocument};
use strixonomy_reasoner::ReasonerSnapshot;

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticSummary {
    pub code: String,
    pub severity: String,
    pub message: String,
    pub file: String,
    pub line: Option<u64>,
    pub column: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_iri: Option<String>,
}

impl From<&Diagnostic> for DiagnosticSummary {
    fn from(d: &Diagnostic) -> Self {
        Self {
            code: d.display_code(),
            severity: d.severity.as_str().to_string(),
            message: d.message.clone(),
            file: d.file.display().to_string(),
            line: d.range.line,
            column: d.range.column,
            entity_iri: d.entity_iri.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct IndexWorkspaceParams {
    /// Workspace root URI (`file://…`). Accepts legacy camelCase `workspaceUri` during migration.
    #[serde(alias = "workspaceUri", default)]
    pub workspace_uri: Option<String>,
    /// Persist parse snapshots under `.strixonomy/cache/`.
    #[serde(default)]
    pub disk_cache: bool,
}

#[derive(Debug, Serialize)]
pub struct IndexWorkspaceResult {
    pub stats: CatalogStats,
    pub indexed_at: u64,
}

#[derive(Debug, Serialize)]
pub struct CatalogSnapshot {
    pub documents: Vec<OntologyDocument>,
    pub entities: Vec<Entity>,
    pub hierarchy: ClassHierarchy,
    pub diagnostics: Vec<DiagnosticSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoner: Option<ReasonerSnapshot>,
    /// Workspace catalog statistics (same shape as `indexWorkspace` stats).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<CatalogStats>,
    /// Active ontology document id (path or ontology IRI) when set by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_ontology_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CommandEnablement {
    Always,
    HasOntology,
    IsDirty,
    HasSelection,
    ReasonerRunning,
    ReasonerIdle,
    CanEditSelection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandDescriptor {
    pub id: String,
    pub title: String,
    pub category: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enablement: Vec<CommandEnablement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undo_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListCommandsResult {
    pub commands: Vec<CommandDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OntologyRegistryEntrySnapshot {
    pub id: String,
    pub uri: String,
    pub path: String,
    pub format: String,
    pub role: String,
    pub editable: bool,
    pub dirty: bool,
    pub version: u32,
    pub active: bool,
}

#[derive(Debug, Deserialize)]
pub struct WorkspaceUiStateParams {
    /// Optional focused entity IRI from the client.
    #[serde(default)]
    pub selection_iri: Option<String>,
    /// Client-reported dirty document count.
    #[serde(default)]
    pub dirty_document_count: u32,
    /// Active ontology id preferred by the client.
    #[serde(default)]
    pub active_ontology_id: Option<String>,
    /// Host-owned ontology registry snapshot (v0.20 workspace runtime).
    #[serde(default)]
    pub ontology_registry: Vec<OntologyRegistryEntrySnapshot>,
}

#[derive(Debug, Serialize)]
pub struct WorkspaceUiState {
    pub has_ontology: bool,
    pub ontology_count: usize,
    pub is_dirty: bool,
    pub has_selection: bool,
    pub selection_iri: Option<String>,
    pub selection_editable: bool,
    pub reasoner_running: bool,
    pub reasoner_dirty: bool,
    pub reasoner_consistent: Option<bool>,
    pub active_ontology_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<CatalogStats>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub ontology_registry: Vec<OntologyRegistryEntrySnapshot>,
}

#[derive(Debug, Deserialize)]
pub struct GetDialogSchemaParams {
    pub dialog_id: String,
}

#[derive(Debug, Serialize)]
pub struct DialogFieldSchema {
    pub id: String,
    pub label: String,
    pub field_type: String,
    #[serde(default)]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DialogSchema {
    pub id: String,
    pub title: String,
    pub fields: Vec<DialogFieldSchema>,
    pub primary_action: String,
}

#[derive(Debug, Serialize)]
pub struct GetDialogSchemaResult {
    pub schema: DialogSchema,
}

#[derive(Debug, Deserialize)]
pub struct CreateOntologyParams {
    pub path: String,
    pub ontology_iri: String,
    #[serde(default)]
    pub version_iri: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub prefixes: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Serialize)]
pub struct CreateOntologyResult {
    pub path: String,
    pub ontology_iri: String,
}

#[derive(Debug, Deserialize)]
pub struct ExportOntologyParams {
    pub source_path: String,
    pub output_path: String,
    #[serde(default)]
    pub format: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExportOntologyResult {
    pub output_path: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetActiveOntologyParams {
    pub ontology_id: String,
}

#[derive(Debug, Serialize)]
pub struct SetActiveOntologyResult {
    pub active_ontology_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteImpactParams {
    pub entity_iri: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteImpactResult {
    pub entity_iri: String,
    pub usage_count: usize,
    pub axiom_count: usize,
    pub referencing_entities: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetEntityParams {
    pub iri: String,
}

#[derive(Debug, Serialize)]
pub struct GetEntityResult {
    pub detail: EntityDetail,
}

#[derive(Debug, Deserialize)]
pub struct ApplyAxiomPatchParams {
    pub document_uri: String,
    /// JSON array of patch operations (Turtle [`strixonomy_owl::PatchOp`] or OBO [`strixonomy_obo::OboPatchOp`]).
    pub patches: serde_json::Value,
    #[serde(default)]
    pub preview_only: bool,
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub sql: String,
}

#[derive(Debug, Deserialize)]
pub struct SparqlParams {
    pub query: String,
}

#[derive(Debug, Deserialize)]
pub struct DlQueryParams {
    pub expression: String,
    #[serde(default = "default_dl_query_profile")]
    pub profile: String,
    /// `inferred` (default) or `asserted`
    #[serde(default = "default_dl_query_mode")]
    pub mode: String,
    #[serde(default)]
    pub document_uri: Option<String>,
}

fn default_dl_query_profile() -> String {
    "dl".to_string()
}

fn default_dl_query_mode() -> String {
    "inferred".to_string()
}

#[derive(Debug, Serialize)]
pub struct DlQueryResult {
    pub expression: String,
    pub normalized: String,
    pub query_class_iri: String,
    pub subclasses: Vec<String>,
    pub superclasses: Vec<String>,
    pub equivalents: Vec<String>,
    pub instances: Vec<String>,
    pub profile: String,
    pub mode: String,
    pub duration_ms: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub query: String,
    /// Max hits to return (default 100, capped at 500).
    #[serde(default)]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub entities: Vec<EntityDetail>,
}

#[derive(Debug, Serialize)]
pub struct TabularQueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ParseManchesterParams {
    pub expression: String,
    pub axiom_kind: String,
    #[serde(default)]
    pub entity_iri: Option<String>,
    #[serde(default)]
    pub document_uri: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ManchesterCompletions {
    pub classes: Vec<String>,
    pub object_properties: Vec<String>,
    pub data_properties: Vec<String>,
    pub datatypes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ParseManchesterResult {
    pub normalized: String,
    pub turtle_fragment: String,
    pub tree: serde_json::Value,
    pub diagnostics: Vec<strixonomy_owl::PatchDiagnostic>,
    pub completions: ManchesterCompletions,
}

#[derive(Debug, Serialize)]
pub struct ApplyAxiomPatchResult {
    #[serde(flatten)]
    pub patch: strixonomy_owl::ApplyPatchResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_detail: Option<EntityDetail>,
    /// Set when the patch was written but workspace reindex failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reindex_warning: Option<String>,
    /// Full-document edit so the client can sync open editors with disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_edit: Option<lsp_types::WorkspaceEdit>,
    /// Inverted patch ops for workspace-level semantic undo (v0.20+).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undo_patches: Option<serde_json::Value>,
}

/// LSP JSON error payload for custom `strixonomy/*` methods (not [`strixonomy_core::StrixonomyError`]).
#[derive(Debug, Serialize)]
pub struct LspErrorPayload {
    pub code: String,
    pub message: String,
    pub recoverable: bool,
    pub user_action: Option<String>,
}

impl LspErrorPayload {
    pub fn not_indexed() -> Self {
        Self {
            code: "NOT_INDEXED".to_string(),
            message: "Workspace has not been indexed yet".to_string(),
            recoverable: true,
            user_action: Some("Run Strixonomy: Index Workspace".to_string()),
        }
    }

    pub fn not_found(iri: &str) -> Self {
        Self {
            code: "ENTITY_NOT_FOUND".to_string(),
            message: format!("Entity not found: {iri}"),
            recoverable: true,
            user_action: None,
        }
    }

    pub fn index_failed(message: String) -> Self {
        Self {
            code: "INDEX_FAILED".to_string(),
            message,
            recoverable: true,
            user_action: Some("Check ontology files for parse errors".to_string()),
        }
    }

    pub fn invalid_params(message: String) -> Self {
        Self { code: "INVALID_PARAMS".to_string(), message, recoverable: true, user_action: None }
    }

    pub fn graph_failed(message: String) -> Self {
        Self {
            code: "GRAPH_FAILED".to_string(),
            message,
            recoverable: true,
            user_action: Some("Adjust graph kind, root IRI, or filters".to_string()),
        }
    }

    pub fn robot_failed(message: String) -> Self {
        Self {
            code: "ROBOT_FAILED".to_string(),
            message,
            recoverable: true,
            user_action: Some("Check ROBOT CLI installation and arguments".to_string()),
        }
    }

    pub fn patch_invalid(message: String) -> Self {
        Self {
            code: "PATCH_INVALID".to_string(),
            message,
            recoverable: true,
            user_action: Some("Check patch parameters and entity IRIs".to_string()),
        }
    }

    pub fn unsupported_format(message: String) -> Self {
        Self {
            code: "UNSUPPORTED_FORMAT".to_string(),
            message,
            recoverable: true,
            user_action: Some("Save as Turtle (.ttl) for write-back".to_string()),
        }
    }

    pub fn query_failed(message: String) -> Self {
        Self {
            code: "QUERY_FAILED".to_string(),
            message,
            recoverable: true,
            user_action: Some("Check query syntax and virtual table names".to_string()),
        }
    }

    pub fn manchester_invalid(message: String) -> Self {
        Self {
            code: "MANCHESTER_INVALID".to_string(),
            message,
            recoverable: true,
            user_action: Some("Fix the Manchester class expression".to_string()),
        }
    }

    pub fn applied_not_indexed(message: String) -> Self {
        Self {
            code: "APPLIED_NOT_INDEXED".to_string(),
            message,
            recoverable: true,
            user_action: Some("Patch was saved; run Strixonomy: Index Workspace".to_string()),
        }
    }

    pub fn reasoner_failed(message: String) -> Self {
        Self {
            code: "REASONER_FAILED".to_string(),
            message,
            recoverable: true,
            user_action: Some(
                "Try a different reasoner profile or fix ontology axioms".to_string(),
            ),
        }
    }

    pub fn explanation_failed(message: String) -> Self {
        Self {
            code: "EXPLANATION_FAILED".to_string(),
            message,
            recoverable: true,
            user_action: Some("Run the reasoner first or choose another class".to_string()),
        }
    }

    pub fn refactor_failed(message: String) -> Self {
        Self {
            code: "REFACTOR_FAILED".to_string(),
            message,
            recoverable: true,
            user_action: Some("Preview the refactor plan and check Turtle files".to_string()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RunReasonerParams {
    #[serde(default = "default_reasoner_profile")]
    pub profile: String,
    #[serde(default = "default_auto_profile")]
    pub auto_detect: bool,
}

fn default_reasoner_profile() -> String {
    "el".to_string()
}

fn default_auto_profile() -> bool {
    true
}

#[derive(Debug, Serialize)]
pub struct RunReasonerResult {
    pub profile_used: String,
    pub consistent: bool,
    pub unsatisfiable: Vec<String>,
    pub inferred_edge_count: usize,
    pub new_inferences: Vec<SubclassEdge>,
    pub warnings: Vec<strixonomy_reasoner::ReasonerWarning>,
    pub duration_ms: u64,
    pub snapshot: ReasonerSnapshot,
}

#[derive(Debug, Deserialize)]
pub struct GetExplanationParams {
    pub class_iri: String,
    #[serde(default = "default_reasoner_profile")]
    pub profile: String,
}

#[derive(Debug, Serialize)]
pub struct GetGraphResult {
    pub graph: strixonomy_catalog::GraphPayload,
}

#[derive(Debug, Deserialize)]
pub struct RunRobotParams {
    pub subcommand: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub robot_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RunRobotResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Deserialize)]
pub struct CheckInstanceParams {
    pub individual_iri: String,
    pub class_iri: String,
    #[serde(default = "default_reasoner_profile")]
    pub profile: String,
}

#[derive(Debug, Serialize)]
pub struct CheckInstanceResult {
    pub individual_iri: String,
    pub class_iri: String,
    pub entailed: bool,
    pub profile_used: String,
    pub duration_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct GetExplanationResult {
    pub class_iri: String,
    pub steps: Vec<strixonomy_reasoner::ExplanationStep>,
    pub text: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternatives: Vec<strixonomy_reasoner::ExplanationResult>,
    pub indexed_at: u64,
    pub content_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct FindUsagesParams {
    pub iri: String,
}

#[derive(Debug, Serialize)]
pub struct UsageSummary {
    pub iri: String,
    pub referenced_iri: String,
    pub file: String,
    pub line: Option<u64>,
    pub column: Option<u64>,
    pub kind: String,
    pub context: String,
}

#[derive(Debug, Serialize)]
pub struct FindUsagesResult {
    pub usages: Vec<UsageSummary>,
}

#[derive(Debug, Deserialize)]
pub struct PreviewRefactorParams {
    #[serde(flatten)]
    pub request: strixonomy_refactor::RefactorRequest,
}

#[derive(Debug, Serialize)]
pub struct PreviewRefactorResult {
    #[serde(flatten)]
    pub plan: strixonomy_refactor::RefactorPlan,
}

#[derive(Debug, Deserialize)]
pub struct ApplyRefactorParams {
    pub plan: strixonomy_refactor::RefactorPlan,
    pub request: strixonomy_refactor::RefactorRequest,
    #[serde(default)]
    pub preview_only: bool,
}

#[derive(Debug, Deserialize)]
pub struct SemanticDiffParams {
    /// Git left ref (e.g. `main`) or `WORKSPACE` for indexed catalog.
    #[serde(default)]
    pub left_ref: Option<String>,
    /// Git right ref or `WORKTREE` for working tree / indexed workspace.
    #[serde(default)]
    pub right_ref: Option<String>,
    /// Optional left directory when comparing two paths on disk.
    #[serde(default)]
    pub left_path: Option<String>,
    /// Optional right directory.
    #[serde(default)]
    pub right_path: Option<String>,
    /// When true, enrich the diff with reasoner unsatisfiability changes.
    #[serde(default)]
    pub reasoner: bool,
    /// Output format hint: `pr-summary` returns Markdown in `formatted`.
    #[serde(default)]
    pub format: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SemanticDiffResult {
    pub diff: strixonomy_diff::DiffResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListSqlSchemaResult {
    pub tables: Vec<strixonomy_query::SqlTableSchema>,
}

#[derive(Debug, Serialize)]
pub struct ApplyRefactorResult {
    pub files_written: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reindex_warning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_edit: Option<lsp_types::WorkspaceEdit>,
}

#[derive(Debug, Serialize)]
pub struct ListPluginsResult {
    pub plugins: Vec<strixonomy_plugin::PluginDescriptor>,
}

/// SWRL rule summary for Rule Browser (v0.23).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwrlRuleListItem {
    pub id: String,
    pub label: String,
    pub body_count: usize,
    pub head_count: usize,
    pub enabled: bool,
    /// Full JSON for editor round-trip when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontology_iri: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListSwrlRulesResult {
    pub rules: Vec<SwrlRuleListItem>,
}

#[derive(Debug, Deserialize)]
pub struct ValidateSwrlRuleParams {
    pub rule_json: String,
}

#[derive(Debug, Serialize)]
pub struct ValidateSwrlRuleResult {
    pub diagnostics: Vec<strixonomy_swrl::SwrlDiagnostic>,
}

#[derive(Debug, Deserialize)]
pub struct ParseSwrlRuleParams {
    pub rule_json: String,
}

#[derive(Debug, Serialize)]
pub struct ParseSwrlRuleResult {
    pub rule: strixonomy_swrl::SwrlRule,
    pub diagnostics: Vec<strixonomy_swrl::SwrlDiagnostic>,
}

#[derive(Debug, Deserialize)]
pub struct RunPluginParams {
    pub plugin_id: String,
    #[serde(default = "default_validate_action")]
    pub action: String,
    #[serde(default)]
    pub step: Option<String>,
    /// Used with `action = "ui_view"`.
    #[serde(default)]
    pub view_id: Option<String>,
    /// Query text for `query.run`.
    #[serde(default)]
    pub query: Option<String>,
    /// Focus / root IRI for `refactor.preview` or `graph.build`.
    #[serde(default)]
    pub focus_iri: Option<String>,
}

fn default_validate_action() -> String {
    "validate".to_string()
}

#[derive(Debug, Serialize)]
pub struct RunPluginResult {
    pub diagnostics: Vec<DiagnosticSummary>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_paths: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<String>,
    /// Optional HTML payload for plugin-defined views.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_html: Option<String>,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsatisfiable: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_iris: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_iris: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
}
