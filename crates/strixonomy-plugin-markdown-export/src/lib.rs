use std::path::{Path, PathBuf};
use strixonomy_catalog::OntologyCatalog;
use strixonomy_docs::{export_workspace, ExportOptions};
use strixonomy_plugin::ExporterPlugin;

pub const PLUGIN_ID: &str = "strixonomy.markdown-export";

pub struct MarkdownExportPlugin;

impl ExporterPlugin for MarkdownExportPlugin {
    fn id(&self) -> &str {
        PLUGIN_ID
    }

    fn export(
        &self,
        catalog: &OntologyCatalog,
        _workspace: &Path,
        options: ExportOptions,
    ) -> Result<Vec<PathBuf>, strixonomy_docs::ExportError> {
        export_workspace(catalog, options)?;
        Ok(vec![])
    }
}
