#!/usr/bin/env bash
# Sync and activate a Read the Docs version for a release branch or tag.
#
# Requires READTHEDOCS_API_TOKEN (project maintainer token from RTD account settings).
#
# Usage:
#   ./scripts/readthedocs-activate-version.sh release/v0.13.0
#   ./scripts/readthedocs-activate-version.sh v0.13.0
#
# RTD slugifies branch names by replacing "/" with "-", e.g. release/v0.13.0 → release-v0.13.0.

set -euo pipefail

RTD_PROJECT="${RTD_PROJECT:-strixonomy}"
RTD_API="${RTD_API:-https://app.readthedocs.org/api/v3}"
TOKEN="${READTHEDOCS_API_TOKEN:-${RTD_TOKEN:-}}"

REF="${1:-${GITHUB_REF_NAME:-}}"
if [[ -z "$REF" ]]; then
  echo "usage: $0 <branch-or-tag>" >&2
  echo "example: $0 release/v0.13.0" >&2
  exit 1
fi

if [[ -z "$TOKEN" ]]; then
  if [[ "${RTD_ACTIVATION_REQUIRED:-}" == "1" ]]; then
    echo "error: set READTHEDOCS_API_TOKEN (Read the Docs account API token)" >&2
    exit 1
  fi
  echo "warning: READTHEDOCS_API_TOKEN not set; skipping RTD version activation" >&2
  echo "One-time setup: docs/releasing.md § Read the Docs" >&2
  exit 0
fi

# release/v0.13.0 → release-v0.13.0; v0.13.0 is unchanged.
VERSION_SLUG="${REF//\//-}"

auth() {
  printf 'Authorization: Token %s' "$TOKEN"
}

sync_versions() {
  echo "Syncing versions for project ${RTD_PROJECT}..."
  curl -fsS -X POST \
    -H "$(auth)" \
    "${RTD_API}/projects/${RTD_PROJECT}/sync-versions/"
}

version_exists() {
  local status
  status="$(curl -s -o /dev/null -w "%{http_code}" \
    -H "$(auth)" \
    "${RTD_API}/projects/${RTD_PROJECT}/versions/${VERSION_SLUG}/" || true)"
  [[ "$status" == "200" ]]
}

activate_version() {
  echo "Activating RTD version ${VERSION_SLUG}..."
  local status
  status="$(curl -s -o /tmp/rtd-activate-response.json -w "%{http_code}" \
    -X PATCH \
    -H "$(auth)" \
    -H "Content-Type: application/json" \
    -d '{"active": true, "hidden": false}' \
    "${RTD_API}/projects/${RTD_PROJECT}/versions/${VERSION_SLUG}/")"

  if [[ "$status" != "204" ]]; then
    echo "error: activate PATCH returned HTTP ${status}" >&2
    cat /tmp/rtd-activate-response.json >&2 || true
    return 1
  fi

  echo "Activated https://strixonomy.readthedocs.io/en/${VERSION_SLUG}/"
}

sync_versions

attempts=18
wait_seconds=10
for ((i = 1; i <= attempts; i++)); do
  if version_exists; then
    activate_version
    exit 0
  fi

  echo "Waiting for RTD version ${VERSION_SLUG} (${i}/${attempts})..."
  sleep "$wait_seconds"

  # Re-sync a few times while the background sync job catches up.
  if (( i == 3 || i == 9 || i == 15 )); then
    sync_versions || true
  fi
done

echo "error: RTD version ${VERSION_SLUG} not found after sync (ref: ${REF})" >&2
exit 1
