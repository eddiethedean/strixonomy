# API stability (pre-1.0)

Strixonomy and Strixonomy are **pre-1.0**. Published crates use **0.27.x** on crates.io (latest tagged). Minor releases may add or change APIs until v1.0.0.

**Canonical capabilities:** [What ships today](../SHIPPED.md)

## v0.17 API freeze scope (path to 1.0)

The following modules are **documented and intended to stabilize** toward 1.0:

| Module / surface | Crate | Notes |
|------------------|-------|-------|
| `Workspace`, catalog index | `strixonomy` | Primary embedding entry |
| Core model types | `strixonomy-core` | `Entity`, `Diagnostic`, IRI helpers |
| SQL / SPARQL query | `strixonomy-query` | Virtual tables; new tables may be added pre-1.0 |
| Diagnostics | `strixonomy-diagnostics` | Rule codes + `DiagnosticConfig` |
| Semantic diff | `strixonomy-diff` | `DiffResult`, `format_diff_*` |
| Docs export | `strixonomy-docs` | `export_workspace`, hierarchy/property renderers |
| OWL / OBO patch | `strixonomy-owl`, `strixonomy-obo` | Patch op JSON shapes |
| Plugin host (SDK 1.0 wire) | `strixonomy-plugin` | Manifest schema, `PluginHost` — wire frozen as SDK 1.0; marketplace/owlmake remain product **1.0** — [Plugin policy](plugin-policy.md) |

**May still change pre-1.0:** internal indexer modules, LSP field additions, webview `postMessage` types (ship with extension), SQL column additions (additive).

**Frozen at 1.0 (product target):** CLI command names, exit codes, stable Rust types above, documented LSP `strixonomy/*` methods, curated plugin marketplace. **Already frozen as Plugin SDK 1.0:** TOML + subprocess JSON wire (`api_version = "1"`).

## Stability tiers

| Tier | Surface | Stability | Notes |
|------|---------|-----------|-------|
| **A — Stable enough for CI** | `strixonomy validate`, `query`, `sparql`, `classify`, `realize`, `check-instance`, `dl-query`, `refactor` (incl. merge/replace), `diff`, `docs`, `patch`, `robot`, `plugins`, `workflow` CLI | High for **commands and exit codes** | Pin with `cargo install ontocore-cli --locked --version 0.27.0`. Exit codes documented in [workspace limits](../workspace-limits.md). No `strixonomy swrl` CLI — SWRL via IDE/LSP/patches. |
| **B — Documented, may evolve** | LSP custom methods (`strixonomy/*`) | Medium | Wire format in [LSP API](../lsp-api.md) and [JSON Schema](../lsp-protocol.schema.json). Minor releases may add fields or methods. |
| **C — Library APIs** | `strixonomy` and `strixonomy-*` Rust crates | Medium-low | Public types used by CLI/LSP are more stable than internal modules. Pin exact versions in `Cargo.toml`. |
| **D — Experimental / product-1.0** | Curated plugin marketplace, production owlmake, SHACL full validation, MCP, Python/TS SDKs | Low until product **1.0** | Plugin **SDK 1.0** wire is frozen today — see [Plugin authoring](plugins.md) and [Plugin policy](plugin-policy.md). |

## What we commit to before v1.0

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
cargo install ontocore-cli --locked --version 0.27.0
```

**Rust embedding:**

```toml
ontocore = "0.27"
strixonomy-core = "0.27"
```

**VS Code:** install Strixonomy **0.27.0** (latest tagged) from Marketplace, Open VSX, or a release VSIX — the bundled `strixonomy-lsp` matches the extension version.

## Enterprise evaluation

- **Production readiness:** [Production readiness](production-readiness.md)
- **Security:** [Security](../security.md)
- **LGPL (horned-owl):** [LGPL compliance](lgpl-compliance.md)
