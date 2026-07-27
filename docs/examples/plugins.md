# Plugins cookbook (SDK 1.0)

CLI recipes for discovering and running workspace plugins. Authoring: [Plugin authoring](../guides/plugins.md) · Policy: [Plugin policy](../guides/plugin-policy.md).

From a git clone, use `cargo run --` or an installed `strixonomy`. Samples below use [`examples/plugin-workspace/`](https://github.com/eddiethedean/strixonomy/tree/v0.27.0/examples/plugin-workspace).

## List and inspect

```bash
strixonomy plugins list examples/plugin-workspace
strixonomy plugins list examples/plugin-workspace --format json

strixonomy plugins info org.example.demo-graph examples/plugin-workspace
strixonomy plugins info org.example.demo-graph examples/plugin-workspace --format json
```

`info` prints lifecycle fields (`state`, `activation`, `enabled`, `depends_on`).

## Enable / disable

```bash
strixonomy plugins disable org.example.demo-graph examples/plugin-workspace
strixonomy plugins enable org.example.demo-graph examples/plugin-workspace
```

`enable` / `disable` follow activation policy and cascade dependents.

## Run actions

```bash
strixonomy plugins run org.example.demo-reasoner --action reasoner.classify examples/plugin-workspace
strixonomy plugins run org.example.demo-query --action query.run --query "SELECT short_name FROM classes" examples/plugin-workspace
strixonomy plugins run org.example.demo-refactor --action refactor.preview --iri http://example.org/Person examples/plugin-workspace
strixonomy plugins run org.example.demo-graph --action graph.build --iri http://example.org/Person examples/plugin-workspace
```

Legacy actions also work: `validate`, `export`, `workflow` (with `--step` when needed). Full flag table: [CLI reference — plugins](../cli-reference.md#plugins).

## Clone shortcut

```bash
cargo run -- plugins list examples/plugin-workspace
cargo run -- validate examples/plugin-workspace
```

## Next

| Goal | Document |
|------|----------|
| Manifests and providers | [Plugin authoring](../guides/plugins.md) |
| Compatibility promise | [Plugin policy](../guides/plugin-policy.md) |
| IDE plugin UI | [Feature tour — Plugins](../ide/feature-tour.md) |
