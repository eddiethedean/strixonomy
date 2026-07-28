# MENUS.md

# Protégé Menu System Reverse Engineering

## Purpose

The Protégé menu system provides global access to ontology project actions, editing commands, reasoner controls, refactoring tools, plugin commands, window management, and help resources. Unlike tab-specific views, menu actions usually operate at the application, project, ontology, or selected-entity level.

This document describes the functional role of Protégé menus as a reverse-engineering reference for building Strixonomy feature parity and modernization.

---

# High-Level Menu Structure

Protégé Desktop commonly organizes global actions into menus similar to:

```text
File
Edit
Active Ontology
Refactor
Reasoner
Tools
Window
Help
```

Plugin installations may add new top-level menus or add commands under existing menus.

---

# Menu Design Principles

Protégé menus are designed around these assumptions:

1. Ontology editing is project-centered.
2. Most edits apply to the currently active ontology or selected entity.
3. Reasoning is a global workspace operation.
4. Refactoring should be explicit and discoverable.
5. Plugins should be able to contribute commands.
6. Advanced users need access to low-frequency but important actions.

---

# File Menu

## Purpose

The File menu manages ontology projects, ontology files, imports, saving, loading, exporting, and application-level file operations.

## Common Responsibilities

- Create new ontology project
- Open ontology project
- Open recent project
- Save ontology
- Save ontology as
- Save all
- Close project
- Import ontology
- Export ontology
- Reload ontology
- Manage physical ontology documents
- Exit application

## Typical Actions

### New Project / New Ontology

Creates a new Protégé project, usually backed by a new ontology document.

Expected user decisions:

- Ontology IRI
- Version IRI
- Physical file location
- Serialization format
- Prefix configuration

### Open

Loads an existing ontology file or project.

Supported ontology formats may include:

- RDF/XML
- OWL/XML
- Turtle
- Manchester Syntax
- Functional Syntax
- OBO, depending on plugins and import support

### Open Recent

Provides quick access to recently opened ontology projects.

### Save

Writes current ontology changes to the known physical location.

### Save As

Allows the user to choose a new location or serialization format.

### Save All

Saves all dirty ontologies in the current project, including imports when editable.

### Close

Closes the current ontology project.

### Import

Adds an ontology import declaration or loads an external ontology into the project.

### Export

Serializes ontology content to another file or format.

### Exit / Quit

Closes the Protégé application.

## Strixonomy Parity Requirements

- [x] New ontology command
- [x] Open ontology command
- [x] Recent projects list
- [x] Save
- [x] Save as
- [x] Save all
- [x] Close project
- [x] Import ontology
- [x] Export ontology
- [x] Dirty-state tracking
- [x] Unsaved-changes warning
- [x] Serialization format selection
- [x] File recovery support

## Strixonomy Modernization Opportunities

- Git-aware save state
- Workspace-level project files
- Autosave with semantic checkpoints
- Save conflict resolution
- Import source trust indicators
- Format conversion preview
- Cloud and local workspace parity

---

# Edit Menu

## Purpose

The Edit menu exposes generic editing commands and selection-based actions.

## Common Responsibilities

- Undo
- Redo
- Cut
- Copy
- Paste
- Delete
- Select all
- Find
- Replace
- Entity search
- Preferences

## Typical Actions

### Undo

Reverts the most recent ontology edit.

Ontology edits may include:

- Adding a class
- Deleting a property
- Changing an annotation
- Adding an axiom
- Removing an axiom
- Renaming an entity

### Redo

Reapplies an undone ontology edit.

### Cut / Copy / Paste

Used for text fields, entity references, axioms, and sometimes tree selections.

### Delete

Deletes selected entities, axioms, annotations, or text depending on context.

### Find

Searches for entities, labels, IRIs, annotations, or text.

### Preferences

Opens application preferences.

Preference categories may include:

- Rendering
- Reasoner behavior
- Editor behavior
- Plugin settings
- UI layout
- Entity display
- New entity creation policy

## Strixonomy Parity Requirements

- [x] Undo
- [x] Redo
- [x] Clipboard support
- [x] Delete selected object
- [x] Global search
- [x] Preferences dialog
- [x] Context-sensitive enablement
- [x] Text-field editing support
- [x] Entity-aware copy/paste

## Strixonomy Modernization Opportunities

