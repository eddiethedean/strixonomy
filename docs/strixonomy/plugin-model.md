# Strixonomy plugin model

> **Canonical author guide:** **[Plugin authoring](../guides/plugins.md)** — implement from that page only.
>
> **Do not implement from this page.** Categories and manifest sketches below are overview/background. The shipped host uses `.strixonomy/plugins/*.toml` + subprocess entries (not native `.so` libraries).

> **Status:** Plugin **SDK 1.0** freezes the TOML + subprocess JSON wire (`api_version = "1"`) — safe to author against today. A curated marketplace and production owlmake integration remain **product 1.0** goals. See [Plugin policy](../guides/plugin-policy.md).

The plugin system allows users and organizations to extend **Strixonomy** and **Strixonomy** without modifying the core project. **Plugins integrate with Strixonomy; they are not part of Strixonomy.**

Historical trait-based design (do not implement from): [PLUGIN_SPEC.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/PLUGIN_SPEC.md) — superseded by the shipped manifest + subprocess host in v0.14+.

## Architecture

```text
Strixonomy (IDE)
     │
     ▼
Strixonomy (workspace engine + plugin host)
     ├── Ontologos (reasoning — core integration, not a plugin)
     └── External plugins (owlmake, validators, exporters, …)
```

[owlmake](https://github.com/INCATools/owlmake) is the **reference external workflow plugin** — ROBOT/ODK-style build, QC, and release automation without becoming a core dependency.

## Plugin categories (overview — not an implementation contract)

| Category | Purpose | Examples |
|----------|---------|----------|
| **Build** | Compile, merge, materialize | Generate import modules, build release `.owl` |
| **Release** | Version and publish artifacts | OBO Foundry release bundles |
| **Workflow** | Multi-step pipelines | ODK release via **owlmake** (reference) |
| **Validation / QC** | Quality checks | SHACL, naming rules, OBO compliance |
| **Documentation** | Human-readable output | Markdown/HTML ontology docs |
| **AI / MCP** | Agent integration | MCP server, review assistants |
| Validator | Custom diagnostics | Required labels, deprecated imports |
| Exporter | Output formats | CSV, JSON catalogs |
| Reasoner | Native reasoner integration | Custom Rust/WASM reasoners ([ADR-0014](../design/adr/0014-rust-native-reasoners-only.md)) |
| Query function | SQL extensions | `descendants(iri)`, `ontology_depth(iri)` |
| UI | VS Code views | Custom inspectors (Strixonomy layer) |

Built-in reasoner adapters (`el`, `rl`, `rdfs`, `dl`, `auto`) ship in `strixonomy-reasoner` as thin wrappers over [OntoLogos](https://github.com/eddiethedean/ontologos).

## Strixonomy vs Strixonomy plugins

| Layer | Plugin scope |
|-------|--------------|
| **Strixonomy** | Build, release, workflow, validators, exporters, reasoners, query functions — run in CLI/LSP/Rust library |
| **Strixonomy** | UI plugins — VS Code views, webview panels, workflow action surfaces |

## Reference interfaces (sketch)

See [Plugin authoring](../guides/plugins.md) for manifest format, permissions, and reference plugins. [PLUGIN_SPEC.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/PLUGIN_SPEC.md) is a historical design doc — do not implement from it.

## Manifest (historical sketch — do not copy)

Use the Toml + subprocess examples in [Plugin authoring](../guides/plugins.md). The sketch below is obsolete (native library `entry`):

```toml
[plugin]
id = "org.example.demo"
name = "demo"
version = "0.1.0"
api_version = "1"
kind = "validator"
# Real plugins: entry = a CLI executable or script, not a .so
entry = "./bin/my-validator"
```

## Timeline

- **v0.14:** Plugin host MVP — manifest discovery, reference validator/exporter plugins, OntoUI inspector cards
- **v1.0:** Stable plugin API + reference plugins; Strixonomy surfaces workflow actions in IDE

See [Platform roadmap](../roadmap.md) and [OBO & ROBOT interop](../design/OBO_ROBOT_SPEC.md).
