# Project governance and sustainability

What enterprise evaluators can determine from **published documentation and repository policy**. This is not a commercial vendor statement.

## Project model

| Aspect | Status |
|--------|--------|
| **Product** | Open-source Strixonomy (VS Code IDE) + Strixonomy (Rust engine) |
| **License** | MIT OR Apache-2.0 (application crates); third-party licenses in [LICENSES.md](../design/LICENSES.md) |
| **Distribution** | GitHub Releases (VSIX, CLI, LSP), [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy), [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy), [crates.io](https://crates.io/search?q=strixonomy) |
| **Commercial support** | **Not offered** — community via [GitHub issues](https://github.com/eddiethedean/strixonomy/issues) |
| **Vendor / company** | Not documented as a separate legal entity |

Fortune 500 teams should plan **internal OSS adoption** with their own escalation path and pinned versions.

## Release cadence (observed)

Recent documented releases (see [changelog](../changelog.md)):

- **0.28.x** — Current tagged release (compat shim removal; PyPI `strixonomy` reservation; Python SDK planned v0.34).

| Version | Date (changelog) |
|---------|------------------|
| 0.28.0 | 2026-07-28 |
| 0.27.0 | 2026-07-27 |
| 0.25.0 | 2026-07-15 |
| 0.24.0 | 2026-07-14 |
| 0.22.0 | 2026-07-13 |
| 0.20.0 | 2026-07-13 |
| 0.19.0 | 2026-07-13 |
| 0.18.2 | 2026-07-13 |
| 0.18.1 | 2026-07-12 |
| 0.18.0 | 2026-07-11 |
| 0.15.0 | 2026-07-08 |
| 0.14.0 | 2026-07-09 |
| 0.13.0 | 2026-07-08 |
| 0.12.0 | 2026-07-06 |
| 0.11.3 | 2026-07-06 |
| 0.11.2 | 2026-07-06 |
| 0.11.1 | 2026-07-06 |
| 0.11.0 | 2026-07-05 |
| 0.10.0 | 2026-07-04 |
| 0.9.0 | 2026-07-03 |
| 0.8.0 | 2026-06-26 |
| 0.7.0 | 2026-06-25 |
| 0.6.0 | 2026-06-24 |

v0.29–v0.30 releases may ship frequently. **No committed future cadence** is documented.

Maintainers follow [releasing.md](../releasing.md): version bump, CHANGELOG, SHIPPED matrix, `./scripts/build-docs.sh`, `./scripts/check-doc-versions.sh`, GitHub Release artifacts with `SHA256SUMS` and `NOTICES`.

## Version support policy

| Stream | Security support (documented) |
|--------|-------------------------------|
| **0.28.x** | In progress (unreleased) |
| **0.28.x** | Yes — current tagged release |
| **0.19.x** | Yes |
| **0.14.x** | Best effort |
| **0.11.x** | No |
| **0.10.x** | Best effort |
| **≤ 0.9.x** | No |

Pin versions in CI and desktop rollouts; do not assume automatic long-term backports. Canonical table: [security policy](../security.md).

## Security response

- Report via [GitHub Security Advisories](https://github.com/eddiethedean/strixonomy/security/advisories/new) — not public issues
- Acknowledgment target: within a few business days ([SECURITY.md](https://github.com/eddiethedean/strixonomy/blob/main/SECURITY.md))
- **No published SLA** for patch delivery
- Historical advisories: check the repository **Security** tab (not summarized in docs)

Supply chain: `cargo audit` in CI; release integrity via SHA256 — [release integrity](../release-integrity.md). Code signing: **not shipped**.

## Quality gates (documented)

| Gate | Where documented |
|------|------------------|
| Rust CI (fmt, clippy, tests) | README, [contributing.md](../contributing.md) |
| Extension tests + VS Code E2E | README, contributing |
| MkDocs strict build | [releasing.md](../releasing.md) |
| Doc version sync | `./scripts/check-doc-versions.sh` |

## Roadmap governance

- **Target specs** live under Contributing → Design (may describe future behavior)
- **Shipped behavior** is canonical in [SHIPPED.md](../SHIPPED.md)
- **v0.30** is a product goal, not a committed date — [Release timeline (non-commitment)](release-timeline.md)

## Contributing

Community contributions welcome — [contributing.md](../contributing.md). No documented contributor license agreement beyond standard GitHub inbound licensing.

## Enterprise implications

| Question | Documented answer |
|----------|-------------------|
| Bus factor / team size | Not documented |
| Funding model | Not documented |
| Paid enterprise tier | Not offered |
| Partner program | Not documented |
| SOC 2 / ISO | Not claimed — [production readiness](production-readiness.md) |

## Related

- [Enterprise evaluation](enterprise-eval.md)
- [Production readiness](production-readiness.md)
- [Release timeline](release-timeline.md)
- [Security policy](../security.md)