- Command palette integration
- Multi-step undo timeline
- Semantic undo labels
- Keyboard-first editing
- Search across project, imports, Git history, and documentation
- AI-assisted find/replace for ontology patterns

---

# Active Ontology Menu

## Purpose

The Active Ontology menu manages actions that apply specifically to the ontology currently selected as the active editable ontology.

## Common Responsibilities

- Ontology annotations
- Ontology IRI management
- Prefix management
- Import management
- Ontology metrics
- Ontology format settings
- Ontology closure view
- Physical document mapping

## Typical Actions

### Edit Ontology Annotations

Allows the user to add, edit, or remove annotations on the ontology itself.

Examples:

- rdfs:label
- rdfs:comment
- dc:title
- dc:creator
- owl:versionInfo
- license metadata

### Manage Imports

Adds, removes, or reloads imported ontologies.

Import behavior must account for:

- Logical IRI
- Physical IRI
- Missing imports
- Circular imports
- Import closure
- Editable versus read-only imports

### Manage Prefixes

Controls prefix mappings used for compact entity rendering.

Examples:

```text
owl:  http://www.w3.org/2002/07/owl#
rdf:  http://www.w3.org/1999/02/22-rdf-syntax-ns#
rdfs: http://www.w3.org/2000/01/rdf-schema#
xsd:  http://www.w3.org/2001/XMLSchema#
```

### Ontology Metrics

Displays metrics such as:

- Number of classes
- Number of object properties
- Number of data properties
- Number of annotation properties
- Number of individuals
- Number of axioms
- Logical axiom count
- Annotation axiom count
- Imported ontology count

### Set Active Ontology

In multi-ontology projects, determines where new axioms and entities are added.

## Strixonomy Parity Requirements

- [x] Active ontology selector
- [x] Ontology annotation editor
- [x] Prefix editor
- [x] Import manager
- [x] Ontology metrics
- [x] Logical/physical IRI mapping
- [x] Editable import support
- [x] Missing import diagnostics
- [x] Reload imports

## Strixonomy Modernization Opportunities

- Visual import graph
- Import health dashboard
- Dependency lockfile
- Package-manager-style ontology imports
- Prefix conflict detection
- Metadata quality scoring
- AI-suggested ontology annotations

---

# Refactor Menu

## Purpose

The Refactor menu provides higher-level ontology transformation tools that preserve or intentionally alter semantics across multiple axioms.

## Common Responsibilities

- Rename entity
- Move entity
- Merge entities
- Delete entity safely
- Extract module
- Convert entity type
- Replace entity references
- Normalize labels
- Change entity IRIs

## Typical Actions

### Rename Entity

Changes an entity IRI and updates references.

Expected behavior:

- Update all axioms referencing the entity
- Preserve annotations when appropriate
- Warn about imported or read-only references
- Optionally update labels

### Move Entity

Changes an entity's position in a hierarchy.

Examples:

- Move class under a different superclass
- Move property under a different superproperty

### Merge Entities

Combines two or more ontology entities.

Expected behavior:

- Consolidate axioms
- Merge annotations
- Resolve duplicate labels
- Warn about semantic conflicts

### Delete Entity

Deletes an entity and related axioms.

A safe delete should show:

- Direct usages
- Indirect usages
- Referencing axioms
- Imported references
- Impact summary

### Extract Module

Creates a smaller ontology module from selected entities and dependencies.

Potential module strategies:

- Top module
- Bottom module
- Star module
- Signature-based extraction

## Strixonomy Parity Requirements

- [x] Rename entity
- [x] Move entity
- [x] Merge entities
- [x] Safe delete
- [x] Replace entity
- [x] Extract module
- [x] Preview refactor impact
- [x] Undoable refactors
- [x] Refactor conflict warnings

## Strixonomy Modernization Opportunities

- IDE-quality rename previews
- Refactor diff viewer
- Semantic impact analysis
- Git branch refactor workflow
- AI-assisted ontology cleanup
- Bulk refactoring recipes
- Typed refactor APIs for plugins

---

# Reasoner Menu

## Purpose

The Reasoner menu controls ontology classification, consistency checking, reasoner selection, and inference-related operations.

## Common Responsibilities

- Select reasoner
- Start reasoner
- Stop reasoner
- Synchronize reasoner
- Classify ontology
- Check consistency
- Precompute inferences
- Show inferred hierarchy
- Configure reasoner
- Dispose reasoner

