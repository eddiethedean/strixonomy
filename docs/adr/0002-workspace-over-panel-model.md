# ADR-0002 — Workspace model over panel model

## Status

Accepted — **implemented v0.13**

## Context

Today Strixonomy webviews are **panels** (inspector, graph, query workbench) with separate state and LSP call patterns. Users experience disconnected tools rather than one semantic workspace ([ui/WORKSPACE_MODEL.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/WORKSPACE_MODEL.md)).

## Decision

Replace the panel-centric model with **Workspaces** (product surfaces): Entity, Graph, Query, Reasoning, Refactoring, etc. Registered in **WorkspaceRegistry**; routed from a single OntoUI shell instead of independent `?panel=` islands long term.

Panels remain as deployment mechanism (VS Code webviews) until host supports embedded layout.

## Consequences

**Positive:** Unified navigation, shared Current Focus, one event bus.

**Negative:** Migration effort; App.tsx and extension host routing must change incrementally.

## References

- [platform/WORKSPACE_RUNTIME.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/WORKSPACE_RUNTIME.md)
- [adr/0004-workspacestore-ui-source-of-truth.md](0004-workspacestore-ui-source-of-truth.md)
