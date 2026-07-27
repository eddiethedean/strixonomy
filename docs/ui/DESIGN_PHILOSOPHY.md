# Strixonomy Design Philosophy

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


## Purpose

Strixonomy exists to redefine ontology engineering for the modern software
era. Rather than emulating legacy ontology editors, it treats ontologies
as living software systems and provides an integrated engineering
environment built around semantic understanding, developer workflows,
and AI-assisted productivity.

## Vision

**Become the JetBrains IDE for ontology engineering.**

The product should feel familiar to software developers while exposing
the power of semantic technologies. Every interaction should reinforce
the idea that users are working within a connected knowledge graph
rather than isolated files.

## Guiding Principles

### 1. Context over Windows

The interface revolves around a single *Current Focus* (entity, axiom,
query, or diagnostic). Every view reacts to that focus automatically.
Users should never have to manually synchronize multiple panels.

### 2. Workflows over Tools

The UI is optimized around common workflows:

-   Explore
-   Understand
-   Edit
-   Validate
-   Refactor
-   Review
-   Document
-   Publish

Panels exist only to support these workflows.

### 3. Progressive Disclosure

The default experience is intentionally simple. Advanced OWL constructs,
reasoning options, and semantic internals appear naturally as users
become more experienced or when a task requires them.

### 4. Workspace First

An ontology project is a semantic workspace, not a collection of files.
Navigation, reasoning, documentation, search, and AI all operate across
the entire workspace.

### 5. Semantic Native

Every interaction should understand semantic meaning rather than textual
representation.

Examples include:

-   semantic rename
-   semantic references
-   semantic search
-   semantic diff
-   semantic merge
-   semantic pull requests

### 6. Instant Feedback

Editing should immediately update diagnostics, graph visualizations,
references, documentation previews, and reasoning status whenever
practical.

### 7. AI as a Collaborator

AI is integrated throughout the interface instead of existing as a
standalone chat window.

Examples:

-   Explain modeling decisions
-   Generate documentation
-   Suggest refactors
-   Detect anti-patterns
-   Repair ontology inconsistencies
-   Translate between Manchester, OWL, SHACL, and SQL

### 8. Keyboard-First Productivity

Every workflow should be accessible from a command palette and keyboard
shortcuts. Mouse interactions enhance productivity but are never
required.

## Product Values

The interface should consistently feel:

-   Fast
-   Calm
-   Predictable
-   Discoverable
-   Context-aware
-   Professional

## Information Architecture

The primary workspace consists of:

-   Explorer
-   Central editor/workspace
-   Inspector
-   Bottom utility dock
-   Universal search and command palette

Everything else is contextual.

## User Experience Goals

A user should be able to answer questions such as:

-   What is this?
-   Why does it exist?
-   Where is it used?
-   What changed?
-   Is it correct?
-   How do I improve it?

within seconds.

## Long-Term Ambition

Strixonomy should establish a new standard for ontology engineering
comparable to what JetBrains accomplished for software development: a
cohesive, extensible, AI-native engineering environment centered on
semantic knowledge rather than source files.
