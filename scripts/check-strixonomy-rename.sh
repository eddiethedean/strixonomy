#!/usr/bin/env bash
# Fail if product-primary surfaces still use OntoCode/OntoCore as the *current*
# product identity outside an allowlist (historical docs, migration, design).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if ! command -v rg >/dev/null 2>&1; then
  echo "FAIL: ripgrep (rg) is required for the Strixonomy rename audit" >&2
  echo "Install: https://github.com/BurntSushi/ripgrep#installation" >&2
  exit 1
fi

# Intentional rename / migration / historical mentions. Paths and phrases only —
# do not allowlist whole trees that should stay Strixonomy-primary.
ALLOW_REGEX='(scripts/check-strixonomy-rename\.sh|scripts/check-doc-versions\.sh|scripts/parity_|scripts/check-parity|parity/|examples/protege-roundtrip/|tests/protege_port_|docs/migration/|docs/guides/product-identity|docs/glossary\.md|docs/design/adr/0018|docs/design/adr/0022|docs/design/v0\.27|docs/design/ARCHITECTURE|docs/changelog|CHANGELOG\.md|ontocode\.dev/ns#swrlRule|OntoIndex|historical|superseded|formerly OntoCore|Legacy OntoCore|OntoCore →|OntoCode →|OntoCore name|OntoCore/Strixonomy|migration/v0\.|PRE_1_0|ROADMAP\.md|docs/roadmap\.md|docs/protege-parity/|docs/PROTEGE_REVERSE|docs/design/v0\.|mutants\.|site/|target/|Cargo\.lock|node_modules|extension/(dist|out)/|webview-ui/dist|\.git/|LICENSE|abbreviate_string|ONTOCODE_CURRENT|ONTOCODE_0\.18|ONTOCODE_PARITY|pre–v0\.27|pre-v0\.27)'

WORKSPACE_VERSION="$(sed -nE 's/^version = "([^"]+)"/\1/p' Cargo.toml | head -1)"
TAGGED_VERSION="$(tr -d '[:space:]' < docs/TAGGED_RELEASE)"
if [[ "$WORKSPACE_VERSION" != "$TAGGED_VERSION" ]]; then
  ALLOW_REGEX="${ALLOW_REGEX%?}|docs/SHIPPED\\.md|latest tagged|[Cc]urrent tagged|latest public release|published extension|v0\\.26\\.2|marketplace\\.visualstudio\\.com|open-vsx\\.org|crates\\.io)"
fi

FAIL=0

check_pattern() {
  local pattern="$1"
  local label="$2"
  local hits
  hits="$(rg -n --hidden \
    --glob '!target/**' --glob '!site/**' --glob '!node_modules/**' \
    --glob '!mutants.out*/**' --glob '!.git/**' --glob '!Cargo.lock' \
    --glob '!extension/out/**' --glob '!extension/dist/**' \
    --glob '!extension/webview-ui/dist/**' \
    -e "$pattern" . | grep -Ev "$ALLOW_REGEX" || true)"
  if [[ -n "$hits" ]]; then
    echo "FAIL: stale $label found outside allowlist:" >&2
    echo "$hits" | head -40 >&2
    FAIL=1
  else
    echo "ok: no unexpected $label"
  fi
}

check_pattern '\bOntoCode\b' 'OntoCode brand'
check_pattern '\bOntoCore\b' 'OntoCore brand'
check_pattern 'ontocode\.ontocode' 'legacy extension id'
check_pattern '"publisher": "ontocode"' 'legacy publisher'
check_pattern 'name = "ontocore-cli"' 'legacy CLI package'
check_pattern 'name = "ontocore"' 'legacy binary or crate name'
check_pattern 'name = "ontocore-lsp"' 'legacy LSP binary name'

if rg -n 'const CACHE_DIR: &str = "\.ontocore' crates 2>/dev/null | grep -v allow; then
  echo "FAIL: primary cache still .ontocore" >&2
  FAIL=1
else
  echo "ok: primary cache path"
fi

if [[ "$FAIL" -ne 0 ]]; then
  exit 1
fi
echo "Strixonomy rename audit passed."
