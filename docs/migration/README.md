# Migration guides

Upgrade notes between Strixonomy releases (historically OntoCore / OntoCode).

!!! warning "Upgrading to v0.28?"
    v0.28 **removes** OntoCore/OntoCode compat shims. Read [Migration v0.27 → v0.28](v0.28.md) before upgrading from v0.27 if you still use legacy names, paths, or LSP methods.

| From → To | Guide |
|-----------|-------|
| v0.5 → v0.6 | [Migration v0.5 → v0.6](v0.6.md) — reasoner, LSP methods, VS Code panels |
| v0.6 → v0.7 | [Migration v0.6 → v0.7](v0.7.md) — React inspector/graphs, OBO index, ROBOT CLI |
| v0.7 → v0.8 | [Migration v0.7 → v0.8](v0.8.md) — refactoring, React Query/Manchester panels, disjoint axioms |
| v0.8 → v0.9 | [Migration v0.8 → v0.9](v0.9.md) — Strixonomy identity, `strixonomy` façade crate (no API breaks) |
| v0.9 → v0.10 | [Migration v0.9 → v0.10](v0.10.md) — semantic workspace, incremental index, multi-root, semantic diff |
| v0.10 → v0.11 | [Migration v0.10 → v0.11](v0.11.md) — completion, code actions, docs export, imports UI, Open VSX |
| v0.11 → v0.12 | [Migration v0.11 → v0.12](v0.12.md) — authoring parity, OBO write-back, OWL/XML read, DL explanations |
| v0.12 → v0.13 | [Migration v0.12 → v0.13](v0.13.md) — OntoUI platform, schema browser, PR summary, diagnostics config |
| v0.13 → v0.14 | [Migration v0.13 → v0.14](v0.14.md) — plugin host MVP, CLI/LSP plugin hooks, capability registry |
| v0.14 → v0.15 | [Migration v0.14 → v0.15](v0.15.md) — plugin permissions, UI views, explanation alternatives, graph modes |
| v0.15 → v0.16 | [Migration v0.15 → v0.16](v0.16.md) — plugin preferences, context actions, imports and layout polish |
| v0.16 → v0.17 | [Migration v0.16 → v0.17](v0.17.md) — menus, dialogs, keyboard workflows, layouts, and perspectives |
| v0.17 → v0.18 | [Migration v0.17 → v0.18](v0.18.md) — Protégé Desktop parity gate, reasoner cancel, stale explanations, layout reopen |
| v0.18.0 → v0.18.1 | [Migration v0.18.0 → v0.18.1](v0.18.1.md) — named unsatisfiable expansion; stronger tests |
| v0.18.1 → v0.18.2 | [Migration v0.18.1 → v0.18.2](v0.18.2.md) — Windows paths; reasoner cancel; Manchester/Find Usages/Turtle/panel fixes |
| v0.18.2 → v0.19.0 | [Migration v0.18.2 → v0.19.0](v0.19.md) — semantic transaction apply path; parity manifest skeleton |
| v0.19.0 → v0.20.0 | [Migration v0.19 → v0.20](v0.20.md) — workspace runtime; Turtle patch matching hardening |
| v0.20.0 → v0.21.0 | [Migration v0.20 → v0.21](v0.21.md) — RDF/XML + OWL/XML write-back |
| v0.21.0 → v0.22.0 | [Migration v0.21 → v0.22](v0.22.md) — complete OWL 2 authoring |
| v0.22.0 → v0.23.0 | [Migration v0.22 → v0.23](v0.23.md) — reasoning parity + SWRL |
| v0.23.0 → v0.24.0 | [Migration v0.23 → v0.24](v0.24.md) — refactoring + DL Query |
| v0.24.0 → v0.25.0 | [Migration v0.24 → v0.25](v0.25.md) — viz + Plugin SDK 1.0 + a11y + parity CI |
| v0.25.0 → v0.26.0 | [Migration v0.25 → v0.26](v0.26.md) — Protégé Desktop JUnit behavioral test port |
| v0.26.0 → v0.26.1 | [Migration v0.26.0 → v0.26.1](v0.26.1.md) — OWL remove/match + annotation/range/XML write-back fixes |
| v0.26.1 → v0.26.2 | [Migration v0.26.1 → v0.26.2](v0.26.2.md) — host focus/transaction integrity + residual false positives |
| v0.26.2 → v0.27.0 | [Migration v0.26 → v0.27](v0.27.md) — **Strixonomy rename** (OntoCore/OntoCode → Strixonomy) |
| v0.27.0 → v0.28.0 | [Migration v0.27 → v0.28](v0.28.md) — **Remove OntoCore/OntoCode compat shims** |
| v0.28.0 → v0.28.1 | [Migration v0.28.0 → v0.28.1](v0.28.1.md) — Ontology Graph blank-canvas fix |

v0.29–v0.30: library APIs, LSP JSON, and SQL virtual table columns may change between minor releases. See [API stability](../guides/api-stability.md) and [workspace limits](../workspace-limits.md).

Full release history: [Changelog](../changelog.md)
