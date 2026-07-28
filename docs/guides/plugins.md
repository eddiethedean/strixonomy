# Plugin authoring (SDK 1.0)

> **This is the only page to implement against today:** workspace TOML manifests + subprocess JSON contract.
>
> | Doc | Role |
> |-----|------|
> | **This guide** | Canonical — ship plugins from here |
> | [Plugin policy](plugin-policy.md) | SDK 1.0 compatibility / support stance |
> | [Plugin manager plan](../plugin-manager-plan.md) | Planned install, lockfile, registry, trust, and marketplace behavior — not shipped |
> | [plugin-model.md](../strixonomy/plugin-model.md) | Overview only — do not implement sketches |
> | [ui/PLUGIN_API_SPEC.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/PLUGIN_API_SPEC.md) | Future OntoUI host — **not** the shipped VS Code contract |
> | [ui/PLUGIN_PLATFORM.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/PLUGIN_PLATFORM.md) | Future Capability Providers — **not** the shipped host |
> | [design/PLUGIN_SPEC.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/PLUGIN_SPEC.md) | Historical — do not implement |

Strixonomy’s **Plugin SDK 1.0** supports permissions, versioned API (`api_version = "1"`), UI contributions, lifecycle (`depends_on`, `activation`), and provider kinds (reasoner / query / refactor / graph).

## Contribution matrix (SDK 1.0)

### UI

| Manifest field | VS Code | CLI / LSP | Status |
|----------------|---------|-----------|--------|
| `[[ui.commands]]` | Command palette (**Plugins: Run Command…**) | `strixonomy/runPlugin` | **Frozen** |
| `[[ui.views]]` | Dockable panel (**Plugins: Open View…**) | `action: "ui_view"` | **Frozen** |
| `[[ui.inspector_cards]]` | Entity Inspector cards | Via validate/index | **Frozen** |
| `[[ui.preferences_pages]]` | **Plugins: Open Preferences…** | Via `listPlugins` | **Frozen** |
| `[[ui.context_actions]]` | Entity / ontology context menus | Via `listPlugins` | **Frozen** |

### Runtime / providers

| Kind / capability | Protocol action | Status |
|-------------------|-----------------|--------|
| `validator` / `validate` | `validate` | **Frozen** |
| `exporter` / `export` | `export` | **Frozen** |
| `workflow` / `build` | `workflow` | **Frozen** |
| `reasoner` | `reasoner.classify` | **Frozen** |
| `query` | `query.run` | **Frozen** |
| `refactor` | `refactor.preview` | **Frozen** |
| `graph` | `graph.build` | **Frozen** |
| `ui` | `ui-view` | **Frozen** |
| `editor` / `language_service` / `tool_window` | — | **Reserved** (manifest rejected until hosted) |
| `ai` | — | **Reserved** (v0.31+; not hosted) |

## Lifecycle

```toml
[plugin]
depends_on = ["strixonomy.naming-validator"]
activation = "on_startup"   # on_startup | on_command | on_workspace_open
```

| State | Meaning |
|-------|---------|
| discovered → validated → registered | Parsed and dependency-checked |
| active | Eligible to run (startup / workspace_open, or after command run) |
| disabled | User-disabled (persisted in `.strixonomy/plugin-disabled.json`); dependents cascade-disable |

Activation order is a topological sort of `depends_on` (cycles and missing deps fail discovery).

```bash
strixonomy plugins info <id> /path/to/workspace
strixonomy plugins enable <id> /path/to/workspace
strixonomy plugins disable <id> /path/to/workspace
```

LSP `listPlugins` exposes `state`, `enabled`, `depends_on`, and `activation`.

## Permissions

Declare in `[plugin]`:

```toml
permissions = ["workspace.read", "workspace.write", "external_process"]
```

| Permission | Required for |
|------------|--------------|
| `workspace.read` | Validate, export, providers, list plugins |
| `workspace.write` | Workspace writes; **required for subprocess `validate` and `ui_view`** |
| `external_process` | Subprocess `entry` binaries |
| `network` / `filesystem.*` | Declared for trust UI; deny-by-default escapes are still jailed |

Manifests that omit `permissions` receive backward-compatible defaults (`workspace.read`, and `external_process` when `entry` is set). Those defaults are **not** sufficient for subprocess `validate` or `ui_view` — declare `workspace.write` as well.

## Quick start

1. Create `.strixonomy/plugins/*.toml` in your ontology workspace.
2. Run `strixonomy plugins list` or `strixonomy validate` to execute validator plugins.
3. Install the Strixonomy extension and index the workspace — plugin diagnostics appear in the Problems panel.

