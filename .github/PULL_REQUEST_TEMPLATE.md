## Summary

<!-- What does this PR change and why? -->

## Component

- [ ] Strixonomy (Rust crates / CLI / LSP)
- [ ] Strixonomy (VS Code extension)
- [ ] Documentation / specs
- [ ] CI / release

## Test plan

Check **only** what applies — see [testing matrix](https://strixonomy.readthedocs.io/en/latest/guides/testing-matrix/) and [CONTRIBUTING.md](../CONTRIBUTING.md).

**Docs-only / specs-only**

- [ ] Markdown / MkDocs preview locally if you touched `docs/` or `mkdocs.yml` (`./scripts/serve-docs.sh` or RTD preview)
- [ ] No Rust/extension rebuild required

**Rust (crates / CLI / LSP)**

- [ ] Scoped tests for touched crates (preferred) or `cargo test --workspace`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` (if Rust changed)
- [ ] `./scripts/run-ci-local.sh` only when you want full CI parity (30–60+ min cold)

**Extension / webview**

- [ ] `cd extension && npm test` (if extension TS changed)
- [ ] Rebuild webviews after React changes: `cd extension && npm run build:webview` (or `npm run compile`) — `npm run watch` does **not** rebuild the Vite webview UI
- [ ] Manual verification (describe):

## Docs

- [ ] README / `docs/` updated if user-facing behavior changed
- [ ] Spec docs labeled implemented vs planned if LSP/architecture behavior changed
- [ ] CHANGELOG updated (for release-worthy changes)

## Related issues

<!-- Fixes #123 -->
