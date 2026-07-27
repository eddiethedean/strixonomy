# AI orchestration

> **Status:** Proposed (v1.1+ AI workflows) · **ADR:** [0006-ai-preview-before-apply.md](../adr/0006-ai-preview-before-apply.md), [design/adr/0010-ai-features-opt-in.md](../design/adr/0010-ai-features-opt-in.md)

## Scope

Normative architecture for AI-assisted ontology editing: context building, tool routing, and **safe semantic patch workflow**. AI features are **opt-in** per [ADR-0010](../design/adr/0010-ai-features-opt-in.md).

## Action lifecycle

```text
Idle → ContextBuilt → ProposalGenerated → PatchValidated → PreviewShown
  → UserApproved → Applied → Indexed → AuditLogged
  → UserRejected → Idle
```

| State | Description |
|-------|-------------|
| `ContextBuilt` | Current Focus, entity, diagnostics, graph neighborhood assembled |
| `ProposalGenerated` | Model returns structured patch or refactor ops |
| `PatchValidated` | Strixonomy validates ops against catalog |
| `PreviewShown` | User sees diff / impact summary (**required**) |
| `Applied` | LSP `applyAxiomPatch` or refactor apply after explicit approval |

**Rule:** No direct apply from AI without preview + user approval ([ADR-0006](../adr/0006-ai-preview-before-apply.md)).

## Context builder inputs

- Current Focus ([WORKSPACE_RUNTIME.md](WORKSPACE_RUNTIME.md))
- Entity metadata and axioms
- Reasoning state and unsatisfiable classes
- Active diagnostics
- Query result snippets (optional)
- Git diff (optional)

## AI tools (structured, not free text)

| Tool | Returns |
|------|---------|
| `readEntity` | Entity detail from LSP |
| `findReferences` | Usage list |
| `runQuery` | Query result rows |
| `explainInference` | Explanation payload |
| `previewRefactoring` | Refactor preview |
| `generateDocs` | Markdown draft |
| `validateChanges` | Diagnostic list |

## Provider router

Supports OpenAI, Anthropic, local models, enterprise endpoints, and **Capability Provider** plugins implementing `AIProvider`. Routes through host network policy.

## Store integration (planned)

```ts
interface AIState {
  lifecycle: AILifecycleState
  pendingProposal: PatchProposal | null
  auditLog: AIAuditEntry[]
}
```

WorkspaceStore slice; AI sidebar and inline suggestions subscribe to lifecycle events.

## Links

- [ui/AI_ORCHESTRATION_ARCHITECTURE.md](../ui/AI_ORCHESTRATION_ARCHITECTURE.md) (UX detail)
- [ui/AI_EXPERIENCE.md](../ui/AI_EXPERIENCE.md)
- [SEMANTIC_REFACTORING.md](SEMANTIC_REFACTORING.md) (shared preview envelope)
- [cursor-prompts/09-add-ai-action-lifecycle.md](../cursor-prompts/09-add-ai-action-lifecycle.md)

## Evolution

Normative workflow extracted from [ui/AI_ORCHESTRATION_ARCHITECTURE.md](../ui/AI_ORCHESTRATION_ARCHITECTURE.md) § Change Safety.
