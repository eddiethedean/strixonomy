//! Security-focused plugin host integration tests.
//!
//! These tests validate that untrusted subprocess plugins cannot escape workspace path jails.

mod support;

use strixonomy_catalog::IndexBuilder;
use strixonomy_core::is_path_within;
use strixonomy_plugin_builtins::load_plugin_host;

#[test]
fn plugin_diagnostic_file_allows_workspace_escape_via_dotdot() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().join("ws");
    std::fs::create_dir_all(workspace.join(".strixonomy/plugins")).expect("plugins dir");

    // Minimal ontology so `build_catalog()` succeeds.
    std::fs::write(workspace.join("demo.ttl"), "@prefix ex: <http://ex/> .\nex:Ok a owl:Class .\n")
        .expect("write demo.ttl");

    // Create a subprocess plugin script that emits a diagnostic pointing outside the workspace.
    let plugin_bin = workspace.join("evil_plugin.sh");
    std::fs::write(
        &plugin_bin,
        r#"#!/bin/sh
echo '{"diagnostics":[{"code":"pwn","severity":"error","message":"escape","file":"../../etc/passwd"}]}'"#,
    )
    .expect("write evil plugin");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&plugin_bin).expect("metadata").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&plugin_bin, perms).expect("chmod");
    }

    // Plugin manifest. Use an absolute entry path so discovery resolves it directly.
    let manifest = format!(
        r#"[plugin]
name = "evil"
version = "0.1.0"
kind = "validator"
id = "org.example.evil"
api_version = "1"
entry = "{}"
permissions = ["workspace.read","workspace.write","external_process"]

[capabilities]
validate = true
diagnostics = true
"#,
        plugin_bin.display()
    );
    std::fs::write(workspace.join(".strixonomy/plugins/evil.toml"), manifest.as_bytes())
        .expect("write manifest");

    // Run via the Rust API (avoids coupling to the CLI argument contract).
    let catalog =
        IndexBuilder::new().workspace(workspace.clone()).build().expect("index workspace");
    let host = load_plugin_host(&workspace).expect("load plugin host");
    let result =
        host.run_validate_plugin("org.example.evil", &catalog).expect("run validator plugin");
    let file = &result.first().expect("one diagnostic").file;

    let root = workspace.canonicalize().expect("canonical root");
    assert!(is_path_within(&root, file), "plugin diagnostic file must be jailed");
}

#[test]
fn subprocess_validate_requires_workspace_write() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().join("ws");
    std::fs::create_dir_all(workspace.join(".strixonomy/plugins")).expect("plugins dir");
    std::fs::write(workspace.join("demo.ttl"), "@prefix ex: <http://ex/> .\nex:Ok a owl:Class .\n")
        .expect("write demo.ttl");

    let plugin_bin = workspace.join("readonly_plugin.sh");
    std::fs::write(
        &plugin_bin,
        r#"#!/bin/sh
echo '{"diagnostics":[]}'"#,
    )
    .expect("write plugin");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&plugin_bin).expect("metadata").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&plugin_bin, perms).expect("chmod");
    }

    let manifest = format!(
        r#"[plugin]
name = "readonly-validator"
version = "0.1.0"
kind = "validator"
id = "org.example.readonly"
api_version = "1"
entry = "{}"
permissions = ["workspace.read","external_process"]

[capabilities]
validate = true
diagnostics = true
"#,
        plugin_bin.display()
    );
    std::fs::write(workspace.join(".strixonomy/plugins/readonly.toml"), manifest.as_bytes())
        .expect("write manifest");

    let catalog =
        IndexBuilder::new().workspace(workspace.clone()).build().expect("index workspace");
    let host = load_plugin_host(&workspace).expect("load plugin host");
    let err = host
        .run_validate_plugin("org.example.readonly", &catalog)
        .expect_err("subprocess validate without workspace.write must fail (#315)");
    let msg = err.to_string();
    assert!(
        msg.contains("WorkspaceWrite")
            || msg.contains("workspace.write")
            || msg.contains("Missing"),
        "expected missing write permission, got {msg}"
    );
}

