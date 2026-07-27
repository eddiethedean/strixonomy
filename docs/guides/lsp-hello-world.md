# LSP hello world (minimal client)

Build a custom editor or smoke-test **Strixonomy LSP** (`strixonomy-lsp`) over stdio. For the full method list and JSON payloads, see [LSP API reference](../lsp-api.md).

## Prerequisites

- Built or installed `strixonomy-lsp` (`cargo install strixonomy-lsp --locked` or `cargo build -p strixonomy-lsp --bins`)
- A directory of ontology files (use your path, or clone this repo and point at `fixtures/`)

## Spawn the server

```bash
strixonomy-lsp
# reads JSON-RPC from stdin, writes responses to stdout
```

The server expects LSP `initialize` with a `rootUri` or `workspaceFolders` entry pointing at your ontology directory.

## Minimal Python client

```python
#!/usr/bin/env python3
"""Smoke-test strixonomy-lsp: initialize + indexWorkspace."""
import json
import subprocess
import sys

WORKSPACE = sys.argv[1] if len(sys.argv) > 1 else "."
LSP = subprocess.Popen(
    ["strixonomy-lsp"],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    text=True,
    bufsize=1,
)

def send(msg: dict) -> None:
    body = json.dumps(msg)
    LSP.stdin.write(f"Content-Length: {len(body)}\r\n\r\n{body}")
    LSP.stdin.flush()

def read_message() -> dict:
  header = ""
  while True:
    line = LSP.stdout.readline()
    if not line.strip():
      break
    header += line
  length = int(header.split(":", 1)[1].strip())
  return json.loads(LSP.stdout.read(length))

send({
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": {
        "processId": None,
        "rootUri": f"file://{WORKSPACE}",
        "capabilities": {},
    },
})
print("initialize →", read_message().get("result", {}).keys())

send({"jsonrpc": "2.0", "method": "initialized", "params": {}})
send({
    "jsonrpc": "2.0",
    "id": 2,
    "method": "strixonomy/indexWorkspace",
    "params": {"workspace_uri": f"file://{WORKSPACE}"},
})
print("indexWorkspace →", read_message().get("result", {}))

LSP.terminate()
```

Run:

```bash
python3 lsp_smoke.py /path/to/ontologies
```

## Reference implementation

The Strixonomy VS Code extension client is the production reference:

- [`extension/src/lsp/client.ts`](https://github.com/eddiethedean/strixonomy/blob/main/extension/src/lsp/client.ts) — method routing and custom `strixonomy/*` requests
- [`docs/lsp-protocol.schema.json`](../lsp-protocol.schema.json) — JSON Schema for custom payloads (v0.13)

## Next steps

| Goal | Doc |
|------|-----|
| Full method catalog | [LSP API](../lsp-api.md) |
| Webview hosting (Strixonomy only) | [Webview protocol](../webview-protocol.md) |
| Build the VS Code extension | [Extension development](extension-development.md) |
