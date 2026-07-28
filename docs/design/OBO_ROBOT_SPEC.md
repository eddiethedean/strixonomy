# OBO & ROBOT Interop Specification (v0.30)

> **Status: Target design for remaining interop.** Do **not** implement from this page as product truth.
>
> **Shipped today:** OBO index + write-back, ROBOT CLI wrappers, OBO workflows — see [What ships today](../SHIPPED.md), [OBO workflows](../guides/obo-workflow.md), and [ROBOT interop](../guides/robot-interop.md). Historical P0 framing: [PROTEGE_PARITY.md](PROTEGE_PARITY.md).
>
> Strixonomy does **not** reimplement ROBOT or ODK — it wraps the official ROBOT CLI today and integrates external workflow plugins (such as [owlmake](https://github.com/INCATools/owlmake)) for future Java-free pipelines.

## 1. Purpose

Enable biomedical ontology maintainers to use Strixonomy as a **primary IDE** alongside standard OBO/ROBOT/ODK release pipelines.

## 2. Strategy summary

| Layer | What it does | Status |
|-------|----------------|--------|
| **`strixonomy-robot`** | Thin wrapper around the official ROBOT Java CLI | **Shipped** (v0.7) |
| **Strixonomy diagnostics** | Built-in lint and parse checks | **Shipped** |
| **owlmake** (external) | Rust-native portable ROBOT/ODK-style workflows | **Reference plugin** (v0.33 integration target) |
| **Strixonomy core** | Does **not** reimplement ROBOT merge/template/report or ODK Makefile logic | By design |

Strixonomy should integrate with the ontology toolchain, not absorb it. See [PLUGIN_SPEC.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/PLUGIN_SPEC.md).

## 3. OBO format (P0)

### Parser / writer

- Read and write OBO Format 1.4 (`.obo`) via [`fastobo`](https://crates.io/crates/fastobo) + [`fastobo-owl`](https://crates.io/crates/fastobo-owl) ([DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md))
- Validate with [`fastobo-validator`](https://crates.io/crates/fastobo-validator) where applicable
- Map OBO ids to IRIs in catalog (`obo_id`, `iri`, `namespace`)
- Support `synonymtypedef`, `property_value`, `xref` in catalog annotations table

### UI

- Explorer shows OBO shorthand ids where applicable
- Manchester / completion resolves OBO ids in biomedical workspaces
- Syntax highlighting for `.obo` files in VS Code

### Milestone

**v0.7b** — OBO format support before v0.30.

## 4. ROBOT interop (P0) — current path

Thin CLI wrappers in `strixonomy-robot` crate (`strixonomy robot` subcommand):

```bash
strixonomy robot validate ./ontology
strixonomy robot merge --inputs a.owl b.owl --output merged.owl
strixonomy robot report ./ontology --report report.tsv
```

### Requirements

- Detect `robot` on `PATH`; clear error if missing with install link
- Pass through exit codes for CI
- Optional `strixonomy.robotPath` setting (workspace-trusted, like `lspPath`)

### Documentation

- Side-by-side: Strixonomy diagnostics vs ROBOT `validate` / `report`
- When to use Strixonomy-only CI vs ROBOT-only vs both

## 5. owlmake — future external integration path

[owlmake](https://github.com/INCATools/owlmake) provides **Rust-native, portable** ROBOT/ODK-style workflow execution. It is a **reference external workflow plugin**, not a core Strixonomy crate.

```text
Strixonomy IDE
     │
     ▼
Strixonomy (index, diagnostics, LSP)
     │
     ├── strixonomy-robot ──► ROBOT CLI (Java)     ← shipped today
     │
     └── owlmake plugin ──► Rust-native workflows ← v0.33 integration target
```

### Integration goals (v0.30)

| Goal | Description |
|------|-------------|
| Import existing ODK projects | Recognize `src/ontology/`, catalog files, import structure |
| Run project QC | Execute QC steps; surface results as Strixonomy diagnostics |
| Run release workflows | Trigger build/release pipelines from IDE or CLI |
| Inspect build outputs | Index and browse release artifacts in Strixonomy |
| Surface workflow errors | Map owlmake/ROBOT failures to Problems panel diagnostics |

### Clarifications

- **`strixonomy-robot`** wraps the current ROBOT CLI — this remains the supported path when Java + `robot` are available.
- **owlmake** may provide future **Java-free** workflow execution for teams that want portable Rust tooling.
- **Strixonomy must not** reimplement all of ROBOT/ODK internally; plugins and CLI wrappers delegate to established semantics.
- Strixonomy **surfaces** workflow actions; Strixonomy **hosts** plugin APIs; owlmake **implements** workflow automation.

See [PLUGIN_SPEC.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/PLUGIN_SPEC.md) for `WorkflowPlugin` / `BuildPlugin` interfaces.

## 6. OBO/ODK project workflow goals (v0.30)

| Workflow | Strixonomy / Strixonomy role |
|----------|--------------------------|
| Open ODK repo | Index standard layout; show imports and modules in explorer |
| Edit source `.ttl` / `.obo` | Turtle and OBO write-back (v0.12); RDF/XML read-only in VS Code |
| Run QC | `strixonomy validate` + ROBOT `report` + owlmake QC plugin |
| Run release | owlmake or ROBOT via plugin/CLI; inspect outputs in workspace |
| CI gates | `strixonomy validate`, `strixonomy classify`, `strixonomy robot validate` |

## 7. Example workspace

`examples/obo-workflow/` — minimal mixed OBO + OWL repo demonstrating:

- Edit in Strixonomy
- `strixonomy validate` + `strixonomy robot validate` in CI
- `strixonomy robot merge` for release

## 8. Non-goals

- Reimplementing ROBOT merge/template/report logic inside Strixonomy
- Replacing ODK Makefile or owlmake with built-in Strixonomy code
- OBO Foundry automated compliance as a core feature (P1 validator plugin)

## 9. Parity tracking

See [PROTEGE_PARITY.md](PROTEGE_PARITY.md) — OBO & biomedical section.
