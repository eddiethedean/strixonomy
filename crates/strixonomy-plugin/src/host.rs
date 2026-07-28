use crate::discovery::{discover_plugins, PluginDiscoveryError};
use crate::lifecycle::{activation_order, dependents_of, LifecycleError};
use crate::manifest::{
    DiscoveredPlugin, PluginActivation, PluginKind, PluginLifecycleState, PluginManifest,
    PluginPermission,
};
use crate::protocol::{jail_output_paths, plugin_diagnostic, PluginOutput};
use crate::subprocess::{run_plugin_subprocess, SubprocessError, SubprocessRequest};
use crate::traits::{
    ExporterPlugin, GraphPlugin, GraphProviderResult, QueryPlugin, QueryProviderResult,
    ReasonerPlugin, ReasonerProviderResult, RefactorPlugin, RefactorProviderResult,
    ValidatorPlugin, WorkflowPlugin, WorkflowRequest,
};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use strixonomy_catalog::OntologyCatalog;
use strixonomy_core::{validate_workspace_scope, Diagnostic, DiagnosticSeverity};
use strixonomy_docs::ExportOptions;
use thiserror::Error;

/// Default in-process export directory (relative to the workspace root).
const DEFAULT_PLUGIN_EXPORT_DIR: &str = ".strixonomy/plugin-out";
/// Persisted user-disabled plugin ids (workspace-relative).
const DISABLED_STATE_FILE: &str = ".strixonomy/plugin-disabled.json";

