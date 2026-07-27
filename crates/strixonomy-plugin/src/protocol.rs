use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use strixonomy_core::{
    validate_workspace_scope, Diagnostic, DiagnosticCode, DiagnosticSeverity, SourceLocation,
};

/// JSON wire format returned by subprocess plugins on stdout.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginOutput {
    #[serde(default)]
    pub diagnostics: Vec<PluginDiagnosticWire>,
    #[serde(default)]
    pub output_paths: Vec<String>,
    #[serde(default)]
    pub logs: Option<String>,
    #[serde(default)]
    pub exit_message: Option<String>,
    /// Optional HTML payload for UI view contributions.
    #[serde(default)]
    pub view_html: Option<String>,
    /// Non-fatal security violations (e.g. rejected paths) detected by the host.
    #[serde(default)]
    pub violations: Vec<String>,
    /// Provider payload (reasoner / query / refactor / graph) — additive in SDK 1.0.
    #[serde(default)]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub columns: Option<Vec<String>>,
    #[serde(default)]
    pub rows: Option<Vec<Vec<String>>>,
    #[serde(default)]
    pub unsatisfiable: Option<Vec<String>>,
    #[serde(default)]
    pub affected_iris: Option<Vec<String>>,
    #[serde(default)]
    pub root_iris: Option<Vec<String>>,
    #[serde(default)]
    pub graph_kind: Option<String>,
    #[serde(default)]
    pub hints: Option<Vec<String>>,
    #[serde(default)]
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDiagnosticWire {
    pub code: String,
    pub severity: String,
    pub message: String,
    pub file: String,
    #[serde(default)]
    pub line: Option<u64>,
    #[serde(default)]
    pub column: Option<u64>,
    #[serde(default)]
    pub entity_iri: Option<String>,
}

impl PluginDiagnosticWire {
    pub fn into_diagnostic(self, plugin_id: &str, workspace: &Path) -> Diagnostic {
        let requested = PathBuf::from(&self.file);
        let (file, message) = match validate_workspace_scope(&requested, workspace) {
            Ok(file) => (file, self.message),
            Err(err) => (
                workspace.to_path_buf(),
                format!(
                    "{} (rejected plugin diagnostic file path '{}': {})",
                    self.message, self.file, err
                ),
            ),
        };
        Diagnostic {
            code: DiagnosticCode::PluginViolation,
            severity: match self.severity.as_str() {
                "error" => DiagnosticSeverity::Error,
                "warning" => DiagnosticSeverity::Warning,
                _ => DiagnosticSeverity::Info,
            },
            message,
            file,
            range: SourceLocation {
                line: self.line,
                column: self.column,
                start_byte: None,
                end_byte: None,
            },
            entity_iri: self.entity_iri,
            quick_fix: None,
            plugin_id: Some(plugin_id.to_string()),
            plugin_code: Some(self.code),
        }
    }
}

pub fn jail_output_paths(
    plugin_id: &str,
    workspace: &Path,
    mut output: PluginOutput,
) -> PluginOutput {
    let mut jailed = Vec::new();
    for p in output.output_paths.drain(..) {
        let requested = PathBuf::from(&p);
        match validate_workspace_scope(&requested, workspace) {
            Ok(path) => jailed.push(path.display().to_string()),
            Err(err) => output
                .violations
                .push(format!("rejected plugin output path '{p}' for {plugin_id}: {err}")),
        }
    }
    output.output_paths = jailed;
    output
}

pub fn plugin_diagnostic(
    plugin_id: &str,
    code: &str,
    severity: DiagnosticSeverity,
    message: impl Into<String>,
    file: PathBuf,
    entity_iri: Option<String>,
) -> Diagnostic {
    Diagnostic {
        code: DiagnosticCode::PluginViolation,
        severity,
        message: message.into(),
        file,
        range: SourceLocation::default(),
        entity_iri,
        quick_fix: None,
        plugin_id: Some(plugin_id.to_string()),
        plugin_code: Some(code.to_string()),
    }
}
