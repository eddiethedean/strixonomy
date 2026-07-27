# Architecture

## High-Level Design

```text
                Strixonomy
                    │
 ┌──────────────────┼──────────────────┐
 │                  │                  │
Workspace      Command System     Plugin Host
 │                  │                  │
 ├──────────────┬───┴───────┬──────────┤
 │              │           │
Editors      Reasoning   Visualization
 │              │           │
 └──────────────┼───────────┘
                │
          Workspace Event Bus
                │
     Persistence / Collaboration
```

## Major Subsystems

### Workspace
Coordinates layout, projects, selection, and lifecycle.

### Editors
Specialized editing experiences for ontology entities.

### Reasoning
Pluggable engines (HermiT, ELK, Pellet, future Rust engines).

### Visualization
Graph-native exploration and editing synchronized with the workspace.

### Plugin Platform
Language-agnostic extension model supporting Rust, TypeScript, Python, and future WASM plugins.

### Collaboration
Shared workspaces, reviews, comments, semantic history, and Git integration.

### AI
Optional services for explanations, documentation, refactoring, and ontology authoring.

## Architectural Principles

- Event-driven communication
- Service-oriented platform
- Pluggable capabilities
- Workspace-first design
- Strong accessibility
- Offline-capable architecture
- Cross-platform desktop and web clients

## Relationship to Protégé

Strixonomy should maintain compatibility with OWL standards and established ontology engineering workflows while replacing legacy architectural constraints with a modern, modular platform.