#[derive(Debug, Error)]
pub enum PluginHostError {
    #[error(transparent)]
    Discovery(#[from] PluginDiscoveryError),
    #[error(transparent)]
    Subprocess(#[from] SubprocessError),
    #[error(transparent)]
    Lifecycle(#[from] LifecycleError),
    #[error("plugin not found: {0}")]
    NotFound(String),
    #[error("plugin is disabled: {0}")]
    Disabled(String),
    #[error("plugin is not active: {0}")]
    NotActive(String),
    #[error("plugin {0} is missing required permission: {1}")]
    MissingPermission(String, String),
    #[error("plugin {0} does not support action {1}")]
    UnsupportedAction(String, String),
    #[error("export failed: {0}")]
    Export(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginDescriptor {
    pub id: String,
    pub name: String,
    pub version: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<PluginPermission>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    pub activation: String,
    pub state: String,
    pub capabilities: crate::manifest::PluginCapabilities,
    pub manifest_path: String,
    pub ui: crate::manifest::PluginUiContributions,
    pub in_process: bool,
    /// True when user-disabled or inactive (not Active).
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunPluginResult {
    pub diagnostics: Vec<Diagnostic>,
    pub output_paths: Vec<String>,
    pub logs: Option<String>,
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

impl RunPluginResult {
    fn from_output(plugin_id: &str, workspace: &Path, output: PluginOutput, success: bool) -> Self {
        let output = jail_output_paths(plugin_id, workspace, output);
        let has_violations = !output.violations.is_empty();
        let diags = wire_to_diagnostics(plugin_id, workspace, output.clone());
        Self {
            diagnostics: diags,
            output_paths: output.output_paths,
            logs: output.logs,
            view_html: output.view_html,
            // #348: jail violations must not report silent success.
            success: success && !has_violations,
            result: output.result,
            columns: output.columns,
            rows: output.rows,
            unsatisfiable: output.unsatisfiable,
            affected_iris: output.affected_iris,
            root_iris: output.root_iris,
            graph_kind: output.graph_kind,
            hints: output.hints,
            profile: output.profile,
        }
    }

    fn empty_ok() -> Self {
        Self {
            diagnostics: Vec::new(),
            output_paths: Vec::new(),
            logs: None,
            view_html: None,
            success: true,
            result: None,
            columns: None,
            rows: None,
            unsatisfiable: None,
            affected_iris: None,
            root_iris: None,
            graph_kind: None,
            hints: None,
            profile: None,
        }
    }
}

pub struct PluginHost {
    workspace: PathBuf,
    discovered: Vec<DiscoveredPlugin>,
    /// User-disabled plugin ids (persists across activate_all).
    disabled: HashSet<String>,
    states: HashMap<String, PluginLifecycleState>,
    /// Deterministic activation order from the last activate_all / resolve.
    activation_events: Vec<String>,
    validators: HashMap<String, Box<dyn ValidatorPlugin>>,
    exporters: HashMap<String, Box<dyn ExporterPlugin>>,
    workflows: HashMap<String, Box<dyn WorkflowPlugin>>,
    reasoners: HashMap<String, Box<dyn ReasonerPlugin>>,
    queries: HashMap<String, Box<dyn QueryPlugin>>,
    refactors: HashMap<String, Box<dyn RefactorPlugin>>,
    graphs: HashMap<String, Box<dyn GraphPlugin>>,
}

impl PluginHost {
    pub fn new(workspace: impl AsRef<Path>) -> Self {
        let workspace = workspace.as_ref().to_path_buf();
        let disabled = load_disabled_ids(&workspace);
        Self {
            workspace,
            discovered: Vec::new(),
            disabled,
            states: HashMap::new(),
            activation_events: Vec::new(),
            validators: HashMap::new(),
            exporters: HashMap::new(),
            workflows: HashMap::new(),
            reasoners: HashMap::new(),
            queries: HashMap::new(),
            refactors: HashMap::new(),
            graphs: HashMap::new(),
        }
    }

    pub fn discover(&mut self) -> Result<(), PluginHostError> {
        self.discovered = discover_plugins(&self.workspace)?;
        for p in &self.discovered {
            let id = p.plugin_id().to_string();
            self.states.entry(id.clone()).or_insert(PluginLifecycleState::Discovered);
            if p.manifest.kind.is_hosted() {
                *self.states.get_mut(&id).unwrap() = PluginLifecycleState::Validated;
            }
        }
        // Resolve dependency graph early so missing deps surface at discover time.
        let _ = activation_order(&self.discovered)?;
        for p in &self.discovered {
            let id = p.plugin_id().to_string();
            if self.disabled.contains(&id) {
                *self.states.get_mut(&id).unwrap() = PluginLifecycleState::Disabled;
            } else if p.manifest.kind.is_hosted() {
                *self.states.get_mut(&id).unwrap() = PluginLifecycleState::Registered;
            }
        }
        Ok(())
    }

    /// Activate plugins with `on_startup` / `on_workspace_open` activation in dependency order.
    pub fn activate_all(&mut self) -> Result<Vec<String>, PluginHostError> {
        let order = activation_order(&self.discovered)?;
        self.activation_events.clear();
        for id in &order {
            let plugin = self.find_plugin(id)?;
            if self.disabled.contains(id) {
                *self.states.entry(id.clone()).or_insert(PluginLifecycleState::Disabled) =
                    PluginLifecycleState::Disabled;
                continue;
            }
            if !plugin.manifest.kind.is_hosted() {
                continue;
            }
            match plugin.manifest.activation {
                PluginActivation::OnStartup | PluginActivation::OnWorkspaceOpen => {
                    self.activate(id)?;
                }
                PluginActivation::OnCommand => {
                    *self.states.entry(id.clone()).or_insert(PluginLifecycleState::Registered) =
                        PluginLifecycleState::Registered;
                }
            }
        }
        Ok(self.activation_events.clone())
    }

    pub fn activate(&mut self, plugin_id: &str) -> Result<(), PluginHostError> {
        if self.disabled.contains(plugin_id) {
            return Err(PluginHostError::Disabled(plugin_id.to_string()));
        }
        let plugin = self.find_plugin(plugin_id)?.clone();
        if !plugin.manifest.kind.is_hosted() {
            return Err(PluginHostError::UnsupportedAction(
                plugin_id.to_string(),
                "activate (reserved kind)".into(),
            ));
        }
        for dep in &plugin.manifest.depends_on {
            if self.disabled.contains(dep) {
                return Err(PluginHostError::Lifecycle(LifecycleError::MissingDependency(
                    plugin_id.to_string(),
                    format!("{dep} (disabled)"),
                )));
            }
            if self.state_of(dep) != PluginLifecycleState::Active {
                self.activate(dep)?;
            }
        }
        *self.states.entry(plugin_id.to_string()).or_insert(PluginLifecycleState::Active) =
            PluginLifecycleState::Active;
        if !self.activation_events.iter().any(|e| e == plugin_id) {
            self.activation_events.push(plugin_id.to_string());
        }
        Ok(())
    }

    pub fn deactivate(&mut self, plugin_id: &str) -> Result<(), PluginHostError> {
        let _ = self.find_plugin(plugin_id)?;
        for dep in dependents_of(&self.discovered, plugin_id) {
            if self.state_of(&dep) == PluginLifecycleState::Active {
                self.deactivate(&dep)?;
            }
        }
        *self.states.entry(plugin_id.to_string()).or_insert(PluginLifecycleState::Registered) =
            PluginLifecycleState::Registered;
        self.activation_events.retain(|e| e != plugin_id);
        Ok(())
    }

    pub fn state_of(&self, plugin_id: &str) -> PluginLifecycleState {
        self.states.get(plugin_id).copied().unwrap_or(PluginLifecycleState::Discovered)
    }

    pub fn activation_events(&self) -> &[String] {
        &self.activation_events
    }

    pub fn register_validator(&mut self, plugin: Box<dyn ValidatorPlugin>) {
        self.validators.insert(plugin.id().to_string(), plugin);
    }

    pub fn register_exporter(&mut self, plugin: Box<dyn ExporterPlugin>) {
        self.exporters.insert(plugin.id().to_string(), plugin);
    }

    pub fn register_workflow(&mut self, plugin: Box<dyn WorkflowPlugin>) {
        self.workflows.insert(plugin.id().to_string(), plugin);
    }

    pub fn register_reasoner(&mut self, plugin: Box<dyn ReasonerPlugin>) {
        self.reasoners.insert(plugin.id().to_string(), plugin);
    }

    pub fn register_query(&mut self, plugin: Box<dyn QueryPlugin>) {
        self.queries.insert(plugin.id().to_string(), plugin);
    }

    pub fn register_refactor(&mut self, plugin: Box<dyn RefactorPlugin>) {
        self.refactors.insert(plugin.id().to_string(), plugin);
    }

    pub fn register_graph(&mut self, plugin: Box<dyn GraphPlugin>) {
        self.graphs.insert(plugin.id().to_string(), plugin);
    }

    pub fn workspace(&self) -> &Path {
        &self.workspace
    }

    pub fn discovered(&self) -> &[DiscoveredPlugin] {
        &self.discovered
    }

    pub fn disable_plugin(&mut self, plugin_id: &str) -> Result<(), PluginHostError> {
        let _ = self.find_plugin(plugin_id)?;
        let dependents = dependents_of(&self.discovered, plugin_id);
        for dep in &dependents {
            if dep != plugin_id {
                let _ = self.disable_plugin(dep);
            }
        }
        let _ = self.deactivate(plugin_id);
        self.disabled.insert(plugin_id.to_string());
        *self.states.entry(plugin_id.to_string()).or_insert(PluginLifecycleState::Disabled) =
            PluginLifecycleState::Disabled;
        self.validators.remove(plugin_id);
        self.exporters.remove(plugin_id);
        self.workflows.remove(plugin_id);
        self.reasoners.remove(plugin_id);
        self.queries.remove(plugin_id);
        self.refactors.remove(plugin_id);
        self.graphs.remove(plugin_id);
        persist_disabled_ids(&self.workspace, &self.disabled);
        Ok(())
    }

    pub fn enable_plugin(&mut self, plugin_id: &str) -> Result<(), PluginHostError> {
        let activation = self.find_plugin(plugin_id)?.manifest.activation;
        self.disabled.remove(plugin_id);
        persist_disabled_ids(&self.workspace, &self.disabled);
        *self.states.entry(plugin_id.to_string()).or_insert(PluginLifecycleState::Registered) =
            PluginLifecycleState::Registered;
        if matches!(activation, PluginActivation::OnStartup | PluginActivation::OnWorkspaceOpen) {
            self.activate(plugin_id)?;
        }
        Ok(())
    }

    pub fn is_disabled(&self, plugin_id: &str) -> bool {
        self.disabled.contains(plugin_id)
    }

    pub fn plugin_info(&self, plugin_id: &str) -> Result<PluginDescriptor, PluginHostError> {
        self.list_plugins()
            .into_iter()
            .find(|p| p.id == plugin_id)
            .ok_or_else(|| PluginHostError::NotFound(plugin_id.to_string()))
    }

    pub fn list_plugins(&self) -> Vec<PluginDescriptor> {
        self.discovered
            .iter()
            .map(|p| {
                let id = p.plugin_id().to_string();
                let state = self.state_of(&id);
                let disabled = self.is_disabled(&id) || state == PluginLifecycleState::Disabled;
                PluginDescriptor {
                    id: id.clone(),
                    name: p.manifest.name.clone(),
                    version: p.manifest.version.clone(),
                    kind: p.manifest.kind.as_str().to_string(),
                    api_version: p.manifest.api_version.clone(),
                    permissions: p.manifest.permissions.clone(),
                    depends_on: p.manifest.depends_on.clone(),
                    activation: p.manifest.activation.as_str().to_string(),
                    state: state.as_str().to_string(),
                    capabilities: p.manifest.capabilities.clone(),
                    manifest_path: p.manifest_path.display().to_string(),
                    ui: p.manifest.ui.clone(),
                    in_process: self.validators.contains_key(&id)
                        || self.exporters.contains_key(&id)
                        || self.workflows.contains_key(&id)
                        || self.reasoners.contains_key(&id)
                        || self.queries.contains_key(&id)
                        || self.refactors.contains_key(&id)
                        || self.graphs.contains_key(&id),
                    disabled,
                    enabled: !disabled && state == PluginLifecycleState::Active,
                }
            })
            .collect()
    }

    fn find_plugin(&self, plugin_id: &str) -> Result<&DiscoveredPlugin, PluginHostError> {
        self.discovered
            .iter()
            .find(|p| p.plugin_id() == plugin_id)
            .ok_or_else(|| PluginHostError::NotFound(plugin_id.to_string()))
    }

    fn ensure_runnable(&self, plugin_id: &str) -> Result<(), PluginHostError> {
        if self.is_disabled(plugin_id) {
            return Err(PluginHostError::Disabled(plugin_id.to_string()));
        }
        let plugin = self.find_plugin(plugin_id)?;
        // Lazily allow on_command plugins to run without prior activate_all (auto-activate).
        if self.state_of(plugin_id) != PluginLifecycleState::Active
            && plugin.manifest.activation == PluginActivation::OnCommand
        {
            // Caller must activate — return NotActive so CLI can activate first, or we activate in run_plugin_action.
            return Ok(());
        }
        if self.state_of(plugin_id) != PluginLifecycleState::Active
            && self.state_of(plugin_id) != PluginLifecycleState::Registered
        {
            return Err(PluginHostError::NotActive(plugin_id.to_string()));
        }
        Ok(())
    }

    fn ensure_permission(
        plugin: &DiscoveredPlugin,
        plugin_id: &str,
        required: PluginPermission,
    ) -> Result<(), PluginHostError> {
        if plugin.manifest.permissions.contains(&required) {
            return Ok(());
        }
        Err(PluginHostError::MissingPermission(plugin_id.to_string(), required.to_string()))
    }

    pub fn run_validate_plugin(
        &self,
        plugin_id: &str,
        catalog: &OntologyCatalog,
    ) -> Result<Vec<Diagnostic>, PluginHostError> {
        self.ensure_runnable(plugin_id)?;
        let plugin = self.find_plugin(plugin_id)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceRead)?;
        if !plugin.manifest.capabilities.supports_validation() {
            return Err(PluginHostError::UnsupportedAction(
                plugin_id.to_string(),
                "validate".into(),
            ));
        }
        if let Some(v) = self.validators.get(plugin_id) {
            return Ok(v.validate(catalog, &self.workspace));
        }
        if plugin.manifest.entry.is_some() {
            Self::ensure_permission(plugin, plugin_id, PluginPermission::ExternalProcess)?;
            Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceWrite)?;
            let output = run_plugin_subprocess(
                plugin,
                SubprocessRequest {
                    action: "validate",
                    workspace: &self.workspace,
                    step: None,
                    extra_args: &[],
                },
            )?;
            let output = jail_output_paths(plugin_id, &self.workspace, output);
            return Ok(wire_to_diagnostics(plugin_id, &self.workspace, output));
        }
        Err(PluginHostError::NotFound(format!("no runtime for plugin {plugin_id}")))
    }

    pub fn run_all_validators(&self, catalog: &OntologyCatalog) -> Vec<Diagnostic> {
        let mut all = Vec::new();
        for plugin in &self.discovered {
            if !plugin.manifest.capabilities.supports_validation() {
                continue;
            }
            let id = plugin.plugin_id();
            if self.is_disabled(id) || self.state_of(id) == PluginLifecycleState::Disabled {
                continue;
            }
            match self.run_validate_plugin(id, catalog) {
                Ok(mut diags) => all.append(&mut diags),
                Err(err) => all.push(plugin_diagnostic(
                    id,
                    "plugin_error",
                    DiagnosticSeverity::Error,
                    err.to_string(),
                    self.workspace.clone(),
                    None,
                )),
            }
        }
        all
    }

    pub fn run_export_plugin(
        &self,
        plugin_id: &str,
        catalog: &OntologyCatalog,
        options: ExportOptions,
    ) -> Result<RunPluginResult, PluginHostError> {
        self.ensure_runnable(plugin_id)?;
        let plugin = self.find_plugin(plugin_id)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceRead)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceWrite)?;
        if !plugin.manifest.capabilities.export {
            return Err(PluginHostError::UnsupportedAction(plugin_id.to_string(), "export".into()));
        }
        let options = jail_export_options(&self.workspace, options)?;
        if let Some(e) = self.exporters.get(plugin_id) {
            let paths = e
                .export(catalog, &self.workspace, options)
                .map_err(|e| PluginHostError::Export(e.to_string()))?;
            let mut result = RunPluginResult::empty_ok();
            result.output_paths = paths.iter().map(|p| p.display().to_string()).collect();
            return Ok(result);
        }
        if plugin.manifest.entry.is_some() {
            Self::ensure_permission(plugin, plugin_id, PluginPermission::ExternalProcess)?;
            let output = run_plugin_subprocess(
                plugin,
                SubprocessRequest {
                    action: "export",
                    workspace: &self.workspace,
                    step: None,
                    extra_args: &[],
                },
            )?;
            return Ok(RunPluginResult::from_output(plugin_id, &self.workspace, output, true));
        }
        Err(PluginHostError::NotFound(format!("no runtime for plugin {plugin_id}")))
    }

    pub fn run_workflow_plugin(
        &self,
        plugin_id: &str,
        step: &str,
    ) -> Result<RunPluginResult, PluginHostError> {
        self.ensure_runnable(plugin_id)?;
        let plugin = self.find_plugin(plugin_id)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceRead)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceWrite)?;
        if plugin.manifest.kind != PluginKind::Workflow && plugin.manifest.kind != PluginKind::Build
        {
            return Err(PluginHostError::UnsupportedAction(
                plugin_id.to_string(),
                "workflow".into(),
            ));
        }
        if let Some(w) = self.workflows.get(plugin_id) {
            let result =
                w.run(&self.workspace, WorkflowRequest { step: step.to_string(), dry_run: false });
            let mut out = RunPluginResult::empty_ok();
            out.diagnostics = result.diagnostics;
            out.logs = Some(result.logs);
            out.success = result.success;
            return Ok(out);
        }
        if plugin.manifest.entry.is_some() {
            Self::ensure_permission(plugin, plugin_id, PluginPermission::ExternalProcess)?;
            let output = run_plugin_subprocess(
                plugin,
                SubprocessRequest {
                    action: "workflow",
                    workspace: &self.workspace,
                    step: Some(step),
                    extra_args: &[],
                },
            )?;
            let success = output.exit_message.is_none();
            return Ok(RunPluginResult::from_output(plugin_id, &self.workspace, output, success));
        }
        Err(PluginHostError::NotFound(format!("no runtime for plugin {plugin_id}")))
    }

    pub fn run_reasoner_plugin(
        &self,
        plugin_id: &str,
        catalog: &OntologyCatalog,
    ) -> Result<RunPluginResult, PluginHostError> {
        self.ensure_runnable(plugin_id)?;
        let plugin = self.find_plugin(plugin_id)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceRead)?;
        if !plugin.manifest.capabilities.reasoner && plugin.manifest.kind != PluginKind::Reasoner {
            return Err(PluginHostError::UnsupportedAction(
                plugin_id.to_string(),
                "reasoner.classify".into(),
            ));
        }
        if let Some(r) = self.reasoners.get(plugin_id) {
            let result = r.classify(catalog, &self.workspace);
            return Ok(provider_reasoner_result(result));
        }
        self.run_subprocess_provider(plugin, "reasoner.classify", &[])
    }

