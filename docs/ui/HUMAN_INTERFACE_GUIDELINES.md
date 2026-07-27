# HUMAN_INTERFACE_GUIDELINES.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy Human Interface Guidelines (HIG)

## Purpose

These guidelines define the user experience standards for every Strixonomy
interface. Every screen, component, workflow, and interaction should
follow these principles to create a cohesive, modern ontology
engineering environment.

------------------------------------------------------------------------

# Design Goals

The interface should feel:

-   Modern
-   Calm
-   Fast
-   Intelligent
-   Predictable
-   Discoverable
-   Professional

Users should spend their mental energy understanding ontologies---not
learning the UI.

------------------------------------------------------------------------

# Core Principles

## Context First

The interface revolves around a single **Current Focus**. Every major
view reacts automatically to the selected entity.

Never require users to manually synchronize multiple windows.

## Progressive Disclosure

Start simple.

Only reveal advanced semantic concepts when users need them.

Novice users should never feel overwhelmed by OWL terminology.

## Workflow First

Optimize for tasks instead of tools.

Primary workflows include:

-   Explore
-   Understand
-   Edit
-   Validate
-   Refactor
-   Review
-   Document
-   Publish

------------------------------------------------------------------------

# Layout

## Global Structure

Top - Universal Search / Command Palette

Left - Explorer - Favorites - Recent Items

Center - Workspace

Right - Inspector

Bottom - Problems - Graph - Query - AI - Git - Output

No dedicated "Graph Mode" or "Reasoner Mode."

------------------------------------------------------------------------

# Navigation

Always provide:

-   Breadcrumbs
-   Back/Forward navigation
-   Recently viewed entities
-   Favorites
-   Command palette
-   Keyboard shortcuts

Navigation should never trap users.

------------------------------------------------------------------------

# Visual Hierarchy

Emphasize information in this order:

1.  Entity name
2.  Human-readable description
3.  Semantic relationships
4.  Diagnostics
5.  Technical metadata
6.  Internal identifiers

IRIs should never dominate the interface.

------------------------------------------------------------------------

# Editing

Prefer inline editing over modal dialogs.

Changes should:

-   update immediately
-   validate continuously
-   support undo/redo
-   provide non-blocking feedback

------------------------------------------------------------------------

# Search

Search is the primary navigation mechanism.

Search should index:

-   entities
-   relationships
-   annotations
-   documentation
-   queries
-   diagnostics
-   git history
-   AI actions

Results should be grouped by type.

------------------------------------------------------------------------

# Panels

Panels must:

-   synchronize automatically
-   remember layout
-   support keyboard navigation
-   avoid duplicate information

Each panel should answer one question well.

------------------------------------------------------------------------

# Feedback

Use subtle feedback.

Prefer:

-   inline validation
-   status chips
-   badges
-   lightweight notifications

Avoid disruptive modal dialogs whenever possible.

------------------------------------------------------------------------

# Motion

Animations should communicate state changes.

Use motion for:

-   expanding trees
-   graph transitions
-   docking panels
-   loading

Animations should be fast (150--250 ms) and never delay interaction.

------------------------------------------------------------------------

# Accessibility

Support:

-   WCAG 2.2 AA
-   full keyboard navigation
-   screen readers
-   high contrast
-   reduced motion
-   scalable typography

Accessibility is a core requirement, not an enhancement.

------------------------------------------------------------------------

# AI

AI should appear as contextual actions rather than a separate
application.

Examples:

-   Explain
-   Suggest improvements
-   Generate documentation
-   Repair issues
-   Refactor
-   Translate syntax

AI should always explain why it made a recommendation.

------------------------------------------------------------------------

# Error Handling

Errors should:

-   identify the affected entity
-   explain the problem in plain language
-   provide suggested fixes
-   link directly to the source

Never display cryptic parser errors without context.

------------------------------------------------------------------------

# Performance

Target interactions:

-   Navigation: \<100 ms
-   Search: \<100 ms
-   Inspector updates: \<50 ms
-   Workspace switches: \<150 ms

Perceived performance is as important as measured performance.

------------------------------------------------------------------------

# Extensibility

Plugins should inherit the same interaction patterns.

Extension authors should be able to create native-feeling experiences
without reimplementing navigation, layout, or styling.

------------------------------------------------------------------------

# Success Metric

A new user should be able to install Strixonomy and begin exploring an
ontology within minutes, while experienced ontology engineers should
feel they have the power and efficiency of a first-class modern IDE.
