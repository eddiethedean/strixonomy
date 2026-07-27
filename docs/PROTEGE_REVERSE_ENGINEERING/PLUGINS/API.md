# API.md

# Strixonomy Plugin API
## Reverse Engineering Specification for Protégé Plugins and Design Blueprint for Strixonomy

## Purpose

Protégé exposes an extensible Java-based plugin system that allows third parties to contribute views, tabs, reasoners, renderers, menu items, and tools. Strixonomy should preserve this extensibility while replacing the Java-centric architecture with a language-agnostic, versioned plugin platform.

This document defines the conceptual Plugin API for Strixonomy.

---

# Goals

The Plugin API should:

- Support Rust, TypeScript, and Python plugins
- Be stable and versioned
- Isolate plugins for reliability
- Allow discovery of capabilities
- Support hot loading where practical
- Preserve workspace responsiveness

---

# Architecture

```text
+----------------------+
|      Strixonomy        |
+----------------------+
| Plugin Host          |
| Command Registry     |
| Event Bus            |
| Workspace Services   |
| Reasoning Services   |
+----------------------+
          │
  -------------------------
  │          │           │
Rust      TypeScript   Python
Plugins     Plugins    Plugins
```

---

# Plugin Lifecycle

1. Discover plugin
2. Validate manifest
3. Resolve dependencies
4. Load runtime
5. Register contributions
6. Activate
7. Receive events
8. Deactivate
9. Unload

Plugins should never directly manipulate internal state without using published APIs.

---

# Manifest

Every plugin should contain a manifest describing:

- Plugin ID
- Name
- Version
- Author
- License
- Description
- Runtime
- Required API version
- Permissions
- Dependencies
- Contributions

Example:

```json
{
  "id": "com.example.visualizer",
  "name": "Graph Visualizer",
  "version": "1.0.0",
  "runtime": "typescript",
  "apiVersion": "1.0",
  "contributes": {
    "views": ["graph"]
  }
}
```

---

# Contribution Types

Plugins may contribute:

- Commands
- Views
- Editors
- Toolbars
- Menus
- Context menus
- Dialogs
- Preferences
- Reasoners
- Validators
- Graph layouts
- Import providers
- Exporters
- AI tools
- Automation tasks

---

# Core Services

Plugins interact through services such as:

- Workspace Service
- Ontology Service
- Command Service
- Event Service
- Reasoning Service
- Selection Service
- Notification Service
- Storage Service
- Logging Service

Services should be accessed through dependency injection or a stable service locator.

---

# Command API

Commands should define:

- ID
- Title
- Category
- Shortcut
- Enablement rule
- Execute handler

Commands should automatically integrate with:

- Menus
- Toolbars
- Context menus
- Command palette

---

# Event Bus

Representative events:

- WorkspaceOpened
- OntologyChanged
- SelectionChanged
- ReasonerCompleted
- ValidationCompleted
- CommandExecuted
- PluginActivated

Plugins subscribe without tightly coupling to UI components.

---

# Views

Plugins may register:

- Dockable panels
- Inspector views
- Graph views
- Dashboards
- Diagnostics

Views should support:

- Persistence
- Activation
- Theming
- Accessibility

---

# Editors

Plugins may contribute specialized editors for:

- Entity types
- Metadata
- Domain-specific ontologies
- Visualization-backed editing

---

# Validation

Plugins may add validation rules.

Each rule should produce:

- Severity
- Message
- Entity reference
- Optional quick fix

---

# Reasoners

Reasoner plugins should implement a common interface including:

- initialize()
- classify()
- checkConsistency()
- explain()
- cancel()
- dispose()

Capabilities should be advertised via feature flags.

---

# Security

Plugins should request permissions for:

- File access
- Network access
- External processes
- AI providers
- Workspace modification

Users should approve elevated permissions.

---

# Versioning

The API should follow semantic versioning.

Compatibility policy:

- Patch: backward compatible
- Minor: additive
- Major: breaking changes

Plugins should declare supported API ranges.

---

# Testing

Plugin SDK should support:

- Unit testing
- Integration testing
- Mock workspace
- Mock ontology
- Mock event bus

---

# Accessibility

Plugins must:

- Expose accessible labels
- Support keyboard navigation
- Respect themes
- Avoid color-only communication

---

# Strixonomy Modernization

Recommended enhancements:

- Marketplace
- Plugin signing
- Sandboxed execution
- Hot reload during development
- Telemetry opt-in
- Rich diagnostics
- Cross-language SDKs
- WASM plugin support

---

# Feature Parity Checklist

Platform

- [ ] Plugin discovery
- [ ] Activation
- [ ] Deactivation
- [ ] Dependency resolution

Contributions

- [ ] Commands
- [ ] Views
- [ ] Menus
- [ ] Editors
- [ ] Validators
- [ ] Reasoners

Infrastructure

- [ ] Event bus
- [ ] Services
- [ ] Permissions
- [ ] Versioning
- [ ] Testing support

---

# Summary

Protégé demonstrated the value of an extensible plugin ecosystem, but its Java-centric architecture limits accessibility and modernization. Strixonomy should provide a stable, language-agnostic plugin API that enables Rust, TypeScript, and Python extensions while integrating seamlessly with commands, views, reasoning, validation, AI, and collaboration features.
