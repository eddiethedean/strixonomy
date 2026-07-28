# strixonomy-lsp

> Part of **Strixonomy** (semantic workspace engine).

Language server for Strixonomy and LSP integrators — stdio transport, custom `strixonomy/*` methods.

**v0.10:** multi-root workspace indexing, incremental reindex, semantic diff (`strixonomy/semanticDiff`), optional disk cache via `strixonomy/indexWorkspace`.

**v0.11:** Turtle `textDocument/completion` (prefix, QName, IRI); diagnostic quick fixes via `textDocument/codeAction`; import patch ops (`add_import`, `remove_import`).

## Install

```bash
cargo install strixonomy-lsp --locked
```

## Documentation

- [VS Code extension docs](https://strixonomy.readthedocs.io/en/latest/guides/vscode-extension/)
- [Rust & CLI docs](https://strixonomy.readthedocs.io/en/latest/guides/rust-crates/)
- [LSP API](https://strixonomy.readthedocs.io/en/latest/lsp-api/)
- [Install VS Code](https://strixonomy.readthedocs.io/en/latest/vscode-install/)
- [docs.rs](https://docs.rs/strixonomy-lsp)

**Current version: 0.27.0**

## License

MIT OR Apache-2.0
