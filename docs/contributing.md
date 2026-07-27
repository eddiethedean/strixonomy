# Contributing to Strixonomy / Strixonomy

Thank you for contributing. This repository contains:

- **Strixonomy** — Rust semantic workspace engine under `crates/` (`strixonomy` façade, `strixonomy-*` implementation, `strixonomy` CLI, `strixonomy-lsp`)
- **Strixonomy** — VS Code extension under `extension/`
- **User guides** — install, SQL, and LSP API under `docs/`

**AI / agent contributors:** start with [AGENTS.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/AGENTS.md) (SHIPPED-first; do not implement from `docs/design/` targets).

### Docs-only contributors (~15 minutes)

**No Rust or Node required** for documentation-only PRs. Expect **under one hour** end-to-end including review prep:

1. Edit pages under `docs/` (this site). Update root `README.md`, `CONTRIBUTING.md`, or `extension/README.md` when install or marketplace text changes.
2. Run `./scripts/check-doc-versions.sh`.
3. Optional preview: `pip install -r docs/requirements.txt && ./scripts/serve-docs.sh`
4. Open a focused PR.

**Public install pins** must match [`docs/TAGGED_RELEASE`](TAGGED_RELEASE) — not the unreleased workspace version in `Cargo.toml`.

Platform content: edit [vision.md](vision.md), [roadmap.md](roadmap.md), and [architecture.md](architecture.md) here first; keep [GitHub root mirrors](https://github.com/eddiethedean/strixonomy/blob/main/CONTRIBUTING.md#canonical-documentation-paths) in sync when those change.

### Canonical documentation paths

| Topic | GitHub (root) | Read the Docs (`docs/`) |
|-------|---------------|-------------------------|
| Vision | [VISION.md](https://github.com/eddiethedean/strixonomy/blob/main/VISION.md) | [vision.md](vision.md) |
| Architecture | [ARCHITECTURE.md](https://github.com/eddiethedean/strixonomy/blob/main/ARCHITECTURE.md) | [architecture.md](architecture.md) |
| Platform roadmap | [ROADMAP.md](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md) | [roadmap.md](roadmap.md) |
| Engineering specs | — | [design/README.md](design/README.md) |
| Platform planning (v0.14+) | — | [platform/OVERVIEW.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/OVERVIEW.md) |
| Product ADRs | — | [adr/README.md](adr/README.md) |
| Engineering ADRs | — | [design/adr/README.md](design/adr/README.md) |
| Documentation map | — | [documentation-index.md](documentation-index.md) |

Root `VISION.md`, `ARCHITECTURE.md`, and `ROADMAP.md` are mirrored under `docs/` for Read the Docs. **Prefer editing this `docs/` copy first**, then update root mirrors when platform-facing content changes. **Release pin truth** is a single file: [`TAGGED_RELEASE`](TAGGED_RELEASE) — run `./scripts/check-doc-versions.sh` to catch drift (including stale “latest tagged” claims).

**Contributor process (this page):** Edit [`docs/contributing.md`](contributing.md) first for Read the Docs; keep root [`CONTRIBUTING.md`](https://github.com/eddiethedean/strixonomy/blob/main/CONTRIBUTING.md) in sync as the GitHub mirror. Do not treat both as independent sources of truth — one edit, then mirror.

- **Specs** — product and architecture docs under `docs/design/` ([DEPENDENCY_MATRIX.md](design/DEPENDENCY_MATRIX.md) for external crates)

The root Cargo package `strixonomy` is unpublished and hosts workspace integration tests (`tests/`).

**New contributors:** start with [Architecture tour](guides/architecture-tour.md) (~15 min) and [internals.md](internals.md) for role-based paths (Rust, extension, docs, LSP).

### First PR paths

**Smoke PR (~15 minutes docs-only; Rust smoke is longer on a cold machine)** — docs-only or small Rust fix:

1. Install **Rust 1.88+** (Node 20 only if you touch `extension/`).
2. `git clone https://github.com/eddiethedean/strixonomy.git && cd strixonomy`
3. Docs-only: edit docs and run `./scripts/check-doc-versions.sh`.
4. Small Rust fix: `cargo test -p strixonomy-core --lib` (or your touched crate). **Cold first compile of Strixonomy dependencies is often 15–30+ minutes** — not part of the “docs ~15 min” path. Warm cache is much faster; see [testing matrix](guides/testing-matrix.md).
5. `cargo fmt --all --check` when you touch Rust; always run `./scripts/check-doc-versions.sh` for docs changes.
6. Open a focused PR with a short description.

**Full contributor setup (~60+ minutes warm cache; 2–4+ hours cold)** — extension, LSP, or release-impacting changes:

1. Complete smoke steps above.
2. `cargo test --workspace` and `cargo build -p strixonomy-lsp --bins`.
3. Extension: `cd extension && npm ci && ONTOCORE_LSP_BIN=../target/debug/strixonomy-lsp npm test`.
4. Webview: `cd extension/webview-ui && npm ci && npm test`.
5. Before opening a PR: `./scripts/run-ci-local.sh` (30–60+ minutes; matches GitHub Actions).

See [Testing matrix](guides/testing-matrix.md) for which commands to run by change type.

### Plugin contributors (v0.14+)

| Task | Start here |
|------|------------|
| Author a workspace plugin (manifest, validator, exporter) | **[guides/plugins.md](guides/plugins.md)** — canonical author guide |
| Reference implementation | `crates/strixonomy-plugin-naming/`, `examples/plugin-workspace/` |
| Historical trait-based design (do not implement from) | [PLUGIN_SPEC.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/PLUGIN_SPEC.md) (excluded from public docs) |
| Wire LSP / CLI integration | [lsp-api.md](lsp-api.md), `crates/strixonomy-lsp/` |

Run `./scripts/run-ci-local.sh` before PRs that touch plugin host, reference plugins, or extension capability registry.

## Before opening a PR

Use the **[testing matrix](guides/testing-matrix.md)** for the minimum commands for your change type. Do **not** run the full Rust/extension suite for docs-only PRs.

**Docs-only checklist:**

```bash
./scripts/check-doc-versions.sh
# optional: pip install -r docs/requirements.txt && ./scripts/build-docs.sh
```

**Engine / extension checklist (most code changes):**

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build -p strixonomy-lsp --bins
cd extension/webview-ui && npm ci && npm test
cd extension && ONTOCORE_LSP_BIN=../target/debug/strixonomy-lsp npm ci && npm run compile && npm test
./scripts/check-doc-versions.sh
```

**Full CI parity (release branches or broad changes):** `./scripts/run-ci-local.sh` (~30+ minutes). This matches GitHub Actions (rustfmt, clippy, tests, extension e2e, mkdocs, packaging).

Open an issue or discussion before large features. Follow existing commit message style in `git log`.

### Good first issues

Look for GitHub labels `good first issue` and `docs`. Useful first PRs:

- Docs clarity / broken links / version pin nits under `docs/`
- Small unit-test additions next to existing tests in a single crate
- Copy fixes in `extension/README.md` or tutorial steps in [first-success](guides/first-success.md)

## Prerequisites

- Rust **1.88+** (see `rust-version` in `Cargo.toml`)
- Node.js **20** (extension CI)
- `npm` (extension build)
- **`cargo-audit`** — `cargo install cargo-audit` if you run `./scripts/run-ci-local.sh` locally (CI runs `cargo audit`; not required for every PR smoke check)

## Optional dependencies

- **Java 11+** and **[ROBOT](http://robot.obolibrary.org/)** on `PATH` — optional; needed only for manual `strixonomy robot` / ROBOT interop development (not required for `cargo test --workspace`)
- **Python 3.12** — for MkDocs doc site (`pip install -r docs/requirements.txt`)

> **Canonical contributor guide (this page):** Edit [`docs/contributing.md`](contributing.md) for Read the Docs, then sync root [`CONTRIBUTING.md`](https://github.com/eddiethedean/strixonomy/blob/main/CONTRIBUTING.md) on GitHub. Do not treat them as independent sources of truth.

## Build and test

### Rust (full workspace)

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Build the language server binary:

```bash
cargo build -p strixonomy-lsp --bins
```

Run CLI smoke commands against fixtures:

```bash
cargo run -- query fixtures "SELECT * FROM classes"
cargo run -- validate fixtures
```

`cargo run --` uses the workspace default member `strixonomy-cli` (binary name: `strixonomy`).

### Extension

```bash
cd extension
npm ci
npm run compile
npm test
```

### Webview UI (React panels)

```bash
cd extension/webview-ui
npm ci
npm test
```

Extension tests expect a built `strixonomy-lsp` binary. From the repo root:

```bash
cargo build -p strixonomy-lsp --bins
cd extension
export ONTOCORE_LSP_BIN="$(pwd)/../target/debug/strixonomy-lsp"
npm test
```

**F5 / Run Extension:** Open the `extension/` folder in VS Code, build LSP (`cargo build -p strixonomy-lsp --bins`), optionally set `strixonomy.lspPath` to your debug binary, then launch **Run Extension**.

**LSP integration smoke test** (workspace crate):

```bash
cargo test -p strixonomy --test lsp_smoke
cargo test -p strixonomy --test lsp_reasoner
```

**VS Code E2E matrix** (separate workflow): see `.github/workflows/extension-vscode-e2e.yml`. Run locally:

```bash
cargo build -p strixonomy-lsp --bins
cd extension && npm ci && npm run compile && npm run test:vscode
```

See [Debugging guide](debugging.md) for LSP, extension host, and webview workflows.

Full extension packaging (bundles LSP for current platform):

```bash
./scripts/package-extension.sh
```

### Golden and fixture snapshots

Some tests compare output to committed snapshots. To update (env var names retain the legacy `ONTOINDEX_*` prefix):

```bash
# SQL/query golden snapshots (tests/golden/snapshots/)
ONTOINDEX_UPDATE_GOLDEN=1 cargo test golden_classes

# Extension catalog fixture snapshot
ONTOINDEX_UPDATE_FIXTURE_SNAPSHOT=1 cargo test -p strixonomy --test fixture_snapshot
```

Review the diffs before committing.

### Examples

```bash
cargo run -p strixonomy --example index_and_query
cargo run -p strixonomy --example strixonomy_workspace
cargo run -p strixonomy --example workspace_operations
cargo run -p strixonomy --example semantic_diff
```

See [Examples index](examples/index.md) for all runnable assets.

### Documentation site (MkDocs / Read the Docs)

```bash
pip install -r docs/requirements.txt   # Python 3.12 in CI
./scripts/serve-docs.sh
./scripts/build-docs.sh   # CI-equivalent; must pass with no warnings
./scripts/check-doc-versions.sh   # README / RTD / extension version sync
# ENABLE_GIT_REVISION_DATE=true ./scripts/build-docs.sh  # optional: RTD-like stamps (slow)
```

Local/CI builds skip `git-revision-date-localized` for speed; Read the Docs still publishes page stamps. Open http://127.0.0.1:8000. Configuration: [`mkdocs.yml` on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/mkdocs.yml), [`.readthedocs.yaml` on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/.readthedocs.yaml).

### Full local CI (recommended before PR)

Script index: [`scripts/README.md` on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/scripts/README.md).

Mirrors [`.github/workflows/ci.yml` on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/.github/workflows/ci.yml) plus VS Code e2e for your platform:

```bash
./scripts/run-ci-local.sh
```

This runs rustfmt, `./scripts/check-doc-versions.sh`, clippy, workspace tests, MSRV (1.88), CLI smoke, release build, LSP smoke tests, webview-ui tests, extension build/tests, `cargo audit`, crate packaging dry-run, mkdocs strict build, and VS Code extension e2e. Expect 30+ minutes on a cold build. **Not required for docs-only PRs** — see the [testing matrix](guides/testing-matrix.md).

## Pull requests

1. Fork and branch from `main`.
2. Keep changes focused; match existing style and naming.
3. Run the **scoped** checks from the [testing matrix](guides/testing-matrix.md) (docs-only PRs do not need `cargo test --workspace`).
4. Update docs when behavior or public API changes (`README.md`, `docs/`, `CHANGELOG.md` as appropriate). On release, follow the checklist in [releasing.md](releasing.md). Keep root/`docs/` mirrors in sync where the mirror policy applies.
5. For spec changes, label whether they describe **implemented** vs **planned** behavior (see [lsp-api.md](lsp-api.md) as a model).

## Documentation conventions

| Audience | Where to write |
|----------|----------------|
| New users (install, SQL, LSP) | `docs/` |
| Product vision / platform roadmap | Root `VISION.md` / `ROADMAP.md` **and** mirrors under `docs/` (edit both) |
| Product & platform ADRs | `docs/adr/` |
| Engineering ADRs (crate/design decisions) | `docs/design/adr/` (do not add a top-level `adrs/` folder) |
| Engineering specs / dependency matrix | `docs/design/` |
| Extension settings and commands | `extension/README.md` |

**Mirror policy:** Root `VISION.md`, `ARCHITECTURE.md`, `ROADMAP.md`, `SECURITY.md`, `CONTRIBUTING.md`, and `CHANGELOG.md` are mirrored under `docs/` for Read the Docs. When you change platform-facing content, update **both** copies (or expect `./scripts/check-doc-versions.sh` to fail).

### Adding dependencies

Before adding a Rust crate dependency:

1. Check [DEPENDENCY_MATRIX.md](design/DEPENDENCY_MATRIX.md) — prefer listed crates over new alternatives.
2. Follow [ADR-0016](design/adr/0016-dependency-first-implementation.md) — `strixonomy-*` crates are facades, not reimplementations.
3. Update DEPENDENCY_MATRIX and [LICENSES.md](design/LICENSES.md) if the crate is new or uses a non-MIT/Apache license.
4. Pin in workspace `[workspace.dependencies]` in root `Cargo.toml`.

## Code of conduct

See [CODE_OF_CONDUCT.md](https://github.com/eddiethedean/strixonomy/blob/main/CODE_OF_CONDUCT.md).

## Security

Report vulnerabilities per [SECURITY.md](https://github.com/eddiethedean/strixonomy/blob/main/SECURITY.md) — please do not open public issues for security reports.
