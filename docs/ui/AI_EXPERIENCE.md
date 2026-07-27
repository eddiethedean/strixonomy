# AI_EXPERIENCE.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy AI Experience Specification

## Purpose

Artificial Intelligence is not a separate feature of Strixonomy.

It is a capability woven throughout every workflow.

The goal is to create the world's first AI-native ontology engineering
environment where AI augments human expertise without replacing human
judgment.

------------------------------------------------------------------------

# Vision

AI should behave like an experienced ontology engineer sitting beside
the user.

It should:

-   explain
-   teach
-   recommend
-   automate
-   review
-   document
-   validate

while always leaving the human in control.

------------------------------------------------------------------------

# Core Principles

## Context First

AI always understands:

-   Current Focus
-   Open workspace
-   Current ontology
-   Current graph neighborhood
-   Current query
-   Diagnostics
-   Git changes
-   User selection

Users should never have to repeatedly explain context.

------------------------------------------------------------------------

## AI Everywhere

There is no dedicated "AI Mode."

Every surface exposes contextual intelligence.

Examples:

Entity Editor

-   Explain entity
-   Improve documentation
-   Suggest relationships

Graph Workspace

-   Explain neighborhood
-   Detect patterns
-   Recommend layout

Query Workbench

-   Generate query
-   Optimize query
-   Explain execution

Reasoning

-   Explain inconsistency
-   Suggest repair

------------------------------------------------------------------------

# AI Interaction Model

Every AI action follows the same lifecycle.

1.  User requests assistance.

2.  AI gathers semantic context.

3.  AI produces:

-   explanation
-   recommendation
-   preview

4.  User reviews.

5.  User accepts, edits, or dismisses.

AI never silently changes the ontology.

------------------------------------------------------------------------

# AI Capabilities

## Explain

Explain:

-   entities
-   relationships
-   restrictions
-   reasoning
-   diagnostics
-   queries
-   ontology structure

Responses should assume the user's expertise level.

------------------------------------------------------------------------

## Generate

Generate:

-   documentation
-   labels
-   comments
-   examples
-   queries
-   SHACL
-   SQL
-   SPARQL

Generated content is editable before insertion.

------------------------------------------------------------------------

## Review

Review:

-   modeling quality
-   anti-patterns
-   naming
-   documentation
-   consistency
-   ontology organization

Highlight opportunities---not just problems.

------------------------------------------------------------------------

## Refactor

Recommend:

-   normalization
-   modularization
-   merge entities
-   split entities
-   simplify restrictions

Every refactor includes:

-   explanation
-   preview
-   undo

------------------------------------------------------------------------

## Repair

Detect:

-   inconsistencies
-   missing documentation
-   duplicate entities
-   cyclic structures
-   weak modeling

Offer guided repair workflows.

------------------------------------------------------------------------

# AI Suggestions

Suggestions appear inline.

Examples:

✨ Missing documentation

✨ Equivalent classes detected

✨ Possible duplicate

✨ Relationship may be redundant

Suggestions never interrupt editing.

------------------------------------------------------------------------

# AI Sidebar

Optional sidebar displays:

Recent conversations

Workspace context

Suggested actions

History

Saved prompts

The sidebar supplements---not replaces---contextual actions.

------------------------------------------------------------------------

# Semantic Context

AI receives structured context instead of raw text whenever possible.

Examples:

Current entity

Relationships

Reasoning state

Diagnostics

Graph neighborhood

Workspace history

This produces more reliable recommendations.

------------------------------------------------------------------------

# Explainability

Every recommendation answers:

What?

Why?

How confident?

What changes?

Can I undo this?

Transparency builds trust.

------------------------------------------------------------------------

# Prompt Model

Users may interact through:

Natural language

Slash commands

Command palette

Toolbar actions

Context menus

Inline suggestions

The same AI capability should be accessible from multiple entry points.

------------------------------------------------------------------------

# AI Workflows

Examples

"Document this ontology."

"Normalize this module."

"Generate SHACL."

"Review pull request."

"Summarize graph."

"Explain reasoning."

"Generate onboarding guide."

AI workflows may span multiple steps while remaining reviewable.

------------------------------------------------------------------------

# Collaboration

Future features:

Shared AI conversations

Saved prompts

Team prompt libraries

AI-assisted reviews

Ontology design discussions

------------------------------------------------------------------------

# Safety

AI never:

Deletes content without confirmation

Publishes changes automatically

Hides uncertainty

Claims unsupported reasoning

Users remain the final authority.

------------------------------------------------------------------------

# Performance

Targets

Context gathering

\<100 ms

Small responses

2--5 seconds

Large generation

Streaming

Suggestions

Asynchronous

AI should never block the IDE.

------------------------------------------------------------------------

# Plugin Architecture

Third-party providers may contribute:

Models

Prompt templates

Reviewers

Refactoring engines

Documentation generators

Validation assistants

All providers implement a common interface.

------------------------------------------------------------------------

# Success Criteria

The AI experience succeeds when users stop thinking of AI as a chatbot
and instead experience it as an intelligent layer integrated into every
aspect of ontology engineering. AI should feel like a trusted
collaborator that understands the current semantic context, explains its
reasoning, previews every change, and helps users produce higher-quality
ontologies without taking control away from them.
