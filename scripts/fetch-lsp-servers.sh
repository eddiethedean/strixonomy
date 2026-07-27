#!/usr/bin/env bash
# Copy pre-built LSP server binaries into extension/server/ for local VSIX packaging.
# Usage: ./scripts/fetch-lsp-servers.sh v0.3.0
set -euo pipefail

TAG="${1:?Usage: $0 <tag> e.g. v0.3.0}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SERVER_DIR="$ROOT/extension/server"
REPO="eddiethedean/ontocode"

mkdir -p "$SERVER_DIR"

platforms=(
  linux-x64
  linux-arm64
  darwin-arm64
  darwin-x64
  win32-x64
)

for platform in "${platforms[@]}"; do
  dest="$SERVER_DIR/$platform"
  mkdir -p "$dest"
  if [[ "$platform" == win32-* ]]; then
    asset="strixonomy-lsp-${TAG}-${platform}.zip"
    tmp="$(mktemp -d)"
    gh release download "$TAG" --repo "$REPO" --pattern "$asset" --dir "$tmp"
    unzip -q "$tmp/$asset" -d "$tmp/out"
    cp "$tmp/out/strixonomy-lsp-${TAG}-${platform}.exe" "$dest/strixonomy-lsp.exe"
    rm -rf "$tmp"
  else
    asset="strixonomy-lsp-${TAG}-${platform}.tar.gz"
    tmp="$(mktemp -d)"
    gh release download "$TAG" --repo "$REPO" --pattern "$asset" --dir "$tmp"
    tar -xzf "$tmp/$asset" -C "$tmp"
    cp "$tmp/strixonomy-lsp-${TAG}-${platform}" "$dest/strixonomy-lsp"
    chmod +x "$dest/strixonomy-lsp"
    rm -rf "$tmp"
  fi
  echo "Installed $platform"
done

echo "All servers installed under $SERVER_DIR"
