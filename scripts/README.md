# Scripts

Maintainer and contributor helpers at the repo root. Prefer [CONTRIBUTING.md](../CONTRIBUTING.md) and the [testing matrix](https://strixonomy-vs.readthedocs.io/en/latest/guides/testing-matrix/) for when to run what.

| Script | Purpose |
|--------|---------|
| `run-ci-local.sh` | Approximate GitHub Actions CI locally (Rust + optional extension). **30–60+ minutes** cold — not required for docs-only PRs. |
| `serve-docs.sh` | Local MkDocs preview (`pip install -r docs/requirements.txt` first). Skips git-revision stamps; kills leftover `mkdocs` processes first. |
| `build-docs.sh` | Strict docs build (CI parity). Fast path skips git-revision + HTML minify; RTD enables both via `READTHEDOCS`. Opt in: `ENABLE_GIT_REVISION_DATE=true` / `ENABLE_MKDOCS_MINIFY=true`. |
| `check-doc-versions.sh` | Guardrails for version pins and Tier-1 doc sync (invoked from CI / local CI). |
| `package-extension.sh` | Build / package the VS Code VSIX (maintainers). |
| `prepare-extension-server.sh` | Stage `strixonomy-lsp` into `extension/server/`. |
| `fetch-lsp-servers.sh` | Fetch platform LSP binaries for packaging. |
| `package-tutorial-zip.sh` | Build `ontocode-tutorial.zip` for GitHub Releases. |
| `run-mutants.sh` | Optional mutation testing — see [mutants baseline](https://strixonomy-vs.readthedocs.io/en/latest/mutants-baseline/). |
| `validate-parity-manifest.py` | Internal Protégé parity YAML checks (contributors on that program). |

Docs-only PRs: preview with `./scripts/serve-docs.sh` or `./scripts/build-docs.sh`. Skip `run-ci-local.sh` unless you changed CI scripts or need full parity.
