# Strixonomy

[![VS Code Marketplace](https://badgen.net/vs-marketplace/v/strixonomy.strixonomy?label=VS%20Code%20Marketplace)](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy)
[![Open VSX](https://img.shields.io/open-vsx/v/strixonomy/strixonomy)](https://open-vsx.org/extension/strixonomy/strixonomy)

**Ontology IDE for VS Code** — browse and edit Turtle/OBO/RDF/XML/OWL/XML, query, reason, validate, and diff. **Not a full Protégé replacement** — see [Known limitations](https://strixonomy.readthedocs.io/en/latest/known-limitations/) and [What ships today](https://strixonomy.readthedocs.io/en/latest/SHIPPED/).

**Current release: v0.28.0**

## Start here

1. Install from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy) or [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy) (Cursor).
2. Follow **[First success (~10 min)](https://strixonomy.readthedocs.io/en/latest/guides/first-success/)** on Read the Docs (sample files + edit `Person`).

You do **not** need to Trust the workspace for the default bundled language server.

## Docs

| Topic | Link |
|-------|------|
| What ships today | [SHIPPED](https://strixonomy.readthedocs.io/en/latest/SHIPPED/) |
| Known limitations | [Known limitations](https://strixonomy.readthedocs.io/en/latest/known-limitations/) |
| Install options (VSIX, offline) | [Install VS Code](https://strixonomy.readthedocs.io/en/latest/vscode-install/) |
| CLI / CI / Rust crates (optional) | [Install CLI & CI (detail)](https://strixonomy.readthedocs.io/en/latest/install-cli-ci/) |
| Full documentation | [Read the Docs](https://strixonomy.readthedocs.io/en/latest/) |
| Extension overview | [VS Code extension docs](https://strixonomy.readthedocs.io/en/latest/ide/vscode-extension/) |

> **Editable today:** Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), and OWL/XML (`.owx`). XML is semantic re-serialize (not Protégé byte-identical). JSON-LD / TriG / N-Triples remain read-only.

> **Names:** **Strixonomy IDE** = this extension. **Strixonomy engine** = Rust CLI + `strixonomy-lsp`. Install the CLI with `cargo install strixonomy-cli`, not `strixonomy`. Most IDE users never need the CLI.

## Features (summary)

Explorer, Entity Inspector, Query Workbench (SQL subset + SPARQL + DL), Manchester editor, graphs, reasoner (EL–DL profiles, realize / instance check), SWRL Rule Browser/Editor, semantic diff, Manage Imports, refactoring preview, Plugin SDK 1.0 (frozen wire — manager v0.33, official registry v0.34, marketplace v0.35+).

Details: [Feature tour](https://strixonomy.readthedocs.io/en/latest/strixonomy/feature-tour/) · [Supported formats](https://strixonomy.readthedocs.io/en/latest/supported-formats/) · [Protégé vs Strixonomy](https://strixonomy.readthedocs.io/en/latest/guides/protege-decision/)

## Development

See [Contributing](https://github.com/eddiethedean/strixonomy/blob/main/CONTRIBUTING.md) and [Extension development](https://strixonomy.readthedocs.io/en/latest/guides/extension-development/).

License: MIT (extension). Strixonomy engine: MIT OR Apache-2.0.
