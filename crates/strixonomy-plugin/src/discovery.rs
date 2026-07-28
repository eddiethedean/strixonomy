use crate::manifest::{parse_manifest, DiscoveredPlugin};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Relative directory scanned for plugin manifests inside a workspace.
pub const PLUGIN_DIR: &str = ".strixonomy/plugins";

#[derive(Debug, Error)]
pub enum PluginDiscoveryError {
    #[error("IO error reading {path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },
    #[error("invalid manifest {path}: {message}")]
    InvalidManifest { path: PathBuf, message: String },
}

/// Discover plugin manifests under `.strixonomy/plugins/`.
pub fn discover_plugins(workspace: &Path) -> Result<Vec<DiscoveredPlugin>, PluginDiscoveryError> {
    let plugins_dir = strixonomy_core::plugins_dir(workspace);
    if !plugins_dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut discovered = Vec::new();
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
        discovered.push(DiscoveredPlugin { manifest, manifest_path: path });
    }
    Ok(discovered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_plugins_dir() {
        let dir = tempfile::tempdir().unwrap();
        let plugins = dir.path().join(PLUGIN_DIR);
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
