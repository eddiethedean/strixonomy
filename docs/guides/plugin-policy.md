# Plugin SDK 1.0 compatibility policy

Strixonomy ships **Plugin SDK 1.0** as a frozen TOML + subprocess JSON host (`api_version = "1"`) — safe to author against today. A curated marketplace, commercial support, and production owlmake integration remain **product 1.0** goals. This page states the **product policy** — not how to author a plugin. For authoring, see [Plugin authoring](plugins.md).

## Support stance

| Topic | Policy |
|-------|--------|
| Commercial plugin support | **Not offered** — community / GitHub issues |
| Marketplace / app store | **None** — no curated marketplace SLA |
| Compatibility promise | Within major version **1.x**, the wire contract (`api_version = "1"`) is **additive**; breaking removals require a new major |
| Security review | Third-party plugins run as **subprocesses** with declared permissions — treat untrusted plugins as untrusted code |
| Stability target | **SDK 1.0** freezes the wire contract for authoring today; curated marketplace and production owlmake remain **product 1.0** — see [API stability](api-stability.md) |

## What must not change in 1.x

- Manifest root shape: `[plugin]`, `[capabilities]`, `[config]`, `[[ui.*]]`
- Frozen wire: `api_version = "1"`
- Subprocess invocation: `<entry> <action> --workspace <path> …` with JSON stdout
- Existing actions: `validate`, `export`, `workflow`, `ui-view`
- Permission string ids (`workspace.read`, `external_process`, …)

## What may be added in 1.x (non-breaking)

- New optional manifest fields (ignored by older hosts)
- New optional JSON stdout fields
- New provider actions (`reasoner.classify`, `query.run`, `refactor.preview`, `graph.build`)
- New hosted `kind` values documented as hosted; reserved kinds stay rejected until promoted

## Deprecation

Fields or actions marked deprecated remain for at least one minor release before removal, and only leave in a **major** bump.

## Migration from v0.14–v0.24 authoring

Manifests written for Strixonomy **v0.16+** remain valid. New provider kinds (`reasoner`, `query`, `refactor`, `graph`) and `depends_on` / `activation` are **opt-in**.

## What is not promised

- Binary ABI / in-process Rust trait stability across majors
- Java Protégé plugin compatibility
- Guaranteed availability of community plugins
- Marketplace or remote code execution

## Enterprise guidance

Treat plugins as **internal tooling**: pin versions, review manifests, and keep a rollback path (`strixonomy plugins disable <id>` writes `.strixonomy/plugin-disabled.json`). See [Production readiness](production-readiness.md) and [Security](../security.md).

## Related

- [Plugin authoring](plugins.md)
- [API stability](api-stability.md)
- [What ships today](../SHIPPED.md)
