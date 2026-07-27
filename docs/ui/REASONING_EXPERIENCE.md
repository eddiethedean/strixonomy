# REASONING_EXPERIENCE.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy Reasoning Experience Specification

## Purpose

Reasoning is one of the defining capabilities of ontology engineering. In Strixonomy, reasoning should feel like compiling software in a modern IDE.

> **Implementation architecture:** [platform/REASONING_COMPILER.md](../platform/REASONING_COMPILER.md) · **Partial:** reasoner panels v0.9–v0.12; reasoning store slice v0.13

## Continuous Feedback

Reasoning should run automatically when practical.

Users should immediately see:

-   New inferences
-   Broken assumptions
-   Unsatisfiable classes
-   Missing relationships
-   Modeling issues

Long-running reasoners execute asynchronously.

------------------------------------------------------------------------

## Explain Everything

Every inference should answer:

-   What was inferred?
-   Why was it inferred?
-   Which axioms contributed?
-   Can I navigate to them?

Reasoning should never feel like a black box.

------------------------------------------------------------------------

## Workspace Integrated

Reasoning is not its own application.

Results appear naturally inside:

-   Entity Editor
-   Graph Workspace
-   Explorer
-   Problems Panel
-   Query Workbench
-   AI
-   Semantic Refactoring

------------------------------------------------------------------------

# Build Model

Reasoning behaves like a semantic build.

Stages:

1.  Parse
2.  Validate
3.  Classify
4.  Infer
5.  Diagnose
6.  Generate workspace diagnostics

Users can see progress throughout the pipeline.

------------------------------------------------------------------------

# Problems Panel

Reasoning contributes diagnostics alongside parser and validation
issues.

Severity levels:

-   Error
-   Warning
-   Information
-   Suggestion

Each diagnostic includes:

-   Description
-   Explanation
-   Source axioms
-   Navigation links
-   Suggested fixes

------------------------------------------------------------------------

# Entity-Level Reasoning

Every entity displays:

-   Inferred parents
-   Equivalent classes
-   Unsatisfied restrictions
-   Active diagnostics
-   Explanation links

Reasoning is contextual rather than global.

------------------------------------------------------------------------

# Graph Integration

Reasoning overlays may visualize:

-   Inferred edges
-   Redundant edges
-   Cycles
-   Inconsistencies
-   Equivalent classes

Users can toggle each overlay independently.

------------------------------------------------------------------------

# Reasoning Dashboard

A dedicated workspace summarizes:

-   Overall ontology health
-   Classification status
-   Unsatisfiable classes
-   Diagnostics by severity
-   Recent reasoning runs
-   Performance statistics

This complements---not replaces---contextual feedback.

------------------------------------------------------------------------

# Explain Inference

Selecting an inferred relationship opens an explanation.

Display:

Inference

↓

Supporting axioms

↓

Reasoning chain

↓

Visualization

↓

Suggested improvements

Users should understand every conclusion.

------------------------------------------------------------------------

# Quick Fixes

Diagnostics expose semantic code actions.

Examples:

-   Add missing restriction
-   Merge duplicate classes
-   Remove redundant axiom
-   Normalize hierarchy
-   Generate documentation

Quick fixes integrate with Semantic Refactoring.

------------------------------------------------------------------------

# AI Integration

AI assists with:

-   Explaining reasoning
-   Summarizing inference chains
-   Suggesting repairs
-   Detecting anti-patterns
-   Predicting reasoning impact

AI never replaces the formal reasoner.

------------------------------------------------------------------------

# Multiple Reasoners

Support multiple reasoning engines.

Examples:

-   ELK
-   HermiT
-   Pellet
-   Future Rust-native engines

Users may compare results when appropriate.

------------------------------------------------------------------------

# Performance

Support:

-   Incremental reasoning
-   Cached classifications
-   Background execution
-   Cancellation
-   Parallel processing

Large ontologies should remain responsive.

------------------------------------------------------------------------

# Build History

Maintain history of reasoning runs.

Track:

-   Timestamp
-   Duration
-   Reasoner
-   Diagnostics
-   Classification changes
-   Performance metrics

Useful for regression analysis.

------------------------------------------------------------------------

# Plugin Extension Points

Plugins may contribute:

-   Reasoners
-   Diagnostics
-   Visualizations
-   Explanation providers
-   Quick fixes
-   Dashboards

------------------------------------------------------------------------

# Accessibility

Reasoning feedback must support:

-   Keyboard navigation
-   Screen readers
-   High contrast
-   Reduced motion
-   Plain-language explanations

------------------------------------------------------------------------

# Success Criteria

The reasoning experience succeeds when ontology engineers think about
semantic correctness the same way software engineers think about
compilation: an always-available source of confidence, guidance, and
continuous feedback. Users should be able to understand not only *that*
something is inferred or inconsistent, but *why*, how to navigate to the
supporting evidence, and what actions they can take next---all without
leaving their current workflow.
