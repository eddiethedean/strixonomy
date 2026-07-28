use crate::error::{Result, RobotError};
use std::io::{ErrorKind, Read};
use std::path::{Component, Path};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct RobotOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

const ALLOWED_SUBCOMMANDS: &[&str] =
    &["validate-profile", "validate", "merge", "report", "reason", "query", "convert"];

/// Default kill deadline for ROBOT subprocesses (#341).
pub const DEFAULT_ROBOT_TIMEOUT_SECS: u64 = 300;

/// Max bytes buffered per stdout/stderr stream (#423). Matches plugin host default.
pub const MAX_STDIO_BYTES: usize = 1024 * 1024; // 1 MiB per stream

/// Resolve a ROBOT executable. Explicit paths must be named `robot` / `robot.cmd` / `robot.bat`.
pub fn detect_robot(explicit_path: Option<&str>) -> Result<String> {
    if let Some(path) = explicit_path.filter(|p| !p.is_empty()) {
        let p = Path::new(path);
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("").to_ascii_lowercase();
        if !matches!(name.as_str(), "robot" | "robot.cmd" | "robot.bat" | "robot.exe") {
            return Err(RobotError::NotFound);
        }
        // Reject path traversal in the explicit path.
        if p.components().any(|c| matches!(c, Component::ParentDir)) {
            return Err(RobotError::NotFound);
        }
        if p.exists() {
            return Ok(path.to_string());
        }
        return Err(RobotError::NotFound);
    }
    // Probe PATH by spawning allowlisted names. Avoids Unix-only `which` (#316);
    // Windows PATHEXT resolves `robot` → `robot.cmd` / `.bat` / `.exe` when present.
    for name in ["robot", "robot.cmd", "robot.bat", "robot.exe"] {
        if robot_exists_on_path(name) {
            return Ok(name.to_string());
        }
    }
    Err(RobotError::NotFound)
}

fn robot_exists_on_path(name: &str) -> bool {
    match Command::new(name).arg("--version").output() {
        Ok(_) => true,
        Err(err) => err.kind() != ErrorKind::NotFound,
    }
}

fn read_capped(mut reader: impl Read, cap: usize) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 8192];
    loop {
        let n = reader.read(&mut chunk)?;
        if n == 0 {
            break;
        }
        if buf.len().saturating_add(n) > cap {
            return Err(RobotError::OutputTooLarge(cap));
        }
        buf.extend_from_slice(&chunk[..n]);
    }
    Ok(buf)
}

pub fn run_robot(robot_path: Option<&str>, args: &[String]) -> Result<RobotOutput> {
    run_robot_with_timeout(robot_path, args, Duration::from_secs(DEFAULT_ROBOT_TIMEOUT_SECS))
}

pub(crate) fn run_robot_with_timeout(
    robot_path: Option<&str>,
    args: &[String],
    timeout: Duration,
) -> Result<RobotOutput> {
    if let Some(sub) = args.first() {
        let base = sub.split_whitespace().next().unwrap_or(sub);
        if !ALLOWED_SUBCOMMANDS.contains(&base) {
            return Err(RobotError::Run(format!(
                "ROBOT subcommand '{base}' is not allowed; permitted: {}",
                ALLOWED_SUBCOMMANDS.join(", ")
            )));
        }
    } else {
        return Err(RobotError::Run("ROBOT requires a subcommand".to_string()));
    }
    let robot = detect_robot(robot_path)?;
    let mut cmd = Command::new(&robot);
    cmd.args(args).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
    configure_process_group(&mut cmd);
    let mut child = cmd.spawn()?;
    let mut stdout = child.stdout.take().expect("stdout piped");
    let mut stderr = child.stderr.take().expect("stderr piped");
    let out_handle = thread::spawn(move || read_capped(&mut stdout, MAX_STDIO_BYTES));
    let err_handle = thread::spawn(move || read_capped(&mut stderr, MAX_STDIO_BYTES));

    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let out = out_handle
                    .join()
                    .map_err(|_| RobotError::Run("stdout reader panicked".into()))??;
                let err = err_handle
                    .join()
                    .map_err(|_| RobotError::Run("stderr reader panicked".into()))??;
                return Ok(RobotOutput {
                    exit_code: status.code().unwrap_or(1),
                    stdout: String::from_utf8_lossy(&out).into_owned(),
                    stderr: String::from_utf8_lossy(&err).into_owned(),
                });
            }
            Ok(None) => {
                if Instant::now() >= deadline {
                    kill_robot_tree(&mut child);
                    let _ = child.wait();
                    let _ = out_handle.join();
                    let _ = err_handle.join();
                    return Err(RobotError::TimedOut(timeout.as_secs().max(1)));
                }
                thread::sleep(Duration::from_millis(25));
            }
            Err(e) => {
                kill_robot_tree(&mut child);
                let _ = child.wait();
                let _ = out_handle.join();
                let _ = err_handle.join();
                return Err(RobotError::Io(e));
            }
        }
    }
}

fn configure_process_group(cmd: &mut Command) {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    let _ = cmd;
}

