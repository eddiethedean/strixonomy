# Internals (design targets)

> **These pages describe target architecture and contributor specs — not necessarily what ships in the current release.**
>
> For product capabilities, use **[What ships today](SHIPPED.md)**. For a 10-minute tutorial, use **[First success](guides/first-success.md)**.

Design documents, ADRs, and backlogs live under [design/](design/README.md). **Platform implementation architecture:** [platform/OVERVIEW.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/OVERVIEW.md). Product UX specs: [ui/README.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/README.md) — mapped to releases in [ROADMAP_MAPPING.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/ROADMAP_MAPPING.md). **Product ADRs:** [adr/README.md](adr/README.md). **Cursor prompts:** [cursor-prompts/README.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/cursor-prompts/README.md). Always cross-check [SHIPPED.md](SHIPPED.md). See also [Roadmap hub](roadmap-hub.md) and [Glossary](glossary.md).

## Contributor paths by role

Start with **[Architecture tour](guides/architecture-tour.md)** (~15 min) for a single map of crates, LSP, extension, and webviews.

### Rust-only (Strixonomy crates)

1. [Contributing guide](contributing.md) — build, test, MSRV
2. [Testing matrix](guides/testing-matrix.md) — commands by change type
3. [Strixonomy crate map](strixonomy/crate-map.md) — façade vs `strixonomy-*` layout
3. [Design architecture](design/ARCHITECTURE.md) — crate responsibilities
4. Run `./scripts/run-ci-local.sh` before opening a PR

### Extension-only (VS Code + OntoUI)

1. [Extension development](guides/extension-development.md) — `extension/` layout, F5, tests
2. [Debugging guide](debugging.md) — LSP output, webview devtools
3. [Webview protocol](webview-protocol.md) — host ↔ React messages
4. `cargo build -p strixonomy-lsp --bins` then `cd extension && npm run compile && npm test`

### Docs-only

1. [Contributing guide](contributing.md) — mirror rules for `VISION.md` / `ARCHITECTURE.md` / `ROADMAP.md`
2. `./scripts/check-doc-versions.sh` and `./scripts/build-docs.sh`
3. [Documentation index](documentation-index.md) — find the right page to edit

### LSP / custom editor integrators

1. [LSP hello world](guides/lsp-hello-world.md) — minimal stdio client
2. [LSP API reference](lsp-api.md) — shipped methods through the current tagged release (**v0.28**)
3. Do **not** use [design/LSP_SPEC.md](design/LSP_SPEC.md) for current behavior (future target)

## Start here for contributors

- [Documentation index](documentation-index.md)
- [Glossary](glossary.md)
- [Platform architecture](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/OVERVIEW.md)
- [Contributing guide](contributing.md)
- [Extension development](guides/extension-development.md)
- [Debugging guide](debugging.md)
- [Cursor implementation prompts](https://github.com/eddiethedean/strixonomy/blob/main/docs/cursor-prompts/README.md)
- [Design overview](design/README.md)
- [Product design / UI specs](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/README.md)
- [UI roadmap mapping](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/ROADMAP_MAPPING.md)
- [Strixonomy implementation architecture](design/ARCHITECTURE.md)
- [Product ADRs](adr/README.md)
- [Engineering ADRs](design/adr/README.md)
- [Releasing](releasing.md)

## Plugin model

Plugin **SDK 1.0** freezes the TOML + subprocess JSON wire (`api_version = "1"`) — safe to author against today. A curated marketplace and production owlmake integration remain **product 1.0** goals. See [plugin model](strixonomy/plugin-model.md), [Plugin policy](guides/plugin-policy.md), and **[Plugin authoring guide](guides/plugins.md)** (canonical).
