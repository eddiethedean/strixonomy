# QUERY_WORKBENCH.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy Query Workbench Specification

## Purpose

The Query Workbench is the primary environment for exploring,
validating, analyzing, and transforming semantic data. It should provide
a developer experience comparable to JetBrains DataGrip while remaining
tightly integrated with Strixonomy's semantic model.

Rather than being "a place to run SPARQL," the Query Workbench should
become the command center for ontology exploration.

------------------------------------------------------------------------

# Vision

Users should be able to:

-   Discover ontology structure
-   Explore semantic relationships
-   Build reusable queries
-   Validate assumptions
-   Compare ontology revisions
-   Visualize results
-   Generate documentation
-   Feed AI workflows

---all without leaving the workbench.

------------------------------------------------------------------------

# Design Principles

-   Query-first exploration
-   Immediate feedback
-   Rich visualization
-   AI-assisted authoring
-   Workspace synchronization
-   Keyboard-first workflow

------------------------------------------------------------------------

# Primary Layout

    +----------------------------------------------------------------+
    | Toolbar | Connection | Query Language | Run | AI | Saved Views |
    +----------------------------------------------------------------+

    | Query Editor                                | Schema Browser   |
    |                                              |                 |
    |                                              |                 |
    +----------------------------------------------------------------+

    | Results | Graph | JSON | Explain | AI | History | Diagnostics |
    +----------------------------------------------------------------+

------------------------------------------------------------------------

# Supported Query Languages

Native support:

-   SQL (OntoSQL)
-   SPARQL
-   GraphQL (future)
-   SHACL validation queries
-   Datalog (future)

The workbench should expose a common interaction model regardless of
language.

------------------------------------------------------------------------

# Query Editor

Features

-   Syntax highlighting
-   Autocomplete
-   Semantic completion
-   Inline diagnostics
-   Multi-cursor editing
-   Code folding
-   Snippets
-   Formatting
-   Live validation

------------------------------------------------------------------------

# Schema Browser

Displays:

-   Classes
-   Properties
-   Individuals
-   Namespaces
-   Modules
-   Saved queries

Supports drag-and-drop into the editor.

------------------------------------------------------------------------

# Query Execution

Execution should provide:

-   Progress
-   Cancellation
-   Runtime
-   Row count
-   Warnings
-   Execution statistics

Long-running queries remain asynchronous.

------------------------------------------------------------------------

# Results

Support multiple synchronized views.

## Table

-   Sort
-   Filter
-   Resize
-   Copy
-   Export

------------------------------------------------------------------------

## Graph

Automatically visualize query results.

Users may:

-   Expand neighbors
-   Pin nodes
-   Save layouts

------------------------------------------------------------------------

## JSON

Raw structured output.

Useful for debugging and automation.

------------------------------------------------------------------------

## Explain

Execution plan.

Displays:

-   Optimization
-   Cost estimates
-   Join strategy
-   Semantic reasoning steps

------------------------------------------------------------------------

## AI

Explain query

Optimize query

Generate query

Summarize results

Suggest visualizations

------------------------------------------------------------------------

# Saved Queries

Users may:

-   Organize into folders
-   Tag
-   Favorite
-   Version
-   Share
-   Execute from command palette

Saved queries become workspace assets.

------------------------------------------------------------------------

# Query History

Automatically stores:

-   Timestamp
-   Runtime
-   User
-   Parameters
-   Result count

Supports replay.

------------------------------------------------------------------------

# Parameters

Parameterized queries support:

-   strings
-   numbers
-   booleans
-   dates
-   entity references

Interactive parameter prompts appear before execution.

------------------------------------------------------------------------

# Visual Builder (Future)

Provide a graphical query builder for new users.

Capabilities:

-   Drag entities
-   Create joins visually
-   Preview generated SQL/SPARQL
-   Round-trip editing

------------------------------------------------------------------------

# Workspace Integration

Selecting an entity in query results updates:

-   Explorer
-   Inspector
-   Graph Workspace
-   Entity Editor
-   Breadcrumbs

Queries participate fully in the Workspace Model.

------------------------------------------------------------------------

# AI Capabilities

AI assists with:

-   Writing queries
-   Translating SQL ↔ SPARQL
-   Explaining semantics
-   Detecting inefficient patterns
-   Suggesting indexes
-   Summarizing results
-   Creating reusable reports

Every AI action includes a preview before execution.

------------------------------------------------------------------------

# Collaboration

Future support:

-   Shared queries
-   Review comments
-   Query collections
-   Team libraries
-   Published dashboards

------------------------------------------------------------------------

# Performance Targets

Autocomplete

\<50 ms

Query validation

\<100 ms

Small query execution

\<500 ms

Large queries

Progressive streaming

Result rendering

Virtualized tables

------------------------------------------------------------------------

# Plugin Extension Points

Plugins may contribute:

-   Query languages
-   Result renderers
-   Export formats
-   AI providers
-   Schema browser nodes
-   Explain analyzers
-   Toolbar actions

------------------------------------------------------------------------

# Accessibility

Support:

-   Keyboard navigation
-   Screen readers
-   High contrast
-   Reduced motion
-   Accessible tables
-   Scalable fonts

------------------------------------------------------------------------

# Success Criteria

The Query Workbench succeeds when ontology engineers think of querying
as a natural part of daily development rather than a specialized expert
task. It should combine the power of professional database IDEs with
semantic awareness, rich visualizations, AI assistance, and deep
integration into the Strixonomy workspace, making it one of the primary
environments for understanding and evolving knowledge graphs.
