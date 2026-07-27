//! Plugin SDK 1.0 compatibility / regression harness.
//!
//! Covers manifest schema, lifecycle order, permissions, and provider golden path.

use std::path::Path;
use strixonomy_catalog::IndexBuilder;
use strixonomy_plugin::{parse_manifest, PluginHost, PluginHostError, PluginLifecycleState};
use strixonomy_plugin_builtins::load_plugin_host;

fn write_executable(path: &Path, body: &str) {
    std::fs::write(path, body).expect("write script");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(path).expect("meta").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(path, perms).expect("chmod");
    }
}

#[test]
fn manifest_rejects_bad_api_version_and_reserved_kind() {
    assert!(parse_manifest(
        r#"
[plugin]
name = "x"
version = "0.1.0"
kind = "validator"
api_version = "2"
"#
    )
    .is_err());
    assert!(parse_manifest(
        r#"
[plugin]
name = "x"
version = "0.1.0"
kind = "editor"
"#
    )
    .is_err());
}

#[test]
fn manifest_accepts_provider_kinds_and_depends_on() {
    let m = parse_manifest(
        r#"
[plugin]
name = "g"
version = "0.1.0"
kind = "graph"
id = "g"
api_version = "1"
depends_on = ["r"]
activation = "on_command"
permissions = ["workspace.read"]

[capabilities]
graph = true
"#,
    )
    .expect("parse");
    assert_eq!(m.depends_on, vec!["r"]);
    assert!(m.capabilities.graph);
}

#[test]
fn missing_workspace_write_fails_subprocess_validate() {
    let dir = tempfile::tempdir().unwrap();
    let workspace = dir.path().join("ws");
    std::fs::create_dir_all(workspace.join(".strixonomy/plugins")).unwrap();
    std::fs::write(workspace.join("demo.ttl"), "@prefix ex: <http://ex/> .\nex:A a owl:Class .\n")
        .unwrap();
    let bin = workspace.join("stub.sh");
    write_executable(&bin, "#!/bin/sh\necho '{\"diagnostics\":[]}'\n");
    std::fs::write(
        workspace.join(".strixonomy/plugins/stub.toml"),
        format!(
            r#"
[plugin]
name = "stub"
version = "0.1.0"
kind = "validator"
id = "org.example.stub"
api_version = "1"
entry = "{}"
permissions = ["workspace.read", "external_process"]

[capabilities]
validate = true
"#,
            bin.display()
        ),
    )
    .unwrap();

    let catalog = IndexBuilder::new().workspace(workspace.clone()).build().unwrap();
    let host = load_plugin_host(&workspace).unwrap();
    let err = host.run_validate_plugin("org.example.stub", &catalog).unwrap_err();
    match err {
        PluginHostError::MissingPermission(_, p) => assert_eq!(p, "workspace.write"),
        other => panic!("expected missing workspace.write, got {other}"),
    }
}

#[test]
fn lifecycle_activation_order_and_disable_persist() {
    let dir = tempfile::tempdir().unwrap();
    let workspace = dir.path().join("ws");
    let plugins = workspace.join(".strixonomy/plugins");
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

[capabilities]
graph = true
"#,
    )
    .unwrap();

    let mut host = PluginHost::new(&workspace);
    host.discover().unwrap();
    let order = host.activate_all().unwrap();
    assert_eq!(order, vec!["a".to_string(), "b".to_string()]);

    host.disable_plugin("a").unwrap();
    assert_eq!(host.state_of("a"), PluginLifecycleState::Disabled);
    assert_eq!(host.state_of("b"), PluginLifecycleState::Disabled);

    // Persist across reload.
    let host2 = PluginHost::new(&workspace);
    let mut host2 = host2;
    host2.discover().unwrap();
    assert!(host2.is_disabled("a"));
    host2.enable_plugin("a").unwrap();
    assert!(!host2.is_disabled("a"));
}

