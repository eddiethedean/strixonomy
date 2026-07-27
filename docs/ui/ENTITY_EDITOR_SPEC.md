# ENTITY_EDITOR_SPEC.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy Entity Editor Specification

## Purpose

The Entity Editor is the primary workspace of Strixonomy. It replaces the
fragmented editing experience of traditional ontology tools with a
unified, context-aware editor centered on a single semantic entity.

The editor should feel to ontology engineers what the code editor is to
software developers.

------------------------------------------------------------------------

# Design Goals

The Entity Editor must be:

-   Fast
-   Focused
-   Discoverable
-   AI-assisted
-   Context-aware
-   Keyboard-first
-   Fully synchronized with the Workspace Model

Users should rarely need to leave this screen.

------------------------------------------------------------------------

# Mental Model

Every semantic object opens in its own editor tab.

Examples:

-   Class
-   Individual
-   Object Property
-   Data Property
-   Annotation Property
-   Ontology
-   SHACL Shape
-   Rule

The editor presents every relevant aspect of that object in one cohesive
experience.

------------------------------------------------------------------------

# Layout

    +--------------------------------------------------------------+
    | Breadcrumbs                             Actions              |
    +--------------------------------------------------------------+

    | Overview                                              Status |

    +--------------------------------------------------------------+

    | Tabs                                                   Search |

     Overview | Hierarchy | Relationships | Constraints | Docs |
     History | References | Reasoning | AI | Metadata

    +--------------------------------------------------------------+

    | Active Section                                             |

    +--------------------------------------------------------------+

The layout remains consistent regardless of entity type.

------------------------------------------------------------------------

# Header

Displays:

-   Entity name
-   Entity type
-   Namespace
-   Status
-   Diagnostics
-   Favorite toggle

Quick actions:

-   Rename
-   Show Graph
-   Find References
-   Explain
-   Generate Docs
-   History
-   Copy IRI

------------------------------------------------------------------------

# Overview

Primary landing page.

Contains:

Description

Parents

Children

Relationships

Statistics

Recent Changes

Diagnostics

Recent AI Suggestions

Users should understand the entity within seconds.

------------------------------------------------------------------------

# Hierarchy

Displays:

-   Parents
-   Children
-   Siblings

Supports:

-   Collapse
-   Expand
-   Drag-and-drop
-   Keyboard navigation
-   Jump to entity

------------------------------------------------------------------------

# Relationships

Displays semantic relationships grouped by type.

Example:

Object Properties

Data Properties

Restrictions

Equivalent Classes

Disjoint Classes

Supports inline editing.

------------------------------------------------------------------------

# Constraints

Displays:

OWL restrictions

Cardinality

Domains

Ranges

Logical expressions

Provides live validation while editing.

------------------------------------------------------------------------

# Documentation

Integrated documentation editor.

Supports:

-   Markdown
-   Rich preview
-   AI generation
-   Cross references
-   Version history

------------------------------------------------------------------------

# References

Lists every semantic usage.

Examples:

Imported by

Referenced by

Queries

Rules

SHACL

Documentation

Git

Each result is clickable.

------------------------------------------------------------------------

# Reasoning

Shows reasoning relevant to this entity.

Examples:

Inferred parents

Unsatisfied constraints

Equivalent classes

Consistency

Explanation

Users should never need a separate reasoning window.

------------------------------------------------------------------------

# AI

Context-aware assistant.

Capabilities:

Explain

Improve

Normalize

Generate documentation

Detect anti-patterns

Repair issues

Translate syntax

Every recommendation includes:

Reason

Preview

Apply

Dismiss

------------------------------------------------------------------------

# Metadata

Displays technical information.

Examples:

IRI

Namespace

Version

Source ontology

Creation date

Modification date

Plugin metadata

Metadata remains secondary.

------------------------------------------------------------------------

# Editing

All common editing occurs inline.

Double-click:

Rename

Descriptions

Labels

Comments

Restrictions

Relationships

Avoid modal dialogs whenever possible.

------------------------------------------------------------------------

# Validation

Validation runs continuously.

Feedback types:

Success

Information

Warning

Error

Every validation issue includes:

Explanation

Suggested fix

Navigation

------------------------------------------------------------------------

# Keyboard Shortcuts

Examples:

Enter

Edit

Escape

Cancel

Ctrl/Cmd+S

Save

Ctrl/Cmd+F

Search entity

Ctrl/Cmd+Shift+P

Command palette

Alt+Left

Back

Alt+Right

Forward

Every interaction should have a keyboard equivalent.

------------------------------------------------------------------------

# Search Within Entity

Supports:

Properties

Restrictions

Annotations

Documentation

Relationships

History

Results update live.

------------------------------------------------------------------------

# History

Timeline view.

Examples:

Created

Renamed

Relationship added

Constraint changed

Documentation updated

Git commit

Supports comparison between revisions.

------------------------------------------------------------------------

# Notifications

Prefer inline status indicators.

Examples:

Saved

Unsaved

Reasoning complete

AI suggestion available

Validation failed

Avoid intrusive popups.

------------------------------------------------------------------------

# Collaboration

Future support:

Comments

Review requests

Semantic pull requests

Assignments

Live collaboration

------------------------------------------------------------------------

# Accessibility

Support:

Screen readers

Keyboard-only navigation

High contrast

Reduced motion

Scalable fonts

Accessible labels

------------------------------------------------------------------------

# Performance

Targets:

Open entity

\<100 ms

Switch tabs

\<50 ms

Inline edit

Immediate

Validation

\<100 ms

Search

\<50 ms

AI suggestions

Asynchronous

------------------------------------------------------------------------

# Extension Points

Plugins may contribute:

Inspector cards

Relationship sections

Validation rules

Toolbar actions

Documentation panels

AI providers

Timeline entries

------------------------------------------------------------------------

# Success Criteria

A user should be able to spend an entire ontology engineering session
inside the Entity Editor without feeling the need to constantly switch
windows or hunt through disconnected panels. The editor should become
the natural center of the Strixonomy experience, bringing together
editing, reasoning, documentation, history, AI assistance, and semantic
navigation in one coherent workspace.
