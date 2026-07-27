use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn lsp_indexes_fixture_workspace() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");
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
        serde_json::json!({ "workspaceUri": workspace_uri }),
    );

    let index_resp = wait_for_id(&rx, 2, Duration::from_secs(10))
        .unwrap_or_else(|| panic!("index response timeout"));
    if index_resp.get("error").is_some() {
        panic!("indexWorkspace error: {index_resp}");
    }
    let stats =
        index_resp.get("result").and_then(|r| r.get("stats")).expect("stats in index result");
    assert!(stats.get("class_count").and_then(|v| v.as_u64()).unwrap_or(0) >= 4);
    assert_eq!(stats.get("individual_count").and_then(|v| v.as_u64()), Some(2));
    assert_eq!(stats.get("error_count").and_then(|v| v.as_u64()), Some(0));

    send_request(&mut stdin, 3, "strixonomy/getCatalogSnapshot", serde_json::json!(null));

    let snapshot = wait_for_id(&rx, 3, Duration::from_secs(10)).expect("snapshot response");
    let entities = snapshot
        .get("result")
        .and_then(|r| r.get("entities"))
        .and_then(|e| e.as_array())
        .expect("entities array");
    let iris: Vec<&str> =
        entities.iter().filter_map(|e| e.get("iri").and_then(|v| v.as_str())).collect();
    assert!(iris.contains(&"http://example.org/people#Person"));

    let person = entities
        .iter()
        .find(|e| e.get("iri").and_then(|v| v.as_str()) == Some("http://example.org/people#Person"))
        .expect("Person entity in snapshot");
    assert_eq!(person.get("kind").and_then(|v| v.as_str()), Some("class"));

    let documents = snapshot
        .get("result")
        .and_then(|r| r.get("documents"))
        .and_then(|d| d.as_array())
        .expect("documents array");
    let diagnostics = snapshot
        .get("result")
        .and_then(|r| r.get("diagnostics"))
        .and_then(|d| d.as_array())
        .expect("diagnostics array");
    assert_eq!(diagnostics.len(), 0, "clean fixtures must not emit diagnostics: {diagnostics:?}");
    let example_doc = documents
        .iter()
        .find(|d| {
            d.get("path").and_then(|v| v.as_str()).is_some_and(|p| p.ends_with("example.ttl"))
        })
        .expect("example.ttl in documents");
    assert_eq!(example_doc.get("parse_status").and_then(|v| v.as_str()), Some("ok"));

    send_request(
        &mut stdin,
        4,
        "strixonomy/getEntity",
        serde_json::json!({ "iri": "http://example.org/people#Person" }),
    );

    let entity_resp = wait_for_id(&rx, 4, Duration::from_secs(10)).expect("getEntity response");
    if entity_resp.get("error").is_some() {
        panic!("getEntity error: {entity_resp}");
    }
    let short_name = entity_resp
        .get("result")
        .and_then(|r| r.get("detail"))
        .and_then(|d| d.get("entity"))
        .and_then(|e| e.get("short_name"))
        .and_then(|v| v.as_str());
    assert_eq!(short_name, Some("Person"));

    send_request(
        &mut stdin,
        5,
        "strixonomy/query",
        serde_json::json!({ "sql": "SELECT short_name FROM classes" }),
    );
    let query_resp = wait_for_id(&rx, 5, Duration::from_secs(10)).expect("query response");
    assert!(query_resp.get("result").and_then(|r| r.get("rows")).is_some());

    let example_path = workspace.join("example.ttl");
    let doc_uri = format!("file://{}", example_path.canonicalize().unwrap().display());
    send_request(
        &mut stdin,
        7,
        "textDocument/completion",
        serde_json::json!({
            "textDocument": { "uri": doc_uri },
            "position": { "line": 11, "character": 3 },
        }),
    );
    let completion_resp =
        wait_for_id(&rx, 7, Duration::from_secs(10)).expect("completion response");
    if completion_resp.get("error").is_some() {
        panic!("completion error: {completion_resp}");
    }
    let items =
        completion_resp.get("result").and_then(|r| r.as_array()).expect("completion items array");
    let labels: Vec<&str> =
        items.iter().filter_map(|i| i.get("label").and_then(|v| v.as_str())).collect();
    assert!(
        labels.iter().any(|l| l.contains("Person") || l.contains("Thing")),
        "expected catalog class completions, got: {labels:?}"
    );

    send_request(&mut stdin, 6, "shutdown", serde_json::json!(null));
    let _ = wait_for_id(&rx, 6, Duration::from_secs(5));
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
