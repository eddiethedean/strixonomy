# PLUGIN_SPEC.md

> **Historical design spec — not the shipping contract.**  
> **Authoring plugins today:** [Plugin authoring](https://strixonomy.readthedocs.io/en/latest/guides/plugins/) (TOML manifest + subprocess host).  
> This file is **excluded from the public Read the Docs site** so adopters do not discover the wrong API via search. Trait-based sketches below are background only; the shipped host uses `.strixonomy/plugins/*.toml`.

## 1. Purpose

The plugin system allows users and organizations to extend Strixonomy and Strixonomy **without modifying the core project**. Plugins expose **Capability Providers** — see [platform/CAPABILITY_PROVIDERS.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/CAPABILITY_PROVIDERS.md) and [adr/0005](../adr/0005-capability-provider-plugin-model.md).

**Strixonomy hosts plugins; plugins are not part of Strixonomy.** Build, release, workflow orchestration, and toolchain-specific validation live in external plugins that integrate through stable Strixonomy APIs. Strixonomy provides the semantic workspace (index, query, diagnostics, refactoring, LSP); plugins add domain-specific automation on top.

[owlmake](https://github.com/INCATools/owlmake) is the **reference external workflow plugin** — it demonstrates how ROBOT/ODK-style build, QC, and release pipelines integrate with Strixonomy without becoming a core dependency.

## 2. Architecture

```text
┌─────────────────────────────────────────────────────────────┐
│  Strixonomy (VS Code) — surfaces core + plugin workflows      │
└────────────────────────────┬────────────────────────────────┘
                             │ strixonomy-lsp
┌────────────────────────────▼────────────────────────────────┐
│  Strixonomy — workspace engine + plugin host                  │
│  index · query · diagnostics · refactor · LSP               │
└──────────────┬──────────────────────────────┬───────────────┘
               │                              │
               ▼                              ▼
        ┌─────────────┐              ┌───────────────────────┐
        │  Ontologos  │              │  External plugins     │
        │  reasoning  │              │  owlmake (reference)  │
        └─────────────┘              │  ROBOT/ODK adapters   │
                                     │  validators, exporters │
                                     └───────────────────────┘
```

See also [Platform architecture](../architecture.md) and [OBO & ROBOT interop](OBO_ROBOT_SPEC.md).

## 3. Plugin categories

### 3.1 Build plugins

Compile, merge, and materialize ontologies. Invoke external or native build tools; write artifacts to workspace output directories.

**Example capabilities:**

- build ontology from source modules
- generate import modules
- merge release branches
- materialize inferred axioms

### 3.2 Release plugins

Version, tag, and publish ontology artifacts for releases.

**Example capabilities:**

- generate release artifacts (`.owl`, `.obo`, JSON)
- bump ontology version IRIs
- produce release bundles for OBO Foundry or custom registries

### 3.3 Workflow plugins

Orchestrate multi-step pipelines (ROBOT/ODK-style). **owlmake** is the reference implementation.

**Example capabilities:**

- run end-to-end ODK release workflow
- chain build → QC → report → publish
- execute Makefile or GitHub Actions equivalents from the IDE/CLI

### 3.4 Validation / QC plugins

Add quality checks beyond built-in Strixonomy diagnostics.

**Example capabilities:**

- run workflow-specific validation
- enforce naming conventions
- SHACL validation ([SHACL_SPEC.md](SHACL_SPEC.md))
- OBO Foundry compliance checks

### 3.5 Documentation plugins

Generate human-readable documentation from workspace state.

**Example capabilities:**

- generate Markdown/HTML ontology docs
- produce CSV/JSON catalog reports
- emit custom documentation portals

### 3.6 AI / MCP plugins

Expose semantic workspace context to AI agents and MCP clients.

**Example capabilities:**

- MCP server over Strixonomy catalog
- ontology review and modeling suggestions
- documentation generation with LLM assistance

### 3.7 Validator plugins (built-in category)

Add custom diagnostics surfaced in the Problems panel and `strixonomy validate`.

Examples:

- require labels on every class
- forbid deprecated imports
- validate organization-specific annotation properties

### 3.8 Exporter plugins

Generate output formats from the indexed catalog.

Examples:

- Markdown, HTML, CSV reports
- JSON catalogs, SHACL shapes

### 3.9 Reasoner plugins

Integrate **native** reasoners (Rust binary or WASM). JVM subprocess reasoners are **not supported** ([ADR-0014](adr/0014-rust-native-reasoners-only.md)).

Built-in adapters (`el`, `dl`, `rl`, `rdfs`, `auto`) ship in `strixonomy-reasoner` as thin wrappers over [OntoLogos](https://github.com/eddiethedean/ontologos) — see [REASONER_SPEC.md](REASONER_SPEC.md), [ADR-0015](adr/0015-adopt-ontologos-reasoner.md).

### 3.10 Query function plugins

Add functions to the SQL-like query layer.

Examples:

- `ontology_depth(iri)`
- `descendants(iri)`
- `ancestor_path(iri)`

### 3.11 UI plugins

Future extension point for VS Code views or custom inspectors.

## 4. Reference external workflow plugin: owlmake

[owlmake](https://github.com/INCATools/owlmake) is **not** a core Strixonomy dependency. It is the first reference plugin showing how external workflow tools integrate:

| Integration point | How owlmake uses Strixonomy |
|-------------------|---------------------------|
| Workspace index | Read catalog, imports, and entity metadata |
| Diagnostics | Surface workflow errors as Strixonomy diagnostics |
| LSP / IDE | Strixonomy runs workflow actions and shows build output |
| CLI | Optional `strixonomy` subcommand or sidecar binary |

Strixonomy does **not** reimplement owlmake, ROBOT, or ODK internally. Plugins call out to those tools (or Rust-native equivalents) and report results back through the plugin API.

## 5. Plugin manifest

```toml
[plugin]
name = "example-workflow"
version = "0.1.0"
kind = "workflow"

[capabilities]
build = true
validate = true
release = true
diagnostics = true
```

Kinds: `validator`, `exporter`, `build`, `release`, `workflow`, `documentation`, `reasoner`, `query`, `ui`, `ai`.

## 6. Plugin interfaces

### Validator

```rust
pub trait ValidatorPlugin {
    fn name(&self) -> &str;
    fn validate(&self, catalog: &OntologyCatalog) -> Vec<Diagnostic>;
}
```

### Exporter

```rust
pub trait ExporterPlugin {
    fn name(&self) -> &str;
    fn export(&self, catalog: &OntologyCatalog, options: ExportOptions) -> Result<ExportResult>;
}
```

### Build plugin

```rust
pub trait BuildPlugin {
    fn name(&self) -> &str;
    /// Build ontology artifacts from the indexed workspace.
    fn build(&self, workspace: &Workspace, options: BuildOptions) -> Result<BuildResult>;
}
```

`BuildResult` includes output paths, logs, and diagnostics to surface in the IDE.

### Workflow plugin

```rust
pub trait WorkflowPlugin {
    fn name(&self) -> &str;
    /// Run a named workflow step or full pipeline (e.g. ODK release).
    fn run(&self, workspace: &Workspace, request: WorkflowRequest) -> Result<WorkflowResult>;
}
```

`WorkflowRequest` may specify `step` (`build`, `qc`, `release`, `report`), config path, and dry-run flag. **owlmake** implements this trait as the reference external workflow plugin.

## 7. Stability

v1.0 plugin APIs should be semver-stable.

Before v1.0, plugin APIs may change. See [strixonomy/plugin-model.md](../strixonomy/plugin-model.md).

## 8. v1.0 reference plugins (P1)

Ship with v1.0 as examples and optional builtins:

| Plugin | Kind | Purpose |
|--------|------|---------|
| `naming-convention-validator` | Validator | Enforce IRI/label naming rules |
| `markdown-docs-exporter` | Exporter | Markdown ontology docs |
| `shacl-validator` | Validator | SHACL via adapter ([SHACL_SPEC.md](SHACL_SPEC.md)) |
| **owlmake** (external) | Workflow | Reference ROBOT/ODK-style build, QC, release — not bundled in Strixonomy |

These demonstrate the plugin API; they do not replace Protégé's plugin catalog (P2 in [PROTEGE_PARITY.md](PROTEGE_PARITY.md)).
