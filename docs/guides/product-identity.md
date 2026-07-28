# Product identity

Strixonomy is one brand with two shipping surfaces. Use these labels in docs and support so install paths stay unambiguous.

| Name | What it is | How you get it |
|------|------------|----------------|
| **Strixonomy IDE** | VS Code / Cursor extension — explorer, inspector, Query Workbench, Manchester editor, diagnostics | Marketplace / Open VSX / `strixonomy-v*.vsix` |
| **Strixonomy engine** | Rust semantic workspace — `strixonomy` CLI, `strixonomy-lsp`, `strixonomy` / `strixonomy-*` crates | crates.io, Linux x64 release tarball, or `cargo install strixonomy-cli` |
| **Ontologos** | External OWL reasoner used by the engine (EL/RL/RDFS/DL) | Bundled with Strixonomy — not a separate install for normal use |

## Legacy names (pre–v0.27)

| Legacy | Now |
|--------|-----|
| OntoIndex / `ontoindex` | Strixonomy engine (renamed earlier; see [v0.9 migration](../migration/v0.9.md)) |
| OntoCore | Strixonomy engine |
| OntoCode | Strixonomy IDE |
| `ontocore-cli` / `ontocore` | `strixonomy-cli` / `strixonomy` ([v0.27 migration](../migration/v0.27.md)) |
| `ontocode-v*.vsix` | `strixonomy-v*.vsix` |

Deprecated binary aliases (`ontocore`, `ontocore-lsp`) may still exist for a compatibility window — prefer the Strixonomy names.

## Install gotcha

The GitHub repository and façade crate are named `strixonomy`. The **CLI package** on crates.io is **`strixonomy-cli`** (binary `strixonomy`). Do **not** run `cargo install strixonomy`.

## See also

- [Which artifact?](which-artifact.md)
- [Install](../install.md)
- [Glossary](../glossary.md)
- [v0.27 migration](../migration/v0.27.md)
