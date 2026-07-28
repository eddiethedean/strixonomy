# Graph architecture

> **Status:** Partial (v0.12) — GraphPanel shipped; focus sync and progressive loading planned

## Scope

Graph Workspace data flow: LSP graph payload → layout → render → Current Focus sync.

## Layers

```text
GraphData (LSP strixonomy/getGraph)
    ↓
GraphModel (nodes, edges, inferred flags)
    ↓
LayoutEngine (focus neighborhood first)
    ↓
SceneGraph / Renderer (Canvas/SVG)
    ↓
Interaction (pan, zoom, select → FocusChanged)
```

## Current implementation

- `extension/webview-ui/src/panels/GraphPanel.tsx`
- LSP `strixonomy/getGraph` with graph kind (class, property, import, neighborhood)
- Inferred edge toggle via hierarchy mode (host command)

## Data model

```ts
interface GraphNode {
  id: string
  ref: SemanticRef
  position?: Point
  badges: Badge[]
  collapsed?: boolean
}

interface GraphEdge {
  id: string
  source: string
  target: string
  kind: EdgeKind
  inferred?: boolean
}
```

## Planned (v0.13–v0.30)

- Progressive loading: focus neighborhood first, expand on demand
- Saved layouts and filters ([ui/GRAPH_WORKSPACE.md](../ui/GRAPH_WORKSPACE.md))
- Reasoning overlays on edges
- Graph state in WorkspaceStore; selection updates Current Focus

## Links

- [ui/GRAPH_RENDERING_ARCHITECTURE.md](../ui/GRAPH_RENDERING_ARCHITECTURE.md)
- [ui/GRAPH_WORKSPACE.md](../ui/GRAPH_WORKSPACE.md)
- [cursor-prompts/07-improve-graph-workspace.md](../cursor-prompts/07-improve-graph-workspace.md)

## Evolution

Rendering detail in [ui/GRAPH_RENDERING_ARCHITECTURE.md](../ui/GRAPH_RENDERING_ARCHITECTURE.md); this doc ties graph to workspace runtime.
