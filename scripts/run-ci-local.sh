#!/usr/bin/env bash
# Run CI workflow steps locally (mirrors .github/workflows/ci.yml + extension-vscode-e2e darwin-arm64).
#
# GitHub CI runs jobs on separate VMs with remote rust-cache. Locally, running many
# cargo jobs in parallel with isolated target dirs forces full recompiles and CPU/IO
# contention — usually slower than sequential with one shared target/.
#
# PARALLEL=1 (default): overlap only non-Rust jobs (rustfmt, doc pins, mkdocs, audit)
# while Rust/extension steps run sequentially with shared target/.
# PARALLEL=0: fully sequential (original behavior).
set -uo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export ROOT
cd "$ROOT"

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export TMPDIR="${TMPDIR:-$ROOT/target/tmp}"
mkdir -p "$TMPDIR" "$CARGO_TARGET_DIR"

PLATFORM="$(uname -s | tr '[:upper:]' '[:lower:]')"
case "$(uname -m)" in
  x86_64) ARCH="x64" ;;
  aarch64 | arm64) ARCH="arm64" ;;
  *) ARCH="$(uname -m)" ;;
esac
EXT_PLATFORM="${PLATFORM}-${ARCH}"

FAILED=()
PASSED=()

PARALLEL="${PARALLEL:-1}"
LOG_DIR="${LOG_DIR:-$ROOT/target/ci-local-logs}"
mkdir -p "$LOG_DIR"

BG_PIDS=()
BG_NAMES=()
BG_LOGS=()

run_step() {
  local name="$1"
  shift
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "▶ ${name}"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  if "$@"; then
    echo "✓ PASS: ${name}"
    PASSED+=("${name}")
  else
    echo "✗ FAIL: ${name}"
    FAILED+=("${name}")
  fi
}

run_bg_step() {
  local name="$1"
  local slug="$2"
  shift 2

  local log="$LOG_DIR/${slug}.log"
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "▶ ${name} (background)"
  echo "  log: ${log}"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  (
    set -euo pipefail
    "$@"
  ) >"$log" 2>&1 &

  BG_PIDS+=("$!")
  BG_NAMES+=("$name")
  BG_LOGS+=("$log")
}

wait_bg_steps() {
  for i in "${!BG_PIDS[@]}"; do
    local pid="${BG_PIDS[$i]}"
    local name="${BG_NAMES[$i]}"
    local log="${BG_LOGS[$i]}"
    if wait "$pid"; then
      echo "✓ PASS: ${name}"
      PASSED+=("${name}")
    else
      echo "✗ FAIL: ${name}"
      echo "  log: ${log}"
      FAILED+=("${name}")
    fi
  done
  BG_PIDS=()
  BG_NAMES=()
  BG_LOGS=()
}

