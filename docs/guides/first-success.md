# First success: install, browse, and edit (~10 min)

This is the **canonical tutorial** for new Strixonomy users. You do not need to clone this repository.

**Prerequisites:** VS Code **1.85+**; network access to download tutorial files (step 2). New to OWL/RDF? Skim [Ontology concepts](../concepts.md).

!!! warning "Write-back formats"
    Inspector write-back: **`.ttl`, `.obo`, `.owl`/`.rdf`, `.owx`**. XML is **semantic re-serialize** (not byte-identical to Protégé). JSON-LD / TriG / N-Triples are read-only — [Supported formats](../supported-formats.md).

## Core path (~10 minutes)

### 1. Install Strixonomy

**VS Code:** Extensions → search **Strixonomy** (`strixonomy.strixonomy`) → **Install** → reload if prompted.

**Cursor:** install from [Open VSX](https://open-vsx.org/extension/ontocode/ontocode).

For offline VSIX installs, see [Install VS Code](../vscode-install.md).

### 2. Open a folder

**Canonical (online):** download the three sample files below, then **File → Open Folder…**.

=== "macOS / Linux"

    ```bash
    mkdir strixonomy-tutorial && cd strixonomy-tutorial
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/strixonomy/v0.26.2/fixtures/example.ttl
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/strixonomy/v0.26.2/fixtures/complex-classes.ttl
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/strixonomy/v0.26.2/examples/obo-workflow/demo.obo
    ```

=== "Windows (PowerShell)"

    ```powershell
    mkdir strixonomy-tutorial; cd strixonomy-tutorial
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/strixonomy/v0.26.2/fixtures/example.ttl -OutFile example.ttl
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/strixonomy/v0.26.2/fixtures/complex-classes.ttl -OutFile complex-classes.ttl
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/strixonomy/v0.26.2/examples/obo-workflow/demo.obo -OutFile demo.obo
    ```

**Offline pack:** Prefer the curl/PowerShell commands above when you have network access. For offline sample files, download [`ontocode-tutorial.zip`](https://github.com/eddiethedean/strixonomy/releases/download/v0.26.2/ontocode-tutorial.zip) from the [v0.26.2 GitHub Release](https://github.com/eddiethedean/strixonomy/releases/tag/v0.26.2), unzip, and **File → Open Folder…**. For offline extension install, download `ontocode-v0.26.2.vsix` from that Release. Alternatively open `fixtures/` from a clone.

Or browse [v0.26.2 fixtures](https://github.com/eddiethedean/strixonomy/tree/v0.26.2/fixtures).

!!! tip "Workspace Trust"
    The **bundled** language server works in Restricted Mode. **Do not Trust the workspace** unless you configured `strixonomy.lspPath` or `strixonomy.robotPath`.

### 3. Browse the explorer

![Strixonomy explorer with Classes and Entity Inspector](../assets/screenshots/explorer-inspector.png)

1. Click the **Strixonomy** Activity Bar icon (ontology / book-like icon on the left sidebar; or run **Strixonomy: Index Workspace**).
2. Expand **Ontologies**, then **Classes** / **Properties** / **Individuals**.
3. Click **`Person`** to open the **Entity Inspector**.

!!! success "Success looks like"
    - **Classes** contains `Person` (from `example.ttl`).
    - **Ontologies** lists `example.ttl`, `complex-classes.ttl`, and `demo.obo` with no parse errors.

If trees stay empty: run **Strixonomy: Index Workspace**, then check **View → Output → Strixonomy Language Server**.

### 4. Edit a Turtle entity

1. With `Person` selected, in the Inspector **Edit** section change a **label** or **comment**, or add a **parent**.
2. Confirm `example.ttl` updates on disk.

**You are done with the core path.** Optional follow-ups below.

---

## Explore next (optional)

| Next | Link |
|------|------|
| **Your next steps (day 2)** | Edit → query → reason → CI — [Your next steps](day-2.md) |
| Query Workbench | Run `SELECT short_name FROM classes` — [Query Workbench](../ide/query-workbench.md) |
| Manchester axioms | [Manchester editor](../ide/manchester-editor.md) |
| Reason / realize / SWRL | [Reasoner](reasoner.md) · [Realize](../examples/realize.md) · [SWRL](../examples/swrl.md) |
| Refactor / graphs / OBO / XML | [Feature tour](../ide/feature-tour.md) · [OBO authoring](../ide/obo-authoring.md) · [OWL/XML write-back](owl-xml-workflow.md) |
| CLI / CI (optional) | [Install](../install.md) · [CI integration](../ci-integration.md) |
| Fit check | [Known limitations](../known-limitations.md) · [What ships today](../SHIPPED.md) |

## Troubleshooting

| Symptom | Fix |
|---------|-----|
| Empty explorer | Index Workspace; Output → Strixonomy Language Server |
| Cannot edit | Confirm writable format (`.ttl`/`.obo`/`.owl`/`.rdf`/`.owx`); see [Supported formats](../supported-formats.md) |
| Custom LSP path ignored | Trust the workspace |

Full help: [Troubleshooting](../troubleshooting.md) · [FAQ](../faq.md).
