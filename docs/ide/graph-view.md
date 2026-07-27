# Graph view

Strixonomy provides **React graph panels** for exploring ontology structure. Graphs are built by **Strixonomy** and delivered over LSP (`strixonomy/getGraph`).

## Open a graph

From the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`):

| Command | Graph |
|---------|-------|
| **Strixonomy: Open Class Graph** | Class hierarchy (`subClassOf` edges) |
| **Strixonomy: Open Property Graph** | Property domain/range relationships |
| **Strixonomy: Open Object Property Graph** | Object property hierarchy (`subPropertyOf`) + domain/range |
| **Strixonomy: Open Data Property Graph** | Data property hierarchy + domain/range |
| **Strixonomy: Open Individual Graph** | Types and object-property assertions around a selected individual |
| **Strixonomy: Open Import Graph** | Ontology import edges |
| **Strixonomy: Open Dependency Graph** | Import closure / dependency walk |
| **Strixonomy: Open Neighborhood Graph** | BFS neighborhood around a selected class |

For **neighborhood** / **individual** graphs, select an entity in the explorer first, then run the matching command.

You can also run **Strixonomy: Open Graph** for a neighborhood around the focused entity.

**From results:** Query Workbench **Open as graph** and Refactor Preview **Show graph** open `query_result` / `refactor_preview` graphs seeded from result IRIs.

## Using the graph panel

- **Pan and zoom** — standard React Flow controls; only visible elements are rendered for large graphs.
- **Click a node** — opens the [Entity Inspector](inspector.md) for that IRI (focus relay syncs selection). Shift-click for multi-select.
- **Context menu** — Inspect, Reveal in hierarchy, Jump to editor, Expand neighborhood.
- **Keyboard** — focus the canvas, then arrow keys to move selection, Enter to inspect, Escape to clear.
- **Graph \| List** — accessible list alternate of the same nodes/edges.
- **Graph mode** — **Asserted**, **Inferred only**, or **Combined** edges (requires a successful reasoner run for inferred/combined).
- **Unsatisfiable overlay** — badges unsatisfiable classes from the last reasoner run.
- **Layout** — **Grid**, **Circle**, or **Stack** node layouts.
- **Search** — dims / filters canvas nodes by label or IRI; **Center match** still available.
- **Filters** — ontology IRI, entity kinds, namespaces, relationship kinds, hide deprecated.
- **Depth** — BFS expansion depth for neighborhood / individual / result graphs.
- **History** — Back / Forward through recent kind/root/depth states; **Fit to view**.

Inferred edges are animated when shown. Run **`Strixonomy: Run Reasoner`** before switching to inferred or combined modes.

If the graph hits server limits, the panel shows a truncation notice. Limits: **2,000 nodes** and **5,000 edges** ([workspace limits](../workspace-limits.md)). Prefer focusing a root or lowering depth.

## Integrators

Call `strixonomy/getGraph` via Strixonomy LSP — see [LSP API](../lsp-api.md) and [Strixonomy LSP](../strixonomy/lsp.md).

There is no standalone `strixonomy graph` CLI command; use the VS Code panel or LSP.

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| Empty graph | Re-index workspace; check ontology has relevant axioms |
| Truncation notice | Narrow filter, reduce depth, or open neighborhood from a specific entity |
| Panel won't open | Check Output → Strixonomy for LSP errors |
