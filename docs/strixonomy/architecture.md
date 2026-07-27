# Strixonomy architecture

> **Which architecture doc?**
>
> | Read this | When |
> |-----------|------|
> | [Platform architecture](../architecture.md) | **Canonical** ecosystem overview for evaluators |
> | [Implementation architecture](../design/ARCHITECTURE.md) | Crate layout and internal modules for contributors |
> | **This page** | Short Strixonomy stack summary only |
>
> **Status:** Strixonomy is the platform identity for the Rust engine shipped since v0.2. See [ADR-0018](../design/adr/0018-ontocore-platform-identity.md).

## High-level diagram

```text
+---------------------------+
|        Strixonomy UI        |
| VS Code trees + commands  |
| React webviews            |
+-------------+-------------+
              |
              v
+---------------------------+
|   Strixonomy LSP            |
|   (strixonomy-lsp)         |
+-------------+-------------+
              |
              v
+---------------------------+
|       Strixonomy            |
| strixonomy façade + crates  |
| catalog/query/diagnostics |
| refactor/reasoner/robot   |
+-------------+-------------+
              |
      +-------+-------+-------+
      v       v               v
+-----------+ +-----------+ +------------------+
| Oxigraph  | | HornedOWL | | OntoLogos        |
| RDF/SPARQL| | OWL axioms| | reasoners        |
+-----+-----+ +-----+-----+ +--------+---------+
      |               |              |
      +───────┬───────┘              |
              v                      |
+---------------------------+        |
|     Workspace Files       |◄───────┘
| owl/rdf/ttl/jsonld/obo    |
+---------------------------+
```

## Responsibility split

| Owner | Responsibilities |
|-------|------------------|
| **Strixonomy** | Workspace discovery, indexing, RDF/OWL/OBO parsing, entity catalog, symbol graph, import graph, SQL/SPARQL, diagnostics, refactoring, reasoning integration, patch write-back, CLI, LSP, **plugin hosting** (build/validation/doc/workflow plugins), future Python/TypeScript bindings, future MCP server |
| **Strixonomy** | VS Code activity bar, explorer UI, React webviews, inspector, Query Workbench UI, Manchester editor UI, graph panels, extension commands, marketplace packaging, user onboarding, **toolchain workflow UI** (owlmake and plugin actions) |
| **OntoLogos** | OWL reasoning, classification, consistency, explanations, inference profiles |
| **External plugins (e.g. owlmake)** | ROBOT/ODK-style build, validation, release, and documentation workflows — integrate via Strixonomy plugin APIs; not core dependencies |

## Façade and implementation

| Layer | Crates |
|-------|--------|
| Public façade | `strixonomy` — re-exports and `Workspace` API |
| Core types | `strixonomy-core` |
| Parsing | `strixonomy-parser`, `strixonomy-owl` |
| Catalog | `strixonomy-catalog` |
| Query | `strixonomy-query` |
| Diagnostics | `strixonomy-diagnostics` |
| Reasoning | `strixonomy-reasoner` → OntoLogos |
| Refactoring | `strixonomy-refactor` |
| ROBOT | `strixonomy-robot` (external Java CLI) |
| CLI | `strixonomy-cli` |
| LSP | `strixonomy-lsp` |

See [crate map](crate-map.md) for details.

## Key design rules

- **Dual stack ([ADR-0013](../design/adr/0013-dual-stack-oxigraph-horned-owl.md)):** Catalog entities/axioms from Horned-OWL; triple counts and SPARQL from Oxigraph.
- **Reasoner adapters ([ADR-0008](../design/adr/0008-reasoner-adapters-not-built-in-reasoner.md), [ADR-0015](../design/adr/0015-adopt-ontologos-reasoner.md)):** Strixonomy delegates reasoning to OntoLogos; no JVM reasoners.
- **Local-first ([ADR-0005](../design/adr/0005-local-first-by-default.md)):** Indexing and queries run on the user's machine by default.
- **LSP boundary ([ADR-0007](../design/adr/0007-language-server-boundary.md)):** Strixonomy extension talks to Strixonomy only via LSP — no Rust logic in TypeScript.

Implementation architecture (crate tables, dependency rules): [design/ARCHITECTURE.md](../design/ARCHITECTURE.md).
