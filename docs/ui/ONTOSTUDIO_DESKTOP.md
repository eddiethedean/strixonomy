# ONTOSTUDIO_DESKTOP.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# OntoStudio Desktop Specification

## Purpose

OntoStudio is the long-term desktop flagship of the Ontologos ecosystem.
While Strixonomy provides a world-class VS Code experience, OntoStudio
removes the constraints of an editor extension and delivers a dedicated
semantic engineering environment built around Strixonomy.

OntoStudio is not intended to replace Strixonomy---it shares the same
backend, design system, and plugin ecosystem while providing an
optimized desktop experience.

------------------------------------------------------------------------

# Vision

Become the premier desktop IDE for ontology and knowledge graph
engineering.

Comparable ambitions:

-   IntelliJ IDEA for semantic engineering
-   DataGrip for semantic querying
-   Figma for semantic visualization
-   Obsidian for knowledge exploration
-   Cursor for AI-native workflows

------------------------------------------------------------------------

# Design Principles

## One Platform

Strixonomy and OntoStudio share:

-   Strixonomy
-   Design System
-   Workspace Model
-   Plugin Platform
-   AI Experience
-   Component Library

The desktop application should feel familiar to Strixonomy users.

------------------------------------------------------------------------

## Workspace First

Users work inside semantic workspaces rather than windows or files.

Example workspaces:

-   Entity
-   Graph
-   Query
-   Reasoning
-   Review
-   Documentation
-   Architecture
-   AI

------------------------------------------------------------------------

## Native Performance

The desktop application should feel instantaneous.

Goals:

-   Native menus
-   Native file dialogs
-   GPU-accelerated rendering
-   Fast graph visualization
-   Low memory footprint

------------------------------------------------------------------------

# Technology Stack

Recommended stack:

Frontend

-   React
-   TypeScript
-   Zustand
-   TanStack Query

Desktop Shell

-   Tauri

Backend

-   Strixonomy (Rust)

Rendering

-   WebGPU (future)
-   SVG/Canvas hybrid
-   Incremental graph rendering

------------------------------------------------------------------------

# Application Layout

    +----------------------------------------------------------------+
    | Menu | Workspace | Search | AI | Git | User                    |
    +----------------------------------------------------------------+

    | Explorer | Active Workspace | Inspector                        |
    |          |                  |                                  |
    |          |                  |                                  |
    +----------------------------------------------------------------+

    | Problems | Graph | Query | AI | Logs | Terminal                |
    +----------------------------------------------------------------+

------------------------------------------------------------------------

# Multiple Workspaces

Support opening multiple workspaces simultaneously.

Examples:

Clinical Ontology

Security Ontology

Reference Ontology

Each workspace maintains:

-   layout
-   history
-   tabs
-   graph views

------------------------------------------------------------------------

# Multi-Window

Future support:

-   Detached graph window
-   Detached reasoning dashboard
-   Presentation mode
-   Multi-monitor layouts

------------------------------------------------------------------------

# Graph Workspace

Desktop enables:

-   larger canvases
-   higher frame rates
-   GPU rendering
-   advanced layouts
-   presentation mode

------------------------------------------------------------------------

# AI Workspace

Dedicated orchestration environment.

Capabilities:

-   Long-running tasks
-   Project-wide documentation
-   Large-scale refactoring
-   Review generation
-   Architecture analysis

------------------------------------------------------------------------

# Offline First

Everything except cloud services should function offline.

Examples:

-   Editing
-   Reasoning
-   Graphs
-   Documentation
-   Local AI
-   Plugins

------------------------------------------------------------------------

# Local AI

Support:

-   Local LLMs
-   Cloud providers
-   Hybrid execution

Users control where inference occurs.

------------------------------------------------------------------------

# Plugin Marketplace

Desktop marketplace provides:

-   Browse
-   Install
-   Update
-   Ratings
-   Reviews
-   Publisher verification

Integrated into the application.

------------------------------------------------------------------------

# Collaboration

Desktop collaboration includes:

-   Reviews
-   Discussions
-   Semantic pull requests
-   Live collaboration
-   Notifications

------------------------------------------------------------------------

# Enterprise Features

Future support:

-   SSO
-   Managed plugins
-   Policy enforcement
-   Audit logs
-   Secure environments

------------------------------------------------------------------------

# Performance Targets

Startup

\<2 seconds

Workspace open

\<500 ms

Entity navigation

\<50 ms

Graph interaction

60 FPS target

Large ontology loading

Progressive

------------------------------------------------------------------------

# Distribution

Supported platforms:

-   Windows
-   macOS
-   Linux

Provide:

-   Native installers
-   Automatic updates
-   Portable mode
-   Enterprise deployment packages

------------------------------------------------------------------------

# Relationship to Strixonomy

Strixonomy:

-   VS Code extension
-   Lightweight
-   Embedded workflows

OntoStudio:

-   Dedicated desktop IDE
-   Full workspace experience
-   Advanced visualization
-   Large-scale collaboration

Both products share a common architecture and evolve together.

------------------------------------------------------------------------

# Future Opportunities

-   Cloud synchronization
-   Team workspaces
-   Shared semantic canvases
-   Mobile companion
-   Browser companion
-   VR/AR ontology exploration (research)

------------------------------------------------------------------------

# Success Criteria

OntoStudio succeeds when ontology engineers choose it as their primary
engineering environment because it combines the power of a modern IDE,
the flexibility of an infinite semantic workspace, the intelligence of
integrated AI, and the performance of a native Rust application. It
should stand on its own as the flagship desktop experience of the
Ontologos ecosystem while remaining fully compatible with Strixonomy and
every shared platform capability.
