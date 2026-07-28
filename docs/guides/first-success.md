# First success: install, browse, and edit (~10 min)

This is the **canonical tutorial** for new Strixonomy users. You do not need to clone this repository.

**Prerequisites:** VS Code **1.85+**; network access to download tutorial files (step 2). New to OWL/RDF? Skim [Ontology concepts](../concepts.md).

!!! warning "Write-back formats"
    Inspector write-back: **`.ttl`, `.obo`, `.owl`/`.rdf`, `.owx`**. Details: [Supported formats](../supported-formats.md).

## Core path (~10 minutes)

### 1. Install Strixonomy

**VS Code:** Extensions → search **Strixonomy** → **Install** → reload if prompted. Extension id: `strixonomy.strixonomy`.

**Cursor:** install from [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy).

Coming from **OntoCode**? See [v0.27 migration](../migration/v0.27.md). Offline VSIX: [Install VS Code](../vscode-install.md).

### 2. Open a folder

Download samples, then **File → Open Folder…** (open the **folder**, not a single file).

=== "Offline zip (any OS)"

    Download [`strixonomy-tutorial.zip`](https://github.com/eddiethedean/strixonomy/releases/download/v0.27.0/strixonomy-tutorial.zip) from the [v0.27.0 Release](https://github.com/eddiethedean/strixonomy/releases/tag/v0.27.0), unzip, and open that folder.

=== "macOS / Linux (curl)"

    ```bash
    mkdir strixonomy-tutorial && cd strixonomy-tutorial
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/strixonomy/v0.27.0/fixtures/example.ttl
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/strixonomy/v0.27.0/fixtures/complex-classes.ttl
    curl -fsSLO https://raw.githubusercontent.com/eddiethedean/strixonomy/v0.27.0/examples/obo-workflow/demo.obo
    ```

=== "Windows (PowerShell)"

    ```powershell
    mkdir strixonomy-tutorial; cd strixonomy-tutorial
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/strixonomy/v0.27.0/fixtures/example.ttl -OutFile example.ttl
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/strixonomy/v0.27.0/fixtures/complex-classes.ttl -OutFile complex-classes.ttl
    Invoke-WebRequest -Uri https://raw.githubusercontent.com/eddiethedean/strixonomy/v0.27.0/examples/obo-workflow/demo.obo -OutFile demo.obo
    ```

!!! tip "Restricted Mode works out of the box"
    The **bundled** language server indexes ontologies without trusting the workspace. **Trust the folder** only if you configured `strixonomy.lspPath` or `strixonomy.robotPath` — those settings are ignored in Restricted Mode.

### 3. Browse the explorer

![Strixonomy explorer with Classes and Entity Inspector](../assets/screenshots/explorer-inspector.png)

1. Click the **Strixonomy** Activity Bar icon on the **left** sidebar (same strip as Explorer / Search — owl / book-style product icon). Or Command Palette → **Strixonomy: Index Workspace**.
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
| Custom LSP path ignored | **Trust the workspace** (Restricted Mode ignores `strixonomy.lspPath`) |

Full help: [Troubleshooting](../troubleshooting.md) · [FAQ](../faq.md).
