# CI integration

Run Strixonomy validation in continuous integration to catch ontology lint and parse errors before merge.

## Exit codes

`strixonomy validate` follows the rules in [workspace-limits.md](workspace-limits.md):

| Outcome | Exit code |
|---------|-----------|
| No diagnostic **errors** (warnings/info allowed) | **0** |
| One or more diagnostic **errors** | **non-zero** |

Warnings and info are printed to stderr but do not fail the job.

## What is safe to automate (pre-1.0)

See [Automation and stability](automation-stability.md) for what is stable enough for CI gating and how to pin versions.

## GitHub Actions (release binary — recommended for Linux CI)

**Fastest path:** download the prebuilt Linux x64 CLI from GitHub Releases. No Rust compile step.

```yaml
name: Ontology validation

on:
  push:
    branches: [main]
  pull_request:

jobs:
  validate-ontologies:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Download strixonomy CLI, verify checksum, validate
        run: |
          VERSION=0.28.0
          ASSET="strixonomy-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
          BIN="strixonomy-v${VERSION}-x86_64-unknown-linux-gnu"
          curl -fsSL -o "${ASSET}" \
            "https://github.com/eddiethedean/strixonomy/releases/download/v${VERSION}/${ASSET}"
          curl -fsSL -o SHA256SUMS \
            "https://github.com/eddiethedean/strixonomy/releases/download/v${VERSION}/SHA256SUMS"
          grep "${ASSET}" SHA256SUMS | sha256sum -c -
          tar xzf "${ASSET}"
          chmod +x "${BIN}"
          ./"${BIN}" validate .
```

Adjust the validate path (`.` or `ontologies/`) to the directory containing your `.ttl`, `.owl`, etc. Verify checksums per [release-integrity.md](release-integrity.md) in production pipelines.

!!! tip "Pin the CLI version pre-1.0"
    Set `VERSION=0.28.0` explicitly so CI does not pick up breaking minor releases unexpectedly. See [FAQ](faq.md).

## GitHub Actions (cargo install — macOS/Windows or when building from source)

Use when you need a platform without a release tarball, or when developing against a git branch:

```yaml
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo registry
        uses: Swatinem/rust-cache@v2
        with:
          workspaces: ""

      - name: Install strixonomy CLI
        run: cargo install strixonomy-cli --locked --version 0.28.0

      - name: Validate ontology files
        run: strixonomy validate .
```

First install on a cold runner can take **15–30+ minutes** without cache. Prefer the release binary on Linux x64 when possible.

## Classify in CI

Fail the job when EL classification finds unsatisfiable classes:

=== "Linux x64 (release binary)"

    ```yaml
          - name: Classify ontologies (EL)
            run: |
              VERSION=0.28.0
              BIN="strixonomy-v${VERSION}-x86_64-unknown-linux-gnu"
              curl -fsSL -o "${BIN}.tar.gz" \
                "https://github.com/eddiethedean/strixonomy/releases/download/v${VERSION}/strixonomy-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
              tar xzf "${BIN}.tar.gz"
              chmod +x "${BIN}"
              ./"${BIN}" classify . --profile el --format json
    ```

=== "cargo install"

    ```yaml
          - name: Install strixonomy CLI
            run: cargo install strixonomy-cli --locked --version 0.28.0

          - name: Classify ontologies (EL)
            run: strixonomy classify . --profile el --format json
    ```

`classify` exits **non-zero** when `consistent` is false. See [workspace-limits.md](workspace-limits.md) and [Reasoner guide](guides/reasoner.md).

## Query diagnostics in CI

```bash
strixonomy query . "SELECT code, severity, message, file FROM diagnostics WHERE severity = 'error'"
```

Use `--format json` for machine-readable output.

## Patch automation

Apply Turtle patches in CI (preview first):

```bash
strixonomy patch ./ontology.ttl patches.json --preview
strixonomy patch ./ontology.ttl patches.json
strixonomy validate .
```

Patch format: [patch-reference.md](patch-reference.md).

## Semantic diff in CI

Compare git refs in pull requests (requires a git checkout with history):

```yaml
      - name: Semantic diff (breaking changes only)
        run: |
          VERSION=0.28.0
          BIN="strixonomy-v${VERSION}-x86_64-unknown-linux-gnu"
          curl -fsSL -o "${BIN}.tar.gz" \
            "https://github.com/eddiethedean/strixonomy/releases/download/v${VERSION}/strixonomy-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
          tar xzf "${BIN}.tar.gz"
          chmod +x "${BIN}"
          ./"${BIN}" diff --format markdown --breaking-only HEAD..WORKTREE
```

See [Semantic diff guide](ide/semantic-diff.md) for ref syntax and VS Code panel usage.

## Tips

- Index only the ontology subtree if the repo is large: `strixonomy validate ./src/ontologies`
- Resource limits apply — see [workspace-limits.md](workspace-limits.md)
- For VS Code-only workflows, the same rules apply via `strixonomy validate` in CI; the extension is not required in the pipeline
