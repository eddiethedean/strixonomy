# P0 accessibility audit inventory (v0.25 / EPIC-010)

Lightweight gap tags for Strixonomy-owned surfaces. Host chrome (Explorer, Command Palette, Settings) is **N/A — VS Code provided**.

| Surface | Owner | Keyboard | Focus | SR | Contrast/motion | Notes |
|---------|-------|----------|-------|----|-----------------|-------|
| Entity Inspector | webview | gap→fix | gap→fix | gap→fix | shared CSS | Landmarks + entity live region |
| Query Workbench | webview | gap→fix | ok | gap→fix | shared CSS | Result/error live regions; labelled Run |
| Reasoner + Explanation | webview | partial | ok | partial→fix | badges text | Status `aria-live`; profile already labelled |
| Graph (canvas + list) | webview | shipped | ok | partial→fix | reduced-motion | EPIC-008 list alt; selection announce |
| Refactor Preview | webview | gap→fix | ok | gap→fix | shared CSS | Summary live region |
| Imports | webview | gap→fix | ok | gap→fix | shared CSS | Landmark + errors |
| Manchester editor | webview | gap→fix | ok | gap→fix | shared CSS | Editor labelled |
| Rule Browser / Editor | webview | gap→fix | ok | gap→fix | shared CSS | Landmarks |
| Semantic Diff | webview | gap→fix | ok | gap→fix | shared CSS | Landmark |
| New Ontology / Prefix dialogs | DialogShell | gap→fix | trap/restore | fix labelledby | shared CSS | Focus trap in EPIC-010 |
| Metrics / About | webview | ok | ok | landmark | shared CSS | Low interaction |
| VS Code Explorer / Palette / Settings | host | N/A | N/A | N/A | N/A | Rely on VS Code a11y |
| Plugin management | host + plugins list | N/A | N/A | N/A | N/A | Commands via VS Code palette |

Evidence report: [ACCESSIBILITY_REPORT.md](ACCESSIBILITY_REPORT.md).
