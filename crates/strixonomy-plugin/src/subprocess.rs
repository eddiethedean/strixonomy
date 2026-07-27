use crate::manifest::DiscoveredPlugin;
use crate::protocol::PluginOutput;
use std::io::Read;
use std::path::{Component, Path};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;

// Conservative defaults to prevent hangs/DoS from untrusted subprocess plugins.
// These can be made configurable later once a stable plugin settings surface exists.
const DEFAULT_TIMEOUT_SECS: u64 = 60;
const MAX_STDIO_BYTES: usize = 1024 * 1024; // 1 MiB per stream
/// Max bytes of stderr/stdout embedded in user-facing error strings (#388).
const ERROR_SNIPPET_MAX: usize = 512;

#[derive(Debug, Error)]
pub enum SubprocessError {
    #[error("plugin binary not found: {0}")]
    NotFound(String),
    #[error("plugin binary path is not allowed: {0}")]
    PathNotAllowed(String),
    #[error("plugin subprocess failed: {0}")]
    RunFailed(String),
    #[error("plugin subprocess timed out after {0}s")]
    TimedOut(u64),
    #[error("plugin subprocess output exceeded {0} bytes")]
    OutputTooLarge(usize),
    #[error("plugin output is not valid JSON: {0}")]
    InvalidOutput(String),
}

pub struct SubprocessRequest<'a> {
    pub action: &'a str,
    pub workspace: &'a Path,
    pub step: Option<&'a str>,
    pub extra_args: &'a [String],
}

pub fn resolve_entry_path(
    plugin: &DiscoveredPlugin,
) -> Result<std::path::PathBuf, SubprocessError> {
    let entry = plugin
        .manifest
        .entry
        .as_deref()
        .ok_or_else(|| SubprocessError::NotFound("missing entry".to_string()))?;
    let path = if Path::new(entry).is_absolute() {
        Path::new(entry).to_path_buf()
    } else {
        plugin.manifest_path.parent().unwrap_or(Path::new(".")).join(entry)
    };
    validate_binary_path(&path)?;
    // #404: jail-check and execute the *canonical* path so a retargeted symlink
    // between check and spawn cannot escape the workspace.
    let resolved =
        path.canonicalize().map_err(|_| SubprocessError::NotFound(path.display().to_string()))?;
    if let Some(root) = infer_workspace_root(&plugin.manifest_path) {
        let root = root.canonicalize().unwrap_or(root);
        if !strixonomy_core::is_path_within(&root, &resolved) {
            return Err(SubprocessError::PathNotAllowed(path.display().to_string()));
        }
    }
    Ok(resolved)
}

fn infer_workspace_root(manifest_path: &Path) -> Option<std::path::PathBuf> {
    // Manifests live under `{workspace}/.strixonomy/plugins/*.toml` or legacy `.ontocore/plugins/`.
    let plugins_dir = manifest_path.parent()?;
    let config_dir = plugins_dir.parent()?;
    let name = config_dir.file_name().and_then(|n| n.to_str())?;
    if name != ".strixonomy" && name != ".ontocore" {
        return None;
    }
    config_dir.parent().map(|p| p.to_path_buf())
}

fn validate_binary_path(path: &Path) -> Result<(), SubprocessError> {
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(SubprocessError::PathNotAllowed(path.display().to_string()));
    }
    Ok(())
}

fn read_capped(mut reader: impl Read, cap: usize) -> Result<Vec<u8>, SubprocessError> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 8192];
    loop {
        let n = reader.read(&mut chunk).map_err(|e| SubprocessError::RunFailed(e.to_string()))?;
        if n == 0 {
            break;
        }
        if buf.len().saturating_add(n) > cap {
            return Err(SubprocessError::OutputTooLarge(cap));
        }
        buf.extend_from_slice(&chunk[..n]);
    }
    Ok(buf)
}

/// Truncate captured stdio for error strings so LSP/CLI do not embed megabytes (#388).
fn summarize_stdio(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if trimmed.len() <= ERROR_SNIPPET_MAX {
        return trimmed.to_string();
    }
    let mut end = ERROR_SNIPPET_MAX;
    while end > 0 && !trimmed.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}… (truncated)", &trimmed[..end])
}

fn configure_process_group(cmd: &mut Command) {
    // Put the plugin in its own process group so timeout can kill grandchildren (#217).
    // Windows: no process-group kill helper here — `Child::kill` terminates the direct child only.
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
}

fn kill_plugin_tree(child: &mut Child) {
    #[cfg(unix)]
    {
        // Negative PID = process group of the plugin (PGID == child PID after process_group(0)).
        let pgid = child.id() as i32;
        // SAFETY: kill(-pgid) is the standard way to signal a process group; pgid > 0 for a
        // spawned child. Falling back to Child::kill if the group signal fails.
        let group_kill = unsafe { libc::kill(-pgid, libc::SIGKILL) };
        if group_kill != 0 {
            let _ = child.kill();
        }
    }
    #[cfg(not(unix))]
    {
        let _ = child.kill();
    }
}

