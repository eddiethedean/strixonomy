# PLUGIN_PLATFORM.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.**
>
> **Do not implement from this page.** Canonical plugin authoring: [guides/plugins.md](../guides/plugins.md). Capability-provider sketches below describe a future OntoUI layer; the VS Code host today uses TOML manifests + subprocess plugins (shipped since v0.14).
>
> See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy Plugin Platform Specification

## Purpose

The Plugin Platform transforms Strixonomy into an ecosystem via **Capability Providers** — typed extension interfaces for reasoning, query, AI, refactoring, and more.

> **Architecture:** [platform/CAPABILITY_PROVIDERS.md](../platform/CAPABILITY_PROVIDERS.md) · **Engineering history:** [design/PLUGIN_SPEC.md](../design/PLUGIN_SPEC.md) · **Shipped host (v0.14+):** [Plugin authoring](../guides/plugins.md)

Every major capability should be implemented as a Capability Provider or exposed through the same APIs available to third-party developers.

## Everything Extensible

Extension points exist for:

-   Workspaces
-   Entity editors
-   Inspector cards
-   Explorer nodes
-   Graph overlays
-   Query languages
-   AI providers
-   Reasoners
-   Refactorings
-   Diagnostics
-   Documentation generators
-   Import/export formats
-   Commands
-   Themes
-   Icons
-   Keybindings

------------------------------------------------------------------------

## Stable APIs

Public APIs should be versioned and semantically stable.

Breaking changes are minimized and communicated well in advance.

------------------------------------------------------------------------

## Sandbox by Default

Plugins execute in isolated environments with explicit permissions.

Permissions include:

-   Read workspace
-   Modify ontology
-   Execute queries
-   Invoke AI
-   Access filesystem
-   Network access

Users can review and revoke permissions.

------------------------------------------------------------------------

# Architecture

    +------------------------------------------------------+
    |                 Strixonomy Platform                    |
    +------------------------------------------------------+
    | Workspace API | Graph API | Query API | AI API       |
    | Reasoning API | UI API    | Events    | Storage API  |
    +------------------------------------------------------+
    | Plugin Runtime                                      |
    +------------------------------------------------------+
    | Installed Plugins                                   |
    +------------------------------------------------------+

------------------------------------------------------------------------

# Plugin Types

## Workspace Plugins

Add new workspaces such as:

-   Architecture
-   Review
-   Analytics

## UI Plugins

Contribute:

-   Inspector cards
-   Toolbars
-   Panels
-   Status indicators

## Language Plugins

Support additional semantic languages:

-   RDF
-   SHACL
-   SKOS
-   Custom DSLs

## Reasoning Plugins

Provide:

-   Reasoners
-   Validators
-   Explanation engines

## AI Plugins

Contribute:

-   Models
-   Prompt libraries
-   Agents
-   Review assistants

## Visualization Plugins

Add:

-   Charts
-   Graph layouts
-   Maps
-   Timelines

------------------------------------------------------------------------

# Plugin Lifecycle

1.  Discover
2.  Install
3.  Validate
4.  Activate
5.  Run
6.  Suspend
7.  Update
8.  Uninstall

Plugins may activate lazily based on workspace context.

------------------------------------------------------------------------

# Workspace Integration

Plugins interact through Workspace APIs.

They consume:

-   Current Focus
-   Workspace State
-   Navigation
-   Events

They should never duplicate global state.

------------------------------------------------------------------------

# Event System

Plugins subscribe to events such as:

-   EntitySelected
-   WorkspaceOpened
-   QueryExecuted
-   ReasoningCompleted
-   DiagnosticsUpdated
-   AICompleted

They may publish their own events.

------------------------------------------------------------------------

# Commands

Plugins register commands discoverable through:

-   Command Palette
-   Context menus
-   Toolbars
-   Keyboard shortcuts

Commands should support undo where applicable.

------------------------------------------------------------------------

# UI Contribution Model

Plugins may contribute:

-   Explorer sections
-   Workspace tabs
-   Inspector cards
-   Dock tools
-   Context menus
-   Graph decorations
-   Entity badges
-   Status bar items

All UI must use the shared design system.

------------------------------------------------------------------------

# Storage

Provide isolated plugin storage for:

-   Settings
-   Caches
-   Templates
-   Session data

No plugin should manipulate another plugin's data directly.

------------------------------------------------------------------------

# Marketplace

Future marketplace features:

-   Ratings
-   Reviews
-   Verified publishers
-   Dependency resolution
-   Automatic updates
-   Compatibility checks

------------------------------------------------------------------------

# Security

Requirements:

-   Signed plugins
-   Permission prompts
-   API isolation
-   Secure updates
-   Crash isolation

Unsafe plugins should not compromise the IDE.

------------------------------------------------------------------------

# Performance

Targets:

-   Startup impact \<100 ms per active plugin
-   Lazy loading by default
-   Background initialization
-   Memory budgeting
-   Profiling tools

------------------------------------------------------------------------

# Developer Experience

Provide:

-   SDK
-   Templates
-   CLI
-   Testing framework
-   Hot reload
-   Documentation
-   Sample plugins

Building plugins should be approachable.

------------------------------------------------------------------------

# Example Plugins

-   Ontology metrics dashboard
-   Biomedical ontology toolkit
-   OBO Foundry utilities
-   SHACL validator
-   ROBOT/OWLMake integration
-   GitHub review assistant
-   Mermaid exporter
-   Architecture explorer

------------------------------------------------------------------------

# Success Criteria

The Plugin Platform succeeds when the community can extend Strixonomy in
ways the core team never anticipated while preserving a consistent user
experience. Every plugin should feel native, performant, secure, and
deeply integrated with the semantic workspace rather than bolted onto
the side of the application.
