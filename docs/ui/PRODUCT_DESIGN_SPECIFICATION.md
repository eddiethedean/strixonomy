# Product Design Specification

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


## 1. Product Vision

Strixonomy is a semantic engineering environment for ontology and knowledge graph development. It should feel like a modern IDE, not a legacy ontology editor.

The long-term vision is to create the JetBrains/Figma/Cursor equivalent for semantic engineering:

- JetBrains-level navigation, refactoring, and diagnostics.
- Figma-level graph/canvas interaction.
- Cursor-level AI assistance.
- GitHub-level semantic collaboration.
- DataGrip-level querying.
- Rust-level performance.

## 2. Product Principles

### 2.1 Context Over Windows

Every view derives from the current semantic context. Selecting an entity updates the editor, inspector, graph, reasoning, documentation, references, and AI suggestions.

### 2.2 Workflows Over Panels

Users do not think in "class tree", "graph", or "reasoner" panels. They think in tasks:

- Understand this entity.
- Fix this diagnostic.
- Refactor this model.
- Review this change.
- Explain this inference.
- Publish this documentation.

### 2.3 Workspace Over Files

Strixonomy treats an ontology repository as a semantic workspace. Files are implementation details. Users navigate entities, relationships, queries, diagnostics, modules, and documentation.

### 2.4 AI as Collaborator

AI is embedded in every workflow. It explains, reviews, repairs, documents, refactors, and teaches, but never applies ontology changes without preview and approval.

### 2.5 Safe Transformation

Every significant change is previewable, undoable, and reasoning-aware.

## 3. User Personas

### 3.1 Ontology Engineer

Needs efficient editing, reasoning, refactoring, navigation, and review.

### 3.2 Domain Expert

Needs simplified semantic views, documentation, diagrams, comments, and guided workflows.

### 3.3 Data Engineer

Needs SQL/SPARQL querying, exports, validation, and integration with pipelines.

### 3.4 Researcher

Needs graph exploration, explanations, documentation, and provenance.

### 3.5 Platform Developer

Needs plugin APIs, SDKs, diagnostics, tests, and stable contracts.

## 4. Core Workspaces

### 4.1 Entity Workspace

The main semantic object editor. Shows overview, hierarchy, relationships, constraints, annotations, documentation, history, references, reasoning, and AI.

### 4.2 Graph Workspace

Interactive semantic canvas. Supports persistent layouts, semantic overlays, reasoning overlays, grouping, AI exploration, saved views, and presentation mode.

### 4.3 Query Workspace

DataGrip-like environment for OntoSQL, SPARQL, SHACL, and future query languages.

### 4.4 Reasoning Workspace

Compiler-like health dashboard with build pipeline, diagnostics, explanations, quick fixes, and reasoning history.

### 4.5 Review Workspace

Semantic pull requests, semantic diffs, review threads, approvals, merge checks, and AI review.

### 4.6 Documentation Workspace

Author, preview, validate, publish, and generate documentation.

### 4.7 AI Workspace

Long-running AI workflows for project-wide documentation, review, refactoring, onboarding, and architecture analysis.

## 5. Global Application Shell

```text
+--------------------------------------------------------------------------------+
| Menu | Search / Command Palette | Workspace Switcher | AI | Git | User         |
+--------------------------------------------------------------------------------+
| Explorer      |                  Active Workspace                  | Inspector |
|               |                                                    |           |
|               |                                                    |           |
+--------------------------------------------------------------------------------+
| Problems | Query | Graph | AI | Git | Output | Terminal | Notifications       |
+--------------------------------------------------------------------------------+
```

## 6. Navigation Model

- Universal search is the primary navigation surface.
- Breadcrumbs provide location awareness.
- Back/forward history is semantic, not file-based.
- Favorites and recent entities support deep ontology workflows.
- Jump-to-definition and find-references work across ontology formats.

## 7. Quality Bar

The application should feel:

- Fast
- Calm
- Professional
- Discoverable
- Accessible
- AI-native
- Safe for large-scale modeling

## 8. Non-Goals

- Do not clone Protégé UI patterns blindly.
- Do not make every feature a separate panel.
- Do not require users to understand serialization formats before they can be productive.
- Do not let AI apply hidden changes.
- Do not expose raw parser or reasoner errors without interpretation.
