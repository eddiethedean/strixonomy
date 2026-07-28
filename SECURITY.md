# Security Policy

Strixonomy IDE and Strixonomy engine are **local-first** tools: they index and parse ontology files on disk and do **not** upload workspace content by default. There is **no telemetry**.

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.27.x   | Yes — latest tagged release |
| 0.26.x   | Yes (N−1) |
| ≤ 0.25.x | No — upgrade to a tagged 0.26.x or 0.27.x release |

Pin production and CI to the latest **tagged** release (`docs/TAGGED_RELEASE` in the repository). Unreleased minors on `main` are not supported for adoption until tagged.

Security fixes land on the latest tagged minor first. Older lines receive advisories only when maintainers choose a backport — there is **no committed patch SLA**.

## Reporting a vulnerability

**Do not open a public GitHub Issue** for security reports.

1. Report privately via [GitHub Security Advisories](https://github.com/eddiethedean/strixonomy/security/advisories/new).
2. Include: affected version(s), component (CLI / LSP / extension / plugin host), reproduction steps, and impact.
3. Allow maintainers time to confirm and prepare a fix before any public disclosure.

**Expectations:** Community project, **no SLA**. Maintainers acknowledge reports when they can and publish advisories on the repository Security tab when a fix or mitigation ships.

Out of scope for private reporting: general product bugs, feature requests, and usage questions — use [Discussions](https://github.com/eddiethedean/strixonomy/discussions) or public Issues instead.

## Threat model summary

| Trust boundary | Behavior |
|----------------|----------|
| **Local disk** | Indexing, parse, query, reason, and write-back operate on workspace files. Content stays on the machine unless the user or an external tool moves it. |
| **Path jail** | File operations are constrained to configured workspace root(s). See [workspace limits](https://strixonomy.readthedocs.io/en/latest/workspace-limits/). |
| **Resource limits** | Caps on file count, file size, entities, triples, and query rows reduce DoS risk when opening large or untrusted repos. |
| **LSP** | `strixonomy-lsp` runs as a **stdio child** of the editor. Do **not** expose it on a network socket without authentication. |
| **VS Code Restricted Mode** | Bundled language server works without Workspace Trust. Custom `strixonomy.lspPath` and `strixonomy.robotPath` are **ignored** until the folder is trusted. |
| **Plugin host** | Workspace plugins declare permissions (`workspace.read`, `workspace.write`, `external_process`, …). Subprocess plugins run as separate processes with host-enforced limits; treat third-party plugins like any other executable you install. |
| **ROBOT** | `strixonomy robot` / LSP `runRobot` spawn an external Java `robot` binary from PATH (or configured path). Review ROBOT’s own security posture for that workflow. |
| **AI / network** | No built-in cloud upload or telemetry. AI features are opt-in by design and **not shipped** in current releases. |

**Supply chain:** Release CLI/VSIX artifacts publish `SHA256SUMS`. CI runs `cargo audit`. Code signing is **not** shipped.

## Scope notes

- Opening an **untrusted ontology repository** can still consume disk/CPU within limits and, after Trust + custom paths, can run binaries you configured.
- XML write-back is **semantic re-serialize** (not byte-identical); that is a fidelity concern, not a network trust boundary.

Related public docs: [Security on Read the Docs](https://strixonomy.readthedocs.io/en/latest/security/), [Enterprise evaluation](https://strixonomy.readthedocs.io/en/latest/guides/enterprise-eval/), [Release integrity](https://strixonomy.readthedocs.io/en/latest/release-integrity/).
