# Mutation testing baseline (path_jail + OWL patch)

Greenfield, **not** a PR gate. Run locally or via `.github/workflows/mutants.yml`
(`workflow_dispatch` / weekly cron):

```bash
cargo install cargo-mutants --locked
./scripts/run-mutants.sh --timeout 120 --jobs 2
```

`scripts/run-mutants.sh` runs `strixonomy-owl` patch mutants with
`--test-package strixonomy-owl --test-package strixonomy` so workspace
`tests/owl_patch_oracles.rs` participates.

## Baseline

### `strixonomy-core` / `path_jail.rs` (2026-07-11 / 2026-07-12)

| Metric | First run | After stronger tests |
|--------|-----------|----------------------|
| Mutants tested | 35 | 35 |
| Caught | 24 | **31** |
| Missed | 11 | **4** |
| Miss rate | ~31% | **~11%** |

Remaining misses (non-blocking): `||`/`&&` edges in `resolve_path_in_workspace` / `is_path_within`, `is_path_within_lexical → true`, and one `ensure_extract_path_within` match-arm delete (covered by sibling parent-escape checks).

Critical escape paths covered by unit tests: `ensure_extract_path_within` (`..`, absolute, empty),
symlink escape, sibling prefix trap, multi-root outside reject, `discover_git_repo_root`.

### `strixonomy-owl` / `patch.rs` — critical arms (2026-07-12)

Focused `--examine-re 'is_safe_iri|apply_one_patch'` with oracle package:

| Metric | Value |
|--------|-------|
| Mutants tested | 6 |
| Caught | **6** |
| Missed | **0** |

Full `patch.rs` has ~401 mutants; nightly/manual full runs via `./scripts/run-mutants.sh`.
Success-path catalog oracles live in `tests/owl_patch_oracles.rs`; injection/hygiene
string tests remain in `patch.rs` unit tests.

Artifacts: `mutants.out/` (gitignored locally when present).
