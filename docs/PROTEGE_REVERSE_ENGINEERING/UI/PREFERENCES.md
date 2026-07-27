# PREFERENCES.md

# Protégé Preferences Reverse Engineering Specification

## Purpose

The Preferences system centralizes user-configurable settings that affect the behavior, appearance, rendering, reasoning, plugins, and persistence of the Protégé application. Preferences are user-specific rather than ontology-specific and are intended to persist across sessions.

This document serves as a functional specification for reproducing—and improving—the Protégé preferences system in Strixonomy.

---

# Design Goals

The preferences system should:

- Keep project-independent settings in one place.
- Persist settings across sessions.
- Validate user input before applying changes.
- Allow plugins to contribute settings.
- Apply changes immediately when safe.
- Support resetting to defaults.

---

# Preference Categories

## General

Controls application-wide behavior.

Typical settings:

- Start page behavior
- Auto-save
- Recent project history
- Confirmation dialogs
- Default ontology location
- Language / locale
- Update checks

### Strixonomy Improvements

- Cloud workspace defaults
- Telemetry controls
- Startup workspace selection
- User profile synchronization

---

## Appearance

Controls the visual presentation of the application.

Typical settings:

- Theme (Light / Dark)
- Font size
- Icon size
- UI scaling
- High contrast mode
- Docking behavior

### Strixonomy Improvements

- Design token support
- Theme marketplace
- Per-workspace themes
- Automatic OS theme detection

---

## Entity Rendering

Determines how ontology entities are displayed.

Typical options:

- Short form
- Label
- IRI
- Prefix rendering
- Custom rendering strategy

Rendering choices affect:

- Trees
- Search
- Editors
- Usage views
- Graph visualizations

---

## Editing

Controls ontology editing behavior.

Typical settings:

- Auto-create labels
- Default namespace
- Default annotations
- Confirmation before delete
- Inline validation
- Automatic formatting

### Strixonomy Improvements

- AI-assisted entity naming
- Live ontology linting
- Bulk edit defaults

---

## Reasoner

Configures reasoning behavior.

Typical settings:

- Default reasoner
- Incremental reasoning
- Auto-synchronize
- Timeout
- Logging
- Precomputation

Reasoner-specific plugins may contribute additional options.

---

## Imports

Controls import handling.

Typical settings:

- Missing import policy
- Auto-resolve imports
- Reload strategy
- Import cache behavior

### Strixonomy Improvements

- Package-manager style dependency resolution
- Version pinning
- Import health dashboard

---

## Search

Search configuration may include:

- Case sensitivity
- Fuzzy search
- Maximum results
- Search scope
- Index refresh policy

---

## Graph Visualization

If graph plugins are installed, configurable options may include:

- Layout algorithm
- Animation
- Node labels
- Edge labels
- Zoom behavior
- Selection highlighting

---

## Plugins

Plugins should be able to expose their own settings.

Required metadata:

- Plugin ID
- Preference page
- Validation rules
- Default values

Plugins should not modify core settings directly.

---

## Keyboard Shortcuts

Modern IDEs allow user-defined shortcuts.

Recommended options:

- View shortcuts
- Edit shortcuts
- Search shortcuts
- Reasoning shortcuts
- Custom command bindings

### Strixonomy Improvements

- VS Code style keybinding editor
- Conflict detection
- Profiles

---

## Performance

Useful settings include:

- Background indexing
- Tree virtualization
- Cache limits
- Graph rendering limits
- Memory usage

---

# Preference Storage

Preferences should be stored separately from ontology projects.

Recommended model:

```text
Preferences
 ├── General
 ├── Appearance
 ├── Editing
 ├── Reasoner
 ├── Imports
 ├── Search
 ├── Plugins
 └── Keybindings
```

Settings should be versioned to allow migration across releases.

---

# Validation

Each preference should define:

- Data type
- Default value
- Allowed range
- Validation rules
- Restart requirement

Invalid values should never be persisted.

---

# UI Requirements

The Preferences dialog should provide:

- Search
- Category navigation
- Inline descriptions
- Validation messages
- Reset to default
- Import/export settings
- Apply without restart when possible

---

# Plugin Contribution API

Plugins should contribute preferences using metadata such as:

- Category
- Title
- Description
- Control type
- Default value
- Validator

The host application should render plugin settings automatically.

---

# Accessibility

Requirements:

- Full keyboard navigation
- Screen reader support
- High contrast compatibility
- Scalable fonts
- Accessible labels

---

# Strixonomy Modernization

Recommended enhancements:

- Settings Sync
- Workspace-specific preferences
- Team preference profiles
- Command palette access to settings
- AI explanations for settings
- JSON-based advanced editor
- Live preview of visual changes

---

# Recommended Strixonomy Preference Categories

- General
- Appearance
- Workspace
- Ontology Editing
- Entity Rendering
- Reasoning
- Validation
- Search
- Git
- Collaboration
- AI
- Plugins
- Keyboard Shortcuts
- Accessibility
- Performance
- Experimental Features

---

# Feature Parity Checklist

General

- [ ] Startup behavior (`autoIndexOnOpen` is a documented no-op)
- [ ] Auto-save (VS Code native)
- [ ] Recent projects (VS Code native)

Appearance

- [x] Theme (VS Code theme)
- [x] Fonts (VS Code font settings)
- [x] Scaling (VS Code zoom)

Editing

- [x] Delete confirmation (impact summary)
- [ ] Default namespace
- [ ] Label generation

Reasoning

- [x] Default reasoner (`strixonomy.reasoner.default` + Preferences hub)
- [ ] Incremental reasoning
- [ ] Timeouts

Rendering

- [ ] Label vs IRI (prefix manager exists; no rendering strategy prefs)
- [x] Prefix rendering (Prefix Manager)

Search

- [ ] Search behavior

Plugins

- [ ] Plugin settings
- [ ] Dynamic preference pages

Platform

- [ ] Persistent storage
- [ ] Validation
- [ ] Reset defaults
- [ ] Import/export settings

---

# Summary

Protégé's preferences system provides centralized configuration for the ontology engineering environment while remaining largely independent of ontology content. Strixonomy should preserve this separation but evolve it into a modern, searchable, extensible settings platform with plugin-defined pages, synchronized profiles, workspace-aware configuration, and AI-assisted guidance.