#[test]
fn plugin_diagnostic_file_rejects_absolute_outside_workspace() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().join("ws");
    std::fs::create_dir_all(workspace.join(".strixonomy/plugins")).expect("plugins dir");

    // Minimal ontology so `build_catalog()` succeeds.
    std::fs::write(workspace.join("demo.ttl"), "@prefix ex: <http://ex/> .\nex:Ok a owl:Class .\n")
        .expect("write demo.ttl");

    // Create a real file outside the workspace and reference it via an absolute path.
    let outside = dir.path().join("outside");
    std::fs::create_dir_all(&outside).expect("outside dir");
    let secret = outside.join("secret.ttl");
    std::fs::write(&secret, "@prefix ex: <http://ex/> .\nex:Secret a owl:Class .\n")
        .expect("write secret.ttl");

    let plugin_bin = workspace.join("evil_abs_plugin.sh");
    std::fs::write(
        &plugin_bin,
        format!(
            r#"#!/bin/sh
echo '{{"diagnostics":[{{"code":"pwn","severity":"error","message":"escape-abs","file":"{}"}}]}}'"#,
            secret.display()
        ),
    )
    .expect("write evil abs plugin");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&plugin_bin).expect("metadata").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&plugin_bin, perms).expect("chmod");
    }

    let manifest = format!(
        r#"[plugin]
name = "evil-abs"
version = "0.1.0"
kind = "validator"
id = "org.example.evilabs"
api_version = "1"
entry = "{}"
permissions = ["workspace.read","workspace.write","external_process"]

[capabilities]
validate = true
diagnostics = true
"#,
        plugin_bin.display()
    );
    std::fs::write(workspace.join(".strixonomy/plugins/evil_abs.toml"), manifest.as_bytes())
        .expect("write manifest");

    let catalog =
        IndexBuilder::new().workspace(workspace.clone()).build().expect("index workspace");
    let host = load_plugin_host(&workspace).expect("load plugin host");
    let result =
        host.run_validate_plugin("org.example.evilabs", &catalog).expect("run validator plugin");
    let file = &result.first().expect("one diagnostic").file;

    let root = workspace.canonicalize().expect("canonical root");
    assert!(is_path_within(&root, file), "absolute diagnostic path must be jailed");
}

#[test]
fn plugin_output_paths_are_jailed() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().join("ws");
    std::fs::create_dir_all(workspace.join(".strixonomy/plugins")).expect("plugins dir");

    // Minimal ontology so `build_catalog()` succeeds.
    std::fs::write(workspace.join("demo.ttl"), "@prefix ex: <http://ex/> .\nex:Ok a owl:Class .\n")
        .expect("write demo.ttl");

    let outside = dir.path().join("outside");
    std::fs::create_dir_all(&outside).expect("outside dir");
    let secret = outside.join("secret.txt");
    std::fs::write(&secret, "secret").expect("write secret");

    let plugin_bin = workspace.join("evil_out_paths.sh");
    std::fs::write(
        &plugin_bin,
        format!(
            r#"#!/bin/sh
echo '{{"output_paths":["../../etc/passwd","{}","plugin-out/demo.md"]}}'"#,
            secret.display()
        ),
    )
    .expect("write plugin");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&plugin_bin).expect("metadata").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&plugin_bin, perms).expect("chmod");
    }

    let manifest = format!(
        r#"[plugin]
name = "evil-out-paths"
version = "0.1.0"
kind = "exporter"
id = "org.example.eviloutpaths"
api_version = "1"
entry = "{}"
permissions = ["workspace.read","workspace.write","external_process"]

[capabilities]
export = true
"#,
        plugin_bin.display()
    );
    std::fs::write(workspace.join(".strixonomy/plugins/evil_out_paths.toml"), manifest.as_bytes())
        .expect("write manifest");

    let catalog =
        IndexBuilder::new().workspace(workspace.clone()).build().expect("index workspace");
    let mut host = load_plugin_host(&workspace).expect("load plugin host");
    let result = host
        .run_plugin_action(
            "org.example.eviloutpaths",
            "export",
            Some(&catalog),
            None,
            None,
            None,
            None,
            None,
        )
        .expect("run export plugin");

    // Only the in-workspace relative path should survive the jail.
    assert_eq!(result.output_paths.len(), 1);
    let p = std::path::PathBuf::from(&result.output_paths[0]);
    let root = workspace.canonicalize().expect("canonical root");
    assert!(is_path_within(&root, &p), "output path must be jailed within workspace");
    // #348: rejected paths must fail the run and surface diagnostics.
    assert!(!result.success, "jail violations must not report success");
    assert!(
        result.diagnostics.iter().any(|d| d.message.contains("rejected plugin output path")),
        "expected path_jail diagnostics: {:?}",
        result.diagnostics
    );
}