    pub fn run_query_plugin(
        &self,
        plugin_id: &str,
        catalog: &OntologyCatalog,
        query: &str,
    ) -> Result<RunPluginResult, PluginHostError> {
        self.ensure_runnable(plugin_id)?;
        let plugin = self.find_plugin(plugin_id)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceRead)?;
        if !plugin.manifest.capabilities.query && plugin.manifest.kind != PluginKind::Query {
            return Err(PluginHostError::UnsupportedAction(
                plugin_id.to_string(),
                "query.run".into(),
            ));
        }
        if let Some(q) = self.queries.get(plugin_id) {
            let result = q.run(catalog, &self.workspace, query);
            return Ok(provider_query_result(result));
        }
        let extra = vec!["--query".to_string(), query.to_string()];
        self.run_subprocess_provider(plugin, "query.run", &extra)
    }

    pub fn run_refactor_plugin(
        &self,
        plugin_id: &str,
        catalog: &OntologyCatalog,
        focus_iri: Option<&str>,
    ) -> Result<RunPluginResult, PluginHostError> {
        self.ensure_runnable(plugin_id)?;
        let plugin = self.find_plugin(plugin_id)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceRead)?;
        if !plugin.manifest.capabilities.refactor && plugin.manifest.kind != PluginKind::Refactor {
            return Err(PluginHostError::UnsupportedAction(
                plugin_id.to_string(),
                "refactor.preview".into(),
            ));
        }
        if let Some(r) = self.refactors.get(plugin_id) {
            let result = r.preview(catalog, &self.workspace, focus_iri);
            return Ok(provider_refactor_result(result));
        }
        let mut extra = Vec::new();
        if let Some(iri) = focus_iri {
            extra.push("--iri".to_string());
            extra.push(iri.to_string());
        }
        self.run_subprocess_provider(plugin, "refactor.preview", &extra)
    }

    pub fn run_graph_plugin(
        &self,
        plugin_id: &str,
        catalog: &OntologyCatalog,
        root_iri: Option<&str>,
    ) -> Result<RunPluginResult, PluginHostError> {
        self.ensure_runnable(plugin_id)?;
        let plugin = self.find_plugin(plugin_id)?;
        Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceRead)?;
        if !plugin.manifest.capabilities.graph && plugin.manifest.kind != PluginKind::Graph {
            return Err(PluginHostError::UnsupportedAction(
                plugin_id.to_string(),
                "graph.build".into(),
            ));
        }
        if let Some(g) = self.graphs.get(plugin_id) {
            let result = g.build(catalog, &self.workspace, root_iri);
            return Ok(provider_graph_result(result));
        }
        let mut extra = Vec::new();
        if let Some(iri) = root_iri {
            extra.push("--root".to_string());
            extra.push(iri.to_string());
        }
        self.run_subprocess_provider(plugin, "graph.build", &extra)
    }

    fn run_subprocess_provider(
        &self,
        plugin: &DiscoveredPlugin,
        action: &str,
        extra_args: &[String],
    ) -> Result<RunPluginResult, PluginHostError> {
        let plugin_id = plugin.plugin_id();
        if plugin.manifest.entry.is_none() {
            return Err(PluginHostError::NotFound(format!("no runtime for plugin {plugin_id}")));
        }
        Self::ensure_permission(plugin, plugin_id, PluginPermission::ExternalProcess)?;
        let output = run_plugin_subprocess(
            plugin,
            SubprocessRequest { action, workspace: &self.workspace, step: None, extra_args },
        )?;
        let success = output.exit_message.is_none();
        Ok(RunPluginResult::from_output(plugin_id, &self.workspace, output, success))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn run_plugin_action(
        &mut self,
        plugin_id: &str,
        action: &str,
        catalog: Option<&OntologyCatalog>,
        export_options: Option<ExportOptions>,
        step: Option<&str>,
        view_id: Option<&str>,
        query: Option<&str>,
        focus_iri: Option<&str>,
    ) -> Result<RunPluginResult, PluginHostError> {
        // Auto-activate on_command / registered plugins when running an action.
        if !self.is_disabled(plugin_id) && self.state_of(plugin_id) != PluginLifecycleState::Active
        {
            let _ = self.activate(plugin_id);
        }
        match action {
            "validate" => {
                let catalog = catalog.ok_or_else(|| {
                    PluginHostError::UnsupportedAction(
                        plugin_id.to_string(),
                        "validate (no catalog)".into(),
                    )
                })?;
                let diags = self.run_validate_plugin(plugin_id, catalog)?;
                let mut out = RunPluginResult::empty_ok();
                out.diagnostics = diags;
                Ok(out)
            }
            "export" => {
                let catalog = catalog.ok_or_else(|| {
                    PluginHostError::UnsupportedAction(
                        plugin_id.to_string(),
                        "export (no catalog)".into(),
                    )
                })?;
                let options = export_options
                    .unwrap_or_else(|| ExportOptions::markdown(DEFAULT_PLUGIN_EXPORT_DIR));
                self.run_export_plugin(plugin_id, catalog, options)
            }
            "workflow" => self.run_workflow_plugin(plugin_id, step.unwrap_or("qc")),
            "ui_view" => {
                let plugin = self.find_plugin(plugin_id)?;
                Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceRead)?;
                // #347: subprocess ui_view can mutate the workspace — require write like validate.
                Self::ensure_permission(plugin, plugin_id, PluginPermission::WorkspaceWrite)?;
                Self::ensure_permission(plugin, plugin_id, PluginPermission::ExternalProcess)?;
                let view = view_id.ok_or_else(|| {
                    PluginHostError::UnsupportedAction(
                        plugin_id.to_string(),
                        "ui_view (missing view_id)".into(),
                    )
                })?;
                let extra = vec!["--view".to_string(), view.to_string()];
                let output = run_plugin_subprocess(
                    plugin,
                    SubprocessRequest {
                        action: "ui-view",
                        workspace: &self.workspace,
                        step: None,
                        extra_args: &extra,
                    },
                )?;
                let success = output.exit_message.is_none();
                Ok(RunPluginResult::from_output(plugin_id, &self.workspace, output, success))
            }
            "reasoner.classify" | "reasoner" => {
                let catalog = catalog.ok_or_else(|| {
                    PluginHostError::UnsupportedAction(
                        plugin_id.to_string(),
                        "reasoner.classify (no catalog)".into(),
                    )
                })?;
                self.run_reasoner_plugin(plugin_id, catalog)
            }
            "query.run" | "query" => {
                let catalog = catalog.ok_or_else(|| {
                    PluginHostError::UnsupportedAction(
                        plugin_id.to_string(),
                        "query.run (no catalog)".into(),
                    )
                })?;
                self.run_query_plugin(plugin_id, catalog, query.unwrap_or("SELECT * FROM classes"))
            }
            "refactor.preview" | "refactor" => {
                let catalog = catalog.ok_or_else(|| {
                    PluginHostError::UnsupportedAction(
                        plugin_id.to_string(),
                        "refactor.preview (no catalog)".into(),
                    )
                })?;
                self.run_refactor_plugin(plugin_id, catalog, focus_iri)
            }
            "graph.build" | "graph" => {
                let catalog = catalog.ok_or_else(|| {
                    PluginHostError::UnsupportedAction(
                        plugin_id.to_string(),
                        "graph.build (no catalog)".into(),
                    )
                })?;
                self.run_graph_plugin(plugin_id, catalog, focus_iri)
            }
            _ => Err(PluginHostError::UnsupportedAction(plugin_id.to_string(), action.into())),
        }
    }
}

