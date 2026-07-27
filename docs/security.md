# Security policy

Strixonomy and Strixonomy are **local-first** tools: they index and parse files on disk and do not upload ontology content by default. There is **no telemetry**.

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.27.x   | Yes — latest tagged release |
| 0.26.x   | Yes (N−1) |
| ≤ 0.25.x | No — upgrade to a tagged 0.26.x or 0.27.x release |

Pin production and CI to the latest **tagged** release ([TAGGED_RELEASE](https://github.com/eddiethedean/strixonomy/blob/main/docs/TAGGED_RELEASE)). Unreleased minors on `main` are not supported until tagged. There is **no committed security patch SLA**.

Canonical policy file (GitHub Security tab): **[SECURITY.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/SECURITY.md)**

## Reporting a vulnerability

**Do not open a public issue.**

Report via [GitHub Security Advisories](https://github.com/eddiethedean/strixonomy/security/advisories/new). Include affected version, component, and reproduction steps. Allow time for a fix before disclosure. Maintainers handle reports on a best-effort basis — see [Support](support.md).

## Threat model summary

| Trust boundary | Behavior |
|----------------|----------|
| **Local disk** | Parse, index, query, reason, and write-back stay on the machine unless you or another tool move the files. |
| **Path jail** | Operations are constrained to workspace root(s). |
| **Resource limits** | Caps on files, sizes, entities, triples, and query rows — [workspace limits](workspace-limits.md). |
| **LSP** | `strixonomy-lsp` is a **stdio** child of the editor — do not expose it on the network without authentication. |
| **Restricted Mode** | Bundled LSP works untrusted; custom `strixonomy.lspPath` / `strixonomy.robotPath` are ignored until Workspace Trust. |
| **Plugins** | Manifest permissions gate read/write/external process; treat third-party plugins as executables you chose to run. |
| **ROBOT** | Spawns external Java `robot` from PATH or configured path — separate trust boundary. |
| **Telemetry / AI** | No telemetry. Cloud AI features are not shipped. |

Release checksums and `cargo audit`: [Release integrity](release-integrity.md). Enterprise checklist: [Enterprise evaluation](guides/enterprise-eval.md).