#[test]
fn in_process_export_default_writes_under_workspace_not_cwd() {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().join("ws");
    std::fs::create_dir_all(workspace.join(".strixonomy/plugins")).expect("plugins dir");
    std::fs::write(
        workspace.join("demo.ttl"),
        "@prefix ex: <http://ex/> .\nex:Ok a <http://www.w3.org/2002/07/owl#Class> .\n",
    )
    .expect("write demo.ttl");
    std::fs::write(
        workspace.join(".strixonomy/plugins/markdown-export.toml"),
        include_str!("../crates/strixonomy-plugin/fixtures/builtin-markdown.toml"),
    )
    .expect("write markdown export manifest");

    let cwd = tempfile::tempdir().expect("cwd tempdir");
    let prev = std::env::current_dir().expect("cwd");
    struct RestoreCwd(std::path::PathBuf);
    impl Drop for RestoreCwd {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.0);
        }
    }
    let _restore = RestoreCwd(prev);
    std::env::set_current_dir(cwd.path()).expect("chdir outside workspace");

    let catalog =
        IndexBuilder::new().workspace(workspace.clone()).build().expect("index workspace");
    let mut host = load_plugin_host(&workspace).expect("load plugin host");
    let result = host
        .run_plugin_action(
            "strixonomy.markdown-export",
            "export",
            Some(&catalog),
            None,
            None,
            None,
            None,
            None,
        )
        .expect("run markdown export");
    assert!(result.success);

    let expected = workspace.join(".strixonomy/plugin-out");
    assert!(
        expected.join("index.md").is_file(),
        "export should land under workspace/.strixonomy/plugin-out"
    );
    assert!(
        !cwd.path().join("plugin-out").exists(),
        "must not write relative plugin-out under process CWD"
    );
    assert!(
        !cwd.path().join(".strixonomy/plugin-out").exists(),
        "must not write .strixonomy/plugin-out under process CWD"
    );
}

#[test]
fn subprocess_ui_view_requires_workspace_write() {
    // #347
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().join("ws");
    std::fs::create_dir_all(workspace.join(".strixonomy/plugins")).expect("plugins dir");
    std::fs::write(workspace.join("demo.ttl"), "@prefix ex: <http://ex/> .\nex:Ok a owl:Class .\n")
        .expect("write demo.ttl");

    let plugin_bin = workspace.join("ui_view_ro.sh");
    std::fs::write(
        &plugin_bin,
        r#"#!/bin/sh
echo '{"view_html":"<p>hi</p>"}'"#,
    )
    .expect("write plugin");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&plugin_bin).expect("metadata").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&plugin_bin, perms).expect("chmod");
    }

    let manifest = format!(
        r#"[plugin]
name = "ui-ro"
version = "0.1.0"
kind = "ui"
id = "org.example.ui-ro"
api_version = "1"
entry = "{}"
permissions = ["workspace.read","external_process"]

[[ui.views]]
id = "demo.view"
title = "Demo"
kind = "dock"
"#,
        plugin_bin.display()
    );
    std::fs::write(workspace.join(".strixonomy/plugins/ui_ro.toml"), manifest.as_bytes())
        .expect("write manifest");

    let catalog =
        IndexBuilder::new().workspace(workspace.clone()).build().expect("index workspace");
    let mut host = load_plugin_host(&workspace).expect("load plugin host");
    let err = host
        .run_plugin_action(
            "org.example.ui-ro",
            "ui_view",
            Some(&catalog),
            None,
            None,
            Some("demo.view"),
            None,
            None,
        )
        .expect_err("ui_view without workspace.write must fail (#347)");
    let msg = err.to_string();
    assert!(
        msg.contains("workspace.write")
            || msg.contains("WorkspaceWrite")
            || msg.contains("Missing"),
        "expected missing write permission, got {msg}"
    );
}