## Typical Actions

### Select Reasoner

Chooses the active reasoner implementation.

Common options may include:

- HermiT
- Pellet
- Fact++
- ELK
- Structural reasoner
- Plugin-provided reasoners

### Start Reasoner

Initializes the selected reasoner for the active ontology.

### Synchronize Reasoner

Pushes ontology changes into the reasoner and recomputes inferences.

### Classify

Computes inferred class and property hierarchies.

### Check Consistency

Determines whether the ontology is logically consistent.

### Explain Inconsistency

Opens explanation tooling to identify axioms contributing to inconsistency or unsatisfiable classes.

### Configure Reasoner

Opens reasoner-specific settings.

Settings may include:

- Timeout
- Incremental reasoning
- Fresh entity policy
- Explanation limits
- Precomputation options
- Logging verbosity

## Strixonomy Parity Requirements

- [x] Reasoner selection
- [x] Reasoner lifecycle management
- [x] Synchronize reasoner
- [x] Classify ontology
- [x] Consistency checking
- [x] Unsatisfiable class detection
- [x] Inferred hierarchy views
- [x] Explanation integration
- [x] Reasoner configuration
- [x] Long-running task cancellation (v0.18: client cancel + ignore late results; in-flight classify may still finish on server)

## Strixonomy Modernization Opportunities

- Rust-native reasoner abstraction
- Reasoner progress UI
- Incremental live reasoning
- Background classification
- Explanation graph visualization
- Reasoning performance profiler
- Reasoner comparison mode
- AI explanation summaries

---

# Tools Menu

## Purpose

The Tools menu collects advanced utilities, plugin actions, validation tools, visualization tools, scripting tools, and project-specific commands.

## Common Responsibilities

- Plugin-provided commands
- Ontology validation
- Ontology metrics
- Visualization tools
- Entity reports
- Batch operations
- Import/export helpers
- Scripting or automation

## Typical Actions

### Ontology Metrics

Displays detailed ontology statistics.

### Visualization

Launches graph-style views when plugins are installed.

Examples:

- OntoGraf
- OWLViz
- VOWL-style views

### Validation

Checks ontology for modeling issues beyond formal logical consistency.

Possible validation categories:

- Missing labels
- Duplicate labels
- Orphan classes
- Cycles where unexpected
- Missing domains/ranges
- Unused properties
- Deprecated entity usage

### Plugin Actions

Plugins may contribute arbitrary tool commands.

## Strixonomy Parity Requirements

- [x] Tool command registry
- [x] Plugin-contributed tools
- [x] Metrics command
- [x] Validation command
- [x] Visualization launch commands
- [x] Batch operation support

## Strixonomy Modernization Opportunities

- Scriptable command runner
- Task pipeline automation
- Integrated linting
- Ontology quality dashboards
- Plugin marketplace
- Workspace tasks similar to VS Code tasks
- AI-generated repair suggestions

---

# Window Menu

## Purpose

The Window menu manages workspace layout, tabs, panels, docking state, and view visibility.

## Common Responsibilities

- Open tabs
- Close tabs
- Reset layout
- Show/hide views
- Manage perspectives
- Restore default workspace
- Switch active window

## Typical Actions

### Reset Layout

Restores the default Protégé workspace layout.

### Show View

Displays a hidden dockable view.

### Switch Tab

Moves focus to a major workspace tab.

### Manage Perspectives

Some configurations may allow saved workspace layouts or perspectives.

## Strixonomy Parity Requirements

- [x] Show/hide panels
- [x] Reset layout
- [x] Switch tabs
- [x] Persist layout (v0.18: restore reopen commands + panel context)
- [x] Restore default workspace
- [x] Plugin-contributed views
- [x] Floating and docked panel support

## Strixonomy Modernization Opportunities

- Named workspaces
- Modeling/reasoning/review perspectives
- Keyboard-driven layout switching
- Split editors
- Multi-window support
- Workspace layout sync
- Per-project layout configuration

---

# Help Menu

## Purpose

The Help menu provides access to documentation, tutorials, diagnostics, updates, and application information.

## Common Responsibilities

- Open documentation
- Show getting started material
- Show about dialog
- Show version information
- Plugin information
- Error logs
- Update checks
- Community links

