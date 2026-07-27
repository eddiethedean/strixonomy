# LGPL compliance guide (horned-owl)

This page helps **legal, security, and platform teams** evaluate copyleft obligations when adopting Strixonomy / Strixonomy **v0.26.2** (latest tagged). It summarizes documentation and dependency licenses — **it is not legal advice**. Engage your counsel for binding decisions.

Dependency inventory: [LICENSES.md](../design/LICENSES.md) · [DEPENDENCY_MATRIX.md](../design/DEPENDENCY_MATRIX.md).

## Why LGPL matters

`strixonomy-owl` links [`horned-owl`](https://crates.io/crates/horned-owl) (**LGPL-3.0**) for:

- OWL axiom modeling in the catalog
- Turtle patch write-back
- Manchester syntax parse/serialize

If your organization **does not use Turtle write-back or Manchester editing**, you may still pull `horned-owl` transitively when using the full VS Code extension or CLI — verify your deployment surface with `cargo license` or the release `NOTICES` file.

Other notable licenses in v0.8:

| Component | License | Trigger |
|-----------|---------|---------|
| Strixonomy / Strixonomy crates | MIT OR Apache-2.0 | Always |
| `horned-owl` | **LGPL-3.0** | Linked in `strixonomy-owl` |
| `reasonable` (via OntoLogos) | BSD-3-Clause | Reasoner (`classify`) |
| Oxigraph, sqlparser, etc. | MIT / Apache-2.0 | Parser, query |

## Deployment scenarios

### Internal use only (employees, no redistribution)

| Activity | Typical enterprise question |
|----------|----------------------------|
| Install VS Code extension from Marketplace | Extension is MIT; bundled `strixonomy-lsp` binary includes LGPL-linked code |
| Run `strixonomy` CLI on build agents | Same — prebuilt Linux binary or `cargo install` |
| `cargo install` on developer laptops | Rust dynamic linking to `horned-owl` rlib |

**Documentation position:** Internal use without distributing Strixonomy binaries to third parties is the **most common enterprise pattern**. LGPL obligations for internal use are generally lighter than redistribution, but **your counsel must confirm** for your jurisdiction and policies.

### Redistributing binaries or VSIX to customers/partners

If you **ship** Strixonomy/Strixonomy artifacts (custom VSIX, embedded CLI, golden VM image):

1. Include **third-party notices** — release `NOTICES` on [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases)
2. Provide **LGPL source offer** for `horned-owl` per LGPL-3.0 Section 6 (corresponding source for the linked library version)
3. Document how recipients can obtain Strixonomy source (this repository is public under MIT/Apache-2.0)

### CI-only (`strixonomy validate` / `classify`, no VS Code)

| Component | horned-owl exposure |
|-----------|---------------------|
| `validate` / `query` / `sparql` | Catalog path may use Horned-OWL for Turtle axioms when parsing `.ttl` |
| `classify` | OntoLogos stack (MIT/Apache + BSD transitive) |

Run `cargo license -p strixonomy-cli` on the pinned version to produce an SBOM for legal review.

## Rust linking model (for counsel)

Strixonomy is a **Rust** workspace. `horned-owl` is linked as a Rust crate dependency (not a separate `.so` on all platforms). Legal teams often ask:

| Question | Documentation pointer |
|----------|----------------------|
| Static or dynamic? | Rust crate linking; platform-dependent artifact layout in release binaries |
| Can we swap horned-owl? | Not without replacing `strixonomy-owl` — no supported alternate today |
| AGPL/LGPL policy conflict? | horned-owl is **LGPL-3.0**, not AGPL |

Provide your legal team the **exact release version** and `NOTICES` file from the GitHub Release you deploy.

## SBOM and NOTICES

| Artifact | Third-party manifest |
|----------|---------------------|
| GitHub Release | `NOTICES` + `SHA256SUMS` |
| crates.io install | Run `cargo license` locally on pinned `Cargo.lock` |
| VSIX | Bundles platform `strixonomy-lsp`; verify release `NOTICES` matches VSIX version |

Maintainers regenerate `NOTICES` before releases per [releasing.md](../releasing.md). **Consumers** should archive `NOTICES` alongside each pinned version used in production CI.

## Risk mitigations (organizational)

| Mitigation | When |
|------------|------|
| **CI-only adoption** | Minimize desktop extension rollout until legal sign-off |
| **Pin versions** | `cargo install strixonomy-cli --locked --version 0.26.2` or release tarball |
| **Archive NOTICES** | Store with internal artifact registry |
| **Protégé coexistence** | Teams that cannot accept LGPL on desktops use Protégé for authoring; Strixonomy in Linux CI only |
| **Air-gapped mirror** | Host VSIX + CLI + `NOTICES` + source snapshot internally — [enterprise deployment](enterprise-deployment.md) |

## Checklist for legal review

- [ ] Deployment model: internal only vs redistribution
- [ ] Surfaces in use: VS Code extension, CLI, both
- [ ] Features used: Turtle write-back (pulls horned-owl path) vs read-only index/query
- [ ] Pinned version and archived `NOTICES`
- [ ] SBOM from `cargo license` on that version
- [ ] LGPL source offer process if redistributing binaries
- [ ] BSD notice for OntoLogos/`reasonable` if using `classify`

## Related

- [LICENSES.md](../design/LICENSES.md)
- [FAQ — LGPL](../faq.md)
- [Enterprise evaluation](enterprise-eval.md)
- [Production readiness](production-readiness.md)
