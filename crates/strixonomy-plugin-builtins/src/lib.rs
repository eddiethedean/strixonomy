//! Register built-in reference plugins on a [`PluginHost`].
use std::path::Path;
use strixonomy_plugin::PluginHost;
use strixonomy_plugin_markdown_export::MarkdownExportPlugin;
use strixonomy_plugin_naming::NamingValidatorPlugin;
use strixonomy_plugin_shacl::ShaclValidatorPlugin;

pub fn load_plugin_host(
    workspace: impl AsRef<Path>,
) -> Result<PluginHost, strixonomy_plugin::PluginHostError> {
    let mut host = PluginHost::new(workspace.as_ref());
    host.discover()?;
    register_builtins(&mut host);
    host.activate_all()?;
    Ok(host)
}

pub fn register_builtins(host: &mut PluginHost) {
    for plugin in host.discovered().to_vec() {
        let id = plugin.plugin_id().to_string();
        match id.as_str() {
            "strixonomy.naming-validator" => {
                let cfg = &plugin.manifest.config;
                host.register_validator(Box::new(NamingValidatorPlugin::from_config(
                    cfg.require_label || plugin.manifest.capabilities.validate,
                    cfg.iri_prefix.clone(),
                )));
            }
            "strixonomy.markdown-export" => {
                host.register_exporter(Box::new(MarkdownExportPlugin));
            }
            "strixonomy.shacl-validator" => {
                let shapes = plugin.manifest.config.shapes_dir.clone();
                host.register_validator(Box::new(ShaclValidatorPlugin::new(
                    host.workspace(),
                    shapes.as_deref(),
                )));
            }
            _ => {}
        }
    }
}

/// Ensure built-in manifests are discoverable when workspace has no `.strixonomy/plugins/`.
pub fn ensure_builtin_manifests(workspace: &Path) -> std::io::Result<()> {
    let dir = workspace.join(strixonomy_plugin::PLUGIN_DIR);
    std::fs::create_dir_all(&dir)?;
    let naming = dir.join("naming-validator.toml");
    if !naming.exists() {
        std::fs::write(naming, include_str!("../fixtures/plugins/naming-validator.toml"))?;
    }
    Ok(())
}
