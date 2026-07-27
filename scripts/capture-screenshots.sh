#!/usr/bin/env bash
# Capture real VS Code screenshots of Strixonomy into docs/ + extension media.
#
# Requires macOS Screen Recording permission for Terminal/Cursor
# (System Settings → Privacy & Security → Screen Recording).
# Also grant Accessibility if window-bounds capture fails.
#
# Usage:
#   ./scripts/capture-screenshots.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "capture-screenshots.sh currently supports macOS only (screencapture)." >&2
  exit 1
fi

chmod +x scripts/macos-capture-window.sh scripts/assemble-product-tour-gif.py

# Precompiled window-bounds helper (avoids swift JIT on every shot)
if [[ ! -x "$ROOT/target/macos-window-bounds" ]] || [[ scripts/macos-window-bounds.swift -nt "$ROOT/target/macos-window-bounds" ]]; then
  echo "==> compile macos-window-bounds"
  mkdir -p "$ROOT/target"
  swiftc -O -o "$ROOT/target/macos-window-bounds" scripts/macos-window-bounds.swift
fi

OUT_DOCS="$ROOT/docs/assets/screenshots"
OUT_EXT="$ROOT/extension/media/screenshots"
mkdir -p "$OUT_DOCS" "$OUT_EXT"

WORK="$ROOT/target/screenshot-workspace"
rm -rf "$WORK"
mkdir -p "$WORK"

# Seed a small git workspace so Semantic Diff HEAD↔WORKTREE is meaningful.
cp fixtures/example.ttl "$WORK/example.ttl"
cp fixtures/reasoner-el.ttl "$WORK/reasoner-el.ttl"
(
  cd "$WORK"
  git init -q
  git config user.email "screenshots@strixonomy.local"
  git config user.name "Strixonomy Screenshots"
  git add example.ttl reasoner-el.ttl
  git commit -q -m "initial ontology"
  # Working-tree change for semantic diff (label edit)
  perl -pi -e 's/rdfs:label "Person"/rdfs:label "Human"/' example.ttl
)

echo "==> compile extension + LSP"
cargo build -p strixonomy-lsp --bins
# Fresh bundle avoids macOS SIGKILL on quarantined LSP binaries
./scripts/prepare-extension-server.sh darwin-arm64
(
  cd extension
  npm run compile
  npm run compile:vscode-test
)

export STRIXONOMY_CAPTURE_SCREENSHOTS=1
export STRIXONOMY_TEST_FIXTURES="$WORK"
export STRIXONOMY_SCREENSHOT_DIR="$OUT_DOCS"
export STRIXONOMY_CAPTURE_SCRIPT="$ROOT/scripts/macos-capture-window.sh"

BEFORE=$(stat -f '%m' "$OUT_DOCS/explorer-inspector.png" 2>/dev/null || echo 0)

echo "==> launch VS Code e2e capture suite"
echo "    (keep the Extension Development Host visible; do not cover it with other apps)"
# Avoid "another instance of Code is running"
pkill -f 'vscode-darwin-arm64.*Visual Studio Code' 2>/dev/null || true
pkill -f 'vscode-test/user-data-screenshots' 2>/dev/null || true
sleep 1
set +e
(
  cd extension
  npm run test:vscode
)
VSCODE_EXIT=$?
set -e

echo "==> verify captures were rewritten"
MISSING=0
for f in explorer-inspector.png query-workbench.png reasoner.png semantic-diff.png product-tour.gif; do
  path="$OUT_DOCS/$f"
  if [[ ! -f "$path" ]]; then
    echo "  MISSING $f" >&2
    MISSING=1
    continue
  fi
  AFTER=$(stat -f '%m' "$path")
  if (( AFTER <= BEFORE )); then
    echo "  STALE $f (mtime not updated — capture likely failed; check renderer.log)" >&2
    MISSING=1
  else
    echo "  ok $f ($(wc -c <"$path" | tr -d ' ') bytes)"
  fi
done

if (( MISSING != 0 || VSCODE_EXIT != 0 )); then
  echo "Capture failed (vscode exit=$VSCODE_EXIT). Latest renderer.log:" >&2
  LATEST=$(ls -td extension/.vscode-test/user-data/logs/*/window1/renderer.log 2>/dev/null | head -1 || true)
  if [[ -n "$LATEST" ]]; then
    rg -i 'failing|Error:|screenshot|SIGKILL|passing' "$LATEST" | tail -40 >&2 || tail -40 "$LATEST" >&2
  fi
  exit 1
fi

echo "==> sync to extension/media/screenshots"
for f in explorer-inspector.png query-workbench.png reasoner.png semantic-diff.png product-tour.gif; do
  cp -f "$OUT_DOCS/$f" "$OUT_EXT/$f"
  echo "  synced $f"
done

echo "Done. Review assets under $OUT_DOCS"
