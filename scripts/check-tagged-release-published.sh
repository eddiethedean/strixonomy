#!/usr/bin/env bash
# Verify docs/TAGGED_RELEASE matches a published git tag and First success sample URLs resolve.
# Run after pushing vX.Y.Z (see docs/releasing.md). Exit 0 on success.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

TAGGED="$(tr -d '[:space:]' < docs/TAGGED_RELEASE)"
if [[ -z "$TAGGED" ]]; then
  echo "error: docs/TAGGED_RELEASE is empty" >&2
  exit 1
fi

TAG="v${TAGGED}"
echo "Checking published tag ${TAG}…"

if ! git rev-parse -q --verify "refs/tags/${TAG}" >/dev/null 2>&1 \
  && ! git ls-remote --exit-code --tags origin "refs/tags/${TAG}" >/dev/null 2>&1; then
  echo "FAIL: tag ${TAG} not found locally or on origin (do not advertise TAGGED_RELEASE=${TAGGED} until the tag exists)" >&2
  exit 1
fi
echo "ok: tag ${TAG} exists"

SAMPLES=(
  "https://raw.githubusercontent.com/eddiethedean/strixonomy/${TAG}/fixtures/example.ttl"
  "https://raw.githubusercontent.com/eddiethedean/strixonomy/${TAG}/fixtures/complex-classes.ttl"
  "https://raw.githubusercontent.com/eddiethedean/strixonomy/${TAG}/examples/obo-workflow/demo.obo"
)

for url in "${SAMPLES[@]}"; do
  code="$(curl -fsSIL -o /dev/null -w '%{http_code}' "$url" || true)"
  if [[ "$code" != "200" ]]; then
    echo "FAIL: ${url} returned HTTP ${code} (First success curls will break)" >&2
    exit 1
  fi
  echo "ok: ${url}"
done

echo "Tagged release publish check passed for ${TAG}."
