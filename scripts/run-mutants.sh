#!/usr/bin/env bash
# Run cargo-mutants on path_jail + OWL patch (manual / nightly).
# Requires: cargo install cargo-mutants
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if ! command -v cargo-mutants >/dev/null 2>&1 && ! cargo mutants --version >/dev/null 2>&1; then
  echo "cargo-mutants not found. Install with: cargo install cargo-mutants" >&2
  exit 1
fi

# File globs with a slash match the full workspace-relative path.
echo "==> mutants: strixonomy-core path_jail.rs"
cargo mutants -p strixonomy-core \
  --file 'crates/strixonomy-core/src/path_jail.rs' \
  --test-tool=cargo \
  "$@"

# Include workspace package so tests/owl_patch_oracles.rs kills apply_one_patch no-ops.
echo "==> mutants: strixonomy-owl patch.rs (tests: strixonomy-owl)"
cargo mutants -p strixonomy-owl \
  --file 'crates/strixonomy-owl/src/patch.rs' \
  --test-package strixonomy-owl \
  --test-tool=cargo \
  "$@"

echo "Mutants runs finished. Inspect mutants.out/ for survivors."
