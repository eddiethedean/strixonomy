//! Workspace configuration directory resolution (`.strixonomy` primary, `.ontocore` legacy).

use std::path::{Path, PathBuf};

/// Primary workspace config directory (v0.27+).
pub const CONFIG_DIR: &str = ".strixonomy";

/// Legacy OntoCore config directory (accepted through at least 1.0).
pub const LEGACY_CONFIG_DIR: &str = ".ontocore";

/// Relative cache directory under the config dir.
pub const CACHE_SUBDIR: &str = "cache";

/// Relative plugins directory under the config dir.
pub const PLUGINS_SUBDIR: &str = "plugins";

/// Diagnostics config filename under the config dir.
pub const DIAGNOSTICS_FILE: &str = "diagnostics.toml";

/// Default plugin export directory (relative to workspace).
pub const PLUGIN_OUT_REL: &str = ".strixonomy/plugin-out";

/// Legacy plugin export directory.
pub const LEGACY_PLUGIN_OUT_REL: &str = ".ontocore/plugin-out";

/// Disabled-plugins state file (relative to workspace).
pub const PLUGIN_DISABLED_REL: &str = ".strixonomy/plugin-disabled.json";

/// Legacy disabled-plugins state file.
pub const LEGACY_PLUGIN_DISABLED_REL: &str = ".ontocore/plugin-disabled.json";

/// Resolve `{workspace}/.strixonomy/{leaf}` preferring an existing primary path,
/// else an existing legacy `.ontocore/{leaf}`, else the primary path for writes.
pub fn resolve_config_path(workspace: &Path, leaf: &str) -> PathBuf {
    let primary = workspace.join(CONFIG_DIR).join(leaf);
    if primary.exists() {
        return primary;
    }
    let legacy = workspace.join(LEGACY_CONFIG_DIR).join(leaf);
    if legacy.exists() {
        return legacy;
    }
    primary
}

/// Like [`resolve_config_path`] but for a relative path that already includes the
/// config dir prefix (e.g. `.strixonomy/cache`).
pub fn resolve_dotted_config_path(
    workspace: &Path,
    primary_rel: &str,
    legacy_rel: &str,
) -> PathBuf {
    let primary = workspace.join(primary_rel);
    if primary.exists() {
        return primary;
    }
    let legacy = workspace.join(legacy_rel);
    if legacy.exists() {
        return legacy;
    }
    primary
}

/// Primary plugins directory path (may not exist yet).
pub fn plugins_dir(workspace: &Path) -> PathBuf {
    resolve_config_path(workspace, PLUGINS_SUBDIR)
}

/// All plugin directories to scan (primary first, then legacy if distinct and present).
pub fn plugin_search_dirs(workspace: &Path) -> Vec<PathBuf> {
    let primary = workspace.join(CONFIG_DIR).join(PLUGINS_SUBDIR);
    let legacy = workspace.join(LEGACY_CONFIG_DIR).join(PLUGINS_SUBDIR);
    let mut dirs = Vec::new();
    if primary.is_dir() {
        dirs.push(primary.clone());
    }
    if legacy.is_dir() && legacy != primary {
        dirs.push(legacy);
    }
    if dirs.is_empty() {
        dirs.push(primary);
    }
    dirs
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
    fn prefers_primary_when_present() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".strixonomy/cache")).unwrap();
        fs::create_dir_all(dir.path().join(".ontocore/cache")).unwrap();
        assert_eq!(cache_dir(dir.path()), dir.path().join(".strixonomy/cache"));
    }

    #[test]
    fn falls_back_to_legacy() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".ontocore/plugins")).unwrap();
        assert_eq!(plugins_dir(dir.path()), dir.path().join(".ontocore/plugins"));
    }

    #[test]
    fn defaults_to_primary_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(
            diagnostics_config_path(dir.path()),
            dir.path().join(".strixonomy/diagnostics.toml")
        );
    }
}
