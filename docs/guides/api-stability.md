# API stability (v0.29–v0.30)

Strixonomy IDE and Strixonomy engine are **v0.29–v0.30**. Published crates use **0.28.x** on crates.io (latest tagged). Minor releases may add or change APIs until v0.30.0.

**Canonical capabilities:** [What ships today](../SHIPPED.md)

## v0.17 API freeze scope (path to v0.30)

The following modules are **documented and intended to stabilize** toward v0.30:

| Module / surface | Crate | Notes |
|------------------|-------|-------|
| `Workspace`, catalog index | `strixonomy` | Primary embedding entry |
| Core model types | `strixonomy-core` | `Entity`, `Diagnostic`, IRI helpers |
| SQL / SPARQL query | `strixonomy-query` | Virtual tables; new tables may be added v0.29–v0.30 |
| Diagnostics | `strixonomy-diagnostics` | Rule codes + `DiagnosticConfig` |
| Semantic diff | `strixonomy-diff` | `DiffResult`, `format_diff_*` |
| Docs export | `strixonomy-docs` | `export_workspace`, hierarchy/property renderers |
| OWL / OBO patch | `strixonomy-owl`, `strixonomy-obo` | Patch op JSON shapes |
| Plugin host (SDK 1.0 wire) | `strixonomy-plugin` | Manifest schema, `PluginHost` — wire frozen as SDK 1.0; manager/owlmake v0.33, official registry v0.34, public marketplace v0.35+ — [Plugin policy](plugin-policy.md) |

**May still change v0.29–v0.30:** internal indexer modules, LSP field additions, webview `postMessage` types (ship with extension), SQL column additions (additive).

**Frozen at 1.0 (product target):** CLI command names, exit codes, stable Rust types above, documented LSP `strixonomy/*` methods, curated plugin marketplace. **Already frozen as Plugin SDK 1.0:** TOML + subprocess JSON wire (`api_version = "1"`).

## Stability tiers

| Tier | Surface | Stability | Notes |
|------|---------|-----------|-------|
| **A — Stable enough for CI** | `strixonomy validate`, `query`, `sparql`, `classify`, `realize`, `check-instance`, `dl-query`, `refactor` (incl. merge/replace), `diff`, `docs`, `patch`, `robot`, `plugins`, `workflow` CLI | High for **commands and exit codes** | Pin with `cargo install strixonomy-cli --locked --version 0.28.0`. Exit codes documented in [workspace limits](../workspace-limits.md). No `strixonomy swrl` CLI — SWRL via IDE/LSP/patches. |
| **B — Documented, may evolve** | LSP custom methods (`strixonomy/*`) | Medium | Wire format in [LSP API](../lsp-api.md) and [JSON Schema](../lsp-protocol.schema.json). Minor releases may add fields or methods. |
| **C — Library APIs** | `strixonomy` and `strixonomy-*` Rust crates | Medium-low | Public types used by CLI/LSP are more stable than internal modules. Pin exact versions in `Cargo.toml`. |
| **D — Experimental / roadmap** | Workflow registry, production owlmake, MCP, Python/TS SDKs, assisted modeling | Low until each surface ships | Plugin **SDK 1.0** wire is frozen today; forward targets are defined per phase in the [roadmap](../roadmap.md). |

## What we commit to before v0.30

- **Document** breaking changes in [migration guides](../migration/README.md) and [changelog](../changelog.md).
- **Keep CLI command names** stable where possible (`validate`, `query`, `classify`, etc.).
- **Publish** LSP JSON Schema alongside releases when wire format changes.

## What may change between minors

- Rust public API on `strixonomy-*` crates (prefer pinning matching minors, e.g. `0.24`, in Cargo.toml).
- LSP request/response fields (clients should tolerate unknown fields).
- SQL virtual table columns (check [sql-reference](../sql-reference.md) per release).
- Webview `postMessage` payloads (extension + webview-ui ship together in the VSIX).
- Plugin manifest schema and subprocess contract (documented in [Plugin authoring](plugins.md)).

## Recommended pinning

**CI / ops:**

```bash
cargo install strixonomy-cli --locked --version 0.28.0
```

**Rust embedding:**

```toml
strixonomy = "0.28"
strixonomy-core = "0.28"
```

**VS Code:** install Strixonomy **0.28.0** (latest tagged) from Marketplace, Open VSX, or a release VSIX — the bundled `strixonomy-lsp` matches the extension version.

## Enterprise evaluation

- **Production readiness:** [Production readiness](production-readiness.md)
- **Security:** [Security](../security.md)
- **LGPL (horned-owl):** [LGPL compliance](lgpl-compliance.md)