fn kill_robot_tree(child: &mut Child) {
    #[cfg(unix)]
    {
        let pgid = child.id() as i32;
        // SAFETY: kill(-pgid) signals the process group created via process_group(0).
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

pub fn robot_validate(robot_path: Option<&str>, ontology_path: &Path) -> Result<RobotOutput> {
    run_robot(
        robot_path,
        &[
            "validate-profile".to_string(),
            "--input".to_string(),
            ontology_path.display().to_string(),
            "--profile".to_string(),
            "DL".to_string(),
        ],
    )
}

pub fn robot_convert(robot_path: Option<&str>, input: &Path, output: &Path) -> Result<RobotOutput> {
    run_robot(
        robot_path,
        &[
            "convert".into(),
            "--input".into(),
            input.display().to_string(),
            "--output".into(),
            output.display().to_string(),
        ],
    )
}

pub fn robot_merge(
    robot_path: Option<&str>,
    inputs: &[String],
    output: &Path,
) -> Result<RobotOutput> {
    let mut args = vec!["merge".to_string()];
    for input in inputs {
        args.push("--input".to_string());
        args.push(input.clone());
    }
    args.push("--output".to_string());
    args.push(output.display().to_string());
    run_robot(robot_path, &args)
}

pub fn robot_report(
    robot_path: Option<&str>,
    ontology_path: &Path,
    report_path: &Path,
) -> Result<RobotOutput> {
    run_robot(
        robot_path,
        &[
            "report".to_string(),
            "--input".to_string(),
            ontology_path.display().to_string(),
            "--output".to_string(),
            report_path.display().to_string(),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::RobotError;
    use std::io::Cursor;

    #[test]
    fn rejects_disallowed_robot_subcommand() {
        let err = match run_robot(Some("/definitely/not/robot"), &[String::from("diff")]) {
            Err(e) => e,
            Ok(output) => panic!("expected disallowed subcommand error, got {output:?}"),
        };
        match err {
            RobotError::Run(msg) => {
                assert!(msg.contains("not allowed"));
                assert!(msg.contains("diff"));
            }
            other => panic!("expected Run error, got {other:?}"),
        }
    }

    #[test]
    fn rejects_empty_robot_args() {
        let err = match run_robot(Some("/definitely/not/robot"), &[]) {
            Err(e) => e,
            Ok(output) => panic!("expected empty-args error, got {output:?}"),
        };
        match err {
            RobotError::Run(msg) => assert!(msg.contains("requires a subcommand")),
            other => panic!("expected Run error, got {other:?}"),
        }
    }

    #[test]
    fn allows_known_subcommands_in_validation() {
        for sub in ["validate-profile", "validate", "merge", "report", "reason", "query", "convert"]
        {
            let result = run_robot(Some("/definitely/not/robot"), &[String::from(sub)]);
            match result {
                Err(RobotError::Run(ref msg)) if msg.contains("not allowed") => {
                    panic!("subcommand {sub} should pass allowlist, got {msg}");
                }
                _ => {}
            }
        }
    }

    #[test]
    fn rejects_parent_dir_in_explicit_robot_path() {
        let err = detect_robot(Some("../robot")).unwrap_err();
        assert!(matches!(err, RobotError::NotFound));
    }

    #[test]
    fn rejects_non_robot_binary_name() {
        let err = detect_robot(Some("/usr/bin/java")).unwrap_err();
        assert!(matches!(err, RobotError::NotFound));
    }

    #[test]
    fn accepts_robot_binary_names() {
        for name in ["robot", "robot.cmd", "robot.bat", "robot.exe"] {
            let path = format!("/definitely/not/{name}");
            let err = detect_robot(Some(&path)).unwrap_err();
            assert!(
                matches!(err, RobotError::NotFound),
                "{name} should be accepted by name check before existence"
            );
        }
    }

    #[test]
    fn path_probe_skips_missing_binary_without_which() {
        // ensure robot_exists_on_path returns false for a nonsense name (no panic / which).
        assert!(!robot_exists_on_path("strixonomy-definitely-missing-robot-bin-xyz"));
    }

    #[test]
    fn read_capped_rejects_oversize() {
        let data = vec![b'x'; 40];
        let err = read_capped(Cursor::new(data), 32).unwrap_err();
        assert!(matches!(err, RobotError::OutputTooLarge(32)));
    }

    #[test]
    fn read_capped_accepts_exact_cap() {
        let data = vec![b'y'; 32];
        let out = read_capped(Cursor::new(data), 32).expect("exact cap ok");
        assert_eq!(out.len(), 32);
    }

    #[cfg(unix)]
    #[test]
    fn robot_timeout_kills_hanging_binary() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().unwrap();
        let robot = dir.path().join("robot");
        std::fs::write(&robot, "#!/bin/sh\nsleep 999\n").unwrap();
        let mut perms = std::fs::metadata(&robot).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&robot, perms).unwrap();

        let err = run_robot_with_timeout(
            Some(robot.to_str().unwrap()),
            &[String::from("validate")],
            Duration::from_millis(400),
        )
        .expect_err("must time out");
        assert!(matches!(err, RobotError::TimedOut(_)), "got {err:?}");
    }

    #[cfg(unix)]
    #[test]
    fn robot_rejects_oversized_stdout() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().unwrap();
        let robot = dir.path().join("robot");
        // Emit just over the stdio cap then exit (allowlisted subcommand is ignored).
        let script = format!("#!/bin/sh\nhead -c {} /dev/zero\n", MAX_STDIO_BYTES + 1);
        std::fs::write(&robot, script).unwrap();
        let mut perms = std::fs::metadata(&robot).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&robot, perms).unwrap();

        let err = run_robot_with_timeout(
            Some(robot.to_str().unwrap()),
            &[String::from("report")],
            Duration::from_secs(10),
        )
        .expect_err("must reject oversized stdout");
        assert!(matches!(err, RobotError::OutputTooLarge(MAX_STDIO_BYTES)), "got {err:?}");
    }
}
