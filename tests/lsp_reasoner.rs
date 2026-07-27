//! LSP integration smoke for `strixonomy/runReasoner` and catalog snapshot reasoner field.

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn el_only_workspace() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    std::fs::copy(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/reasoner-el.ttl"),
        workspace.join("reasoner-el.ttl"),
    )
    .expect("copy fixture");
    (dir, workspace)
}

#[test]
fn lsp_run_reasoner_el_profile() {
    let (_dir, workspace) = el_only_workspace();
    let workspace_uri = format!("file://{}", workspace.canonicalize().unwrap().display());

    let mut child = Command::new(lsp_binary())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn strixonomy-lsp");

    let stdout = child.stdout.take().expect("stdout");
    let mut stdin = child.stdin.take().expect("stdin");

    let reader = BufReader::new(stdout);
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut reader = reader;
        loop {
            match read_lsp_message(&mut reader) {
                Ok(Some(msg)) => {
                    if tx.send(msg).is_err() {
                        break;
                    }
                }
                Ok(None) => break,
                Err(_) => break,
            }
        }
    });

    send_request(
        &mut stdin,
        1,
        "initialize",
        serde_json::json!({
            "processId": std::process::id(),
            "rootUri": workspace_uri,
            "capabilities": {},
        }),
    );
    let init_resp = wait_for_id(&rx, 1, Duration::from_secs(10)).expect("initialize response");
    assert!(init_resp.get("result").is_some());

    send_notification(&mut stdin, "initialized", serde_json::json!({}));

    send_request(
        &mut stdin,
        2,
        "strixonomy/indexWorkspace",
        serde_json::json!({ "workspace_uri": workspace_uri }),
    );
    let index_resp = wait_for_id(&rx, 2, Duration::from_secs(10)).expect("index response");
    assert!(index_resp.get("error").is_none(), "index error: {index_resp}");

    send_request(
        &mut stdin,
        3,
        "strixonomy/runReasoner",
        serde_json::json!({ "profile": "el", "auto_detect": false }),
    );
    let reasoner_resp = wait_for_id(&rx, 3, Duration::from_secs(30)).expect("runReasoner response");
    if reasoner_resp.get("error").is_some() {
        panic!("runReasoner error: {reasoner_resp}");
    }
    let result = reasoner_resp.get("result").expect("result");
    assert_eq!(result.get("profile_used").and_then(|v| v.as_str()), Some("el"));
    assert_eq!(result.get("consistent").and_then(|v| v.as_bool()), Some(true));
    assert!(result.get("snapshot").is_some());

    send_request(&mut stdin, 4, "strixonomy/getCatalogSnapshot", serde_json::json!(null));
    let snapshot = wait_for_id(&rx, 4, Duration::from_secs(10)).expect("snapshot response");
    let reasoner = snapshot
        .get("result")
        .and_then(|r| r.get("reasoner"))
        .expect("reasoner field on snapshot after run");
    assert_eq!(reasoner.get("profile_used").and_then(|v| v.as_str()), Some("el"));

    send_request(&mut stdin, 5, "strixonomy/runReasoner", serde_json::json!({ "profile": "dl" }));
    let dl_resp = wait_for_id(&rx, 5, Duration::from_secs(30)).expect("dl runReasoner response");
    if dl_resp.get("error").is_some() {
        panic!("dl runReasoner error: {dl_resp}");
    }
    let dl_result = dl_resp.get("result").expect("dl result");
    assert_eq!(dl_result.get("profile_used").and_then(|v| v.as_str()), Some("dl"));
    assert_eq!(dl_result.get("consistent").and_then(|v| v.as_bool()), Some(true));

    send_request(&mut stdin, 6, "strixonomy/runReasoner", serde_json::json!({ "profile": "auto" }));
    let auto_resp =
        wait_for_id(&rx, 6, Duration::from_secs(30)).expect("auto runReasoner response");
    if auto_resp.get("error").is_some() {
        panic!("auto runReasoner error: {auto_resp}");
    }
    let auto_result = auto_resp.get("result").expect("auto result");
    assert_eq!(
        auto_result.get("profile_used").and_then(|v| v.as_str()),
        Some("el"),
        "Auto must report concrete EL engine for EL-only workspace"
    );
    assert_eq!(auto_result.get("consistent").and_then(|v| v.as_bool()), Some(true));
    let edges = auto_result
        .pointer("/snapshot/inferred/combined/edges")
        .or_else(|| auto_result.pointer("/inferred/combined/edges"))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    assert!(
        edges.iter().any(|e| {
            e.get("child").and_then(|c| c.as_str()).is_some_and(|c| c.ends_with("#Dog"))
                && e.get("parent").and_then(|p| p.as_str()).is_some_and(|p| p.ends_with("#Mammal"))
        }),
        "expected Dog ⊑ Mammal after auto classify: {edges:?}"
    );

    send_request(&mut stdin, 7, "shutdown", serde_json::json!(null));
    let _ = wait_for_id(&rx, 7, Duration::from_secs(5));
    send_notification(&mut stdin, "exit", serde_json::Value::Null);

    let deadline = std::time::Instant::now() + Duration::from_secs(3);
    while std::time::Instant::now() < deadline {
        if child.try_wait().ok().flatten().is_some() {
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }
    if child.try_wait().ok().flatten().is_none() {
        let _ = child.kill();
    }
    let _ = child.wait();
}

