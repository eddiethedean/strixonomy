---
hide:
  - navigation
  - toc
---

<div class="oc-hero">

<div class="oc-hero-badges">
  <span class="oc-badge oc-badge--accent">Latest tagged v0.27.0</span>
  <span class="oc-badge">VS Code</span>
  <span class="oc-badge">CLI · LSP</span>
</div>

<p class="oc-hero-kicker">Latest tagged v0.27.0</p>

<p class="oc-hero-title">Strixonomy</p>

<p class="oc-hero-lead">
Edit OWL, RDF, and OBO ontologies in VS Code—browse, query, reason, and validate—backed by a Rust workspace engine and language server.
</p>

<p class="oc-hero-ctas">
  <a class="oc-hero-cta" href="guides/first-success/">First success (~10 min) →</a>
</p>

<p class="oc-hero-subcta"><a href="guides/day-2/">Your next steps</a> · <a href="install/">Install</a> · <a href="SHIPPED/">What ships today</a> · <a href="known-limitations/">Known limitations</a></p>

<div class="oc-hero-links">
  <a href="ide/feature-tour/">Feature tour</a>
  <a href="ide/">IDE overview</a>
  <a href="guides/versions-and-channels/">Versions &amp; channels</a>
</div>

</div>

<div class="oc-callout" markdown>

**Primary path:** **[First success (~10 min)](guides/first-success.md)** — install the extension, open sample ontologies, browse and edit. No clone required. Then **[Your next steps](guides/day-2.md)**.

Pins follow [`docs/TAGGED_RELEASE`](https://github.com/eddiethedean/strixonomy/blob/main/docs/TAGGED_RELEASE) (**0.27.0**). Channel lag: [Versions & channels](guides/versions-and-channels.md).

**Also:** [Install](install.md) · [Examples](examples/index.md) · [Feature tour](ide/feature-tour.md) · [What ships today](SHIPPED.md)

![Strixonomy product tour](assets/screenshots/product-tour.gif)

<details markdown>
<summary>Formats and names</summary>

Writable: **`.ttl`**, **`.obo`**, **`.owl`/`.rdf`**, **`.owx`** (XML = semantic re-serialize). JSON-LD / TriG / N-Triples stay read-only — [Supported formats](supported-formats.md). Catalog SQL is a subset — [SQL reference](sql-reference.md).

| Name | What |
|------|------|
| **Strixonomy IDE** | VS Code / Cursor extension |
| **Strixonomy engine** | CLI + LSP + crates (`cargo install strixonomy-cli`) |
| **Ontologos** | Bundled reasoner |

[Product identity](guides/product-identity.md)

</details>

</div>

## Choose your path

<div class="grid cards" markdown>

-   :material-microsoft-visual-studio-code:{ .lg .middle } **VS Code extension**

    ---

    Browse, edit Turtle / OBO / RDF/XML / OWL/XML, run queries and the reasoner from the Strixonomy activity bar.

    [:octicons-arrow-right-24: First success tutorial](guides/first-success.md)

-   :material-console:{ .lg .middle } **CI / CLI only**

    ---

    Validate and classify in CI (Linux tarball preferred). `cargo install` is optional and can take 15–30+ minutes — most IDE users never need it.

    [:octicons-arrow-right-24: Install (CI / CLI)](install.md)

-   :material-clipboard-check-outline:{ .lg .middle } **Evaluate adoption**

    ---

    Capability matrix, Protégé comparison, production readiness, and known limits.

    [:octicons-arrow-right-24: What ships today](SHIPPED.md)

</div>

## What ships today

**Latest tagged: v0.27.0.** Full capability matrix: **[What ships today](SHIPPED.md)**. For channel lag (Marketplace vs crates.io vs docs), see [Versions & channels](guides/versions-and-channels.md).

## Quick start

=== "VS Code"

    1. Install the **Strixonomy extension** from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy) or [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy) (for Cursor)
    2. **File → Open Folder…** with **`.ttl` / `.obo` / `.owl` / `.rdf` / `.owx`** (editable) — JSON-LD / TriG / N-Triples are browse/query only ([Supported formats](supported-formats.md))
    3. The **bundled** language server indexes in Restricted Mode — **do not Trust the workspace** unless you set custom `strixonomy.lspPath` or `strixonomy.robotPath`
    4. Open the **Strixonomy** activity bar → browse **Classes** → click an entity

=== "CI / CLI (optional)"

    Most IDE users skip this. Prefer the [Linux x64 release tarball](https://github.com/eddiethedean/strixonomy/releases/tag/v0.27.0) for CI — [CI integration](ci-integration.md).

    macOS/Windows `cargo install` needs Rust **1.88+** and often takes **15–30+ minutes** — [Install CLI](guides/install-cli.md):

    ```bash
    cargo install strixonomy-cli --locked --version 0.27.0
    strixonomy validate /path/to/ontologies
    ```

    From a clone:

    ```bash
    git clone https://github.com/eddiethedean/strixonomy.git
    cd strixonomy
    cargo run -- validate fixtures
    ```

## Documentation map

| I need… | Read |
|---------|------|
| 10-minute tutorial | [First success](guides/first-success.md) |
| Honest limits | [Known limitations](known-limitations.md) |
| Capability matrix | [SHIPPED.md](SHIPPED.md) |
| Protégé comparison | [Protégé vs Strixonomy](guides/protege-decision.md) |
| CLI / CI | [Install](install.md) · [CI integration](ci-integration.md) |
| Embed in Rust | [Rust library guide](guides/rust-library.md) |
| Roadmap (pick the right doc) | [Roadmap hub](roadmap-hub.md) |
| Feature tour | [ide/feature-tour.md](ide/feature-tour.md) |
| Troubleshooting | [troubleshooting.md](troubleshooting.md) |
| Contributing | [contributing.md](contributing.md) |

Release notes: [CHANGELOG on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/CHANGELOG.md)
