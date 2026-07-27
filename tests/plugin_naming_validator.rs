//! Naming validator plugin integration test.

use std::path::PathBuf;
use strixonomy_catalog::IndexBuilder;
use strixonomy_plugin_builtins::load_plugin_host;

fn plugin_workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/plugin-workspace")
}

#[test]
fn naming_validator_flags_unlabeled_class() {
    let workspace = plugin_workspace();
    let catalog = IndexBuilder::new().workspace(&workspace).build().expect("index workspace");
    let host = load_plugin_host(&workspace).expect("load plugins");
    let diags = host
        .run_validate_plugin("strixonomy.naming-validator", &catalog)
        .expect("run naming validator");
    assert!(
        diags.iter().any(|d| {
            d.plugin_code.as_deref() == Some("missing_label")
                && d.message.contains("UnlabeledClass")
        }),
        "expected missing_label for UnlabeledClass, got: {diags:?}"
    );
}

#[test]
fn plugins_list_discovers_naming_manifest() {
    let workspace = plugin_workspace();
    let host = load_plugin_host(&workspace).expect("load plugins");
    let plugins = host.list_plugins();
    assert!(plugins.iter().any(|p| p.id == "strixonomy.naming-validator"));
}
