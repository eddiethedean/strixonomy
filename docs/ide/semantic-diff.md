# Semantic diff

Compare ontology catalogs **semantically** — added/removed/changed classes, properties, axioms, and breaking-change heuristics — not just raw Turtle line diffs.

Available in **v0.10+** via:

| Surface | Entry point |
|---------|-------------|
| VS Code | **Strixonomy: Semantic Diff…** command → React **Semantic Diff** panel |
| CLI | `strixonomy diff` |
| LSP | `strixonomy/semanticDiff` (alias `strixonomy/getSemanticDiff`) |
| Rust | `strixonomy::diff` module, `Workspace::diff` |

## VS Code

1. Open a **trusted** workspace with ontology files.
2. Run **Strixonomy: Semantic Diff…** from the Command Palette.
3. Choose left/right refs (e.g. `HEAD` vs `WORKTREE`, or `main` vs current branch) when your workspace is version-controlled; otherwise compare directories on disk via the CLI.
4. Review added, removed, and changed entities in the panel. Breaking changes are highlighted when heuristics apply.

The panel uses the same React webview stack as the Query Workbench — see [Webview protocol](../webview-protocol.md).

!!! tip "No version control?"
    Compare two directories on disk with the CLI: `strixonomy diff --left-ref ./a --right-ref ./b`.

## CLI quick start

From a git clone (uses `fixtures`):

```bash
cargo run -- diff HEAD..WORKTREE
cargo run -- diff --format markdown --breaking-only main..feature
```

Installed CLI (your ontology repo):

```bash
strixonomy diff HEAD..WORKTREE
strixonomy diff --left-ref main --right-ref WORKTREE --format json
strixonomy diff --left-ref ./baseline --right-ref ./candidate
```

### Common ref pairs

| Left | Right | Meaning |
|------|-------|---------|
| `HEAD` | `WORKTREE` | Last commit vs working tree (CLI and LSP) |
| `main` | `feature` | Branch compare (git range syntax) |
| `INDEXED` or `CATALOG` | `WORKTREE` | Indexed catalog vs working tree (LSP / VS Code; legacy alias `WORKSPACE` for left) |

The CLI does **not** resolve `INDEXED` / `CATALOG` / `WORKSPACE` — use the LSP or extension for indexed-catalog compares. The CLI accepts git refs, directory paths, and `WORKTREE`.

Output formats: `text` (default), `json`, `markdown`, `pr-summary` (v0.13+). Use `--breaking-only` to filter to likely breaking changes.

### PR summary (v0.13+)

Emit Markdown suitable for pull request descriptions:

```bash
strixonomy diff main..feature --pr-summary
strixonomy diff --format pr-summary HEAD..WORKTREE
```

LSP: `strixonomy/semanticDiff` with `"format": "pr-summary"` returns `{ "formatted": "..." }` in addition to structured `diff`.

Optional `--reasoner` enriches the diff with unsatisfiability changes (requires resolvable workspace paths and reasoner inputs).

Full flags: [CLI reference](../cli-reference.md#diff) · [migration v0.10](../migration/v0.10.md).

## CI usage

Fail or annotate PRs when breaking ontology changes appear:

```yaml
- run: cargo install strixonomy-cli --locked --version 0.26.2
- run: strixonomy diff --pr-summary main..HEAD
```

See [CI integration](../ci-integration.md).

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| `no git repository found` | Run from a git checkout; use `--left-ref` / `--right-ref` directory compare instead |
| Empty diff unexpectedly | Confirm refs exist; re-index with `strixonomy validate` or **Index Workspace** |
| Panel shows no data | Run **Index Workspace**; check **Output → Strixonomy Language Server**; Trust only if using custom `lspPath` |
| Ref compare slow on large repos | Compare feature branch tip only; use `--breaking-only` in CI |

More: [Troubleshooting](../troubleshooting.md) · [FAQ](../faq.md).

## Related

- [Migration v0.10](../migration/v0.10.md)
- [LSP API — semanticDiff](../lsp-api.md)
- [What ships today](../SHIPPED.md)
