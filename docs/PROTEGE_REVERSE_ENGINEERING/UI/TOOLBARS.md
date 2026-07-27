
# TOOLBARS.md

# Protégé Toolbar Reverse Engineering Specification

## Purpose

The Protégé toolbar provides fast access to the most frequently used commands while editing ontologies. It complements the menu system by exposing common project, editing, search, and reasoning operations as one-click actions.

Unlike menus, the toolbar should prioritize high-frequency actions and avoid overwhelming users with rarely used commands.

---

# Design Goals

The toolbar should:

- Minimize clicks for common workflows
- Reflect current workspace state
- Expose the most common commands
- Remain compact
- Update dynamically as project state changes

---

# Default Layout

Typical organization:

```text
+------------------------------------------------------------------+
| New | Open | Save | Undo | Redo | Search | Reasoner | Preferences|
+------------------------------------------------------------------+
```

Plugins may contribute additional toolbar buttons.

---

# Toolbar Groups

## Project

Common commands:

- New Ontology
- Open Ontology
- Save
- Save All

### Behavior

Buttons should reflect project state.

Examples:

- Save disabled when no changes exist.
- Save All enabled when any ontology is dirty.

---

## Editing

Common commands:

- Undo
- Redo

Future improvements:

- Cut
- Copy
- Paste

Undo/Redo should display tooltips describing the pending action.

---

## Search

Provides rapid access to:

- Global entity search
- Find by label
- Find by IRI

Modern implementations should support fuzzy search.

---

## Reasoning

Common actions:

- Select reasoner
- Synchronize
- Start
- Stop
- Classify

The toolbar should clearly communicate:

- active reasoner
- running state
- synchronization progress

Long-running reasoning tasks should display progress indicators.

---

## Preferences

Quick access to application settings.

Potential categories:

- Rendering
- UI
- Reasoner
- Plugins
- Appearance

---

# Dynamic State

Toolbar buttons should react to application state.

Examples:

| Command | Enabled When |
|----------|--------------|
| Save | Dirty ontology exists |
| Undo | Undo stack not empty |
| Redo | Redo stack not empty |
| Synchronize | Reasoner selected and ontology changed |
| Stop Reasoner | Reasoner running |

---

# Visual Design

Toolbar buttons should provide:

- Icon
- Tooltip
- Accessible label
- Keyboard shortcut (if available)

Tooltips should explain the command and display shortcuts.

Example:

```text
Save Ontology
Ctrl+S
Writes all pending ontology changes.
```

---

# Overflow Behavior

If horizontal space is limited:

1. Keep primary commands visible.
2. Collapse secondary actions into an overflow menu.
3. Preserve keyboard access to every command.

---

# Plugin Contributions

Plugins should be able to contribute:

- Buttons
- Toggle buttons
- Dropdowns
- Split buttons
- Progress indicators

Each contribution should specify:

- Command ID
- Icon
- Tooltip
- Placement
- Visibility rule

---

# Accessibility

Requirements:

- Keyboard navigable
- Screen reader labels
- High contrast support
- Focus indicators
- Scalable icons

---

# Strixonomy Modernization

A modern toolbar should support:

- Command palette integration
- Workspace profiles
- AI assistant entry point
- Git status
- Collaboration status
- Background task indicator
- Live validation state

Example:

```text
+----------------------------------------------------------------------------------+
| New Open Save Undo Redo Search AI Git Reasoner Validate Profile Notifications    |
+----------------------------------------------------------------------------------+
```

---

# Command Architecture

Every toolbar item should reference the same command registry used by:

- Menus
- Context menus
- Keyboard shortcuts
- Command palette
- Automation
- Plugins

Toolbar buttons should contain:

- Command ID
- Icon
- Tooltip
- Enablement rule
- Visibility rule

---

# Recommended Strixonomy Toolbar

## Primary

- New
- Open
- Save
- Undo
- Redo
- Search

## Ontology

- New Class
- New Property
- New Individual

## Reasoning

- Select Reasoner
- Synchronize
- Classify

## Quality

- Validate
- Metrics

## Collaboration

- Git Changes
- Share
- Comments

## AI

- Explain
- Generate
- Refactor Suggestions

---

# Feature Parity Checklist

## Project

- [x] New
- [x] Open
- [x] Save
- [x] Save All

## Editing

- [x] Undo
- [x] Redo

## Search

- [x] Entity Search
- [x] Global Search

## Reasoning

- [x] Start
- [x] Stop
- [x] Synchronize
- [x] Classify

## Preferences

- [x] Preferences Button

## Framework

- [x] Dynamic enablement
- [x] Plugin contributions
- [x] Keyboard shortcuts
- [x] Accessible tooltips
- [x] Overflow handling
- [x] Responsive layout

---

# Summary

Protégé's toolbar is intentionally minimal, exposing only the most common ontology engineering actions. Strixonomy should preserve this philosophy while modernizing the implementation with a centralized command system, richer state feedback, responsive layouts, collaboration indicators, AI-assisted workflows, and extensible plugin contributions.
