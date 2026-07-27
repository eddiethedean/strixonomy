#!/usr/bin/env bash
# Package first-success tutorial files for GitHub Releases (strixonomy-tutorial.zip).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

OUT="${1:-strixonomy-tutorial.zip}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

mkdir -p "$TMP/strixonomy-tutorial"
cp fixtures/example.ttl fixtures/complex-classes.ttl "$TMP/strixonomy-tutorial/"
cp examples/obo-workflow/demo.obo "$TMP/strixonomy-tutorial/"

(
  cd "$TMP"
  zip -r "$ROOT/$OUT" strixonomy-tutorial
)

echo "Created $ROOT/$OUT"
