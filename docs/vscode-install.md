> **Canonical install:** [Install](install.md) (SSOT). This page covers VS Code–specific options (Marketplace, VSIX, Restricted Mode, custom `lspPath`). Artifact filenames: [Versions & channels — air-gap manifest](guides/versions-and-channels.md#air-gap-artifact-manifest).

# Installing Strixonomy in VS Code

[![Open VSX](https://img.shields.io/open-vsx/v/strixonomy/strixonomy)](https://open-vsx.org/extension/strixonomy/strixonomy)
[![VS Code Marketplace](https://badgen.net/vs-marketplace/v/strixonomy.strixonomy?label=VS%20Code%20Marketplace)](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy)

**Primary path:** [Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy) (or [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy) for Cursor) → [First success (~10 min)](guides/first-success.md).

> **Multi-root workspaces (v0.10+):** All workspace folders are indexed on open. **Strixonomy: Index Workspace** may prompt you to pick a folder when multiple roots are open.

## Prerequisites

- **VS Code 1.85+** (see [platform compatibility](guides/platform-compatibility.md))
- Strixonomy’s **bundled** language server runs in trusted and **Restricted Mode** without extra setup. **Trust the workspace** only when you configured `strixonomy.lspPath` or `strixonomy.robotPath` — those settings are ignored when the folder is untrusted.

## Install matrix

| Method | Linux | macOS | Windows | Needs Rust? |
|--------|-------|-------|---------|-------------|
| Marketplace extension (bundled language server) | Yes | Yes | Yes | No |
| Open VSX / Cursor marketplace (v0.11+) | Yes | Yes | Yes | No |
| Release VSIX (bundled language server) | Yes | Yes | Yes | No |
| `cargo install strixonomy-lsp` + `strixonomy.lspPath` | Yes | Yes | Yes | Yes (1.88+) |
| Build from source (`package-extension.sh`) | Yes | Yes | Yes | Yes + Node 20 |

CLI install options (separate from the extension): [Install](install.md) · [Install CLI & CI (detail)](install-cli-ci.md).

## Option A — VS Code Marketplace (recommended)

1. Install [Strixonomy from the Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy) (or [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy) for Cursor). If the store lags the latest GitHub tag, use the VSIX under Option B — see [Versions and channels](guides/versions-and-channels.md).
2. **File → Open Folder…** and choose a directory with ontology files.
3. The bundled language server works in **Restricted Mode** — Trust is optional. **Trust the folder** only if you need custom `strixonomy.lspPath` / `strixonomy.robotPath`.
4. Open the **Strixonomy** activity bar and browse ontologies, classes, properties, individuals, and **Diagnostics**.

For a full walkthrough, see [First success in 10 minutes](guides/first-success.md).

**Sample ontology files:** prefer the curl/PowerShell commands in [First success](guides/first-success.md). Offline: download [`strixonomy-tutorial.zip`](https://github.com/eddiethedean/strixonomy/releases/download/v0.28.0/strixonomy-tutorial.zip) from the [v0.28.0 GitHub Release](https://github.com/eddiethedean/strixonomy/releases/tag/v0.28.0), unzip, and open that folder — or open `fixtures/` from a clone.

## Option B — GitHub Release VSIX (offline / air-gapped)

1. Open [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases) and download the latest `strixonomy-v*.vsix` (example: `strixonomy-v0.28.0.vsix`).
2. In VS Code: **Extensions** → **…** menu → **Install from VSIX…**
3. Verify against `SHA256SUMS` — see [release-integrity.md](release-integrity.md).
4. Open a folder containing ontology files (`.ttl`, `.obo`, `.owl`, `.rdf`, `.owx` editable; `.jsonld`, `.nt`, `.nq`, `.trig` browse/query only).

Release VSIX packages bundle `strixonomy-lsp` for Linux, macOS, and Windows.

## Option C — Build from source

**Prerequisites:** Rust **1.88+**, Node **20+**, npm (see [contributing.md](contributing.md)).

From the repository root:

```bash
./scripts/package-extension.sh   # builds LSP + compiles extension (does not emit VSIX)
cd extension && npx vsce package --no-dependencies
```

Install the generated `.vsix` via **Install from VSIX…**, or press **F5** in VS Code with the `extension/` folder open (Run Extension).

## Option D — Language server on PATH

Use this only when the bundled server is missing or you are developing the LSP:

```bash
cargo install strixonomy-lsp --locked --version 0.28.0
```

Set **Strixonomy: Lsp Path** (`strixonomy.lspPath`) to the absolute path of your `strixonomy-lsp` binary. **Trusted workspaces only** — ignored in Restricted Mode.

## Option E — Cursor / Open VSX (v0.11+)

[![Open VSX](https://img.shields.io/open-vsx/v/strixonomy/strixonomy)](https://open-vsx.org/extension/strixonomy/strixonomy)

[Cursor](https://cursor.com/) uses the [Open VSX](https://open-vsx.org/) registry instead of the Microsoft VS Code Marketplace.

1. Open **Extensions** in Cursor and search for **Strixonomy** (publisher `strixonomy`).
2. Install **Strixonomy** (`strixonomy.strixonomy`).
3. Open a folder with ontology files. The bundled language server works in **Restricted Mode** — **Trust** only for custom `strixonomy.lspPath` or `strixonomy.robotPath`.

If Strixonomy does not appear in search (before v0.11 or if Open VSX sync is delayed):

1. Download `strixonomy-v*.vsix` (example: `strixonomy-v0.28.0.vsix`) from [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases).
2. **Cmd+Shift+P** / **Ctrl+Shift+P** → **Extensions: Install from VSIX…**

Release tags from v0.11.3 onward publish automatically to Open VSX.

## Using the sidebar

After indexing, the **Strixonomy** activity bar shows five views:

| View | What you see |
|------|----------------|
| **Ontologies** | Indexed files and parse status |
| **Classes** | Class hierarchy |
| **Properties** | Object, data, and annotation properties |
| **Individuals** | Named individuals |
| **Diagnostics** | Lint issues; click to open source |

**Refresh** — click ↻ on a view title, or run **Strixonomy: Refresh Explorer**.

**Re-index** — run **Strixonomy: Index Workspace** after adding or changing ontology files.

Click an entity name to open the **Entity Inspector**. For **`.ttl`, `.obo`, `.owl`/`.rdf`, and `.owx`**, use the edit section to change labels, parents, or delete entities (XML is semantic re-serialize). See [authoring.md](authoring.md) and [OWL/XML write-back](guides/owl-xml-workflow.md).

## Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `strixonomy.lspPath` | `""` | **Trusted workspaces only.** Path to `strixonomy-lsp`; ignored in Restricted Mode. Empty uses bundled binary |
| `strixonomy.robotPath` | `""` | **Trusted workspaces only.** Path to ROBOT JAR or launcher; ignored in Restricted Mode |
| `strixonomy.indexCache` | `false` | Persist parse snapshots under `.strixonomy/cache/` (add to `.gitignore`) |
| `strixonomy.queryHistoryLimit` | `20` | Max entries in Query Workbench history |
| `strixonomy.reasoner.default` | `el` | Default profile for Run Reasoner (`el`, `rl`, `rdfs`, `dl`, `auto`) |
| `strixonomy.reasoner.autoProfile` | `true` | Profile-detection warnings when running reasoner |
| `strixonomy.hierarchy.mode` | `asserted` | Explorer hierarchy: `asserted`, `inferred`, or `combined` |
| `strixonomy.diagnostics.rules` | `{}` | Per-rule severity overrides (see [diagnostics config](workspace-limits.md)) |

Indexing runs on workspace open. `strixonomy.autoIndexOnOpen` is a legacy setting (no-op); kept for compatibility.

## Commands

- **Strixonomy: Index Workspace** — rebuild catalog
- **Strixonomy: Refresh Explorer** — refresh tree views (including diagnostics)
- **Strixonomy: Open Query Workbench** — SQL and SPARQL against indexed workspace ([guide](ide/query-workbench.md))
- **Strixonomy: Open Manchester Editor** / **Add Manchester Axiom** — complex class expressions ([guide](ide/manchester-editor.md))
- **Strixonomy: Run Reasoner** — EL / RL / RDFS / DL / auto classification ([guide](guides/reasoner.md))
- **Strixonomy: Show Explanation** — justification for unsatisfiable class
- **Strixonomy: Set Hierarchy Mode** — asserted / inferred / combined class tree
- **Strixonomy: Open Class Graph** / **Property Graph** / **Import Graph** / **Neighborhood Graph** — visualization ([guide](ide/graph-view.md))
- **Strixonomy: Create Class / Property / Individual** — authoring in editable formats (richest on `.ttl`)
- **Problems panel** — inline diagnostics from `strixonomy-lsp` after indexing
- **Strixonomy: Show Entity Inspector** / **Jump to Source** — from explorer context menu

## Troubleshooting

### End users (Marketplace / Open VSX install)

| Symptom | Fix |
|---------|-----|
| Extension does not activate | Open a supported ontology file or the **Strixonomy → Ontologies** view |
| `failed to start language server` | Check **Output → Strixonomy Language Server**; uninstall duplicate Strixonomy versions; if using custom `strixonomy.lspPath`, **Trust** the workspace |
| `spawn ... strixonomy-lsp EACCES` (macOS/Linux) | Upgrade to Strixonomy ≥ 0.4.0. Manual: `chmod +x` on the bundled binary path from the error |
| `couldn't create connection to server` | Check **Output → Strixonomy Language Server**. Reinstall the extension or download a fresh VSIX from [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases) |
| Empty explorer after open | Run **Strixonomy: Index Workspace**; check **Output → Strixonomy Language Server**; Trust only if using custom `lspPath`/`robotPath` |
| Inspector has no edit controls | Entity must be in **`.ttl`, `.obo`, `.owl`/`.rdf`, or `.owx`**; JSON-LD / TriG / N-Triples are read-only — [Supported formats](supported-formats.md) |

### Developers (building from source)

| Symptom | Fix |
|---------|-----|
| Bundled LSP missing in dev host | `cargo build -p strixonomy-lsp --bins` then set `strixonomy.lspPath` or run `./scripts/package-extension.sh` |
| Extension tests fail to spawn LSP | `export STRIXONOMY_LSP_BIN="$(pwd)/target/debug/strixonomy-lsp"` before `npm test` |

See [Debugging guide](debugging.md) for F5, webview-ui, and E2E workflows.

See also [troubleshooting.md](troubleshooting.md), [faq.md](faq.md), and [First success in 10 minutes](guides/first-success.md).