See [examples/plugin-workspace](https://github.com/eddiethedean/strixonomy/tree/main/examples/plugin-workspace) for reference providers.

## Manifest schema

```toml
[plugin]
name = "my-validator"
version = "0.1.0"
kind = "validator"          # see contribution matrix
id = "org.example.my-validator"
api_version = "1"
entry = "my-plugin-cli"     # optional subprocess binary
depends_on = []
activation = "on_startup"
permissions = ["workspace.read", "workspace.write", "external_process"]

[capabilities]
validate = true
diagnostics = true
export = false
reasoner = false
query = false
refactor = false
graph = false

[config]
require_label = true
shapes_dir = "shapes"
provider_id = "optional-profile"   # reasoner / graph tip
graph_kind = "optional-kind"

[[ui.commands]]
id = "org.example.check"
title = "Run my validator"

[[ui.views]]
id = "org.example.view"
title = "My dockable view"

[[ui.preferences_pages]]
id = "org.example.prefs"
title = "My plugin"
category = "Validation"

[[ui.context_actions]]
id = "org.example.ctx"
title = "Validate this class"
scope = "entity"
applies_to = ["class"]
command = "org.example.check"

[[ui.inspector_cards]]
id = "my-card"
title = "My validator"
applies_to = ["class"]
```

## Subprocess contract

Plugins with an `entry` binary are invoked as:

```text
<entry> <action> --workspace <path> [--step <name>] [--view <id>] [--query <text>] [--iri <iri>] [--root <iri>]
```

Supported actions: `validate`, `export`, `workflow`, `ui-view`, `reasoner.classify`, `query.run`, `refactor.preview`, `graph.build`.

Stdout must be a single JSON object. Field tables by action:

| Action | Required / typical stdout fields |
|--------|----------------------------------|
| `validate` | `diagnostics` (array of `{ severity, message, file?, line?, column? }`) |
| `export` | `output_paths` (workspace-relative paths written) |
| `workflow` | `logs` optional; exit via process status |
| `ui-view` | `view_html` (HTML fragment for the dockable panel) |
| `reasoner.classify` | `unsatisfiable` (IRI array), `profile` optional |
| `query.run` | `columns` (string array), `rows` (array of arrays) |
| `refactor.preview` | `affected_iris`, `hints` |
| `graph.build` | `graph_kind`, `root_iris` |

Common optional fields on any response: `logs`, `result` (opaque object).

Example (`query.run`):

```json
{
  "columns": ["iri", "label"],
  "rows": [["http://ex/A", "A"]],
  "logs": "optional"
}
```

There is no separate JSON Schema file yet — treat this table as the wire contract for `api_version = "1"`.

## Adding an in-process (host-builtin) Rust plugin

Subprocess plugins are the **supported third-party path**. In-process plugins ship inside the Strixonomy workspace (e.g. naming / markdown-export / SHACL):

1. Add a crate under `crates/strixonomy-plugin-*` implementing the host plugin traits used by `strixonomy-plugin` / `strixonomy-plugin-builtins`.
2. Register it in the builtins aggregator (`strixonomy-plugin-builtins`) so CLI/LSP discovery lists it.
3. Add a workspace fixture under `examples/plugin-workspace/` and a row in [testing matrix](testing-matrix.md) / `cargo test -p <crate>`.
4. Document the plugin id in the reference table below.

Do **not** copy sketches from [design/PLUGIN_SPEC.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/PLUGIN_SPEC.md) — that file is historical.

Reference in-process plugins:

- `strixonomy-plugin-naming`
- `strixonomy-plugin-markdown-export`
- `strixonomy-plugin-shacl`

## Reference plugins

| Id | Kind | Purpose |
|----|------|---------|
| `strixonomy.naming-validator` | validator | Require `rdfs:label` on entities |
| `strixonomy.markdown-export` | exporter | Markdown docs via `strixonomy-docs` |
| `strixonomy.shacl-validator` | validator | SHACL shapes directory check (rudof adapter planned) |
| `org.example.demo-reasoner` | reasoner | Stub classify overlay (fixture workspace) |
| `org.example.demo-query` | query | Fixed tabular rows |
| `org.example.demo-refactor` | refactor | Preview-only rename tip |
| `org.example.demo-graph` | graph | Custom `graph_kind` seed IRIs |
| `owlmake` (external) | workflow | Current manifest scaffold invokes an `owlmake` executable; a maintained [EBISPOT/owlmake](https://github.com/EBISPOT/owlmake) adapter is planned for v0.33 |

Planned maintained integrations (not shipped):

| Project | Target | Role and decision gate |
|---------|--------|------------------------|
| [DataFusion](https://github.com/apache/datafusion) | v0.31 experiment | Advanced analytical query provider; move into `strixonomy-query` if the provider boundary harms performance or coherent query planning |
| [Tantivy](https://github.com/quickwit-oss/tantivy) | v0.31 experiment | Ranked multilingual entity-search provider; move into the engine if index lifecycle and latency require it |
| [Regorus](https://github.com/microsoft/regorus) | v0.32 | Rego policy provider over semantic diff, catalog, and diagnostic facts |
| [rudof](https://github.com/rudof-project/rudof) | v0.33 | SHACL, ShEx, and DCTAP validation/conversion adapter |
| [Typst](https://github.com/typst/typst) | v0.33 | Publication-quality evidence and ontology reports |
| [mdBook](https://github.com/rust-lang/mdBook) | v0.33 | Searchable documentation-site export |
| [EBISPOT/owlmake](https://github.com/EBISPOT/owlmake) | v0.33 | Maintained ODK/ROBOT-style workflow adapter |
| [Wasmtime](https://github.com/bytecodealliance/wasmtime) | v0.34 infrastructure | WASI sandbox runtime pilot; infrastructure, not an end-user plugin |

## CLI

```bash
strixonomy plugins list /path/to/workspace
strixonomy plugins info strixonomy.naming-validator /path/to/workspace
strixonomy plugins enable org.example.demo-graph /path/to/workspace
strixonomy plugins disable org.example.demo-graph /path/to/workspace
strixonomy plugins run strixonomy.naming-validator --action validate /path/to/workspace
strixonomy plugins run org.example.demo-query --action query.run --query "SELECT *" /path/to/workspace
strixonomy plugins run org.example.demo-graph --action graph.build --iri http://example.org/Person /path/to/workspace
strixonomy validate /path/to/workspace
strixonomy docs /path/to/workspace -o out --plugin strixonomy.markdown-export
strixonomy workflow --plugin owlmake --step qc /path/to/workspace
```

The `owlmake` command is currently a generic subprocess scaffold, not a
bundled EBISPOT/owlmake adapter. The v0.33 adapter must translate Strixonomy
workflow requests into owlmake commands and return structured diagnostics,
logs, and artifact paths through Plugin SDK 1.0.

## LSP / Strixonomy

- `strixonomy/listPlugins` — plugins + UI metadata + lifecycle fields
- `strixonomy/runPlugin` — validate/export/workflow/ui_view/provider actions
- Strixonomy command **Plugins: Run Command…**
- Strixonomy command **Plugins: Open View…**
- Strixonomy command **Plugins: Open Preferences…**
- Strixonomy command **Run Workflow (owlmake)**
- Context menus — plugin `context_actions` on entities/ontologies

## Debugging failures

| Symptom | Check |
|---------|-------|
| Plugin not listed | Manifest path `.strixonomy/plugins/*.toml`; `api_version = "1"`; `strixonomy plugins list` |
| Discovery fails | Cycle or missing `depends_on` target |
| `INDEX_FAILED` on `runPlugin` | Subprocess `entry` on path; permissions; valid JSON stdout |
| No diagnostics in Problems | Empty `diagnostics` or validate not triggered |
| Permission denied | Declared `permissions` too narrow |
| Plugin disabled | `strixonomy plugins info <id>` / `.strixonomy/plugin-disabled.json` |

## Security: `external_process`

Plugins with `entry` and `external_process` run **arbitrary binaries** chosen by the workspace manifest. Treat trust like CI scripts:

- Prefer allowlisting plugin directories in enterprise deployments.
- Review `entry` paths before opening untrusted ontology repos.
- Path jailing rejects writes and binary paths outside the workspace root.

Threat-model overview: [Security](../security.md).

## Stability

**SDK 1.0 is frozen.** Additive changes only within `api_version = "1"`; see [Plugin policy](plugin-policy.md).

**Canonical author guide: this page only.** Historical trait sketches live on GitHub as non-product background ([PLUGIN_SPEC.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/PLUGIN_SPEC.md)) and are **excluded from public docs search**. The React/TypeScript [Plugin API spec](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/PLUGIN_API_SPEC.md) is a **future** OntoUI target — not the shipped VS Code/CLI contract.
