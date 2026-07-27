
# DIALOGS.md

# Protégé Dialog System Reverse Engineering Specification

## Purpose

Dialogs are temporary interaction surfaces used to collect user input, confirm destructive actions, configure application behavior, and display focused information. Unlike workspace views, dialogs are modal or transient and are intended to complete a single task before returning the user to the main workspace.

---

# Design Goals

A dialog should:

- Focus on one task
- Prevent invalid input
- Clearly explain consequences
- Support keyboard navigation
- Be accessible
- Integrate with undo/redo where applicable

---

# Dialog Categories

## Project Dialogs

Typical dialogs include:

- New Ontology
- Open Ontology
- Save As
- Import Ontology
- Export Ontology
- Close Project Confirmation

Typical fields:

- Ontology IRI
- Version IRI
- File path
- Serialization format
- Import options

---

## Entity Creation Dialogs

Used to create:

- Classes
- Object Properties
- Data Properties
- Annotation Properties
- Individuals

Common fields:

- IRI
- Label
- Parent entity
- Namespace
- Initial annotations

Validation:

- Duplicate IRIs
- Reserved prefixes
- Missing required fields

---

## Rename Dialog

Purpose:

Safely rename an entity.

Displays:

- Current IRI
- New IRI
- Label updates
- Impact summary

Should preview affected references.

---

## Delete Confirmation

Shown before destructive operations.

Must display:

- Entity name
- Entity type
- Number of affected axioms
- Referencing entities
- Imported ontology impact

Options:

- Delete
- Cancel

---

## Import Management Dialog

Capabilities:

- Add import
- Remove import
- Reload import
- Resolve missing imports

Information displayed:

- Logical IRI
- Physical IRI
- Status
- Editable state

---

## Prefix Manager

Allows users to:

- Add prefix
- Edit prefix
- Remove prefix

Validation:

- Duplicate prefix
- Invalid namespace
- Reserved namespaces

---

## Ontology Metadata Dialog

Supports editing:

- Labels
- Comments
- Version
- Creator
- License
- Custom annotations

---

## Preferences Dialog

Configuration categories may include:

- Appearance
- Rendering
- Reasoner
- Plugins
- Keyboard shortcuts
- Entity rendering
- Workspace layout

Should support search.

---

## Reasoner Configuration

Allows configuration of:

- Active reasoner
- Incremental reasoning
- Timeouts
- Logging
- Explanation behavior

---

## Search Dialog

Supports:

- Entity search
- Label search
- IRI search
- Annotation search

Modern implementations should support fuzzy matching.

---

## Metrics Dialog

Displays ontology statistics including:

- Classes
- Properties
- Individuals
- Imports
- Axioms
- Logical axiom count

---

## Plugin Dialogs

Plugins may provide custom dialogs.

Examples:

- Visualization configuration
- Validation options
- Import wizards
- Export settings

Plugins should register dialogs through a stable API.

---

# Common Dialog Components

Dialogs commonly include:

- Title
- Description
- Input fields
- Validation messages
- Help text
- Primary action
- Secondary action
- Cancel button

---

# Validation

Dialogs should validate:

- Required fields
- Duplicate names
- Invalid IRIs
- Invalid prefixes
- Illegal ontology state

Validation should occur while typing whenever practical.

---

# Keyboard Support

Minimum requirements:

- Tab navigation
- Shift+Tab reverse navigation
- Enter activates primary action
- Escape closes dialog
- Visible focus indicators

---

# Accessibility

Dialogs should provide:

- Screen reader labels
- Logical tab order
- High contrast support
- Scalable fonts
- Accessible error messages

---

# Strixonomy Modernization

Strixonomy should modernize dialogs by introducing:

- Non-blocking sheets where appropriate
- Multi-step wizards
- Live previews
- AI-assisted field completion
- Inline ontology validation
- Command palette alternatives
- Responsive layouts
- Theme-aware styling

---

# Recommended Dialog Framework

Every dialog should define:

- Dialog ID
- Title
- Purpose
- Input schema
- Validation rules
- Primary action
- Secondary action
- Result payload

Dialogs should return structured results rather than directly mutating application state.

---

# Feature Parity Checklist

Project

- [x] New ontology
- [x] Open
- [x] Save As
- [x] Import
- [x] Export

Entity

- [x] Create entity
- [x] Rename
- [x] Delete confirmation

Ontology

- [x] Prefix manager
- [x] Metadata editor
- [x] Import manager

Configuration

- [x] Preferences
- [x] Reasoner settings

Utilities

- [x] Search
- [x] Metrics

Platform

- [x] Validation
- [x] Keyboard support
- [x] Accessibility
- [x] Plugin dialogs

---

# Summary

Protégé relies on dialogs to perform focused configuration and editing tasks while keeping the primary workspace uncluttered. Strixonomy should retain this task-oriented approach but modernize it with live validation, structured dialog APIs, AI-assisted workflows, responsive layouts, and reusable React components backed by a centralized command architecture.
