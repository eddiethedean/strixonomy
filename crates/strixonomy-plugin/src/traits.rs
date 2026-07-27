use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};
use strixonomy_catalog::OntologyCatalog;
use strixonomy_core::Diagnostic;
use strixonomy_docs::{ExportError, ExportOptions};

pub trait ValidatorPlugin: Send + Sync {
    fn id(&self) -> &str;
    fn validate(&self, catalog: &OntologyCatalog, workspace: &Path) -> Vec<Diagnostic>;
}

pub trait ExporterPlugin: Send + Sync {
    fn id(&self) -> &str;
    fn export(
        &self,
        catalog: &OntologyCatalog,
        workspace: &Path,
        options: ExportOptions,
    ) -> Result<Vec<PathBuf>, ExportError>;
}

#[derive(Debug, Clone)]
pub struct WorkflowRequest {
    pub step: String,
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct WorkflowResult {
    pub success: bool,
    pub logs: String,
    pub diagnostics: Vec<Diagnostic>,
}

pub trait WorkflowPlugin: Send + Sync {
    fn id(&self) -> &str;
    fn run(&self, workspace: &Path, request: WorkflowRequest) -> WorkflowResult;
}

/// Thin reasoner adapter (SDK 1.0) — emits unsatisfiable IRIs compatible with reasoner UIs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReasonerProviderResult {
    pub profile: String,
    pub unsatisfiable: Vec<String>,
    #[serde(default)]
    pub logs: Option<String>,
}

pub trait ReasonerPlugin: Send + Sync {
    fn id(&self) -> &str;
    fn classify(&self, catalog: &OntologyCatalog, workspace: &Path) -> ReasonerProviderResult;
}

/// Thin query adapter — tabular rows for Query Workbench / CLI.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryProviderResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    #[serde(default)]
    pub truncated: bool,
}

pub trait QueryPlugin: Send + Sync {
    fn id(&self) -> &str;
    fn run(&self, catalog: &OntologyCatalog, workspace: &Path, query: &str) -> QueryProviderResult;
}

/// Thin refactor preview tip (does not apply edits).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RefactorProviderResult {
    pub affected_iris: Vec<String>,
    pub hints: Vec<String>,
}

pub trait RefactorPlugin: Send + Sync {
    fn id(&self) -> &str;
    fn preview(
        &self,
        catalog: &OntologyCatalog,
        workspace: &Path,
        focus_iri: Option<&str>,
    ) -> RefactorProviderResult;
}

/// Thin graph provider — optional graph_kind + IRI seeds / overlay payload.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GraphProviderResult {
    pub graph_kind: String,
    pub root_iris: Vec<String>,
    #[serde(default)]
    pub result: Option<Value>,
}

pub trait GraphPlugin: Send + Sync {
    fn id(&self) -> &str;
    fn build(
        &self,
        catalog: &OntologyCatalog,
        workspace: &Path,
        root_iri: Option<&str>,
    ) -> GraphProviderResult;
}
