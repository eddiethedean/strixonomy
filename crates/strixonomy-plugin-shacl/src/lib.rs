use std::path::{Path, PathBuf};
use strixonomy_catalog::OntologyCatalog;
use strixonomy_core::DiagnosticSeverity;
use strixonomy_plugin::{plugin_diagnostic, ValidatorPlugin};

pub const PLUGIN_ID: &str = "strixonomy.shacl-validator";

#[derive(Debug, Clone)]
pub struct ShaclValidatorPlugin {
    pub shapes_dir: PathBuf,
}

impl ShaclValidatorPlugin {
    pub fn new(workspace: &Path, shapes_dir: Option<&str>) -> Self {
        let rel = shapes_dir.unwrap_or("shapes");
        Self { shapes_dir: workspace.join(rel) }
    }
}

impl ValidatorPlugin for ShaclValidatorPlugin {
    fn id(&self) -> &str {
        PLUGIN_ID
    }

    fn validate(
        &self,
        _catalog: &OntologyCatalog,
        workspace: &Path,
    ) -> Vec<strixonomy_core::Diagnostic> {
        if !self.shapes_dir.is_dir() {
            return vec![plugin_diagnostic(
                PLUGIN_ID,
                "shapes_missing",
                DiagnosticSeverity::Info,
                format!(
                    "SHACL shapes directory '{}' not found; add .ttl shape files to enable validation",
                    self.shapes_dir.display()
                ),
                workspace.to_path_buf(),
                None,
            )];
        }
        let shape_files: Vec<_> = std::fs::read_dir(&self.shapes_dir)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|x| x.to_str())
                    .is_some_and(|ext| matches!(ext, "ttl" | "rdf" | "shacl"))
            })
            .collect();
        if shape_files.is_empty() {
            return vec![plugin_diagnostic(
                PLUGIN_ID,
                "shapes_empty",
                DiagnosticSeverity::Info,
                format!("No SHACL shape files in '{}'", self.shapes_dir.display()),
                workspace.to_path_buf(),
                None,
            )];
        }
        vec![plugin_diagnostic(
            PLUGIN_ID,
            "shacl_pending",
            DiagnosticSeverity::Info,
            format!(
                "Found {} SHACL shape file(s); full rudof validation ships in a future release",
                shape_files.len()
            ),
            workspace.to_path_buf(),
            None,
        )]
    }
}