fn provider_reasoner_result(result: ReasonerProviderResult) -> RunPluginResult {
    let mut out = RunPluginResult::empty_ok();
    out.unsatisfiable = Some(result.unsatisfiable.clone());
    out.profile = Some(result.profile.clone());
    out.logs = result.logs.clone();
    out.result = serde_json::to_value(&result).ok();
    out
}

fn provider_query_result(result: QueryProviderResult) -> RunPluginResult {
    let mut out = RunPluginResult::empty_ok();
    out.columns = Some(result.columns.clone());
    out.rows = Some(result.rows.clone());
    out.result = serde_json::to_value(&result).ok();
    out
}

fn provider_refactor_result(result: RefactorProviderResult) -> RunPluginResult {
    let mut out = RunPluginResult::empty_ok();
    out.affected_iris = Some(result.affected_iris.clone());
    out.hints = Some(result.hints.clone());
    out.result = serde_json::to_value(&result).ok();
    out
}

fn provider_graph_result(mut result: GraphProviderResult) -> RunPluginResult {
    let mut out = RunPluginResult::empty_ok();
    out.graph_kind = Some(result.graph_kind.clone());
    out.root_iris = Some(result.root_iris.clone());
    out.result = result.result.take().or_else(|| serde_json::to_value(&result).ok());
    out
}

