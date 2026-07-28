# Plugin workspace example (SDK 1.0)

Minimal ontology folder with `.strixonomy/plugins/` manifests for Strixonomy Plugin SDK 1.0 — UI contributions, lifecycle deps, and reference providers.

Full authoring guide: [Plugin authoring](https://strixonomy.readthedocs.io/en/latest/guides/plugins/).

## Layout

```text
plugin-workspace/
  demo.ttl
  demo_ui_view.sh
  demo_providers.sh
  .strixonomy/plugins/
    naming-validator.toml
    demo-ui-view.toml
    demo-reasoner.toml
    demo-query.toml
    demo-refactor.toml
    demo-graph.toml      # depends_on demo-reasoner
    owlmake.toml
```

## Try it (git clone)

```bash
# From repo root
cargo run -- plugins list examples/plugin-workspace
cargo run -- plugins info org.example.demo-graph examples/plugin-workspace
cargo run -- plugins run org.example.demo-reasoner --action reasoner.classify examples/plugin-workspace
cargo run -- plugins run org.example.demo-query --action query.run --query "SELECT *" examples/plugin-workspace
cargo run -- plugins run org.example.demo-refactor --action refactor.preview --iri http://example.org/Person examples/plugin-workspace
cargo run -- plugins run org.example.demo-graph --action graph.build --iri http://example.org/Person examples/plugin-workspace
cargo run -- validate examples/plugin-workspace
```

## VS Code

1. **File → Open Folder…** → `examples/plugin-workspace`
2. Run **Strixonomy: Index Workspace**
3. Use **Plugins: Open View…** / **Plugins: Run Command…** for UI contributions

## Notes

- Workspace-local plugins (not a marketplace).
- Subprocess `entry` binaries live next to the workspace root; symlinks under `.strixonomy/plugins/` resolve them without `..` in the manifest.
- `depends_on` / disable cascade: `strixonomy plugins disable org.example.demo-reasoner` also disables `demo-graph`.
