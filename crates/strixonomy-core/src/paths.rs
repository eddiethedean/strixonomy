//! Workspace configuration directory resolution (`.strixonomy`).

use std::path::{Path, PathBuf};

/// Workspace config directory.
pub const CONFIG_DIR: &str = ".strixonomy";

/// Relative cache directory under the config dir.
pub const CACHE_SUBDIR: &str = "cache";

/// Relative plugins directory under the config dir.
pub const PLUGINS_SUBDIR: &str = "plugins";

/// Diagnostics config filename under the config dir.
pub const DIAGNOSTICS_FILE: &str = "diagnostics.toml";

/// Default plugin export directory (relative to workspace).
pub const PLUGIN_OUT_REL: &str = ".strixonomy/plugin-out";

/// Disabled-plugins state file (relative to workspace).
pub const PLUGIN_DISABLED_REL: &str = ".strixonomy/plugin-disabled.json";

/// Resolve `{workspace}/.strixonomy/{leaf}`.
pub fn resolve_config_path(workspace: &Path, leaf: &str) -> PathBuf {
    workspace.join(CONFIG_DIR).join(leaf)
}

/// Resolve a workspace-relative dotted path (e.g. `.strixonomy/cache`).
pub fn resolve_dotted_config_path(workspace: &Path, rel: &str) -> PathBuf {
    workspace.join(rel)
}

/// Primary plugins directory path (may not exist yet).
pub fn plugins_dir(workspace: &Path) -> PathBuf {
    resolve_config_path(workspace, PLUGINS_SUBDIR)
}

/// Plugin directories to scan (primary only).
pub fn plugin_search_dirs(workspace: &Path) -> Vec<PathBuf> {
    vec![plugins_dir(workspace)]
}

/// Cache root under the workspace.
pub fn cache_dir(workspace: &Path) -> PathBuf {
    resolve_config_path(workspace, CACHE_SUBDIR)
}

/// Diagnostics.toml path.
pub fn diagnostics_config_path(workspace: &Path) -> PathBuf {
    resolve_config_path(workspace, DIAGNOSTICS_FILE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn uses_primary_config_dir() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".strixonomy/cache")).unwrap();
        assert_eq!(cache_dir(dir.path()), dir.path().join(".strixonomy/cache"));
    }

    #[test]
    fn defaults_to_primary_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(
            diagnostics_config_path(dir.path()),
            dir.path().join(".strixonomy/diagnostics.toml")
        );
        assert_eq!(plugins_dir(dir.path()), dir.path().join(".strixonomy/plugins"));
    }
}
