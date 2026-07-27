# Index vs inspect cookbook

Both commands index the workspace and print catalog statistics. Choose based on your goal.

| Command | Output | Use when |
|---------|--------|----------|
| `strixonomy index` | Stats only | CI scripts, machine-readable JSON |
| `strixonomy inspect` | Stats + diagnostic summary (counts + up to 10 samples) | Quick human health check |

## Index (stats only)

```bash
strixonomy index /path/to/ontologies
strixonomy index /path/to/ontologies --format json
```

## Inspect (stats + diagnostics)

```bash
strixonomy inspect /path/to/ontologies
strixonomy inspect /path/to/ontologies --format json
```

For full diagnostic listing and CI gating, use `strixonomy validate`:

```bash
strixonomy validate /path/to/ontologies
```

## From a git clone

```bash
cargo run -- inspect fixtures
cargo run -- index fixtures --format json
```

## Related

- [CLI reference](../cli-reference.md)
- [Production evidence protocol](../guides/production-evidence.md) — uses `inspect --format json` for sizing