fn wire_to_diagnostics(plugin_id: &str, workspace: &Path, output: PluginOutput) -> Vec<Diagnostic> {
    let mut diags: Vec<_> =
        output.diagnostics.into_iter().map(|d| d.into_diagnostic(plugin_id, workspace)).collect();
    // #348: surface path-jail violations as error diagnostics (not silent drops).
    for violation in output.violations {
        diags.push(plugin_diagnostic(
            plugin_id,
            "path_jail",
            DiagnosticSeverity::Error,
            violation,
            workspace.to_path_buf(),
            None,
        ));
    }
    diags
}

fn load_disabled_ids(workspace: &Path) -> HashSet<String> {
    let path = workspace.join(DISABLED_STATE_FILE);
    let Ok(bytes) = std::fs::read(&path) else {
        return HashSet::new();
    };
    serde_json::from_slice::<Vec<String>>(&bytes).unwrap_or_default().into_iter().collect()
}

fn persist_disabled_ids(workspace: &Path, disabled: &HashSet<String>) {
    let path = workspace.join(DISABLED_STATE_FILE);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let mut ids: Vec<_> = disabled.iter().cloned().collect();
    ids.sort();
    if let Ok(bytes) = serde_json::to_vec_pretty(&ids) {
        let _ = std::fs::write(path, bytes);
    }
}