## Typical Actions

### Documentation

Opens official Protégé documentation.

### About

Displays:

- Application version
- Java version
- Build information
- Plugin versions
- License information

### Error Log

Shows application errors, stack traces, or diagnostic logs.

### Plugin Information

Displays installed plugins and versions.

## Strixonomy Parity Requirements

- [x] Documentation links
- [x] About dialog
- [x] Version information
- [x] Plugin list
- [x] Error log viewer
- [x] Diagnostic export
- [x] Support links

## Strixonomy Modernization Opportunities

- Built-in learning mode
- Contextual help side panel
- AI help assistant
- Interactive ontology modeling tutorials
- One-click diagnostic bundle
- Release notes viewer
- Plugin health report

---

# Context Menus

## Purpose

Context menus expose actions relevant to the selected UI object.

They are essential because many Protégé workflows start from hierarchy trees, entity lists, or axiom rows rather than from the top-level menu bar.

## Common Context Menu Locations

- Class hierarchy tree
- Object property tree
- Data property tree
- Annotation property tree
- Individual list
- Axiom list
- Annotation rows
- Import rows
- Search results
- Entity usage views

## Common Context Actions

- Create child class
- Create sibling class
- Rename
- Delete
- Add annotation
- Copy IRI
- Copy short form
- Show usages
- Show in hierarchy
- Move entity
- Add superclass
- Add equivalent class
- Add disjoint class
- Add property assertion

## Strixonomy Parity Requirements

- [x] Entity-aware context menus
- [x] Axiom-aware context menus
- [x] Tree context menus
- [x] Search result context menus
- [x] Plugin-contributed context commands
- [x] Keyboard-accessible context actions

## Strixonomy Modernization Opportunities

- Command palette parity for every context action
- Inline quick actions
- AI-suggested context actions
- Recently used actions
- Batch context operations
- Safe-delete preview from context menu

---

# Menu Enablement Rules

## Purpose

Menu actions should be enabled, disabled, or hidden based on current state.

## Examples

Save should be enabled only when there are unsaved changes.

Rename should be enabled only when an entity is selected and editable.

Reasoner synchronization should be enabled only when a reasoner is selected and ontology changes exist.

Delete should be disabled for read-only imported entities unless the delete only affects local referencing axioms.

Export should be enabled when an ontology project is loaded.

## Strixonomy Requirements

- [x] Central command registry
- [x] Declarative enablement rules
- [x] Context-aware action state
- [x] Read-only import protection
- [x] Dirty-state awareness
- [x] Reasoner-state awareness
- [x] Selection-state awareness

---

# Keyboard Shortcuts

## Purpose

Keyboard shortcuts make menu actions available to power users.

## Common Shortcut Categories

- File operations
- Edit operations
- Search
- Navigation
- Reasoner synchronization
- Entity creation
- Entity deletion
- Workspace switching

## Strixonomy Requirements

- [x] Shortcut registry
- [x] User-customizable keybindings
- [x] Conflict detection
- [x] Command palette integration
- [x] Keyboard shortcut documentation
- [x] Platform-aware defaults

## Modernization Opportunity

Strixonomy should treat every menu action as a command with:

- stable command ID
- label
- description
- shortcut
- icon
- category
- enablement rule
- execution handler
- plugin contribution metadata

This would make Strixonomy more like VS Code or JetBrains IDEs than a traditional desktop editor.

---

# Plugin Menu Contributions

## Purpose

Protégé plugins can extend the UI by contributing menu actions, views, tabs, reasoners, and tools.

## Plugin Contribution Types

- Top-level menu
- Submenu
- Tool action
- View action
- Context menu action
- Reasoner menu entry
- Help/about entry

## Strixonomy Requirements

- [x] Plugin command contribution API
- [x] Plugin view contribution API
- [x] Plugin menu placement rules
- [x] Permission or trust model
- [x] Plugin diagnostics
- [x] Plugin enable/disable support
- [x] Plugin version compatibility checks

## Modernization Opportunity

Strixonomy should support plugin-defined commands through a manifest format.

Example:

```json
{
  "contributes": {
    "commands": [
      {
        "id": "strixonomy.validateLabels",
        "title": "Validate Labels",
        "category": "Ontology Quality",
        "menus": ["tools", "entity/context"],
        "when": "workspace.hasOntology"
      }
    ]
  }
}
```

