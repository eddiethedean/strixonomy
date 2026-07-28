---
hide:
  - navigation
  - toc
---

<div class="oc-hero">

<div class="oc-hero-grid">
  <div class="oc-hero-copy">
    <p class="oc-hero-kicker"><span class="oc-status-dot"></span> Open-source ontology engineering</p>
    <p class="oc-hero-title">Make structured knowledge <span>clear.</span></p>
    <p class="oc-hero-lead">Strixonomy brings ontology authoring, queries, validation, visualization, and reasoning into VS Code—powered by a fast Rust workspace engine.</p>
    <div class="oc-hero-actions">
      <a class="md-button md-button--primary" href="guides/first-success/">Get started in 10 minutes</a>
      <a class="md-button" href="ide/feature-tour/">Explore the feature tour</a>
    </div>
    <p class="oc-hero-meta">
      <a href="SHIPPED/">Latest tagged v0.27.0</a><span>·</span>
      <a href="known-limitations/">Known limitations</a><span>·</span>
      <a href="https://github.com/eddiethedean/strixonomy">GitHub</a>
    </p>
  </div>
  <div class="oc-hero-console" aria-label="Strixonomy capability preview">
    <div class="oc-console-bar"><span></span><span></span><span></span><strong>strixonomy · workspace</strong></div>
    <div class="oc-console-body">
      <p><span class="oc-prompt">›</span> index <strong>organization.ttl</strong></p>
      <p><span class="oc-ok">✓</span> 1,284 axioms · 186 entities</p>
      <p><span class="oc-prompt">›</span> classify <strong>--profile auto</strong></p>
      <p><span class="oc-ok">✓</span> consistent · 24 inferred edges</p>
      <div class="oc-console-rule"></div>
      <p class="oc-console-label">WORK WITH</p>
      <div class="oc-format-list"><span>OWL</span><span>RDF</span><span>OBO</span><span>SPARQL</span></div>
    </div>
  </div>
</div>
</div>

<div class="oc-proof-strip">
  <div><strong>4</strong><span>editable ontology formats</span></div>
  <div><strong>3</strong><span>query modes (Catalog SQL subset · SPARQL · DL)</span></div>
  <div><strong>EL–DL</strong><span>reasoning profiles</span></div>
  <div><strong>1.0</strong><span>plugin wire contract</span></div>
</div>

<div class="oc-start-card" markdown>

!!! info "Documentation version"
    **Public install pin:** [`docs/TAGGED_RELEASE`](https://github.com/eddiethedean/strixonomy/blob/main/docs/TAGGED_RELEASE) (**0.27.0** — latest tagged release). GitHub `main` may be **0.28.0** in development — pin tagged artifacts unless you build from source. [Versions & channels](guides/versions-and-channels.md) · [v0.28 migration](migration/v0.28.md).

**New to Strixonomy?** Start with **[First success (~10 min)](guides/first-success.md)**—install the extension, open sample ontologies, browse, and edit. No clone or separate CLI required.

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

<figure class="oc-product-shot">
  <img src="assets/screenshots/product-tour.gif" alt="Strixonomy product tour showing the ontology explorer and entity inspector">
  <figcaption>Browse, edit, query, and reason without leaving your editor.</figcaption>
</figure>

## What ships today

**Latest tagged: v0.27.0.** Full capability matrix: **[What ships today](SHIPPED.md)**. For channel lag (Marketplace vs crates.io vs docs), see [Versions & channels](guides/versions-and-channels.md).

## Quick start

=== "VS Code"

    1. Install the **Strixonomy extension** from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy) or [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy) (for Cursor)
    2. **File → Open Folder…** with **`.ttl` / `.obo` / `.owl` / `.rdf` / `.owx`** (editable) — JSON-LD / TriG / N-Triples are browse/query only ([Supported formats](supported-formats.md))
    3. The **bundled** language server works in **Restricted Mode** — no Trust required. **Trust the workspace** only if you set custom `strixonomy.lspPath` or `strixonomy.robotPath` ([VS Code install](vscode-install.md))
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
| Which artifact? | [Which artifact?](guides/which-artifact.md) |
| 10-minute tutorial | [First success](guides/first-success.md) |
| Honest limits | [Known limitations](known-limitations.md) |
| Capability matrix | [SHIPPED.md](SHIPPED.md) |
| Protégé comparison | [Protégé vs Strixonomy](guides/protege-decision.md) |
| CLI / CI | [Install](install.md) · [CI integration](ci-integration.md) |
| Embed in Rust | [Rust library guide](guides/rust-library.md) |
| Roadmap (summary) | [Roadmap summary](roadmap-summary.md) · [Roadmap hub](roadmap-hub.md) |
| Feature tour | [ide/feature-tour.md](ide/feature-tour.md) |
| Troubleshooting | [troubleshooting.md](troubleshooting.md) |
| Contributing | [contributing.md](contributing.md) |

Release notes: [CHANGELOG on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/CHANGELOG.md)
