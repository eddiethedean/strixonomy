# API parity matrix (CLI · LSP · Rust · IDE)

Use this page when integrating Strixonomy outside VS Code or embedding the engine. **Capability truth:** [What ships today](../SHIPPED.md).

Legend: **Yes** = supported · **—** = not exposed on that surface · **Partial** = subset or different semantics.

## Core workspace

| Capability | IDE | CLI | LSP | Rust (`strixonomy`) |
|------------|-----|-----|-----|---------------------|
| Index workspace | Yes (auto + command) | `index`, `inspect`, `validate` | `strixonomy/indexWorkspace` | `Workspace::open` |
| Catalog SQL (subset) | Query Workbench | `query` | `strixonomy/query` | `Workspace::query` |
| SPARQL | Query Workbench | `sparql` | `strixonomy/sparql` | `Workspace::sparql` |
| DL Query (Manchester) | Query Workbench DL | `dl-query` | `strixonomy/dlQuery` | via reasoner + query paths |
| Workspace search | Command palette | — | `strixonomy/search` | — |
| Active ontology | Selector UI | — | `setActiveOntology` | `Workspace` session APIs |

## Editing and patches

| Capability | IDE | CLI | LSP | Rust |
|------------|-----|-----|-----|------|
| Entity Inspector write-back | Yes | — | via `applyAxiomPatch` | `strixonomy-owl` / `obo` |
| Patch JSON apply | Inspector, Manchester | `patch` | `strixonomy/applyAxiomPatch` | `owl` / `obo` modules |
| Manchester parse/apply | Manchester editor | — | `parseManchester`, patches | `strixonomy-owl` |
| Create/delete entities | Inspector | `patch` | `applyAxiomPatch` | patch APIs |
| Manage imports | Imports panel | `patch` (`add_import`) | patch ops | patch APIs |
| SWRL rules | Rule browser/editor | patch + — | `listSwrlRules`, `validateSwrlRule`, `parseSwrlRule` | `strixonomy-swrl` |

## Reasoning

| Capability | IDE | CLI | LSP | Rust |
|------------|-----|-----|-----|------|
| Classify (EL/RL/RDFS/DL/auto) | Reasoner panel | `classify` | `runReasoner` | `reasoner` module |
| Explanations | Explanation panel | `explain` | `getExplanation` | reasoner APIs |
| **Realize individuals** | Reasoner panel | **`realize`** | **—** (use CLI) | reasoner APIs |
| Instance check | Reasoner panel | `check-instance` | `checkInstance` | reasoner APIs |
| Cancel long reasoner run | Stop button | — | `$/cancelRequest` | engine cancel flag |

!!! note "CI vs IDE reasoner exit codes"
    `strixonomy classify` exits non-zero on unsatisfiable classes. LSP `runReasoner` returns `{ "consistent": false }` with exit 0. See [Errors reference](../errors.md).

## Refactor and diff

| Capability | IDE | CLI | LSP | Rust |
|------------|-----|-----|-----|------|
| Find usages | Yes | `refactor usages` | `findUsages` | `strixonomy-refactor` |
| Rename / merge / replace | Preview + apply | `refactor …` | `previewRefactor` / `applyRefactor` | refactor crate |
| Move / extract / ontology merge | Preview (Turtle-first) | `refactor …` | preview/apply | refactor crate |
| Semantic diff | Semantic Diff panel | `diff` | `semanticDiff` | `strixonomy-diff` |
| Git PR summary | — | `diff --pr-summary` | via diff payloads | diff crate |

## Tooling and plugins

| Capability | IDE | CLI | LSP | Rust |
|------------|-----|-----|-----|------|
| Diagnostics / lint | Problems panel | `validate` | publishDiagnostics | `diagnostics` module |
| ROBOT interop | — | `robot …` | `runRobot` | `strixonomy-robot` |
| Docs export (Markdown/HTML) | — | `docs` | — | `strixonomy-docs` |
| Plugins (SDK 1.0) | Commands, views, cards | `plugins …`, `workflow` | `listPlugins`, `runPlugin` | `strixonomy-plugin` |
| New ontology scaffold | New Ontology command | `new` | `createOntology` | workspace APIs |
| Graph visualization | Graph panels | — | `getGraph` | catalog graph payloads |

## Editor integration (IDE only)

| Capability | IDE | CLI | LSP | Rust |
|------------|-----|-----|-----|------|
| Hover / go-to-definition / rename | Yes | — | standard LSP | — |
| Semantic tokens (Turtle, OBO) | Editor | — | `semanticTokens/full` | — |
| Diagnostic quick fixes | Yes | — | `codeAction` | — |
| Turtle completion | Editor | — | `completion` | — |

## Format write-back

| Format | IDE inspector | CLI `patch` | LSP patch | Notes |
|--------|---------------|---------------|-----------|-------|
| `.ttl` | Yes | Yes | Yes | Richest Manchester/refactor |
| `.obo` | Yes | Yes | Yes | OBO metadata |
| `.owl` / `.rdf` / `.owx` | Yes | Yes | Yes | Semantic re-serialize |
| JSON-LD / TriG / N-Triples | Read-only | — | — | Index/query only |

Details: [Supported formats](../supported-formats.md) · [Capabilities by format](../guides/capabilities-by-format.md).

## Related

- [CLI reference](../cli-reference.md) · [LSP API](../lsp-api.md) · [Rust API](../strixonomy/rust-api.md)
- [Which artifact?](../guides/which-artifact.md) · [LSP hello world](../guides/lsp-hello-world.md)
