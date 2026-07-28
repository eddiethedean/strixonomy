# Strixonomy

> **Development version: v0.27.0 (unreleased).** The published extension remains `ontocode.ontocode` v0.26.2 until the Strixonomy Marketplace/Open VSX listing is live.

[![VS Code Marketplace](https://vsmarketplacebadges.dev/version/ontocode.ontocode.svg?label=VS%20Code%20Marketplace)](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode)
[![Open VSX](https://img.shields.io/open-vsx/v/ontocode/ontocode)](https://open-vsx.org/extension/ontocode/ontocode)

**Ontology IDE for VS Code** — browse and edit Turtle/OBO/RDF/XML/OWL/XML, query, reason, validate, and diff. **Not a full Protégé replacement** — see [Known limitations](https://strixonomy-vs.readthedocs.io/en/latest/known-limitations/) and [What ships today](https://strixonomy-vs.readthedocs.io/en/latest/SHIPPED/).

**Current release: v0.26.2**

## Start here

1. Install from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=ontocode.ontocode) or [Open VSX](https://open-vsx.org/extension/ontocode/ontocode) (Cursor).
2. Follow **[First success (~10 min)](https://strixonomy-vs.readthedocs.io/en/latest/guides/first-success/)** on Read the Docs (sample files + edit `Person`).

You do **not** need to Trust the workspace for the default bundled language server.

## Docs

| Topic | Link |
|-------|------|
| What ships today | [SHIPPED](https://strixonomy-vs.readthedocs.io/en/latest/SHIPPED/) |
| Known limitations | [Known limitations](https://strixonomy-vs.readthedocs.io/en/latest/known-limitations/) |
| Install options (VSIX, offline) | [Install VS Code](https://strixonomy-vs.readthedocs.io/en/latest/vscode-install/) |
| CLI / CI / Rust crates (optional) | [Install CLI & CI (detail)](https://strixonomy-vs.readthedocs.io/en/latest/install-cli-ci/) |
| Full documentation | [Read the Docs](https://strixonomy-vs.readthedocs.io/en/latest/) |
| Extension overview | [VS Code extension docs](https://strixonomy-vs.readthedocs.io/en/latest/ide/vscode-extension/) |

> **Editable today:** Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), and OWL/XML (`.owx`). XML is semantic re-serialize (not Protégé byte-identical). JSON-LD / TriG / N-Triples remain read-only.

> **Names:** **Strixonomy** = this extension. **Strixonomy** = Rust engine (`ontocore-cli`, `strixonomy-lsp`). Install the CLI with `cargo install ontocore-cli`, not `strixonomy`. Most IDE users never need the CLI.

## Features (summary)

Explorer, Entity Inspector, Query Workbench (SQL subset + SPARQL + DL), Manchester editor, graphs, reasoner (EL–DL profiles, realize / instance check), SWRL Rule Browser/Editor, semantic diff, Manage Imports, refactoring preview, Plugin SDK 1.0 (frozen wire — marketplace still product 1.0).

Details: [Feature tour](https://strixonomy-vs.readthedocs.io/en/latest/strixonomy/feature-tour/) · [Supported formats](https://strixonomy-vs.readthedocs.io/en/latest/supported-formats/) · [Protégé vs Strixonomy](https://strixonomy-vs.readthedocs.io/en/latest/guides/protege-decision/)

## Development

See [Contributing](https://github.com/eddiethedean/strixonomy/blob/main/CONTRIBUTING.md) and [Extension development](https://strixonomy-vs.readthedocs.io/en/latest/guides/extension-development/).

License: MIT (extension). Strixonomy engine: MIT OR Apache-2.0.
