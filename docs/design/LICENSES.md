# Third-Party Licenses

Strixonomy / Strixonomy is licensed under **MIT OR Apache-2.0** at your option (see repository root `LICENSE-MIT` and `LICENSE-APACHE-2.0`).

This document summarizes **third-party licenses** for dependencies named in [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md). It is not legal advice. Regenerate NOTICES before releases when dependency sets change.

## Direct dependencies (shipped v0.9)

| Crate | License | Distribution notes |
|-------|---------|-------------------|
| `oxigraph` | MIT OR Apache-2.0 | Compatible |
| `sqlparser` | Apache-2.0 | Compatible |
| `ignore` | Unlicense OR MIT | Compatible |
| `regex` | MIT OR Apache-2.0 | Compatible (`strixonomy-diagnostics`) |
| `lsp-server` | MIT OR Apache-2.0 | Compatible |
| `lsp-types` | MIT | Compatible |
| `horned-owl` | **LGPL-3.0** | Rust crate link via `strixonomy-owl`. See [LGPL compliance guide](../guides/lgpl-compliance.md). Document LGPL in release artifacts; provide source offer per LGPL-3.0 if distributing binaries. |
| `ontologos-*` | MIT OR Apache-2.0 | Compatible (`strixonomy-reasoner`) |
| `reasonable` (transitive via OntoLogos) | **BSD-3-Clause** | Include BSD notice in NOTICES |
| `fastobo`, `fastobo-owl`, `fastobo-validator` | MIT | Compatible |
| `petgraph` | MIT OR Apache-2.0 | Compatible |
| `serde`, `clap`, `thiserror`, etc. | MIT OR Apache-2.0 | Compatible |

## Planned dependencies (post-v0.9)

| Crate | License | Distribution notes |
|-------|---------|-------------------|
| `rudof` | MIT OR Apache-2.0 | Compatible |
| `git2` | MIT OR Apache-2.0 | Links `libgit2` (GPL-2.0 with linking exception) — review if static linking |
| `notify` | CC0-1.0 | Compatible |
| `pulldown-cmark` | MIT | Compatible |
| `minijinja` | Apache-2.0 | Compatible |
| `datafusion` (if adopted) | Apache-2.0 | Compatible |

## External CLIs (not linked)

| Tool | License | Notes |
|------|---------|-------|
| [ROBOT](https://github.com/ontodev/robot) | BSD-3-Clause | Optional subprocess; user installs separately |

## NOTICES template (release builds)

```text
This product includes software developed by third parties:

- horned-owl (LGPL-3.0) — https://github.com/phillord/horned-owl
- reasonable (BSD-3-Clause) — via OntoLogos — https://github.com/gtfierro/reasonable
- Oxigraph (MIT OR Apache-2.0) — https://github.com/oxigraph/oxigraph
...
```

Full dependency trees: `cargo license` (install `cargo-license` crate) before tagging releases.

## Related

- [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md)
- [ADR-0016](adr/0016-dependency-first-implementation.md)
