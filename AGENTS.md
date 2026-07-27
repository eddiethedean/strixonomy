# Agent / AI contributor guide

Short rules for automated agents and humans using AI coding tools in this repo.

## Capability truth (read first)

1. **[docs/SHIPPED.md](docs/SHIPPED.md)** — what ships in the latest tagged release
2. **[docs/known-limitations.md](docs/known-limitations.md)** — honest gaps
3. **[docs/TAGGED_RELEASE](docs/TAGGED_RELEASE)** — public install pin (must match published GitHub Release)

Do **not** treat `docs/design/*` target specs, `docs/protege-parity/`, or `docs/PROTEGE_REVERSE_ENGINEERING/` as product claims or as the implementation checklist.

## Do not implement from design targets

| Location | Treat as |
|----------|----------|
| `docs/design/OWL_AUTHORING_SPEC.md`, `OBO_ROBOT_SPEC.md`, `SHACL_SPEC.md`, React integration plan, `PLUGIN_SPEC.md` | Historical / target — banners say so; prefer SHIPPED + user guides |
| `docs/design/LSP_SPEC.md` | Mix of shipped + planned — cross-check [docs/lsp-api.md](docs/lsp-api.md) |
| Root `VISION.md` / `ROADMAP.md` | Prefer `docs/vision.md` / `docs/roadmap.md` then sync mirrors |

## Contributor entry points

- [CONTRIBUTING.md](CONTRIBUTING.md) · [docs/contributing.md](docs/contributing.md) (edit docs copy first)
- Testing scopes: [docs/guides/testing-matrix.md](docs/guides/testing-matrix.md)
- Internals map: [docs/internals.md](docs/internals.md)
- Plugins: [docs/guides/plugins.md](docs/guides/plugins.md) (canonical; not PLUGIN_SPEC)
- Local full CI: `./scripts/run-ci-local.sh`
- Doc pin hygiene: `./scripts/check-doc-versions.sh`

## Release / version pins

- Never bump `docs/TAGGED_RELEASE` or First success `raw.githubusercontent.com/.../vX.Y.Z/` URLs until the matching **git tag** is about to be pushed (same day).
- After tagging, verify sample URLs return HTTP 200 and crates.io / GitHub Release artifacts exist.
- See [docs/releasing.md](docs/releasing.md).

## Preferred docs for product behavior

| Need | Doc |
|------|-----|
| Formats / write-back | [docs/supported-formats.md](docs/supported-formats.md) |
| First-time user | [docs/guides/first-success.md](docs/guides/first-success.md) |
| Day-2 IDE path | [docs/guides/day-2.md](docs/guides/day-2.md) |
| LSP wire API | [docs/lsp-api.md](docs/lsp-api.md) + `docs/lsp-protocol.schema.json` |
| Patches | [docs/patch-reference.md](docs/patch-reference.md) |
| CLI | [docs/cli-reference.md](docs/cli-reference.md) |
| Catalog SQL | [docs/sql-reference.md](docs/sql-reference.md) |
| SPARQL | [docs/sparql-reference.md](docs/sparql-reference.md) |
| Errors / exits | [docs/errors.md](docs/errors.md) |
| Plugins | [docs/guides/plugins.md](docs/guides/plugins.md) |
| Rust API | [docs/strixonomy/rust-api.md](docs/strixonomy/rust-api.md) |
