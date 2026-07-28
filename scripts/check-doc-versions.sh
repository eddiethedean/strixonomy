#!/usr/bin/env bash
# Verify user-facing documentation references the workspace package version.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Prefer grep -m1 over `grep | head` — under pipefail, GNU grep exits 2 on SIGPIPE.
VERSION="$(grep -m1 -E '^version = ' Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/')"
if [[ -z "$VERSION" ]]; then
  echo "error: could not read workspace version from Cargo.toml" >&2
  exit 1
fi

TAGGED_VERSION="$(tr -d '[:space:]' < docs/TAGGED_RELEASE)"
if [[ -z "$TAGGED_VERSION" ]]; then
  echo "error: docs/TAGGED_RELEASE is empty" >&2
  exit 1
fi
TAGGED_MINOR="${TAGGED_VERSION%.*}"
MINOR_VERSION="${VERSION%.*}"

# During the v0.27 identity cut, tagged and workspace share Strixonomy names.
# Keep TAGGED_* variables aligned with published GitHub Release / crates.io identity.
TAGGED_FACADE="strixonomy"
TAGGED_OWL="strixonomy-owl"
TAGGED_TUTORIAL="strixonomy-tutorial.zip"
TAGGED_RTD_PROJECT="strixonomy"

echo "Checking documentation for workspace ${VERSION} (latest tagged: ${TAGGED_VERSION})..."

fail=0

check_file_contains() {
  local file="$1"
  local pattern="$2"
  local label="$3"
  if ! grep -qE "$pattern" "$file"; then
    echo "FAIL: $label — expected pattern '$pattern' in $file" >&2
    fail=1
  else
    echo "ok: $label"
  fi
}

check_file_contains "README.md" "v${VERSION}" "README header version"
if ! grep -F -- "--version ${TAGGED_VERSION}" README.md >/dev/null; then
  echo "FAIL: README public install pin — expected --version ${TAGGED_VERSION}" >&2
  fail=1
else
  echo "ok: README public install pin"
fi
if ! grep -F -- "--version ${TAGGED_VERSION}" docs/install-cli-ci.md >/dev/null; then
  echo "FAIL: install-cli-ci install pin — expected --version ${TAGGED_VERSION}" >&2
  fail=1
else
  echo "ok: install-cli-ci install pin"
fi
if ! grep -F -- "--version ${TAGGED_VERSION}" docs/install.md >/dev/null; then
  echo "FAIL: install.md public pin — expected --version ${TAGGED_VERSION}" >&2
  fail=1
else
  echo "ok: install.md public pin"
fi
check_file_contains "docs/install.md" "Canonical install page" "install.md canonical banner"

# Adoption-audit guards: stale current-release / contradictory write-back claims
if ! grep -qE "^\*\*Current release:\*\* v${TAGGED_VERSION}$" ROADMAP.md; then
  echo "FAIL: ROADMAP.md Current release must be v${TAGGED_VERSION}" >&2
  fail=1
else
  echo "ok: ROADMAP.md Current release"
fi
if ! grep -qE "^\*\*Current release:\*\* v${TAGGED_VERSION}$" docs/roadmap.md; then
  echo "FAIL: docs/roadmap.md Current release must be v${TAGGED_VERSION}" >&2
  fail=1
else
  echo "ok: docs/roadmap.md Current release"
fi
if grep -qE '\.owl.*/.*Read-only|Read-only \| Not supported' docs/guides/obo-workflow.md 2>/dev/null; then
  echo "FAIL: docs/guides/obo-workflow.md still claims XML inspector/patch read-only" >&2
  fail=1
else
  echo "ok: obo-workflow write-back table"
fi
if grep -qE 'v0\.22\.0\*\* ships RDF/XML and OWL/XML write-back|v0\.22\.0 ships RDF/XML and OWL/XML write-back' README.md 2>/dev/null; then
  echo "FAIL: README misattributes XML write-back to v0.22 (ships in v0.21)" >&2
  fail=1
else
  echo "ok: README XML write-back attribution"
fi
if ! grep -q 'semantic re-serialize' docs/guides/obo-workflow.md; then
  echo "FAIL: docs/guides/obo-workflow.md should mention semantic re-serialize for XML" >&2
  fail=1
else
  echo "ok: obo-workflow XML re-serialize note"
fi
if [[ ! -f docs/guides/capabilities-by-format.md ]]; then
  echo "FAIL: missing docs/guides/capabilities-by-format.md" >&2
  fail=1
else
  echo "ok: capabilities-by-format.md exists"
