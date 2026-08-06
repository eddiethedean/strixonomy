# Procurement and enterprise appendix

Single entry point for security, platform, and legal questionnaires. Start with the **adoption trio** before reading this appendix:

1. [What ships today](../SHIPPED.md) — capability matrix (latest tagged release)
2. [Known limitations](../known-limitations.md) — honest gaps
3. [Protégé vs Strixonomy](protege-decision.md) — fit decision

## Summary for evaluators

| Question | Answer |
|----------|--------|
| **Production-ready Protégé replacement?** | **No** until v0.30 — pilot/coexistence workflows today |
| **Latest tagged release** | **v0.28.1** — pin installs; see [Versions & channels](versions-and-channels.md) for Marketplace lag |
| **Commercial support** | **Not offered** — community via GitHub issues |
| **Editable formats** | Turtle, OBO, RDF/XML, OWL/XML write-back; XML is semantic re-serialize (not Protégé byte-identical); JSON-LD / TriG / N-Triples read-only |
| **CLI prebuilds** | Linux x64 only; macOS/Windows use `cargo install` or bundled VSIX LSP |

### Non-claims (procurement)

| Topic | Answer |
|-------|--------|
| Commercial support / SLA | **Not offered** |
| SOC 2 / ISO 27001 | **No** |
| Code-signed binaries | **Not yet** (SHA256 only) — signing is v0.31+ candidate, not committed |
| HIPAA BAA | **No** |

Details: [Production readiness](production-readiness.md) § Support and compliance.

## Detailed guides (by topic)

| Topic | Document |
|-------|----------|
| Security policy & threat model | [Security](../security.md) · [SECURITY.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/SECURITY.md) |
| Release integrity (SHA256) | [Release integrity](../release-integrity.md) |
| CI / automation stability | [CI integration](../ci-integration.md) · [Automation and stability](../automation-stability.md) |
| Platform compatibility | [Platform compatibility](platform-compatibility.md) |
| Performance and sizing | [Performance and sizing](performance-sizing.md) *(provisional — not benchmark-certified)* |
| LGPL / third-party licenses | [LGPL compliance](lgpl-compliance.md) · [LICENSES](../design/LICENSES.md) |
| Production readiness tiers | [Production readiness](production-readiness.md) |
| Evidence protocol (pilots) | [Production evidence](production-evidence.md) |
| Governance & release cadence | [Governance](governance.md) · [Release timeline](release-timeline.md) |
| Air-gapped / offline | [Enterprise deployment](enterprise-deployment.md) |
| API stability (v0.29–v0.30) | [API stability](api-stability.md) |

## Enterprise evaluation checklist

1. Confirm required capabilities in [SHIPPED](../SHIPPED.md) (VS Code **and** CLI columns).
2. Read [Known limitations](../known-limitations.md) — especially format write-back and SQL subset.
3. Pin `cargo install strixonomy-cli --locked --version 0.28.1` (or current [TAGGED_RELEASE](https://github.com/eddiethedean/strixonomy/blob/main/docs/TAGGED_RELEASE)) in CI.
4. Verify release artifacts with [SHA256SUMS](../release-integrity.md).
5. Run a pilot using [First success](../guides/first-success.md) + your corpus.

## Related

- [Enterprise evaluation](enterprise-eval.md) — shorter summary page
- [Support and contact](../support.md)
