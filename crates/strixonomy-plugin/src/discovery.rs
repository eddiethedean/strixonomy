use crate::manifest::{parse_manifest, DiscoveredPlugin};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Relative directory scanned for plugin manifests inside a workspace (primary).
pub const PLUGIN_DIR: &str = ".strixonomy/plugins";

/// Legacy OntoCore plugins directory.
#[allow(dead_code)] // public for callers / tests; discovery uses `plugin_search_dirs`
pub const LEGACY_PLUGIN_DIR: &str = ".ontocore/plugins";

#[derive(Debug, Error)]
pub enum PluginDiscoveryError {
    #[error("IO error reading {path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },
    #[error("invalid manifest {path}: {message}")]
    InvalidManifest { path: PathBuf, message: String },
}

/// Discover plugin manifests under `.strixonomy/plugins/` and legacy `.ontocore/plugins/`.
pub fn discover_plugins(workspace: &Path) -> Result<Vec<DiscoveredPlugin>, PluginDiscoveryError> {
    let mut discovered = Vec::new();
    let mut seen_ids = std::collections::HashSet::new();
    for plugins_dir in strixonomy_core::plugin_search_dirs(workspace) {
        if !plugins_dir.is_dir() {
            continue;
        }
        for entry in fs::read_dir(&plugins_dir)
            .map_err(|source| PluginDiscoveryError::Io { path: plugins_dir.clone(), source })?
        {
            let entry = entry
                .map_err(|source| PluginDiscoveryError::Io { path: plugins_dir.clone(), source })?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("toml") {
                continue;
            }
            let text = fs::read_to_string(&path)
                .map_err(|source| PluginDiscoveryError::Io { path: path.clone(), source })?;
            let manifest = parse_manifest(&text).map_err(|e| {
                PluginDiscoveryError::InvalidManifest { path: path.clone(), message: e.to_string() }
            })?;
            if !seen_ids.insert(manifest.id.clone()) {
                // Primary (.strixonomy) wins when both dirs define the same id.
                continue;
            }
            discovered.push(DiscoveredPlugin { manifest, manifest_path: path });
        }
    }
    Ok(discovered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_legacy_plugins_dir() {
        let dir = tempfile::tempdir().unwrap();
        let plugins = dir.path().join(LEGACY_PLUGIN_DIR);
        fs::create_dir_all(&plugins).unwrap();
        fs::write(
            plugins.join("demo.toml"),
            r#"
[plugin]
id = "demo"
name = "Demo"
version = "0.1.0"
kind = "validator"
entry = "./demo"
"#,
        )
        .unwrap();
        let found = discover_plugins(dir.path()).unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].manifest.id.as_deref(), Some("demo"));
    }
}