wait_bg_steps_if_pending() {
  if ((${#BG_PIDS[@]} > 0)); then
    echo ""
    echo "Waiting for background jobs before extension LSP tests (avoid CPU/IO contention)..."
    wait_bg_steps
  fi
}

run_rust_and_extension_steps() {
  run_step "clippy" \
    cargo clippy --workspace --all-targets --all-features -- -D warnings

  run_step "cargo test" bash -c '
    cargo build -p strixonomy-lsp -p strixonomy-cli --bins
    cargo test --workspace
  '

  run_step "CLI smoke" bash -c '
    set -euo pipefail
    cargo run -- query fixtures "SELECT * FROM classes"
    cargo run -- validate fixtures
    cargo run -- query fixtures "SELECT code FROM diagnostics"
    cargo run -- sparql fixtures "SELECT ?s WHERE { ?s ?p ?o } LIMIT 1"
  '

  run_step "release build" bash -c '
    set -euo pipefail
    cargo build --release --locked -p strixonomy-cli -p strixonomy-lsp --bins
    ./target/release/strixonomy inspect fixtures
  '

  run_step "LSP smoke + reasoner tests" bash -c '
    cargo build --locked -p strixonomy-lsp --bins
    cargo test -p strixonomy-workspace --test lsp_smoke
    cargo test -p strixonomy-workspace --test lsp_reasoner
  '

  run_step "crate packaging dry-run" bash -c '
    set -euo pipefail
    cargo publish -p strixonomy-core --dry-run --allow-dirty
    cargo publish -p strixonomy-robot --dry-run --allow-dirty
    cargo build -p strixonomy-obo -p strixonomy-diagnostics -p strixonomy-owl -p strixonomy-cli -p strixonomy
  '

  # mkdocs/audit in PARALLEL=1 mode can starve strixonomy-lsp cold starts (>20s initialize).
  wait_bg_steps_if_pending

  run_step "extension jobs (unit + e2e)" bash -c "
    set -euo pipefail
    cargo build -p strixonomy-lsp --bins

    cd extension
    npm ci

    npm --prefix webview-ui ci
    npm --prefix webview-ui test

    npm run compile
    STRIXONOMY_LSP_BIN=\"$ROOT/target/debug/strixonomy-lsp\" npm test

    mkdir -p \"server/linux-x64\"
    cp \"$ROOT/target/debug/strixonomy-lsp\" \"server/linux-x64/strixonomy-lsp\"
    chmod +x \"server/linux-x64/strixonomy-lsp\"
    npx vsce package --no-dependencies -o /tmp/strixonomy-ci-local.vsix
    rm -rf /tmp/strixonomy-vsix-unpack-local
    unzip -q /tmp/strixonomy-ci-local.vsix -d /tmp/strixonomy-vsix-unpack-local
    export STRIXONOMY_E2E_EXTENSION_ROOT=/tmp/strixonomy-vsix-unpack-local/extension
    export STRIXONOMY_LSP_BIN=\"$ROOT/target/debug/strixonomy-lsp\"
    npm test

    cd \"$ROOT\"
    ./scripts/prepare-extension-server.sh '${EXT_PLATFORM}'
    chmod -x \"extension/server/${EXT_PLATFORM}/strixonomy-lsp\"
    cd extension
    npm run compile
    npm run compile:vscode-test
    npm run test:vscode
  "
}

if [[ "$PARALLEL" == "0" ]]; then
  run_step "rustfmt" cargo fmt --all -- --check
  run_step "documentation version sync" ./scripts/check-doc-versions.sh
  run_step "strixonomy rename audit" ./scripts/check-strixonomy-rename.sh
  run_step "parity manifest validation" python3 scripts/validate-parity-manifest.py --paths
  run_step "protege test-port inventory" python3 scripts/validate-protege-test-port.py
  run_step "parity release-gate report" python3 scripts/check-parity-release-gate.py
  run_step "parity docs sync" python3 scripts/generate-parity-docs.py --check
  run_rust_and_extension_steps
  run_step "cargo audit" cargo audit
  run_step "mkdocs strict build" bash -c '
    pip install -q -r docs/requirements.txt
    ./scripts/build-docs.sh
  '
else
  # Overlap cheap / non-cargo jobs with the shared-target Rust pipeline.
  run_bg_step "rustfmt" "rustfmt" cargo fmt --all -- --check
  run_bg_step "documentation version sync" "doc-versions" ./scripts/check-doc-versions.sh
  run_bg_step "strixonomy rename audit" "rename-audit" ./scripts/check-strixonomy-rename.sh
  run_bg_step "parity verification (EPIC-011)" "parity-manifest" bash -c '
    python3 scripts/validate-parity-manifest.py --paths &&
    python3 scripts/validate-protege-test-port.py &&
    python3 scripts/check-parity-release-gate.py &&
    python3 scripts/generate-parity-docs.py --check
  '
  run_bg_step "cargo audit" "cargo-audit" cargo audit
  run_bg_step "mkdocs strict build" "mkdocs" bash -c '
    pip install -q -r docs/requirements.txt
    ./scripts/build-docs.sh
  '

  run_rust_and_extension_steps
fi

# MSRV uses a different toolchain; run after the main pipeline to avoid lock contention.
run_step "MSRV (1.88)" bash -c '
  rustup run 1.88 cargo build -p strixonomy-lsp -p strixonomy-cli --bins
  rustup run 1.88 cargo test --workspace
'

echo ""
echo "════════════════════════════════════════════════════════════"
echo "CI local summary: ${#PASSED[@]} passed, ${#FAILED[@]} failed"
echo "════════════════════════════════════════════════════════════"
if ((${#FAILED[@]} > 0)); then
  echo "Failed:"
  for f in "${FAILED[@]}"; do
    echo "  - ${f}"
  done
  exit 1
fi
echo "All checks passed."
exit 0