#[test]
fn provider_golden_path_discover_activate_run() {
    let dir = tempfile::tempdir().unwrap();
    let workspace = dir.path().join("ws");
    std::fs::create_dir_all(workspace.join(".strixonomy/plugins")).unwrap();
    std::fs::write(
        workspace.join("demo.ttl"),
        "@prefix ex: <http://ex/> .\nex:Person a owl:Class .\n",
    )
    .unwrap();
    let bin = workspace.join("providers.sh");
    write_executable(
        &bin,
        r#"#!/bin/sh
ACTION="$1"
case "$ACTION" in
  reasoner.classify) echo '{"profile":"t","unsatisfiable":[]}' ;;
  query.run) echo '{"columns":["iri"],"rows":[["http://ex/Person"]]}' ;;
  refactor.preview) echo '{"affected_iris":["http://ex/Person"],"hints":["ok"]}' ;;
  graph.build) echo '{"graph_kind":"n","root_iris":["http://ex/Person"]}' ;;
  *) echo '{"exit_message":"bad"}'; exit 1 ;;
esac
"#,
    );
    for (name, kind, id, caps) in [
        ("reasoner", "reasoner", "org.example.reasoner", "reasoner = true"),
        ("query", "query", "org.example.query", "query = true"),
        ("refactor", "refactor", "org.example.refactor", "refactor = true"),
        ("graph", "graph", "org.example.graph", "graph = true"),
    ] {
        std::fs::write(
            workspace.join(".strixonomy/plugins").join(format!("{name}.toml")),
            format!(
                r#"
[plugin]
name = "{name}"
version = "0.1.0"
kind = "{kind}"
id = "{id}"
api_version = "1"
entry = "{entry}"
activation = "on_command"
permissions = ["workspace.read", "external_process"]

[capabilities]
{caps}
"#,
                entry = bin.display()
            ),
        )
        .unwrap();
    }

    let catalog = IndexBuilder::new().workspace(workspace.clone()).build().unwrap();
    let mut host = load_plugin_host(&workspace).unwrap();

    let reasoner = host
        .run_plugin_action(
            "org.example.reasoner",
            "reasoner.classify",
            Some(&catalog),
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
    assert!(reasoner.success);
    assert_eq!(reasoner.profile.as_deref(), Some("t"));

    let query = host
        .run_plugin_action(
            "org.example.query",
            "query.run",
            Some(&catalog),
            None,
            None,
            None,
            Some("SELECT *"),
            None,
        )
        .unwrap();
    assert_eq!(query.columns.as_deref(), Some(["iri".to_string()].as_slice()));

    let refactor = host
        .run_plugin_action(
            "org.example.refactor",
            "refactor.preview",
            Some(&catalog),
            None,
            None,
            None,
            None,
            Some("http://ex/Person"),
        )
        .unwrap();
    assert_eq!(
        refactor.affected_iris.as_deref(),
        Some(["http://ex/Person".to_string()].as_slice())
    );

    let graph = host
        .run_plugin_action(
            "org.example.graph",
            "graph.build",
            Some(&catalog),
            None,
            None,
            None,
            None,
            Some("http://ex/Person"),
        )
        .unwrap();
    assert_eq!(graph.graph_kind.as_deref(), Some("n"));
}

#[test]
fn cycle_in_depends_on_fails_discover() {
    let dir = tempfile::tempdir().unwrap();
    let workspace = dir.path().join("ws");
    let plugins = workspace.join(".strixonomy/plugins");
    std::fs::create_dir_all(&plugins).unwrap();
    std::fs::write(
        plugins.join("a.toml"),
        r#"
[plugin]
name = "a"
version = "0.1.0"
kind = "validator"
id = "a"
depends_on = ["b"]
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
kind = "validator"
id = "b"
depends_on = ["a"]
permissions = ["workspace.read"]
"#,
    )
    .unwrap();
    let mut host = PluginHost::new(&workspace);
    assert!(host.discover().is_err());
}