---

# Recommended Strixonomy Menu Model

Strixonomy should not directly clone Protégé's menus. It should preserve feature coverage while modernizing the interaction model.

## Proposed Top-Level Menus

```text
File
Edit
View
Navigate
Ontology
Reasoner
Refactor
Tools
Plugins
Window
Help
```

## Why Add View?

Protégé blends view control into Window. Modern IDEs usually distinguish:

- View: visible panels, zoom, display options
- Window: application windows and workspace layout

## Why Add Navigate?

Ontology projects need fast movement between:

- classes
- properties
- individuals
- axioms
- usages
- imports
- errors
- inferred entities

## Why Add Plugins?

A first-class plugin menu makes extension management visible and trusted.

---

# Strixonomy Command Architecture

Every menu item should be backed by a command object.

## Command Metadata

Each command should define:

- ID
- Title
- Category
- Description
- Icon
- Shortcut
- Menu placement
- Context menu placement
- Enablement condition
- Required permissions
- Undo behavior
- Telemetry category
- Plugin owner, if applicable

## Command Execution

Commands should execute through a centralized command bus.

Benefits:

- menu integration
- toolbar integration
- context menu integration
- command palette integration
- keyboard shortcut integration
- macro support
- plugin support
- testability

---

# Feature Parity Checklist

## File

- [x] New ontology
- [x] Open ontology
- [x] Open recent
- [x] Save
- [x] Save as
- [x] Save all
- [x] Close project
- [x] Import ontology
- [x] Export ontology
- [x] Exit application

## Edit

- [x] Undo
- [x] Redo
- [x] Cut
- [x] Copy
- [x] Paste
- [x] Delete
- [x] Find
- [x] Preferences

## Active Ontology / Ontology

- [x] Ontology annotations
- [x] Prefix management
- [x] Import management
- [x] Ontology metrics
- [x] Active ontology selection
- [x] Physical IRI mapping

## Refactor

- [x] Rename entity
- [x] Move entity
- [x] Merge entities
- [x] Safe delete
- [x] Replace entity
- [x] Extract module

## Reasoner

- [x] Select reasoner
- [x] Start reasoner
- [x] Stop reasoner (v0.18: cancel in-flight client request)
- [x] Synchronize reasoner
- [x] Classify
- [x] Check consistency
- [x] Explain inconsistency
- [x] Configure reasoner

## Tools

- [x] Metrics
- [x] Validation
- [x] Visualization
- [x] Batch tools
- [x] Plugin tools

## Window / View

- [x] Show views
- [x] Hide views
- [x] Reset layout
- [x] Switch tabs
- [x] Save layout (v0.18: persist reopen command + context per view type)
- [x] Restore layout (v0.18: deserialize reopens panel with saved context)

## Help

- [x] Documentation
- [x] About
- [x] Plugin list
- [x] Error log
- [x] Diagnostics
- [x] Release notes

---

# Implementation Guidance for Strixonomy

## Minimum Viable Menu System

For an early Strixonomy release, implement:

1. File
2. Edit
3. Ontology
4. Reasoner
5. Refactor
6. View
7. Help

Back every item with a command registry.

## Version 1.0 Menu System

For Strixonomy v0.30, add:

- plugin-contributed commands
- command palette
- customizable shortcuts
- context menus
- workspace layout commands
- Git-aware save/export commands
- semantic refactor previews

## Beyond Protégé

Long-term Strixonomy should support:

- AI-authored ontology edits as commands
- command macros
- batch ontology transformations
- shared command histories
- collaborative command review
- scripted command execution
- command-level permissions
- Git commit generation from command history

---

# Summary

Protégé's menu system is functional, mature, and deeply tied to the ontology editing workflow. Its menus expose the core capabilities needed for ontology engineering: project management, editing, active ontology configuration, refactoring, reasoning, tools, workspace layout, and help.

For Strixonomy, the goal should not be a pixel-for-pixel clone. The goal should be command-level feature parity with a more modern architecture. Every Protégé menu action should map to an Strixonomy command, and every command should be accessible through menus, context menus, keyboard shortcuts, the command palette, plugins, and automation.

This approach preserves Protégé's proven ontology engineering coverage while making Strixonomy feel like a modern IDE for semantic systems.