fi
check_file_contains "docs/index.md" "Latest tagged v${TAGGED_VERSION}" "docs index hero tagged version"
check_file_contains "extension/README.md" "v${VERSION}" "extension README version"
check_file_contains "extension/package.json" "\"version\": \"${VERSION}\"" "extension package.json version"
EXT_LOCK_VERSION="$(grep -m1 -E '"version"' extension/package-lock.json | sed -E 's/.*"([^"]+)".*/\1/')"
if [[ "$EXT_LOCK_VERSION" != "$VERSION" ]]; then
  echo "FAIL: extension/package-lock.json version ($EXT_LOCK_VERSION) != package.json ($VERSION)" >&2
  fail=1
else
  echo "ok: extension lockfile version matches package.json"
fi
check_file_contains "docs/guides/enterprise-eval.md" "v${TAGGED_VERSION}" "enterprise eval version"
check_file_contains "SECURITY.md" "${TAGGED_MINOR}\.x" "SECURITY.md tagged supported version"
if [[ "$VERSION" != "$TAGGED_VERSION" ]]; then
  check_file_contains "SECURITY.md" "${MINOR_VERSION}\.x.*unreleased" "SECURITY.md unreleased workspace version note"
fi
check_file_contains "docs/release-integrity.md" "VERSION=${TAGGED_VERSION}" "release-integrity example version"
check_file_contains "docs/TAGGED_RELEASE" "${TAGGED_VERSION}" "TAGGED_RELEASE file"
check_file_contains "mkdocs.yml" "site_url: https://strixonomy.readthedocs.io/" "mkdocs site_url matches RTD"
check_file_contains "README.md" "readthedocs.org/projects/${TAGGED_RTD_PROJECT}/badge" "RTD docs badge slug"

# Reference page titles must match latest tagged release (public install target)
for file in docs/authoring.md docs/sql-reference.md docs/sparql-reference.md docs/patch-reference.md docs/cli-reference.md docs/errors.md; do
  if grep -qE "^# .+ \(Strixonomy v0\.5\)" "$file"; then
    echo "FAIL: stale v0.5 title in $file" >&2
    fail=1
  elif ! grep -qE "^# .+ \(Strixonomy v${TAGGED_MINOR}\)" "$file"; then
    echo "FAIL: reference title in $file should mention Strixonomy v${TAGGED_MINOR} (latest tagged)" >&2
    fail=1
  else
    echo "ok: reference title $file"
  fi
done

# Install pins must not reference an older release (allow changelog/migration history)
STALE_PIN_PATHS=(README.md docs extension crates .github)
if rg -q 'VERSION=0\.6\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.6.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.6\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.6.0 install pins"
fi

if rg -q 'VERSION=0\.7\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.7.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.7\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.7.0 install pins"
fi

if rg -q 'VERSION=0\.8\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.8.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.8\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.8.0 install pins"
fi

if rg -q 'VERSION=0\.9\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.9.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.9\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.9.0 install pins"
fi

if rg -q '--version 0\.9\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.9.0 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.9\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.9.0 install pins"
fi

if rg -q 'VERSION=0\.10\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.10.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.10\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.10.0 install pins"
fi

if rg -q '--version 0\.10\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.10.0 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.10\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.10.0 install pins"
fi

if rg -q 'VERSION=0\.11\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.11.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.11\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.11.0 install pins"
fi

if rg -q '--version 0\.11\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.11.0 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.11\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.11.0 install pins"
fi

if rg -q 'VERSION=0\.11\.1' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.11.1 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.11\.1' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.11.1 install pins"
fi

if rg -q '--version 0\.11\.1' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.11.1 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.11\.1' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.11.1 install pins"
fi

if rg -q 'VERSION=0\.11\.2' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.11.2 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.11\.2' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.11.2 install pins"
fi

if rg -q '--version 0\.11\.2' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.11.2 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.11\.2' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.11.2 install pins"
fi

if rg -q 'VERSION=0\.11\.3' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.11.3 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.11\.3' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.11.3 install pins"
fi

if rg -q '--version 0\.11\.3' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.11.3 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.11\.3' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.11.3 install pins"
fi

if rg -q 'VERSION=0\.12\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.12.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.12\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.12.0 install pins"
fi

if rg -q '--version 0\.12\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.12.0 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.12\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.12.0 install pins"
fi

if rg -q 'VERSION=0\.13\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.13.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.13\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.13.0 install pins"
fi

if rg -q '--version 0\.13\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.13.0 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.13\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.13.0 install pins"
fi

if rg -q 'latest \*\*v0\.11\.x\*\* tag' README.md docs extension crates .github --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' 2>/dev/null; then
  echo "FAIL: stale v0.11.x release tag reference in user-facing docs" >&2
  fail=1
else
  echo "ok: no stale v0.11.x release tag references"
fi
USER_FACING_DOCS=(
  docs/faq.md
  docs/install-cli-ci.md
  docs/guides/production-readiness.md
  docs/guides/rust-library.md
  docs/concepts.md
  docs/guides/vscode-extension.md
  docs/security.md
)
for file in "${USER_FACING_DOCS[@]}"; do
  if grep -qE '0\.7\.x \(current\)|ships in v0\.7\.0|Strixonomy v0\.7\.0|Strixonomy v0\.7\.0|at \*\*0\.7\.x\*\*' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.7.x current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.7.x current-release claims in user-facing docs"
fi

# User-facing docs must not claim 0.8.x is the current release
for file in "${USER_FACING_DOCS[@]}" docs/guides/protege-decision.md docs/guides/production-evidence.md; do
  if grep -qE '0\.8\.x \(current\)|ships in v0\.8\.0|Strixonomy v0\.8\.0|Strixonomy v0\.8\.0|at \*\*0\.8\.x\*\*' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.8.x current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.8.x current-release claims in user-facing docs"
fi

# User-facing docs must not claim 0.9.x is the current release
for file in "${USER_FACING_DOCS[@]}" docs/guides/protege-decision.md docs/guides/production-evidence.md docs/guides/release-timeline.md docs/guides/platform-compatibility.md docs/guides/obo-workflow.md docs/guides/lgpl-compliance.md docs/authoring.md docs/patch-reference.md; do
  if grep -qE '0\.9\.x \(current\)|0\.9\.0 \| Current|ships in v0\.9\.0|Strixonomy v0\.9\.0|Strixonomy v0\.9\.0|at \*\*0\.9\.x\*\*|for Strixonomy \*\*v0\.9\.0\*\*|Strixonomy \*\*v0\.9\.0\*\*' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.9.x current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.9.x current-release claims in user-facing docs"
fi

# User-facing docs must not claim 0.10.x is the current release
for file in "${USER_FACING_DOCS[@]}" docs/guides/protege-decision.md docs/guides/production-evidence.md docs/guides/release-timeline.md docs/guides/platform-compatibility.md docs/guides/obo-workflow.md docs/guides/lgpl-compliance.md docs/authoring.md docs/patch-reference.md docs/guides/enterprise-eval.md docs/guides/protege-migration.md docs/guides/protege-coexistence.md docs/strixonomy/index.md docs/strixonomy/rust-api.md docs/ide/feature-tour.md docs/architecture.md docs/vision.md docs/lsp-api.md docs/errors.md docs/webview-protocol.md docs/guides/robot-interop.md docs/guides/enterprise-deployment.md docs/guides/performance-sizing.md docs/ci-integration.md docs/guides/first-success.md docs/ide/semantic-diff.md; do
  if grep -qE '0\.10\.x \(current\)|0\.10\.0 \| Current|ships in v0\.10\.0|Strixonomy v0\.10\.0|Strixonomy v0\.10\.0|at \*\*0\.10\.x\*\*|for Strixonomy \*\*v0\.10\.0\*\*|Strixonomy \*\*v0\.10\.0\*\*|Current release: v0\.10\.0|What v0\.10\.0 delivers|Strixonomy v0\.10 is|Strixonomy v0\.10 targets|Strixonomy v0\.10 supports|evaluating Strixonomy \*\*v0\.10\*\*|Strixonomy \*\*v0\.10\*\*|Strixonomy v0\.10\.0|Strixonomy v0\.10\.0\+' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.10.x current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.10.x current-release claims in user-facing docs"
fi

# User-facing docs must not claim 0.11.0 is the current release (0.11.3+)
for file in "${USER_FACING_DOCS[@]}" docs/guides/protege-decision.md docs/guides/production-evidence.md docs/guides/release-timeline.md docs/guides/platform-compatibility.md docs/guides/obo-workflow.md docs/guides/lgpl-compliance.md docs/authoring.md docs/patch-reference.md docs/guides/enterprise-eval.md docs/guides/protege-migration.md docs/guides/protege-coexistence.md docs/strixonomy/index.md docs/strixonomy/rust-api.md docs/ide/feature-tour.md docs/architecture.md docs/vision.md docs/lsp-api.md docs/errors.md docs/webview-protocol.md docs/guides/robot-interop.md docs/guides/enterprise-deployment.md docs/guides/performance-sizing.md docs/ci-integration.md docs/guides/first-success.md docs/ide/semantic-diff.md docs/SHIPPED.md docs/index.md README.md extension/README.md; do
  if grep -qE '0\.11\.0 \| Current|Current release: v0\.11\.0|What ships today \(v0\.11\.0\)|ships in v0\.11\.0|Strixonomy v0\.11\.0|Strixonomy v0\.11\.0|for Strixonomy \*\*v0\.11\.0\*\*|Strixonomy \*\*v0\.11\.0\*\*|documentation · v0\.11\.0' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.11.0 current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.11.0 current-release claims in user-facing docs"
fi

# User-facing docs must not claim 0.11.1 is the current release (0.11.3+)
for file in "${USER_FACING_DOCS[@]}" docs/guides/protege-decision.md docs/guides/production-evidence.md docs/guides/release-timeline.md docs/guides/platform-compatibility.md docs/guides/obo-workflow.md docs/guides/lgpl-compliance.md docs/authoring.md docs/patch-reference.md docs/guides/enterprise-eval.md docs/guides/protege-migration.md docs/guides/protege-coexistence.md docs/strixonomy/index.md docs/strixonomy/rust-api.md docs/ide/feature-tour.md docs/architecture.md docs/vision.md docs/lsp-api.md docs/errors.md docs/webview-protocol.md docs/guides/robot-interop.md docs/guides/enterprise-deployment.md docs/guides/performance-sizing.md docs/ci-integration.md docs/guides/first-success.md docs/ide/semantic-diff.md docs/SHIPPED.md docs/index.md README.md extension/README.md; do
  if grep -qE '0\.11\.1 \| Current|Current release: v0\.11\.1|What ships today \(v0\.11\.1\)|ships in v0\.11\.1|Strixonomy v0\.11\.1|Strixonomy v0\.11\.1|for Strixonomy \*\*v0\.11\.1\*\*|Strixonomy \*\*v0\.11\.1\*\*|documentation · v0\.11\.1|What.s included in v0\.11\.1' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.11.1 current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.11.1 current-release claims in user-facing docs"
fi

# User-facing docs must not claim 0.11.2 is the current release (0.11.3+)
for file in "${USER_FACING_DOCS[@]}" docs/guides/protege-decision.md docs/guides/production-evidence.md docs/guides/release-timeline.md docs/guides/platform-compatibility.md docs/guides/obo-workflow.md docs/guides/lgpl-compliance.md docs/authoring.md docs/patch-reference.md docs/guides/enterprise-eval.md docs/guides/protege-migration.md docs/guides/protege-coexistence.md docs/strixonomy/index.md docs/strixonomy/rust-api.md docs/ide/feature-tour.md docs/architecture.md docs/vision.md docs/lsp-api.md docs/errors.md docs/webview-protocol.md docs/guides/robot-interop.md docs/guides/enterprise-deployment.md docs/guides/performance-sizing.md docs/ci-integration.md docs/guides/first-success.md docs/ide/semantic-diff.md docs/SHIPPED.md docs/index.md README.md extension/README.md; do
  if grep -qE '0\.11\.2 \| Current|Current release: v0\.11\.2|What ships today \(v0\.11\.2\)|ships in v0\.11\.2|Strixonomy v0\.11\.2|Strixonomy v0\.11\.2|for Strixonomy \*\*v0\.11\.2\*\*|Strixonomy \*\*v0\.11\.2\*\*|documentation · v0\.11\.2|What.s included in v0\.11\.2' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.11.2 current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.11.2 current-release claims in user-facing docs"
fi

# User-facing docs must not claim 0.15.x is the current release (0.16+)
CURRENT_RELEASE_STALE_PATHS=(
  ROADMAP.md docs/roadmap.md docs/roadmap-hub.md docs/index.md
  docs/design/README.md extension/README.md docs/platform/OVERVIEW.md
  ARCHITECTURE.md docs/architecture.md
)
for file in "${CURRENT_RELEASE_STALE_PATHS[@]}"; do
  if grep -qE '\*\*Current release:\*\* v0\.15\.0|v0\.15 ships today|v0\.15 foundation shipped|What.s included in v0\.15' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.15.x current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.15.x current-release claims in user-facing docs"
fi

# Reference status banners must not contradict Strixonomy v{N} titles
for file in docs/authoring.md docs/sql-reference.md docs/sparql-reference.md docs/patch-reference.md docs/lsp-api.md docs/errors.md docs/webview-protocol.md; do
  if grep -qE 'Strixonomy v0\.8' "$file" 2>/dev/null; then
    echo "FAIL: stale Strixonomy v0.8 status banner in $file" >&2
    fail=1
  fi
  if grep -qE 'Strixonomy v0\.9' "$file" 2>/dev/null; then
    echo "FAIL: stale Strixonomy v0.9 status banner in $file" >&2
    fail=1
  fi
  if grep -qE 'Strixonomy v0\.10' "$file" 2>/dev/null; then
    echo "FAIL: stale Strixonomy v0.10 status banner in $file" >&2
    fail=1
  fi
  if grep -qE 'Strixonomy v0\.11\.0' "$file" 2>/dev/null; then
    echo "FAIL: stale Strixonomy v0.11.0 status banner in $file" >&2
    fail=1
  fi
  if grep -qE 'Strixonomy v0\.11\.1' "$file" 2>/dev/null; then
    echo "FAIL: stale Strixonomy v0.11.1 status banner in $file" >&2
    fail=1
  fi
  if grep -qE 'Strixonomy v0\.11\.2' "$file" 2>/dev/null; then
    echo "FAIL: stale Strixonomy v0.11.2 status banner in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: reference pages have no Strixonomy v0.8/v0.9/v0.10/v0.11.0/v0.11.1/v0.11.2 banners"
fi

check_file_contains ".github/workflows/release.yml" "publish_crate strixonomy-obo" "release.yml publishes strixonomy-obo"
check_file_contains ".github/workflows/release.yml" "publish_crate strixonomy-edit" "release.yml publishes strixonomy-edit"
check_file_contains ".github/workflows/release.yml" "publish_crate strixonomy-swrl" "release.yml publishes strixonomy-swrl"
# refactor depends on swrl — publish order must list swrl first
if ! awk '/publish_crate strixonomy-swrl/{s=NR} /publish_crate strixonomy-refactor/{r=NR} END{exit !(s && r && s < r)}' .github/workflows/release.yml; then
  echo "FAIL: release.yml must publish strixonomy-swrl before strixonomy-refactor" >&2
  fail=1
else
  echo "ok: release.yml publishes strixonomy-swrl before strixonomy-refactor"
fi
check_file_contains ".github/workflows/release.yml" "publish_crate strixonomy" "release.yml publishes strixonomy"

# docs/contributing.md should track root CONTRIBUTING.md (Strixonomy branding)
if ! grep -q 'Strixonomy' docs/contributing.md; then
  echo "FAIL: docs/contributing.md missing Strixonomy branding" >&2
  fail=1
elif ! grep -q 'Strixonomy' CONTRIBUTING.md; then
  echo "FAIL: CONTRIBUTING.md missing Strixonomy branding" >&2
  fail=1
else
  echo "ok: contributing docs Strixonomy branding"
fi

WEBVIEW_PKG_VERSION="$(grep -m1 -E '"version"' extension/webview-ui/package.json | sed -E 's/.*"([^"]+)".*/\1/')"
WEBVIEW_LOCK_VERSION="$(grep -m1 -E '"version"' extension/webview-ui/package-lock.json | sed -E 's/.*"([^"]+)".*/\1/')"
if [[ "$WEBVIEW_PKG_VERSION" != "$WEBVIEW_LOCK_VERSION" ]]; then
  echo "FAIL: extension/webview-ui/package-lock.json version ($WEBVIEW_LOCK_VERSION) != package.json ($WEBVIEW_PKG_VERSION)" >&2
  fail=1
elif [[ "$WEBVIEW_PKG_VERSION" != "$VERSION" ]]; then
  echo "FAIL: extension/webview-ui version ($WEBVIEW_PKG_VERSION) != extension/workspace ($VERSION)" >&2
  fail=1
else
  echo "ok: webview-ui version matches extension and lockfile"
fi

# docs/security.md supported versions must match SECURITY.md for tagged minor
if ! grep -q "${TAGGED_MINOR}\.x   | Yes" docs/security.md; then
  echo "FAIL: docs/security.md should list ${TAGGED_MINOR}.x as supported (Yes)" >&2
  fail=1
else
  echo "ok: docs/security.md tagged supported version"
fi

# SECURITY.md ↔ docs/security.md N−1 minor must match (previous tagged line)
N1_MINOR="$(python3 - <<PY
maj, min, _ = "${TAGGED_VERSION}".split(".")
print(f"{maj}.{int(min) - 1}")
PY
)"
if ! grep -qE "${N1_MINOR}\\.x[[:space:]]*\\| Yes" SECURITY.md \
  || ! grep -qE "${N1_MINOR}\\.x[[:space:]]*\\| Yes" docs/security.md; then
  echo "FAIL: SECURITY.md and docs/security.md must both list ${N1_MINOR}.x as Yes (N−1)" >&2
  fail=1
else
  echo "ok: SECURITY.md and docs/security.md N−1 (${N1_MINOR}.x) agree"
fi

# Tier-1 pages must not claim DL Query is unshipped when SHIPPED says Yes
if grep -qE 'DL Query \(Workbench DL mode' docs/SHIPPED.md 2>/dev/null \
  && grep -qE 'Shipped' docs/SHIPPED.md 2>/dev/null; then
  for anti_dl in docs/guides/protege-decision.md docs/guides/production-readiness.md docs/guides/enterprise-eval.md; do
    if grep -qE 'DL Query.*(Not shipped|does not)|without Protégé DL Query' "$anti_dl" 2>/dev/null; then
      echo "FAIL: $anti_dl still claims DL Query is unshipped / optional gap after v0.24" >&2
      fail=1
    fi
  done
fi

# CLI refactor merge/replace must not be documented as IDE-only
if grep -qE 'no.*\`strixonomy refactor merge\`|IDE-only:.*Merge entities' docs/cli-reference.md docs/guides/refactoring.md 2>/dev/null; then
  echo "FAIL: cli-reference/refactoring still claim merge/replace are IDE-only" >&2
  fail=1
else
  echo "ok: CLI merge/replace not marked IDE-only"
fi

# Stale global "refactor Turtle-only" claims after multi-format rename/merge/replace
for stale_ref in docs/SHIPPED.md docs/guides/refactoring.md docs/guides/owl-xml-workflow.md docs/guides/best-practices.md docs/errors.md docs/faq.md; do
  if grep -qE 'Refactor apply is Turtle-only|Format policy:.*Turtle \(`.ttl`\) only|Refactoring apply \| No \(Turtle only\)|Refactor apply is \*\*Turtle-only\*\*' "$stale_ref" 2>/dev/null; then
    echo "FAIL: $stale_ref still claims global Turtle-only refactor after multi-format remaps" >&2
    fail=1
  fi
done
if ! grep -qE 'rename.*merge.*replace' docs/SHIPPED.md; then
  echo "FAIL: docs/SHIPPED.md should document multi-format rename/merge/replace" >&2
  fail=1
else
  echo "ok: no stale Turtle-only-only refactor claims on primary surfaces"
fi

# v0.8 docs added in adoption review
for file in docs/guides/refactoring.md docs/migration/v0.8.md docs/migration/v0.9.md docs/migration/v0.10.md docs/examples/refactoring.md docs/ide/semantic-diff.md docs/strixonomy/rust-api.md docs/guides/protege-migration.md docs/ide/feature-tour.md; do
  if [[ ! -f "$file" ]]; then
    echo "FAIL: missing required doc $file" >&2
    fail=1
  else
    echo "ok: $file exists"
  fi
done

check_file_contains "docs/faq.md" "0\.14\.x" "faq crate version"
check_file_contains "docs/guides/release-timeline.md" "${VERSION}.*Current" "release-timeline current version"
check_file_contains "docs/guides/release-timeline.md" "v0\.12.*Shipped" "release-timeline v0.12 shipped"

# Stale multi-root limitation (v0.10 indexes all folders)
MULTIROOT_STALE_PATHS=(docs/SHIPPED.md docs/faq.md docs/vscode-install.md docs/guides/first-success.md docs/troubleshooting.md)
for file in "${MULTIROOT_STALE_PATHS[@]}"; do
  if grep -qE 'only the \*\*first\*\* folder is indexed|Only the \*\*first\*\* folder is indexed' "$file" 2>/dev/null; then
    echo "FAIL: stale first-folder-only multi-root claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale first-folder-only multi-root claims"
fi

# LSP API must document shipped semanticDiff (not list as unimplemented)
if grep -q 'getSemanticDiff.*Not implemented\|not implemented yet.*getSemanticDiff\|until that LSP method lands' docs/lsp-api.md 2>/dev/null; then
  echo "FAIL: docs/lsp-api.md still claims semanticDiff is not implemented" >&2
  fail=1
else
  echo "ok: lsp-api semanticDiff documented"
fi
if ! grep -q 'strixonomy/semanticDiff' docs/lsp-api.md; then
  echo "FAIL: docs/lsp-api.md missing strixonomy/semanticDiff section" >&2
  fail=1
fi

# LSP API must document shipped completion and codeAction (v0.11)
if grep -qE 'Completion \| Planned' docs/lsp-api.md 2>/dev/null; then
  echo "FAIL: docs/lsp-api.md still lists completion as Planned" >&2
  fail=1
else
  echo "ok: lsp-api completion documented as implemented"
fi
if ! grep -q 'textDocument/codeAction' docs/lsp-api.md; then
  echo "FAIL: docs/lsp-api.md missing textDocument/codeAction section" >&2
  fail=1
else
  echo "ok: lsp-api codeAction documented"
fi

check_file_contains "docs/guides/production-readiness.md" "${TAGGED_MINOR}\.x \\(latest tagged\\)" "production-readiness tagged minor"
check_file_contains "docs/strixonomy/index.md" "v${TAGGED_VERSION}" "ontocore index tagged version"
check_file_contains "docs/strixonomy/rust-api.md" "${TAGGED_FACADE} = \"${TAGGED_MINOR}\"" "rust-api version pin"
check_file_contains "docs/strixonomy/crate-map.md" "${TAGGED_FACADE} = \"${TAGGED_MINOR}\"" "crate-map version pin"
check_file_contains "docs/ide/manage-imports.md" "Manage Imports" "manage-imports guide"
check_file_contains "mkdocs.yml" "ide/manage-imports.md" "mkdocs manage-imports guide"
# Migrations listed in docs/migration/README.md (Help → Migration index only)
check_file_contains "mkdocs.yml" "migration/README.md" "mkdocs migration index"
check_file_contains "docs/migration/README.md" "v0.14.md" "migration index links v0.14"
check_file_contains "docs/migration/README.md" "v0.17.md" "migration index links v0.17"
check_file_contains "docs/migration/README.md" "v0.18.md" "migration index links v0.18"
check_file_contains "docs/migration/README.md" "v0.15 → v0.16" "migration index v0.15→v0.16 row"
check_file_contains "docs/migration/README.md" "v0.14 → v0.15" "migration index v0.14→v0.15 row"
check_file_contains "docs/migration/README.md" "v0.16.md" "migration index links v0.16"
check_file_contains "docs/migration/README.md" "v0.15.md" "migration index links v0.15"
check_file_contains "docs/guides/production-readiness.md" "v${TAGGED_VERSION}" "production-readiness version"
check_file_contains "mkdocs.yml" "strixonomy/rust-api.md" "mkdocs Rust API reference"
check_file_contains "mkdocs.yml" "guides/protege-migration.md" "mkdocs Protégé migration guide"
check_file_contains "mkdocs.yml" "ide/feature-tour.md" "mkdocs feature tour"
check_file_contains "mkdocs.yml" "guides/plugins.md" "mkdocs plugins guide"
check_file_contains "mkdocs.yml" "guides/docs-export.md" "mkdocs docs export guide"
check_file_contains "mkdocs.yml" "roadmap-summary.md" "mkdocs roadmap summary"
check_file_contains "mkdocs.yml" "reference/api-parity-matrix.md" "mkdocs api parity matrix"
check_file_contains "mkdocs.yml" "ide/vscode-settings.md" "mkdocs vscode settings"
check_file_contains "docs/roadmap-summary.md" "Roadmap summary" "roadmap summary page"
check_file_contains "docs/reference/api-parity-matrix.md" "API parity matrix" "api parity matrix page"
check_file_contains "docs/ide/vscode-settings.md" "VS Code extension settings" "vscode settings page"
check_file_contains "mkdocs.yml" "announcement" "mkdocs version announcement banner"
check_file_contains "mkdocs.yml" "guides/product-identity.md" "mkdocs product identity guide"
check_file_contains "mkdocs.yml" "install.md" "mkdocs canonical Install page"
check_file_contains "mkdocs.yml" "guides/capabilities-by-format.md" "mkdocs capabilities-by-format"
check_file_contains "mkdocs.yml" "documentation-index.md" "mkdocs documentation index in Get started"
check_file_contains "mkdocs.yml" "guides/plugins.md" "mkdocs plugins guide in Contribute"
check_file_contains "mkdocs.yml" "known-limitations.md" "mkdocs known limitations"
check_file_contains "mkdocs.yml" "Reference:" "mkdocs Reference tab"
check_file_contains "docs/guides/rust-crates.md" "${TAGGED_FACADE} = \"${TAGGED_MINOR}\"" "rust-crates version pin"

# Stale protege-coexistence version banner
if grep -qE 'evaluating Strixonomy \*\*v0\.6\*\*|v0\.6 support' docs/guides/protege-coexistence.md; then
  echo "FAIL: stale v0.6 content in docs/guides/protege-coexistence.md" >&2
  fail=1
else
  echo "ok: protege-coexistence version"
fi

# Enterprise adoption pack pages
for file in \
  docs/guides/protege-decision.md \
  docs/guides/production-evidence.md \
  docs/guides/governance.md \
  docs/guides/platform-compatibility.md \
  docs/guides/release-timeline.md; do
  if [[ ! -f "$file" ]]; then
    echo "FAIL: missing enterprise doc $file" >&2
    fail=1
  else
    echo "ok: $file exists"
  fi
done

check_file_contains "NOTICES" "v${VERSION}" "NOTICES release version"

check_file_contains "docs/guides/obo-workflow.md" "OBO write-back" "obo-workflow documents OBO write-back"
if grep -qE 'read-only in the Entity Inspector|Write-back in VS Code remains \*\*Turtle only\*\*' docs/guides/obo-workflow.md; then
  echo "FAIL: docs/guides/obo-workflow.md still claims OBO inspector is read-only" >&2
  fail=1
else
  echo "ok: obo-workflow OBO edit status"
fi
check_file_contains "docs/guides/protege-coexistence.md" "v0\.21" "protege-coexistence v0.21"
check_file_contains "docs/guides/release-timeline.md" "non-commitment" "release-timeline disclaimer"
if grep -qE 'OBO format \+ ROBOT interop.*Not shipped' docs/guides/enterprise-eval.md; then
  echo "FAIL: enterprise-eval.md contradicts SHIPPED.md on OBO/ROBOT" >&2
  fail=1
else
  echo "ok: enterprise-eval OBO/ROBOT status"
fi

# Tier-1: after XML write-back ships, reject stale "TTL/OBO only" / "XML read-only" claims
# on high-traffic surfaces (exclude historical migration notes and frozen design checklists).
STALE_WRITEBACK_FILES=(
  README.md
  extension/README.md
  docs/index.md
  docs/faq.md
  docs/guides/first-success.md
  docs/guides/protege-decision.md
  docs/guides/production-readiness.md
  docs/guides/enterprise-eval.md
  docs/guides/procurement-appendix.md
  docs/guides/protege-coexistence.md
  docs/guides/protege-migration.md
  docs/guides/best-practices.md
  docs/start.md
  docs/troubleshooting.md
  docs/vscode-install.md
  docs/authoring.md
  docs/ide/feature-tour.md
  docs/ide/obo-authoring.md
  docs/patch-reference.md
  docs/cli-reference.md
  docs/lsp-api.md
)
for f in "${STALE_WRITEBACK_FILES[@]}"; do
  if [[ ! -f "$f" ]]; then
    continue
  fi
  if grep -qE 'Editable today:\*\* Turtle \(`.ttl`\) and OBO \(`.obo`\) only|write-back applies to \*\*`.ttl` and `.obo` only|Entity Inspector edits apply only to \*\*`.ttl` and `.obo`\*\*|No — write-back is \*\*Turtle|in-place write-back does not|RDF/XML and OWL/XML are \*\*read-only\*\*|OWL/XML and RDF/XML index and query as read-only|RDF/XML, OWL/XML, JSON-LD are read-only|Write-back: \*\*Turtle \(`.ttl`\) and OBO \(`.obo`\)\*\*; RDF/XML|Apply \*\*Turtle \(`.ttl`\) and OBO \(`.obo`\)\*\* patch|planned for \*\*v0\.21\*\*|Write-back \| \*\*Turtle and OBO|OWL/XML and RDF/XML read-only' "$f"; then
    echo "FAIL: stale pre-v0.21 write-back claim in $f" >&2
    fail=1
  else
    echo "ok: no stale write-back claim in $f"
  fi
done

# Crate READMEs must not advertise the previous minor as "Current version"
if grep -qE 'Current version: 0\.20\.0|--version 0\.20\.0' crates/strixonomy*/README.md crates/strixonomy/README.md 2>/dev/null; then
  echo "FAIL: crate README still pins 0.20.0 (expected ${TAGGED_VERSION})" >&2
  fail=1
else
  echo "ok: crate README version pins"
fi

# Public roadmap must not list the tagged release as Planned
if grep -qE "^\| 21 \| v0\.21 \| F \| Planned" docs/roadmap.md; then
  echo "FAIL: docs/roadmap.md still marks v0.21 as Planned" >&2
  fail=1
else
  echo "ok: docs/roadmap.md v0.21 status"
fi

# Migration index must not show nonsense same-version → same-version for latest
if grep -qE 'v0\.21\.0 → v0\.21\.0' docs/migration/README.md; then
  echo "FAIL: docs/migration/README.md has v0.21.0 → v0.21.0 row" >&2
  fail=1
else
  echo "ok: migration index versions"
fi

if ! grep -qF -- "--version ${TAGGED_VERSION}" crates/strixonomy-cli/README.md; then
  echo "FAIL: strixonomy-cli README missing --version ${TAGGED_VERSION}" >&2
  fail=1
else
  echo "ok: strixonomy-cli README install pin"
fi

# release-integrity must not pin an old example version
if grep -qE '^VERSION=0\.5\.0' docs/release-integrity.md; then
  echo "FAIL: stale VERSION=0.5.0 in docs/release-integrity.md" >&2
  fail=1
fi

# releasing.md tag example must match workspace version
if ! grep -qE "git tag v${VERSION}" docs/releasing.md; then
  echo "FAIL: docs/releasing.md tag example should use v${VERSION}" >&2
  fail=1
else
  echo "ok: releasing.md tag example"
fi

# Stale v0.5 current-release banners (allow historical mentions in changelog/roadmap)
for file in README.md docs/index.md extension/README.md docs/guides/enterprise-eval.md; do
  if grep -qE 'ships in v0\.5\.0|What ships in v0\.5\.0|included in v0\.5\.0|documentation · v0\.5' "$file"; then
    echo "FAIL: stale v0.5.0 current-release banner in $file" >&2
    fail=1
  fi
done

# RTD URL hygiene (search source paths only — exclude built site/)
RTD_SEARCH_PATHS=(
  README.md CONTRIBUTING.md extension crates docs scripts .github
)

if rg -q 'onto-code\.readthedocs|readthedocs\.org/projects/onto-code' "${RTD_SEARCH_PATHS[@]}"; then
  echo "FAIL: stale onto-code RTD slug found" >&2
  fail=1
else
  echo "ok: no dead onto-code RTD slug"
fi

if rg -q 'strixonomy-vs\.readthedocs|readthedocs\.org/projects/strixonomy-vs' \
  --glob '!scripts/check-doc-versions.sh' "${RTD_SEARCH_PATHS[@]}"; then
  echo "FAIL: stale strixonomy-vs RTD slug found (use https://strixonomy.readthedocs.io)" >&2
  rg -n 'strixonomy-vs\.readthedocs|readthedocs\.org/projects/strixonomy-vs' \
    --glob '!scripts/check-doc-versions.sh' "${RTD_SEARCH_PATHS[@]}" >&2 || true
  fail=1
else
  echo "ok: no dead strixonomy-vs RTD slug"
fi

if rg -q 'https://strixonomy\.readthedocs\.io/en/latest/[^)"[:space:]]+\.md' "${RTD_SEARCH_PATHS[@]}"; then
  echo "FAIL: absolute RTD URLs must not use .md extension (use trailing slash paths)" >&2
  rg -n 'https://strixonomy\.readthedocs\.io/en/latest/[^)"[:space:]]+\.md' "${RTD_SEARCH_PATHS[@]}" >&2 || true
  fail=1
else
  echo "ok: RTD URLs without .md extension"
fi

if rg -q 'https://strixonomy\.readthedocs\.io/"' README.md CONTRIBUTING.md extension crates docs; then
  echo "FAIL: RTD page URLs must include /en/latest/ (not bare project root)" >&2
  rg -n 'https://strixonomy\.readthedocs\.io/"' README.md CONTRIBUTING.md extension crates docs >&2 || true
  fail=1
else
  echo "ok: RTD page URLs use /en/latest/"
fi

check_file_contains "extension/package.json" "guides/first-success/" "extension homepage first-success tutorial"
check_file_contains "extension/README.md" "ide/vscode-extension/" "extension README VS Code docs path"
check_file_contains "docs/guides/vscode-extension.md" "ide/vscode-extension" "vscode hub redirect"
check_file_contains "docs/guides/rust-crates.md" "ide/vscode-extension" "rust hub cross-link"
check_file_contains "crates/strixonomy-cli/src/main.rs" "Strixonomy v${VERSION%.*}" "CLI about string version"
check_file_contains "docs/changelog.md" "v${VERSION}" "docs changelog current release"

# Root VISION / ROADMAP / CONTRIBUTING / ARCHITECTURE are pointers; bodies live under docs/
check_file_contains "ARCHITECTURE.md" "docs/architecture.md" "ARCHITECTURE.md points to canonical docs copy"
check_file_contains "VISION.md" "docs/vision.md" "VISION.md points to canonical docs copy"
check_file_contains "ROADMAP.md" "docs/roadmap.md" "ROADMAP.md points to canonical docs copy"
check_file_contains "CONTRIBUTING.md" "docs/contributing.md" "CONTRIBUTING.md points to canonical docs copy"
check_file_contains "docs/architecture.md" "Ontologos thinks" "docs architecture responsibility line"
check_file_contains "docs/vision.md" "Build the modern open-source platform" "docs vision mission phrase"
check_file_contains "docs/roadmap.md" "v0.11 — Editor depth & distribution" "docs roadmap v0.11 section"
check_file_contains "docs/roadmap.md" "Shipped releases \(v0.1–v0.19\)" "docs roadmap shipped section"
check_file_contains "docs/roadmap.md" "v0.14 — Plugin host MVP \(shipped\)" "docs roadmap v0.14 shipped section"
check_file_contains "docs/roadmap.md" "v1.2 — Ontology Toolchain Platform" "docs roadmap v1.2 milestone"
check_file_contains "docs/roadmap.md" "owlmake" "docs roadmap owlmake integration"
check_file_contains "mkdocs.yml" "vision.md" "mkdocs Platform nav"

# User-facing guides must not claim dl/auto are stubbed or not shipped
DL_STUB_GUIDE_PATHS=(
  docs/guides/enterprise-eval.md
  docs/guides/protege-decision.md
  docs/guides/protege-coexistence.md
  docs/guides/production-readiness.md
  docs/guides/release-timeline.md
)
for file in "${DL_STUB_GUIDE_PATHS[@]}"; do
  if grep -qiE 'dl.*stub|auto.*stub|stubbed until OntoLogos 1\.0|Not shipped.*OntoLogos 1\.0|not shipped in v0\.9.*dl|not shipped in v0\.9.*auto' "$file" 2>/dev/null; then
    echo "FAIL: stale dl/auto stub claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale dl/auto stub claims in enterprise guides"
fi

if grep -qiE 'dl.*stub|auto.*stub' docs/workspace-limits.md 2>/dev/null; then
  echo "FAIL: stale dl/auto stub claim in docs/workspace-limits.md" >&2
  fail=1
else
  echo "ok: workspace-limits dl/auto status"
fi

# User-facing crate pins must not reference a previous minor release
CRATE_PIN_PATHS=(docs README.md extension crates CONTRIBUTING.md)
if rg -q 'strixonomy = "0\.9"' "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' 2>/dev/null; then
  echo "FAIL: stale strixonomy = \"0.9\" pin found outside migration/design/changelog" >&2
  rg -n 'strixonomy = "0\.9"' "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale strixonomy = \"0.9\" user-facing pins"
fi

if rg -q 'strixonomy = "0\.10"' "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' 2>/dev/null; then
  echo "FAIL: stale strixonomy = \"0.10\" pin found outside migration/design/changelog" >&2
  rg -n 'strixonomy = "0\.10"' "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale strixonomy = \"0.10\" user-facing pins"
fi

if rg -q 'strixonomy = "0\.11"' "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' 2>/dev/null; then
  echo "FAIL: stale strixonomy = \"0.11\" pin found outside migration/design/changelog" >&2
  rg -n 'strixonomy = "0\.11"' "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale strixonomy = \"0.11\" user-facing pins"
fi

# Stale crate pins for previous minors (0.14–0.18) when current is newer
PREV_MINOR_MAJOR="${MINOR_VERSION%%.*}"
PREV_MINOR_MINOR="${MINOR_VERSION#*.}"
if [[ "$PREV_MINOR_MAJOR" == "0" ]] && [[ "$PREV_MINOR_MINOR" -ge 17 ]]; then
  for stale in 14 15 16 17 18; do
    if rg -q "strixonomy = \"0\\.${stale}\"" "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' 2>/dev/null; then
      echo "FAIL: stale strixonomy = \"0.${stale}\" pin found outside migration/design/changelog" >&2
      rg -n "strixonomy = \"0\\.${stale}\"" "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' 2>/dev/null || true
      fail=1
    fi
  done
  if [[ "$fail" -eq 0 ]]; then
    echo "ok: no stale strixonomy = \"0.14\"–\"0.18\" user-facing pins"
  fi
fi

# Stale --version install pins for previous minors (0.14–0.18)
INSTALL_PIN_EXCLUDES=(--glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**')
if [[ "$PREV_MINOR_MAJOR" == "0" ]] && [[ "$PREV_MINOR_MINOR" -ge 17 ]]; then
  for stale in 14 15 16 17 18; do
    if rg -q "--version 0\\.${stale}\\." "${STALE_PIN_PATHS[@]}" "${INSTALL_PIN_EXCLUDES[@]}" 2>/dev/null; then
      echo "FAIL: stale --version 0.${stale}.x install pin found outside migration/design/changelog" >&2
      rg -n "--version 0\\.${stale}\\." "${STALE_PIN_PATHS[@]}" "${INSTALL_PIN_EXCLUDES[@]}" 2>/dev/null || true
      fail=1
    fi
  done
  if [[ "$fail" -eq 0 ]]; then
    echo "ok: no stale --version 0.14–0.18 install pins"
  fi
fi

# Stale release-tag guidance (must say current minor, not an older one)
if rg -q 'latest \*\*v0\.15\.x\*\* tag|latest \*\*v0\.14\.x\*\* tag|latest \*\*v0\.16\.x\*\* tag' docs/install-cli-ci.md docs/guides README.md extension 2>/dev/null; then
  echo "FAIL: stale v0.14/v0.15/v0.16 release tag reference in install docs (expected v${MINOR_VERSION}.x)" >&2
  rg -n 'latest \*\*v0\.1[4-6]\.x\*\* tag' docs/install-cli-ci.md docs/guides README.md extension 2>/dev/null || true
  fail=1
else
  echo "ok: no stale v0.14–v0.16 release tag references"
fi
check_file_contains "docs/install-cli-ci.md" "latest \\*\\*v${TAGGED_MINOR}\\.x\\*\\* tag" "install-cli-ci Path D current tag"
check_file_contains "docs/guides/which-artifact.md" "${TAGGED_FACADE} = \"${TAGGED_MINOR}\"" "which-artifact crate pin"
check_file_contains "docs/guides/api-stability.md" "Published crates use \\*\\*${TAGGED_MINOR}\\.x\\*\\*" "api-stability published crates minor"
check_file_contains "docs/ci-integration.md" "VERSION=${TAGGED_VERSION}" "ci-integration release binary pin"
check_file_contains "docs/faq.md" "version ${TAGGED_VERSION}" "faq CI version pin"
check_file_contains "docs/known-limitations.md" "Latest tagged release: v${TAGGED_VERSION}" "known-limitations tagged release banner"
check_file_contains "docs/index.md" "Latest tagged v${TAGGED_VERSION}" "docs index tagged release banner"
if grep -qE 'semantic diff\) is the v1\.0 goal|Full Protégé parity \(.*semantic diff\)' docs/faq.md 2>/dev/null; then
  echo "FAIL: docs/faq.md contradicts SHIPPED on semantic diff" >&2
  fail=1
else
  echo "ok: faq semantic diff status"
fi

# start.md must not list multi-root support under 'When not to use'
if grep -A20 'When not to use Strixonomy' docs/start.md | grep -qE 'Multi-root VS Code workspaces are supported'; then
  echo "FAIL: docs/start.md lists multi-root under 'When not to use'" >&2
  fail=1
else
  echo "ok: start.md multi-root placement"
fi

check_file_contains "docs/ui/ROADMAP_MAPPING.md" "Master checklist" "ui roadmap master checklist"
check_file_contains "docs/start.md" "guides/which-artifact.md" "start.md links which-artifact detail"
check_file_contains "docs/guides/which-artifact.md" "Which artifact do I need" "which-artifact guide title"

# SHIPPED known limitations must reflect Turtle + OBO write-back (v0.12)
if grep -qE '\| Write-back \| \*\*Turtle only\*\*' docs/SHIPPED.md 2>/dev/null; then
  echo "FAIL: docs/SHIPPED.md known limitations still say Turtle-only write-back" >&2
  fail=1
else
  echo "ok: SHIPPED write-back limitations"
fi

# User-facing docs must not claim OBO inspector is read-only (v0.12 write-back)
OBO_READONLY_PATHS=(docs README.md extension crates .github)
if rg -q 'read-only in inspector|OBO is read-only|write-back: Turtle only|write-back is \*\*Turtle only\*\*|writes Turtle only' "${OBO_READONLY_PATHS[@]}" \
  --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/adr/**' 2>/dev/null; then
  echo "FAIL: stale OBO read-only or Turtle-only write-back claim in user-facing docs" >&2
  rg -n 'read-only in inspector|OBO is read-only|write-back: Turtle only|write-back is \*\*Turtle only\*\*|writes Turtle only' "${OBO_READONLY_PATHS[@]}" \
    --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/adr/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale OBO read-only claims"
fi

# Property chain editing shipped in v0.12 — docs must not say view-only
if rg -q 'chains view-only|property chains are view-only|chains are view-only' docs README.md extension crates \
  --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale property chains view-only claim in user-facing docs" >&2
  rg -n 'chains view-only|property chains are view-only|chains are view-only' docs README.md extension crates \
    --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale property chains view-only claims"
fi

# Architecture banner: during an unreleased minor, allow "in progress" wording
if grep -qE "v${MINOR_VERSION} ships today" ARCHITECTURE.md 2>/dev/null; then
  echo "ok: ARCHITECTURE.md v${MINOR_VERSION} ships today banner"
elif grep -qE "v${MINOR_VERSION} in progress" ARCHITECTURE.md 2>/dev/null; then
  echo "ok: ARCHITECTURE.md v${MINOR_VERSION} in progress banner"
else
  echo "FAIL: ARCHITECTURE.md v${MINOR_VERSION} banner — expected 'ships today' or 'in progress'" >&2
  fail=1
fi
if grep -qE "v${MINOR_VERSION} ships today" docs/architecture.md 2>/dev/null; then
  echo "ok: docs/architecture.md v${MINOR_VERSION} ships today banner"
elif grep -qE "v${MINOR_VERSION} in progress" docs/architecture.md 2>/dev/null; then
  echo "ok: docs/architecture.md v${MINOR_VERSION} in progress banner"
else
  echo "FAIL: docs/architecture.md v${MINOR_VERSION} banner — expected 'ships today' or 'in progress'" >&2
  fail=1
fi

# Stale CLI alias notes
if rg -q 'ontocore alias is planned' docs --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale 'ontocore alias is planned' note in docs" >&2
  rg -n 'ontocore alias is planned' docs --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale strixonomy alias notes"
fi

# MkDocs strict: markdown links must not point at directories (use README.md)
if rg -q '\]\(\.\./ui/\)|\]\(ui/\)[^R]' docs --glob '*.md' 2>/dev/null; then
  echo "FAIL: directory-only markdown link (use ui/README.md not ui/)" >&2
  rg -n '\]\(\.\./ui/\)|\]\(ui/\)[^R]' docs --glob '*.md' 2>/dev/null || true
  fail=1
else
  echo "ok: no directory-only ui/ markdown links"
fi

# design/ARCHITECTURE.md shipped banner: current minor if released, else previous tagged minor
PREV_MINOR_VERSION="$(python3 - <<PY
maj, minor, *_ = "${VERSION}".split(".")
print(f"{maj}.{int(minor)-1}")
PY
)"
if grep -qE "Shipped through v${MINOR_VERSION}:" docs/design/ARCHITECTURE.md 2>/dev/null; then
  echo "ok: design ARCHITECTURE shipped banner (current minor)"
elif grep -qE "Shipped through v${PREV_MINOR_VERSION}:" docs/design/ARCHITECTURE.md 2>/dev/null \
  && grep -qE "Unreleased on v${MINOR_VERSION}" docs/design/ARCHITECTURE.md 2>/dev/null; then
  echo "ok: design ARCHITECTURE shipped banner (previous minor + unreleased note)"
elif grep -qE 'Shipped through v0\.(1[0-7]|[0-9]):' docs/design/ARCHITECTURE.md 2>/dev/null; then
  echo "FAIL: docs/design/ARCHITECTURE.md shipped banner still says through an older minor (expected v${MINOR_VERSION} or v${PREV_MINOR_VERSION} + unreleased)" >&2
  fail=1
else
  echo "FAIL: docs/design/ARCHITECTURE.md must say Shipped through v${MINOR_VERSION}: (or v${PREV_MINOR_VERSION}: with Unreleased on v${MINOR_VERSION})" >&2
  fail=1
fi

# LSP API must document OBO write-back alongside Turtle
for file in docs/lsp-api.md docs/strixonomy/lsp.md; do
  if grep -qE 'applyAxiomPatch.*Turtle write-back|Turtle write-back only|true` for Turtle write-back' "$file" 2>/dev/null; then
    echo "FAIL: $file still implies Turtle-only applyAxiomPatch" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: LSP docs mention Turtle+OBO write-back"
fi

# patch-reference intro must mention OBO
if grep -qE '^Turtle write-back uses a JSON array' docs/patch-reference.md 2>/dev/null; then
  echo "FAIL: docs/patch-reference.md intro still says Turtle write-back only" >&2
  fail=1
else
  echo "ok: patch-reference intro covers Turtle+OBO"
fi

# FAQ inspector edit must not contradict OBO write-back (v0.12)
if grep -A2 'I cannot edit in the Entity Inspector' docs/faq.md | grep -qE 'Turtle \(\`\.ttl\`\) only' 2>/dev/null; then
  echo "FAIL: docs/faq.md inspector edit answer still says Turtle-only" >&2
  fail=1
else
  echo "ok: faq inspector edit answer"
fi

check_file_contains "docs/roadmap-hub.md" "v${VERSION}" "roadmap-hub current release"
check_file_contains "docs/guides/api-stability.md" "API stability" "api stability page"
check_file_contains "docs/guides/legacy-guide-urls.md" "Legacy guide URLs" "legacy guide redirects page"
check_file_contains "docs/ide/obo-authoring.md" "OBO authoring" "obo authoring guide"
check_file_contains "mkdocs.yml" "roadmap-hub.md" "mkdocs roadmap hub"
check_file_contains "mkdocs.yml" "guides/api-stability.md" "mkdocs api stability"
check_file_contains "mkdocs.yml" "guides/legacy-guide-urls.md" "mkdocs legacy redirects"
check_file_contains "mkdocs.yml" "ide/obo-authoring.md" "mkdocs obo authoring"

check_file_contains "docs/cursor-prompts/README.md" "Cursor implementation prompts" "cursor prompts index"
check_file_contains "docs/platform/OVERVIEW.md" "Platform overview" "platform overview"
check_file_contains "docs/platform/ONTOUI.md" "OntoUI" "platform OntoUI doc"
check_file_contains "docs/adr/README.md" "Product & platform ADRs" "product adr index"
check_file_contains "docs/glossary.md" "OntoUI" "glossary OntoUI term"
check_file_contains "docs/documentation-index.md" "Documentation index" "docs documentation index"
check_file_contains "docs/documentation-index.md" "v${TAGGED_VERSION}" "documentation-index tagged release"
check_file_contains "mkdocs.yml" "guides/versions-and-channels.md" "mkdocs versions and channels"
check_file_contains "docs/guides/versions-and-channels.md" "Versions and channels" "versions and channels page"
check_file_contains "mkdocs.yml" "guides/architecture-tour.md" "mkdocs architecture tour"
check_file_contains "mkdocs.yml" "guides/testing-matrix.md" "mkdocs testing matrix"
check_file_contains "mkdocs.yml" "guides/procurement-appendix.md" "mkdocs procurement appendix"
check_file_contains "mkdocs.yml" "design/adr/README.md" "mkdocs engineering ADRs"
check_file_contains "mkdocs.yml" "adr/README.md" "mkdocs product ADRs"
check_file_contains "docs/guides/architecture-tour.md" "Architecture tour" "architecture tour page"
check_file_contains "docs/guides/testing-matrix.md" "Contributor testing matrix" "testing matrix page"
check_file_contains "docs/design/adr/README.md" "0020-semantic-transaction" "ADR-0020 in index"
check_file_contains "mkdocs.yml" "Catalog SQL" "mkdocs catalog SQL reference label"
check_file_contains "docs/engineering.md" "Engineering docs \\(GitHub\\)" "engineering pointer page"
check_file_contains "docs/known-limitations.md" "Known limitations" "known limitations page"
check_file_contains "docs/ui/README.md" "OntoUI" "ui readme OntoUI term"

check_file_contains "mkdocs.yml" "guides/owl-xml-workflow.md" "mkdocs owl-xml workflow guide"
check_file_contains "docs/guides/owl-xml-workflow.md" "Horned full-document re-serialize" "owl-xml workflow guide"
check_file_contains "docs/strixonomy/rust-api.md" "Book ↔ docs.rs crosswalk" "rust-api docs.rs crosswalk"
check_file_contains "docs/troubleshooting.md" "Where to start" "troubleshooting decision tree"
check_file_contains "docs/platform/OVERVIEW.md" "v0.20 foundation shipped" "platform overview shipped banner"

# vision.md must reference current shipped release (not v0.11 or v0.12)
if grep -qE 'what ships in \*\*v0\.11\*\*|what ships in \*\*v0\.12\*\*|ships in \*\*v0\.11\*\*|ships in \*\*v0\.12\*\*' docs/vision.md 2>/dev/null; then
  echo "FAIL: docs/vision.md vision banner references stale release (expected v${VERSION%.*})" >&2
  fail=1
fi
if ! grep -qF "what ships in **v${TAGGED_MINOR}**" docs/vision.md 2>/dev/null; then
  echo "FAIL: docs/vision.md must say what ships in tagged v${TAGGED_MINOR}" >&2
  fail=1
else
  echo "ok: vision banner tagged v${TAGGED_MINOR}"
fi
# Root VISION.md is a pointer; still mention current minor for GitHub readers
if ! grep -qF "what ships in **v${TAGGED_MINOR}**" VISION.md 2>/dev/null; then
  echo "FAIL: VISION.md pointer must mention what ships in tagged v${TAGGED_MINOR}" >&2
  fail=1
else
  echo "ok: VISION.md pointer mentions tagged minor"
fi

check_file_contains "docs/glossary.md" "\\*\\*Implemented\\*\\* \\(v${TAGGED_MINOR}\\)" "glossary OntoCore/Strixonomy version"
check_file_contains "docs/glossary.md" "\\*\\*Shipped\\*\\* \\(v${TAGGED_MINOR}\\)" "glossary WorkspaceStore shipped"
check_file_contains "docs/vscode-install.md" "1.85" "vscode-install minimum VS Code version"
check_file_contains "docs/documentation-index.md" "Shipped v${TAGGED_MINOR}" "documentation-index OntoUI shipped"
if grep -q 'Turtle (`.ttl`) only' extension/README.md 2>/dev/null; then
  echo "FAIL: extension/README.md troubleshooting still says Turtle-only inspector" >&2
  fail=1
else
  echo "ok: extension README inspector write-back"
fi
check_file_contains "CONTRIBUTING.md" "build-docs.sh" "contributing documents build-docs script"
check_file_contains "CONTRIBUTING.md" "run-ci-local.sh" "contributing documents local CI script"
check_file_contains "docs/internals.md" "Extension-only" "internals extension-only path"
check_file_contains "docs/guides/lsp-hello-world.md" "strixonomy-lsp" "lsp hello-world guide"
check_file_contains "mkdocs.yml" "guides/extension-development.md" "mkdocs extension development guide"
check_file_contains "mkdocs.yml" "guides/lsp-hello-world.md" "mkdocs lsp hello-world guide"
check_file_contains "docs/guides/extension-development.md" "extension/" "extension development guide"
check_file_contains "crates/strixonomy-plugin/README.md" "Plugin SDK 1.0" "plugin README SDK 1.0 banner"
check_file_contains "crates/strixonomy-obo/README.md" "strixonomy-obo" "strixonomy-obo README"

# errors.md must reference current release
check_file_contains "docs/errors.md" "v${TAGGED_VERSION}" "errors reference version"

# Canonical SHIPPED matrix must match latest tagged release
check_file_contains "docs/SHIPPED.md" "What ships today \\(v${TAGGED_VERSION}" "SHIPPED header tagged version"
check_file_contains "docs/SHIPPED.md" "Latest tagged: v${TAGGED_VERSION}" "SHIPPED latest tagged line"

# LSP API title matches tagged minor; banner may mention workspace when ahead of tag
check_file_contains "docs/lsp-api.md" "^# Strixonomy LSP API \\(v${TAGGED_MINOR}\\)" "lsp-api title minor"
check_file_contains "docs/ide/feature-tour.md" "^# Strixonomy feature tour \\(current: v${TAGGED_MINOR}\\)" "feature-tour tagged minor"

# rust-library crates claim must match tagged minor
if ! grep -qE "Crates are at \\*\\*${TAGGED_MINOR}\\.x\\*\\*" docs/guides/rust-library.md; then
  echo "FAIL: docs/guides/rust-library.md crates version must be **${TAGGED_MINOR}.x**" >&2
  fail=1
else
  echo "ok: rust-library crates version ${TAGGED_MINOR}.x"
fi

# Public install pins must not target unreleased workspace version
INSTALL_PIN_EXCLUDE_GLOBS=(--glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/releasing.md' --glob '!**/TAGGED_RELEASE')
if [[ "$VERSION" != "$TAGGED_VERSION" ]]; then
  if rg -q "--version ${VERSION}" README.md docs extension crates .github "${INSTALL_PIN_EXCLUDE_GLOBS[@]}" 2>/dev/null; then
    echo "FAIL: public --version ${VERSION} install pin (use ${TAGGED_VERSION} from docs/TAGGED_RELEASE)" >&2
    rg -n "--version ${VERSION}" README.md docs extension crates .github "${INSTALL_PIN_EXCLUDE_GLOBS[@]}" 2>/dev/null || true
    fail=1
  else
    echo "ok: no public --version ${VERSION} install pins"
  fi
  if rg -q "VERSION=${VERSION}" README.md docs extension .github "${INSTALL_PIN_EXCLUDE_GLOBS[@]}" 2>/dev/null; then
    echo "FAIL: public VERSION=${VERSION} install pin (use VERSION=${TAGGED_VERSION})" >&2
    rg -n "VERSION=${VERSION}" README.md docs extension .github "${INSTALL_PIN_EXCLUDE_GLOBS[@]}" 2>/dev/null || true
    fail=1
  else
    echo "ok: no public VERSION=${VERSION} install pins"
  fi
  if rg -q "strixonomy = \"${MINOR_VERSION}\"" README.md docs extension crates CONTRIBUTING.md "${INSTALL_PIN_EXCLUDE_GLOBS[@]}" 2>/dev/null; then
    echo "FAIL: public strixonomy = \"${MINOR_VERSION}\" pin (use \"${TAGGED_MINOR}\" for tagged release)" >&2
    fail=1
  else
    echo "ok: no unreleased strixonomy crate pins in user docs"
  fi
fi

# Reject feature-tour / LSP titles pinned to an older minor than tagged (only when workspace == tagged)
if [[ "$VERSION" == "$TAGGED_VERSION" ]]; then
  PREV_MINOR_MINOR="${TAGGED_MINOR##*.}"
  if [[ "$PREV_MINOR_MINOR" =~ ^[0-9]+$ ]] && [[ "$PREV_MINOR_MINOR" -gt 0 ]]; then
    STALE_MINOR="${TAGGED_MINOR%%.*}.$((PREV_MINOR_MINOR - 1))"
    if grep -qE "feature tour \\(current: v${STALE_MINOR}\\)" docs/ide/feature-tour.md 2>/dev/null; then
      echo "FAIL: feature-tour still says current: v${STALE_MINOR}" >&2
      fail=1
    fi
    if grep -qE "^# Strixonomy LSP API \\(v${STALE_MINOR}\\)" docs/lsp-api.md 2>/dev/null; then
      echo "FAIL: lsp-api title still says v${STALE_MINOR}" >&2
      fail=1
    fi
  fi
fi

# Enterprise eval capability table header must match tagged release
check_file_contains "docs/guides/enterprise-eval.md" "What ships today \\(v${TAGGED_VERSION}" "enterprise-eval capability table version"

# Governance must list tagged minor as current supported stream
if grep -qE "\\*\\*${TAGGED_MINOR}\\.x\\*\\* \\| Yes — current tagged release" docs/guides/governance.md 2>/dev/null; then
  echo "ok: governance tagged release stream"
elif grep -qE "\\*\\*${TAGGED_MINOR}\\.x\\*\\* \\| Yes" docs/guides/governance.md 2>/dev/null; then
  echo "ok: governance tagged release stream"
else
  echo "FAIL: governance must list **${TAGGED_MINOR}.x** as current tagged release" >&2
  fail=1
fi
# Unreleased workspace minor (when ahead of tag) may appear as in-progress
if [[ "$VERSION" != "$TAGGED_VERSION" ]]; then
  if grep -qE "\\*\\*${MINOR_VERSION}\\.x\\*\\* \\| In progress \\(unreleased\\)" docs/guides/governance.md 2>/dev/null; then
    echo "ok: governance in-progress development stream"
  else
    echo "FAIL: governance must list **${MINOR_VERSION}.x** as In progress (unreleased) when workspace > tagged" >&2
    fail=1
  fi
fi

# Migration pages are indexed from docs/migration/README.md (not expanded in mkdocs nav)
check_file_contains "docs/migration/README.md" "v0.16.md" "migration index links v0.16"
check_file_contains "docs/migration/README.md" "v0.15.md" "migration index links v0.15"

# Adoption trust: primary surfaces must not contradict TAGGED_RELEASE
check_file_contains "docs/index.md" "\\*\\*Latest tagged: v${TAGGED_VERSION}\\.\\*\\*" "docs index What ships today tagged line"
check_file_contains "ARCHITECTURE.md" "Latest tagged: v${TAGGED_VERSION}" "root ARCHITECTURE tagged banner"
check_file_contains "docs/architecture.md" "Latest tagged: v${TAGGED_VERSION}" "docs architecture tagged banner"
check_file_contains "docs/guides/procurement-appendix.md" "\\*\\*v${TAGGED_VERSION}\\*\\*" "procurement latest tagged"
check_file_contains "docs/guides/release-timeline.md" "Workspace runtime" "release-timeline v0.20 milestone row"
if ! grep -qE '\*\*v0\.20\*\*.*\*\*Shipped\*\*' docs/guides/release-timeline.md 2>/dev/null; then
  echo "FAIL: docs/guides/release-timeline.md must mark v0.20 as Shipped" >&2
  fail=1
else
  echo "ok: release-timeline v${TAGGED_MINOR} shipped"
fi

# Reject stale "latest tagged is older than TAGGED_RELEASE" on primary surfaces
if [[ "$TAGGED_MINOR" =~ ^[0-9]+\.[0-9]+$ ]]; then
  PREV_MINOR_PART="${TAGGED_MINOR##*.}"
  if [[ "$PREV_MINOR_PART" =~ ^[0-9]+$ ]] && [[ "$PREV_MINOR_PART" -gt 0 ]]; then
    STALE_TAGGED_MINOR="${TAGGED_MINOR%%.*}.$((PREV_MINOR_PART - 1))"
    for stale_file in docs/index.md ARCHITECTURE.md docs/architecture.md docs/guides/procurement-appendix.md docs/guides/production-readiness.md; do
      if grep -qE "Latest tagged: v${STALE_TAGGED_MINOR}\\.0" "$stale_file" 2>/dev/null; then
        echo "FAIL: $stale_file still claims Latest tagged: v${STALE_TAGGED_MINOR}.0 (expected v${TAGGED_VERSION})" >&2
        fail=1
      fi
      if grep -qiE "v${TAGGED_MINOR}.*(unreleased|in progress \\(unreleased\\))" "$stale_file" 2>/dev/null; then
        echo "FAIL: $stale_file still claims v${TAGGED_MINOR} unreleased (tagged is ${TAGGED_VERSION})" >&2
        fail=1
      fi
    done
    if grep -qE "\\*\\*v${STALE_TAGGED_MINOR}\\.0\\*\\* — pin installs" docs/guides/procurement-appendix.md 2>/dev/null; then
      echo "FAIL: procurement-appendix still pins v${STALE_TAGGED_MINOR}.0" >&2
      fail=1
    fi
  fi
fi

# Crate README Cargo.toml pins must match tagged minor (not a stale older minor)
STALE_CRATE_PIN_FAIL=0
while IFS= read -r readme; do
  if grep -qE '^\s*strixonomy[a-z0-9-]*\s*=\s*"0\.[0-9]+"' "$readme" 2>/dev/null; then
    if ! grep -qE "strixonomy[a-z0-9-]* = \"${TAGGED_MINOR}\"" "$readme" 2>/dev/null; then
      echo "FAIL: $readme crate version pin must use \"${TAGGED_MINOR}\" (from docs/TAGGED_RELEASE)" >&2
      STALE_CRATE_PIN_FAIL=1
      fail=1
    fi
  fi
done < <(find crates -name README.md -print)
if [[ "$STALE_CRATE_PIN_FAIL" -eq 0 ]]; then
  echo "ok: crate README version pins match tagged minor ${TAGGED_MINOR}"
fi

# Reject previous-minor "current release" pins when tagged has moved on (e.g. 0.22 leftovers after 0.23)
# Historical changelog/migration/design/parity trees are excluded.
if [[ "$TAGGED_MINOR" =~ ^0\.([0-9]+)$ ]]; then
  PREV_TAGGED_PART="${BASH_REMATCH[1]}"
  if [[ "$PREV_TAGGED_PART" -gt 0 ]]; then
    STALE_PREV_MINOR="0.$((PREV_TAGGED_PART - 1))"
    STALE_PREV_FULL="${STALE_PREV_MINOR}.0"
    PREV_PIN_EXCLUDES=(
      --glob '!**/changelog.md'
      --glob '!**/CHANGELOG.md'
      --glob '!**/migration/**'
      --glob '!**/design/**'
      --glob '!**/protege-parity/**'
      --glob '!**/PROTEGE_REVERSE_ENGINEERING/**'
      --glob '!**/adr/**'
      --glob '!**/cursor-prompts/**'
    )
    PREV_PIN_PATHS=(README.md docs extension crates CONTRIBUTING.md ROADMAP.md ARCHITECTURE.md VISION.md SECURITY.md)
    # Install / Cargo pin leftovers that claim the previous release is current
    if rg -q "VERSION=${STALE_PREV_FULL}" "${PREV_PIN_PATHS[@]}" "${PREV_PIN_EXCLUDES[@]}" 2>/dev/null; then
      echo "FAIL: stale VERSION=${STALE_PREV_FULL} found (expected ${TAGGED_VERSION})" >&2
      rg -n "VERSION=${STALE_PREV_FULL}" "${PREV_PIN_PATHS[@]}" "${PREV_PIN_EXCLUDES[@]}" 2>/dev/null || true
      fail=1
    else
      echo "ok: no stale VERSION=${STALE_PREV_FULL} pins"
    fi
    if rg -q "--version ${STALE_PREV_FULL}" "${PREV_PIN_PATHS[@]}" "${PREV_PIN_EXCLUDES[@]}" 2>/dev/null; then
      echo "FAIL: stale --version ${STALE_PREV_FULL} found (expected ${TAGGED_VERSION})" >&2
      rg -n "--version ${STALE_PREV_FULL}" "${PREV_PIN_PATHS[@]}" "${PREV_PIN_EXCLUDES[@]}" 2>/dev/null || true
      fail=1
    else
      echo "ok: no stale --version ${STALE_PREV_FULL} pins"
    fi
    if rg -q "strixonomy = \"${STALE_PREV_MINOR}\"" "${PREV_PIN_PATHS[@]}" "${PREV_PIN_EXCLUDES[@]}" 2>/dev/null; then
      echo "FAIL: stale strixonomy = \"${STALE_PREV_MINOR}\" pin found (expected \"${TAGGED_MINOR}\")" >&2
      rg -n "strixonomy = \"${STALE_PREV_MINOR}\"" "${PREV_PIN_PATHS[@]}" "${PREV_PIN_EXCLUDES[@]}" 2>/dev/null || true
      fail=1
    else
      echo "ok: no stale strixonomy = \"${STALE_PREV_MINOR}\" pins"
    fi
    if rg -q "Canonical pin: \*\*\`${STALE_PREV_FULL}\`\*\*" docs/install.md 2>/dev/null; then
      echo "FAIL: docs/install.md Canonical pin still ${STALE_PREV_FULL}" >&2
      fail=1
    else
      echo "ok: install.md Canonical pin not ${STALE_PREV_FULL}"
    fi
    # "latest tagged is previous" claims on primary surfaces
    for stale_claim_file in \
      docs/install.md \
      docs/supported-formats.md \
      docs/guides/capabilities-by-format.md \
      docs/guides/enterprise-eval.md \
      docs/guides/enterprise-deployment.md \
      docs/guides/lgpl-compliance.md \
      docs/guides/platform-compatibility.md \
      docs/guides/protege-decision.md \
      docs/guides/production-readiness.md \
      docs/guides/versions-and-channels.md \
      docs/lsp-api.md \
      docs/faq.md \
      docs/known-limitations.md \
      docs/strixonomy/index.md; do
      if [[ ! -f "$stale_claim_file" ]]; then
        continue
      fi
      if grep -qE "latest tagged.*v?${STALE_PREV_FULL}|v${STALE_PREV_FULL} \(latest tagged\)|Latest tagged( release)?: v${STALE_PREV_FULL}|Published crates are( at)? \*\*${STALE_PREV_MINOR}\.x\*\*|Recommended installs \(v${STALE_PREV_FULL}\)|Documents behavior in \*\*Strixonomy v${STALE_PREV_FULL}\*\*|Canonical pin: \*\*\`${STALE_PREV_FULL}\`\*\*|badge/[a-z0-9-]+-${STALE_PREV_FULL}-" "$stale_claim_file" 2>/dev/null; then
        echo "FAIL: $stale_claim_file still treats ${STALE_PREV_FULL} / ${STALE_PREV_MINOR}.x as current/latest" >&2
        fail=1
      fi
    done
    if [[ "$fail" -eq 0 ]]; then
      echo "ok: no previous-minor (${STALE_PREV_MINOR}) current-release claims on primary surfaces"
    fi
    # Tutorial / examples curl tags must not pin the previous release when describing current samples
    for curl_file in docs/guides/first-success.md docs/examples/index.md docs/vscode-install.md; do
      if grep -qE "ontocode/v${STALE_PREV_FULL}/|releases/tag/v${STALE_PREV_FULL}" "$curl_file" 2>/dev/null; then
        echo "FAIL: $curl_file still downloads samples from v${STALE_PREV_FULL} (expected v${TAGGED_VERSION})" >&2
        fail=1
      fi
    done
    if [[ "$fail" -eq 0 ]]; then
      echo "ok: tutorial/example sample URLs not pinned to v${STALE_PREV_FULL}"
    fi
    # SHIPPED must not list SWRL under Manchester "Not shipped" when the matrix claims SWRL shipped
    if grep -qE 'SWRL validate / author.*Shipped' docs/SHIPPED.md 2>/dev/null \
      && grep -qE '\*\*Not shipped:\*\*.*SWRL' docs/SHIPPED.md 2>/dev/null; then
      echo "FAIL: docs/SHIPPED.md both ships SWRL and lists SWRL under Not shipped" >&2
      fail=1
    else
      echo "ok: SHIPPED SWRL ship/not-shipped consistency"
    fi
    # Roadmap must not mark the tagged release as Planned
    if grep -qE "\| ${TAGGED_MINOR##*.} \| v${TAGGED_MINOR} \| F \| Planned" docs/roadmap.md 2>/dev/null; then
      echo "FAIL: docs/roadmap.md still marks v${TAGGED_MINOR} as Planned" >&2
      fail=1
    else
      echo "ok: docs/roadmap.md v${TAGGED_MINOR} not Planned"
    fi
    check_file_contains "docs/migration/README.md" "v${TAGGED_MINOR}.md" "migration index links current minor"

    # Content-drift guards (readiness columns, mixed crate pins, pilot wording)
    if grep -qE "v${STALE_PREV_MINOR} readiness|Stability \(v${STALE_PREV_FULL}|Stability \(v${STALE_PREV_MINOR}|v${STALE_PREV_MINOR} policy" docs/guides/production-readiness.md 2>/dev/null; then
      echo "FAIL: docs/guides/production-readiness.md still uses ${STALE_PREV_MINOR} readiness/stability/policy columns" >&2
      fail=1
    else
      echo "ok: production-readiness not using ${STALE_PREV_MINOR} column labels"
    fi
    check_file_contains "docs/guides/production-readiness.md" "v${TAGGED_MINOR} readiness" "production-readiness current readiness column"
    if grep -qE "strixonomy-core = \"0\.[0-9]+\"" docs/guides/api-stability.md 2>/dev/null; then
      if ! grep -qE "strixonomy-core = \"${TAGGED_MINOR}\"" docs/guides/api-stability.md 2>/dev/null; then
        echo "FAIL: docs/guides/api-stability.md strixonomy-core pin must be \"${TAGGED_MINOR}\"" >&2
        fail=1
      else
        echo "ok: api-stability strixonomy-core pin is ${TAGGED_MINOR}"
      fi
    fi
    if grep -qE '\*\*v0\.(1[0-9]|2[0-2])\*\* supports pilot' docs/roadmap.md 2>/dev/null; then
      echo "FAIL: docs/roadmap.md pilot warning still cites a pre-${TAGGED_MINOR} release" >&2
      fail=1
    else
      echo "ok: docs/roadmap.md pilot warning not stuck on older minor"
    fi
    check_file_contains "docs/roadmap.md" "\*\*v${TAGGED_MINOR}\*\* supports pilot" "roadmap pilot warning uses tagged minor"
    check_file_contains "docs/cli-reference.md" "No \`strixonomy swrl\` command" "cli-reference SWRL absence note"
    check_file_contains "docs/guides/dl-query.md" "Not Protégé DL Query" "dl-query honesty page"
    check_file_contains "docs/guides/enterprise-week-2.md" "Enterprise week-2" "enterprise week-2 playbook"
    check_file_contains "docs/guides/plugin-policy.md" "Plugin SDK 1.0 compatibility policy" "plugin policy page"
    check_file_contains "docs/internals.md" "\*\*v${TAGGED_MINOR}\*\*" "internals LSP tagged release"

    # Adoption-audit truth guards (HP-* from docs adoption plan)
    if grep -qiE 'tag pending' docs/SHIPPED.md 2>/dev/null; then
      echo "FAIL: docs/SHIPPED.md must not say 'tag pending' for a tagged release" >&2
      fail=1
    else
      echo "ok: SHIPPED has no 'tag pending'"
    fi
    if grep -qE 'strixonomy-owl = "0\.(1[0-9]|2[0-4])"' docs/guides/rust-library.md 2>/dev/null; then
      echo "FAIL: docs/guides/rust-library.md has stale strixonomy-owl pin (expected ${TAGGED_MINOR})" >&2
      fail=1
    else
      echo "ok: rust-library strixonomy-owl pin not stale"
    fi
    check_file_contains "docs/guides/rust-library.md" "${TAGGED_OWL} = \"${TAGGED_MINOR}\"" "rust-library tagged OWL crate pin"
    check_file_contains "docs/cli-reference.md" "plugins info" "cli-reference plugins info"
    check_file_contains "docs/cli-reference.md" "plugins enable" "cli-reference plugins enable"
    if grep -qE 'DL Query UI → \*\*v0\.2[0-9]\*\*|DL Query UI → \*\*v0\.25\*\*' docs/guides/protege-coexistence.md docs/guides/protege-migration.md 2>/dev/null; then
      echo "FAIL: Protégé guides still claim DL Query UI is future" >&2
      fail=1
    else
      echo "ok: Protégé guides do not defer DL Query UI"
    fi
    if [[ -f docs/getting-started.md ]]; then
      echo "FAIL: docs/getting-started.md must remain renamed to install-cli-ci.md (use mkdocs redirect)" >&2
      fail=1
    else
      echo "ok: getting-started.md removed (install-cli-ci.md is canonical)"
    fi
    check_file_contains "docs/install-cli-ci.md" "Install CLI & CI \\(detail\\)" "install-cli-ci title"
    if grep -qiE 'Marketplace and Open VSX publishes are manual' docs/faq.md docs/guides/versions-and-channels.md 2>/dev/null; then
      echo "FAIL: FAQ/versions still claim Open VSX is always manual" >&2
      fail=1
    else
      echo "ok: Open VSX wording not stuck on always-manual"
    fi

    # Docs trust fixes (post-v0.26 re-audit)
    if grep -qE -- '--prefix[= ]' docs/examples/dl-query.md 2>/dev/null; then
      echo "FAIL: docs/examples/dl-query.md must not document --prefix (flag does not exist)" >&2
      fail=1
    else
      echo "ok: dl-query cookbook has no --prefix"
    fi
    check_file_contains "docs/examples/plugins.md" "plugins list" "plugins cookbook"
    if grep -qiE 'plugin host MVP' docs/faq.md docs/architecture.md extension/README.md 2>/dev/null; then
      echo "FAIL: faq/architecture/extension README still say 'plugin host MVP'" >&2
      fail=1
    else
      echo "ok: no stale 'plugin host MVP' on Tier-1 surfaces"
    fi
    if grep -qiE 'semver-stable ecosystem plugin API remains a \*\*v1\.0 target\*\*|stable third-party plugin ecosystem API' docs/faq.md docs/architecture.md 2>/dev/null; then
      echo "FAIL: faq/architecture still claim stable plugin API is only a v1.0 target (SDK 1.0 wire is frozen)" >&2
      fail=1
    else
      echo "ok: faq/architecture plugin stability wording"
    fi
    if ! grep -qiF "${TAGGED_TUTORIAL}" docs/vscode-install.md 2>/dev/null; then
      echo "FAIL: docs/vscode-install.md must mention ${TAGGED_TUTORIAL}" >&2
      fail=1
    else
      echo "ok: vscode-install tutorial zip"
    fi
    if grep -qiE 'Quick paths to success|Path A — VS Code' docs/install-cli-ci.md 2>/dev/null; then
      echo "FAIL: install-cli-ci.md still has IDE Path A / quick paths (CLI-only page)" >&2
      fail=1
    else
      echo "ok: install-cli-ci is CLI/CI-only"
    fi
  fi
fi

# Reject legacy OntoCore/OntoCode install pins outside migration/design/changelog
LEGACY_PIN_PATHS=(README.md docs extension crates CONTRIBUTING.md SUPPORT.md SECURITY.md .github)
LEGACY_PIN_EXCLUDES=(
  --glob '!**/changelog.md'
  --glob '!**/CHANGELOG.md'
  --glob '!**/migration/**'
  --glob '!**/design/**'
  --glob '!**/protege-parity/**'
  --glob '!**/PROTEGE_REVERSE_ENGINEERING/**'
  --glob '!**/adr/**'
  --glob '!**/compat/**'
)
for legacy_pat in 'cargo install ontocore-cli' 'cargo install ontocore-lsp' 'ontocode-v[0-9]' 'ontocore-v[0-9].*tar\.gz' 'ontocore-v[0-9].*\.vsix'; do
  if rg -q "$legacy_pat" "${LEGACY_PIN_PATHS[@]}" "${LEGACY_PIN_EXCLUDES[@]}" 2>/dev/null; then
    echo "FAIL: legacy install/asset name '$legacy_pat' found outside migration/design/changelog/compat" >&2
    rg -n "$legacy_pat" "${LEGACY_PIN_PATHS[@]}" "${LEGACY_PIN_EXCLUDES[@]}" 2>/dev/null || true
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no legacy ontocore-cli / ontocode-v / ontocore-v install pins"
fi

# Reject "Strixonomy and Strixonomy" / "Strixonomy / Strixonomy" tautology on Tier-1 surfaces
for taut_file in README.md docs/index.md docs/faq.md docs/install.md docs/glossary.md docs/guides/which-artifact.md docs/guides/product-identity.md; do
  if grep -qE 'Strixonomy and Strixonomy|Strixonomy / Strixonomy|Strixonomy & Strixonomy' "$taut_file" 2>/dev/null; then
    echo "FAIL: $taut_file still has Strixonomy/Strixonomy tautology (use IDE vs engine)" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no Strixonomy tautology on Tier-1 naming surfaces"
fi

if [[ "$fail" -ne 0 ]]; then
  echo "Documentation version check failed." >&2
  exit 1
fi

echo "Documentation version check passed."