fn read_lsp_message<R: BufRead>(reader: &mut R) -> std::io::Result<Option<String>> {
    let mut content_length: Option<usize> = None;
    loop {
        let mut header = String::new();
        reader.read_line(&mut header)?;
        if header.is_empty() {
            return Ok(None);
        }
        let header = header.trim();
        if header.is_empty() {
            break;
        }
        if let Some(len) = header.strip_prefix("Content-Length:") {
            content_length = len.trim().parse().ok();
        }
    }

    let len = content_length.ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "missing Content-Length")
    })?;
    let mut body = vec![0u8; len];
    reader.read_exact(&mut body)?;
    Ok(Some(String::from_utf8_lossy(&body).to_string()))
}

fn send_request(stdin: &mut impl Write, id: i64, method: &str, params: serde_json::Value) {
    let msg = serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params,
    });
    write_lsp_message(stdin, &msg.to_string());
}

fn send_notification(stdin: &mut impl Write, method: &str, params: serde_json::Value) {
    let msg = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
    });
    write_lsp_message(stdin, &msg.to_string());
}

fn write_lsp_message(stdin: &mut impl Write, body: &str) {
    let header = format!("Content-Length: {}\r\n\r\n", body.len());
    stdin.write_all(header.as_bytes()).unwrap();
    stdin.write_all(body.as_bytes()).unwrap();
    stdin.flush().unwrap();
}

fn lsp_binary() -> PathBuf {
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_strixonomy-lsp") {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return candidate;
        }
    }

    if let Some(bin) = find_lsp_binary_in_target() {
        return bin;
    }

    // When running workspace-level integration tests, Cargo does not automatically
    // build bin targets like `strixonomy-lsp`. Build it explicitly, then locate it.
    let status = Command::new("cargo")
        .args(["build", "-q", "-p", "strixonomy-lsp", "--bin", "strixonomy-lsp"])
        .status()
        .expect("cargo build strixonomy-lsp");
    assert!(status.success(), "failed to build strixonomy-lsp");

    find_lsp_binary_in_target().unwrap_or_else(|| {
        let target_dir = std::env::var("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("target"));
        panic!("strixonomy-lsp binary not found under {} after build", target_dir.display());
    })
}

fn find_lsp_binary_in_target() -> Option<PathBuf> {
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("target"));

    for subdir in ["debug", "release"] {
        let candidate = target_dir.join(subdir).join("strixonomy-lsp");
        if candidate.exists() {
            return Some(candidate);
        }
    }

    None
}

fn wait_for_id(
    rx: &mpsc::Receiver<String>,
    id: i64,
    timeout: Duration,
) -> Option<serde_json::Value> {
    let deadline = std::time::Instant::now() + timeout;
    while std::time::Instant::now() < deadline {
        if let Ok(line) = rx.recv_timeout(Duration::from_millis(200)) {
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(&line) {
                if value.get("id").and_then(|v| v.as_i64()) == Some(id) {
                    return Some(value);
                }
            }
        }
    }
    None
}
