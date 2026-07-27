# Legacy guide URLs

Older documentation links and bookmarks may point at paths under `guides/` that now live under `ide/`. This page lists redirects — use the **canonical** URLs in new links.

## Redirect map

| Legacy URL | Canonical page |
|------------|----------------|
| [guides/vscode-extension.md](vscode-extension.md) | [ide/vscode-extension.md](../ide/vscode-extension.md) |
| [guides/manchester-editor.md](manchester-editor.md) | [ide/manchester-editor.md](../ide/manchester-editor.md) |
| [guides/query-workbench.md](query-workbench.md) | [ide/query-workbench.md](../ide/query-workbench.md) |
| [guides/graph-visualization.md](graph-visualization.md) | [ide/graph-view.md](../ide/graph-view.md) |

## Why these pages exist

MkDocs and Read the Docs keep legacy paths so external links (blog posts, extension `package.json` homepage history, GitHub issues) do not 404. Each legacy file is a **short redirect stub** pointing at the canonical topic.

## For contributors

When adding documentation:

- Put **user-facing Strixonomy topics** under `docs/ide/`.
- Put **cross-cutting guides** (Protégé, enterprise, reasoner) under `docs/guides/`.
- Do not duplicate full content in legacy stubs — one redirect paragraph is enough.
