# Contributing to Strixonomy

> **Canonical guide:** [docs/contributing.md](docs/contributing.md) (also on [Read the Docs](https://strixonomy-vs.readthedocs.io/en/latest/contributing/)).
>
> Edit **`docs/contributing.md`** first, then keep this root file as a short GitHub landing pointer — not a second full guide.

**AI / agent contributors:** start with [AGENTS.md](AGENTS.md) (SHIPPED-first; do not implement from `docs/design/` targets).

## Quick start

```bash
# Docs-only
./scripts/check-doc-versions.sh
# Optional preview
./scripts/build-docs.sh   # or ./scripts/serve-docs.sh

# Full local CI parity (30–60+ min)
./scripts/run-ci-local.sh
```

Naming: [Product identity](docs/guides/product-identity.md). Env vars: prefer `STRIXONOMY_LSP_BIN` ([v0.27 migration](docs/migration/v0.27.md)).

**Good first issues:** GitHub labels `good first issue` and `docs` — see [docs/contributing.md](docs/contributing.md#good-first-issues).

Full setup, testing matrix, plugin paths, and release notes: **[docs/contributing.md](docs/contributing.md)**.
