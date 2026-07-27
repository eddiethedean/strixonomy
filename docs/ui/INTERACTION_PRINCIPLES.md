# INTERACTION_PRINCIPLES.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy Interaction Principles

## Purpose

This document defines how Strixonomy should *feel* to use. Every
interaction should reinforce speed, clarity, confidence, and semantic
awareness. The goal is to create an IDE that feels closer to JetBrains,
Cursor, and Figma than to traditional ontology editors.

------------------------------------------------------------------------

# Core Philosophy

Users should think about **their ontology**, not the interface.

The interface should disappear behind predictable, responsive
interactions.

------------------------------------------------------------------------

# Guiding Principles

## Context Over Modes

Avoid "modes" wherever possible.

Selecting an entity should update every relevant part of the workspace
automatically.

Never require users to manually synchronize views.

## Direct Manipulation

Whenever practical:

-   Click to select
-   Double-click to edit
-   Drag to reorganize
-   Hover to preview
-   Right-click for contextual actions

Prefer interacting with the object itself over opening dialogs.

## Progressive Disclosure

Default interfaces should expose only the most common actions.

Advanced options appear: - on demand - through expansion - via command
palette - in advanced sections

------------------------------------------------------------------------

# Selection Model

The **Current Focus** drives the application.

Selecting an entity updates:

-   Explorer
-   Workspace
-   Inspector
-   Graph
-   Breadcrumbs
-   Diagnostics
-   AI context
-   Documentation

Support:

-   single selection
-   multi-selection
-   range selection
-   keyboard navigation

Selection should never be ambiguous.

------------------------------------------------------------------------

# Navigation

Navigation should be instantaneous.

Provide:

-   Back
-   Forward
-   Breadcrumbs
-   Recent items
-   Favorites
-   Jump to definition
-   Find references
-   Peek previews

Every navigation action should preserve user context.

------------------------------------------------------------------------

# Editing

Editing should be inline by default.

Examples:

-   Rename directly in place.
-   Edit labels without dialogs.
-   Drag relationships to reorganize.
-   Toggle flags with a click.

Use modal dialogs only for destructive or multi-step workflows.

------------------------------------------------------------------------

# Undo / Redo

Every user action should be reversible.

Support:

-   unlimited logical undo
-   redo
-   grouped operations
-   semantic undo (rename, refactor, merge)

Users should feel safe experimenting.

------------------------------------------------------------------------

# Context Menus

Context menus should expose actions relevant to the current selection
only.

Typical actions:

-   Rename
-   Find References
-   Show Graph
-   Explain
-   Generate Documentation
-   Refactor
-   Copy IRI
-   Reveal in Explorer

Avoid long generic menus.

------------------------------------------------------------------------

# Command Palette

The command palette is the primary power-user interface.

Capabilities:

-   run commands
-   navigate entities
-   open documentation
-   execute queries
-   invoke AI actions
-   refactor
-   switch workspaces

Every major action should be available here.

------------------------------------------------------------------------

# Drag and Drop

Support intuitive drag-and-drop where semantics are clear.

Examples:

-   reorder explorer nodes
-   organize favorites
-   rearrange workspace tabs
-   pin graph nodes

Always preview the outcome before committing.

------------------------------------------------------------------------

# Feedback

Provide immediate, lightweight feedback.

Preferred mechanisms:

-   status chips
-   inline validation
-   badges
-   subtle animations
-   toast notifications

Avoid blocking the user's workflow.

------------------------------------------------------------------------

# Motion

Motion should communicate meaning.

Use animation to show:

-   selection changes
-   panel expansion
-   graph transitions
-   docking
-   loading progress

Target duration: 150--250 ms.

Respect reduced-motion accessibility settings.

------------------------------------------------------------------------

# Keyboard Interaction

Every feature should be keyboard accessible.

Provide:

-   consistent shortcuts
-   discoverable keybindings
-   focus indicators
-   tab navigation
-   command palette access

Keyboard users should never be second-class users.

------------------------------------------------------------------------

# AI Interaction

AI is integrated into workflows.

Every entity may expose:

-   Explain
-   Improve
-   Repair
-   Refactor
-   Document
-   Translate

AI recommendations should:

-   explain reasoning
-   be previewable
-   require explicit acceptance before applying changes

------------------------------------------------------------------------

# Error Recovery

Errors should be actionable.

Every error should answer:

-   What happened?
-   Why?
-   What can I do?
-   Can Strixonomy help fix it?

Never expose raw parser failures without interpretation.

------------------------------------------------------------------------

# Performance Expectations

Perceived responsiveness matters.

Targets:

-   Selection updates: \<50 ms
-   Navigation: \<100 ms
-   Search: \<100 ms
-   Inspector refresh: \<50 ms
-   Workspace switch: \<150 ms

Long-running operations should provide progress and remain cancellable.

------------------------------------------------------------------------

# Consistency Rules

Every interaction should behave consistently across the application.

The same action should:

-   use the same shortcut
-   produce the same animation
-   follow the same terminology
-   generate similar feedback

Consistency builds user confidence.

------------------------------------------------------------------------

# Success Criteria

A successful interaction model is one where users quickly stop thinking
about *how* to operate Strixonomy and instead focus entirely on
understanding, editing, validating, and evolving their knowledge model.
