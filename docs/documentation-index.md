# Documentation index

Master index for Strixonomy IDE / Strixonomy engine planning, architecture, and user documentation.

**Latest tagged release:** v0.27.0 · **Canonical capabilities:** [What ships today](SHIPPED.md) · **Limits:** [Known limitations](known-limitations.md) · **Terms:** [Glossary](glossary.md)

The public site navigation is defined in [`mkdocs.yml`](https://github.com/eddiethedean/strixonomy/blob/main/mkdocs.yml). This page is a reading-order map — not a second nav.

## Recommended reading order

### New users

1. [First success](guides/first-success.md)
2. [Your next steps (day 2)](guides/day-2.md)
3. [Feature tour](ide/feature-tour.md)
4. [Supported formats](supported-formats.md)
5. [Ontology concepts](concepts.md) (if new to OWL)

### Evaluators and adopters

1. [What ships today](SHIPPED.md)
2. [Known limitations](known-limitations.md)
3. [First success (~10 min)](guides/first-success.md)
4. [Ontology concepts](concepts.md) (domain primer) · [Glossary](glossary.md) (product terms)
5. [Roadmap hub](roadmap-hub.md)

### Contributors

1. [Contributing](contributing.md)
2. [Architecture tour](guides/architecture-tour.md) · [Internals](internals.md)
3. [Testing matrix](guides/testing-matrix.md) · [Debugging](debugging.md)
4. [Releasing](releasing.md)
5. APIs: [CLI reference](cli-reference.md) · [LSP API](lsp-api.md) · [Patch JSON](patch-reference.md) · [Catalog SQL](sql-reference.md) · [SPARQL](sparql-reference.md) · [Errors](errors.md) · [Plugins](guides/plugins.md) · [Webview protocol](webview-protocol.md)
6. [Engineering docs on GitHub](engineering.md) — UI specs, platform targets, Cursor prompts

---

## Document layers

| Layer | Location | Use for |
|-------|----------|---------|
| **Shipped (this site)** | [SHIPPED.md](SHIPPED.md), guides, reference | What works today |
| **Evaluate** | Vision, architecture, enterprise pack | Adoption decisions |
| **Engineering (GitHub)** | [engineering.md](engineering.md) | Specs, ADRs, UI, prompts — not in public MkDocs search |

Deep planning docs (`docs/ui/`, `docs/platform/`, `docs/cursor-prompts/`, `docs/PROTEGE_REVERSE_ENGINEERING/`) remain in the repository and are linked from [Engineering docs (GitHub)](engineering.md). They are **excluded from the public MkDocs build**.

---

## Site map (public)

| Section | Start here |
|---------|------------|
| **Get started** | [First success](guides/first-success.md) → [Your next steps](guides/day-2.md) → [Install](install.md) → [Product identity](guides/product-identity.md) → [Documentation index](documentation-index.md) |
| **Use the IDE** | [Overview](ide/index.md) → [Feature tour](ide/feature-tour.md) |
| **Use the engine & CLI** | [Engine overview](strixonomy/index.md) · [Examples](examples/index.md) |
| **Reference** | [CLI](cli-reference.md) · [Rust API](strixonomy/rust-api.md) · [LSP API](lsp-api.md) · [Patch](patch-reference.md) · [SQL](sql-reference.md) · [SPARQL](sparql-reference.md) · [Errors](errors.md) · [docs.rs strixonomy](https://docs.rs/strixonomy) |
| **Evaluate** | [What ships today](SHIPPED.md) · [Enterprise eval](guides/enterprise-eval.md) · [Week-2 playbook](guides/enterprise-week-2.md) |
| **Help** | [FAQ](faq.md) · [Troubleshooting](troubleshooting.md) · [Support](support.md) |
| **Contribute** | [Contributing](contributing.md) · [Plugin policy](guides/plugin-policy.md) · [CLI](cli-reference.md) · [LSP](lsp-api.md) · [Engineering](engineering.md) |

---

## Major planning documents

| Document | Purpose | Audience | Status |
|----------|---------|----------|--------|
| [SHIPPED.md](SHIPPED.md) | Canonical capability matrix | All | Shipped v0.27 |
| [known-limitations.md](known-limitations.md) | Honest limits | All | Active |
| [glossary.md](glossary.md) | Canonical terminology | All | Active |
| [roadmap-hub.md](roadmap-hub.md) | Which roadmap doc to read | All | Active |
| [roadmap.md](roadmap.md) | Platform release plan | All | Active |
| [design/adr/README.md](design/adr/README.md) | Engineering ADRs | Contributor | Active |
| [architecture.md](architecture.md) | User-facing ecosystem overview | Evaluator | Shipped v0.27 |
| [vision.md](vision.md) | Mission and direction | Evaluator | Active |
| [engineering.md](engineering.md) | Pointer to GitHub engineering corpus | Implementer | Active |
| [platform/OVERVIEW.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/OVERVIEW.md) | OntoUI / platform (GitHub) | Implementer | Shipped v0.27 |

---

## User documentation

Published site: [Read the Docs](https://strixonomy-vs.readthedocs.io/en/latest/) · Start at [index.md](index.md).