pub fn run_plugin_subprocess(
    plugin: &DiscoveredPlugin,
    request: SubprocessRequest<'_>,
) -> Result<PluginOutput, SubprocessError> {
    run_plugin_subprocess_with_timeout(plugin, request, Duration::from_secs(DEFAULT_TIMEOUT_SECS))
}

pub(crate) fn run_plugin_subprocess_with_timeout(
    plugin: &DiscoveredPlugin,
    request: SubprocessRequest<'_>,
    timeout: Duration,
) -> Result<PluginOutput, SubprocessError> {
    let binary = resolve_entry_path(plugin)?;
    let mut cmd = Command::new(&binary);
    cmd.arg(request.action)
        .arg("--workspace")
        .arg(request.workspace)
        .env("STRIXONOMY_PLUGIN_ACTION", request.action)
        .env("ONTOCORE_PLUGIN_ACTION", request.action) // legacy alias through 1.0
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(step) = request.step {
        cmd.arg("--step").arg(step);
    }
    for arg in request.extra_args {
        cmd.arg(arg);
    }
    configure_process_group(&mut cmd);
    let mut child = cmd.spawn().map_err(|e| SubprocessError::RunFailed(e.to_string()))?;
    let mut stdout = child.stdout.take().expect("stdout piped");
    let mut stderr = child.stderr.take().expect("stderr piped");

    let out_handle = thread::spawn(move || read_capped(&mut stdout, MAX_STDIO_BYTES));
    let err_handle = thread::spawn(move || read_capped(&mut stderr, MAX_STDIO_BYTES));

    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let out = out_handle.join().map_err(|_| {
                    SubprocessError::RunFailed("stdout reader panicked".to_string())
                })??;
                let err = err_handle.join().map_err(|_| {
                    SubprocessError::RunFailed("stderr reader panicked".to_string())
                })??;

                if !status.success() {
                    let snippet = summarize_stdio(&err);
                    return Err(SubprocessError::RunFailed(if snippet.is_empty() {
                        format!("exit {}", status.code().unwrap_or(-1))
                    } else {
                        format!("exit {}: {snippet}", status.code().unwrap_or(-1))
                    }));
                }

                let stdout = String::from_utf8_lossy(&out);
                if stdout.trim().is_empty() {
                    return Ok(PluginOutput::default());
                }
                return serde_json::from_str(stdout.trim()).map_err(|e| {
                    SubprocessError::InvalidOutput(format!("{e}: {}", summarize_stdio(&out)))
                });
            }
            Ok(None) => {
                if Instant::now() >= deadline {
                    kill_plugin_tree(&mut child);
                    let _ = child.wait();
                    let _ = out_handle.join();
                    let _ = err_handle.join();
                    return Err(SubprocessError::TimedOut(timeout.as_secs().max(1)));
                }
                thread::sleep(Duration::from_millis(25));
            }
            Err(e) => {
                kill_plugin_tree(&mut child);
                let _ = child.wait();
                let _ = out_handle.join();
                let _ = err_handle.join();
                return Err(SubprocessError::RunFailed(e.to_string()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{parse_manifest, DiscoveredPlugin};
    use std::io::Cursor;

    #[test]
    fn read_capped_rejects_oversize() {
        let data = vec![b'a'; 33];
        let err = read_capped(Cursor::new(data), 32).unwrap_err();
        matches!(err, SubprocessError::OutputTooLarge(32));
    }

    #[test]
    fn summarize_stdio_truncates_long_output() {
        let long = "x".repeat(ERROR_SNIPPET_MAX + 80);
        let summary = summarize_stdio(long.as_bytes());
        assert!(summary.contains("truncated"));
        assert!(summary.len() < long.len());
        assert!(summary.starts_with("xxx"));
    }

    #[test]
    fn summarize_stdio_preserves_short_output() {
        assert_eq!(summarize_stdio(b"  boom  "), "boom");
    }

    #[cfg(unix)]
    #[test]
    fn resolve_entry_path_returns_canonical_target() {
        // #404
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().expect("tempdir");
        let workspace = dir.path().join("ws");
        std::fs::create_dir_all(workspace.join(".strixonomy/plugins")).expect("plugins dir");

        let real_bin = workspace.join("real_plugin.sh");
        std::fs::write(&real_bin, "#!/bin/sh\necho '{}'\n").expect("write real");
        let mut perms = std::fs::metadata(&real_bin).expect("metadata").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&real_bin, perms).expect("chmod");

        let link = workspace.join(".strixonomy/plugins/helper");
        std::os::unix::fs::symlink(&real_bin, &link).expect("symlink");

        let manifest_path = workspace.join(".strixonomy/plugins/link.toml");
        let manifest_toml = r#"[plugin]
name = "link"
version = "0.1.0"
kind = "validator"
id = "org.example.link"
api_version = "1"
entry = "helper"
permissions = ["workspace.read","workspace.write","external_process"]

[capabilities]
validate = true
"#;
        std::fs::write(&manifest_path, manifest_toml).expect("write manifest");
        let plugin = DiscoveredPlugin {
            manifest: parse_manifest(manifest_toml).expect("parse"),
            manifest_path,
        };

        let resolved = resolve_entry_path(&plugin).expect("resolve");
        let expected = real_bin.canonicalize().expect("canonical real");
        assert_eq!(resolved, expected, "must exec canonical path, not symlink");
    }

    #[cfg(unix)]
    #[test]
    fn subprocess_error_truncates_long_stderr() {
        // #388
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().expect("tempdir");
        let workspace = dir.path().join("ws");
        std::fs::create_dir_all(workspace.join(".strixonomy/plugins")).expect("plugins dir");

        let plugin_bin = workspace.join("noisy_fail.sh");
        std::fs::write(
            &plugin_bin,
            r#"#!/bin/sh
# Avoid depending on python being present.
i=0
while [ "$i" -lt 2048 ]; do
  printf S >&2
  i=$((i+1))
done
exit 1
"#,
        )
        .expect("write plugin");
        let mut perms = std::fs::metadata(&plugin_bin).expect("metadata").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&plugin_bin, perms).expect("chmod");

        let manifest_path = workspace.join(".strixonomy/plugins/noisy.toml");
        let manifest_toml = format!(
            r#"[plugin]
name = "noisy"
version = "0.1.0"
kind = "validator"
id = "org.example.noisy"
api_version = "1"
entry = "{}"
permissions = ["workspace.read","workspace.write","external_process"]

[capabilities]
validate = true
"#,
            plugin_bin.display()
        );
        std::fs::write(&manifest_path, &manifest_toml).expect("write manifest");
        let plugin = DiscoveredPlugin {
            manifest: parse_manifest(&manifest_toml).expect("parse"),
            manifest_path,
        };

        let err = run_plugin_subprocess(
            &plugin,
            SubprocessRequest {
                action: "validate",
                workspace: &workspace,
                step: None,
                extra_args: &[],
            },
        )
        .expect_err("must fail");
        let msg = err.to_string();
        assert!(msg.contains("truncated"), "stderr must be truncated with marker: {msg}");
        assert!(msg.len() < 900, "error string must stay small, got len={}", msg.len());
        assert!(!msg.contains(&"S".repeat(2048)), "full stderr must not appear in error");
    }

    #[cfg(unix)]
    #[test]
    fn timeout_kills_process_group_grandchildren() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().expect("tempdir");
        let workspace = dir.path().join("ws");
        std::fs::create_dir_all(workspace.join(".strixonomy/plugins")).expect("plugins dir");

        let marker = workspace.join("grandchild.pid");
        let plugin_bin = workspace.join("slow_plugin.sh");
        // Write the pid before entering a long sleep so a short timeout cannot race the
        // marker file on slow filesystems (e.g. external volumes under /Volumes).
        let script = format!(
            r#"#!/bin/sh
set -eu
sleep 999 &
echo $! > "{}"
# Ensure the pid lands on durable storage before we block forever.
sync 2>/dev/null || true
wait
"#,
            marker.display()
        );
        std::fs::write(&plugin_bin, script).expect("write plugin");
        let mut perms = std::fs::metadata(&plugin_bin).expect("metadata").permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&plugin_bin, perms).expect("chmod");

        let manifest_path = workspace.join(".strixonomy/plugins/slow.toml");
        let manifest_toml = format!(
            r#"[plugin]
name = "slow"
version = "0.1.0"
kind = "validator"
id = "org.example.slow"
api_version = "1"
entry = "{}"
permissions = ["workspace.read","workspace.write","external_process"]

[capabilities]
validate = true
diagnostics = true
"#,
            plugin_bin.display()
        );
        std::fs::write(&manifest_path, &manifest_toml).expect("write manifest");
        let plugin = DiscoveredPlugin {
            manifest: parse_manifest(&manifest_toml).expect("parse"),
            manifest_path,
        };

        let err = run_plugin_subprocess_with_timeout(
            &plugin,
            SubprocessRequest {
                action: "validate",
                workspace: &workspace,
                step: None,
                extra_args: &[],
            },
            Duration::from_millis(1_500),
        )
        .expect_err("must time out");
        assert!(matches!(err, SubprocessError::TimedOut(_)), "got {err:?}");

        // Give the kernel a moment to reap; grandchild must be gone.
        // Poll briefly for the marker (process may still be flushing under load).
        let mut pid_text = None;
        for _ in 0..20 {
            if let Ok(text) = std::fs::read_to_string(&marker) {
                if !text.trim().is_empty() {
                    pid_text = Some(text);
                    break;
                }
            }
            thread::sleep(Duration::from_millis(50));
        }
        let pid_text = pid_text.expect("grandchild pid written");
        let pid: i32 = pid_text.trim().parse().expect("pid");
        // Poll for death — kill(0) can succeed briefly for a dying/zombie process under CI load.
        let mut still_alive = true;
        for _ in 0..40 {
            still_alive = unsafe { libc::kill(pid, 0) == 0 };
            if !still_alive {
                break;
            }
            thread::sleep(Duration::from_millis(50));
        }
        assert!(!still_alive, "grandchild pid {pid} should be dead after process-group kill");
    }
}
