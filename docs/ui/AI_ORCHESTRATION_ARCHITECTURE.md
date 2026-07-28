# AI Orchestration Architecture

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


## 1. Purpose

AI orchestration coordinates models, prompts, tools, context, permissions, previews, and semantic changes.

> **Implementation architecture:** [platform/AI_ORCHESTRATION.md](../platform/AI_ORCHESTRATION.md) · **Status:** proposed v0.35

## 2. Architecture

```text
AI Entry Points
├── Command Palette
├── Inline Suggestions
├── AI Sidebar
├── Context Menus
└── Workflow Panels

AI Orchestrator
├── Context Builder
├── Prompt Router
├── Tool Registry
├── Provider Router
├── Response Streamer
├── Preview Generator
└── Audit Logger
```

## 3. Context Builder

Structured context includes:

- Current focus.
- Entity metadata.
- Relationships.
- Reasoning state.
- Diagnostics.
- Graph neighborhood.
- Query results.
- Git changes.
- User selection.

## 4. AI Tools

AI may request tools:

- readEntity
- findReferences
- runQuery
- explainInference
- previewRefactoring
- generateDocs
- validateChanges

Tools return structured data.

## 5. Provider Router

Supports:

- OpenAI
- Anthropic
- local models
- enterprise providers
- plugin providers

## 6. Change Safety

AI-generated changes follow:

1. Generate proposal.
2. Convert to semantic patch.
3. Validate patch.
4. Preview impact.
5. Require user approval.
6. Apply transaction.
7. Record audit event.

## 7. Prompt Templates

Templates are versioned and testable.

Example categories:

- explain entity
- review ontology
- generate documentation
- repair diagnostic
- propose refactoring
- summarize pull request

## 8. Memory

AI memory is scoped.

- session memory
- workspace memory
- project rules
- team prompt libraries

Sensitive context requires explicit policy.

## 9. Evaluation

AI workflows should have regression tests using expected structured outputs.

## 10. Success

AI succeeds when it feels like a trustworthy semantic collaborator, not a generic chatbot.