/// Resolve export output under the workspace root (relative paths join the root, not process CWD).
fn jail_export_options(
    workspace: &Path,
    mut options: ExportOptions,
) -> Result<ExportOptions, PluginHostError> {
    let jailed = validate_workspace_scope(&options.output_dir, workspace)
        .map_err(PluginHostError::Export)?;
    options.output_dir = jailed;
    Ok(options)
}

pub fn merge_plugin_diagnostics(base: &mut Vec<Diagnostic>, plugin: Vec<Diagnostic>) {
    base.extend(plugin);
}

pub fn manifest_for_builtin(id: &str) -> Option<PluginManifest> {
    let text = match id {
        "strixonomy.naming-validator" => include_str!("../fixtures/builtin-naming.toml"),
        "strixonomy.markdown-export" => include_str!("../fixtures/builtin-markdown.toml"),
        "strixonomy.shacl-validator" => include_str!("../fixtures/builtin-shacl.toml"),
        _ => return None,
    };
    crate::manifest::parse_manifest(text).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use strixonomy_core::is_path_within;

    #[test]
    fn jail_export_options_joins_relative_to_workspace_not_cwd() {
        let dir = tempfile::tempdir().unwrap();
        let workspace = dir.path().join("ws");
        std::fs::create_dir_all(&workspace).unwrap();
        let cwd = tempfile::tempdir().unwrap();
        let prev = std::env::current_dir().unwrap();
        struct RestoreCwd(std::path::PathBuf);
        impl Drop for RestoreCwd {
            fn drop(&mut self) {
                let _ = std::env::set_current_dir(&self.0);
            }
        }
        let _restore = RestoreCwd(prev);
        std::env::set_current_dir(cwd.path()).unwrap();

        let options =
            jail_export_options(&workspace, ExportOptions::markdown(DEFAULT_PLUGIN_EXPORT_DIR))
                .expect("jail default export dir");

        let root = workspace.canonicalize().unwrap();
        assert!(is_path_within(&root, &options.output_dir));
        assert!(options.output_dir.ends_with(".strixonomy/plugin-out"));
        assert!(!cwd.path().join("plugin-out").exists());
        assert!(!cwd.path().join(".strixonomy/plugin-out").exists());
    }

    #[test]
    fn jail_export_options_rejects_absolute_outside_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let workspace = dir.path().join("ws");
        std::fs::create_dir_all(&workspace).unwrap();
        let outside = dir.path().join("outside-out");
        let err = jail_export_options(&workspace, ExportOptions::markdown(&outside)).unwrap_err();
        assert!(matches!(err, PluginHostError::Export(_)));
    }

    #[test]
    fn activate_all_orders_dependencies() {
        let dir = tempfile::tempdir().unwrap();
        let plugins = dir.path().join(".strixonomy/plugins");
        std::fs::create_dir_all(&plugins).unwrap();
        std::fs::write(
            plugins.join("a.toml"),
            r#"
[plugin]
name = "a"
version = "0.1.0"
kind = "validator"
id = "a"
api_version = "1"
permissions = ["workspace.read"]
[capabilities]
validate = true
"#,
        )
        .unwrap();
        std::fs::write(
            plugins.join("b.toml"),
            r#"
[plugin]
name = "b"
version = "0.1.0"
kind = "graph"
id = "b"
api_version = "1"
depends_on = ["a"]
permissions = ["workspace.read"]
[capabilities]
graph = true
"#,
        )
        .unwrap();
        let mut host = PluginHost::new(dir.path());
        host.discover().expect("discover");
        let order = host.activate_all().expect("activate");
        assert_eq!(order, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(host.state_of("a"), PluginLifecycleState::Active);
        assert_eq!(host.state_of("b"), PluginLifecycleState::Active);
    }

    #[test]
    fn enable_after_disable_reregisters_in_process_validator() {
        struct StubValidator;
        impl crate::traits::ValidatorPlugin for StubValidator {
            fn id(&self) -> &str {
                "stub.validator"
            }
            fn validate(
                &self,
                _catalog: &strixonomy_catalog::OntologyCatalog,
                _workspace: &std::path::Path,
            ) -> Vec<strixonomy_core::Diagnostic> {
                Vec::new()
            }
        }

        let dir = tempfile::tempdir().unwrap();
        let plugins = dir.path().join(".strixonomy/plugins");
        std::fs::create_dir_all(&plugins).unwrap();
        std::fs::write(
            plugins.join("stub.toml"),
            r#"
[plugin]
name = "stub"
version = "0.1.0"
kind = "validator"
id = "stub.validator"
api_version = "1"
permissions = ["workspace.read"]

[capabilities]
validate = true
"#,
        )
        .unwrap();
        let mut host = PluginHost::new(dir.path());
        host.discover().unwrap();
        host.register_validator(Box::new(StubValidator));
        host.activate_all().unwrap();
        host.disable_plugin("stub.validator").unwrap();
        host.enable_plugin("stub.validator").unwrap();
        let catalog = strixonomy_catalog::IndexBuilder::new()
            .workspace(dir.path())
            .build()
            .expect("catalog");
        let err = host
            .run_validate_plugin("stub.validator", &catalog)
            .expect_err("validator must be re-registered after enable");
        assert!(
            err.to_string().contains("no runtime"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn disable_cascades_dependents() {
        let dir = tempfile::tempdir().unwrap();
        let plugins = dir.path().join(".strixonomy/plugins");
        std::fs::create_dir_all(&plugins).unwrap();
        std::fs::write(
            plugins.join("a.toml"),
            r#"
[plugin]
name = "a"
version = "0.1.0"
kind = "validator"
id = "a"
api_version = "1"
permissions = ["workspace.read"]
"#,
        )
        .unwrap();
        std::fs::write(
            plugins.join("b.toml"),
            r#"
[plugin]
name = "b"
version = "0.1.0"
kind = "graph"
id = "b"
api_version = "1"
depends_on = ["a"]
permissions = ["workspace.read"]
"#,
        )
        .unwrap();
        let mut host = PluginHost::new(dir.path());
        host.discover().unwrap();
        host.activate_all().unwrap();
        host.disable_plugin("a").unwrap();
        assert_eq!(host.state_of("a"), PluginLifecycleState::Disabled);
        assert_eq!(host.state_of("b"), PluginLifecycleState::Disabled);
    }
}
