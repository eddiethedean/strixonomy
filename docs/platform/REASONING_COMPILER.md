# Reasoning compiler architecture

> **Status:** Partial (v0.12) — classify + explanations shipped; continuous compiler UX planned

## Scope

Treat OWL reasoning like a **compiler**: incremental classification, semantic diagnostics, explanation emission, and UI subscription via WorkspaceStore events.

## Pipeline

```text
Source ontologies (indexed)
    ↓
Profile selection (el | rl | rdfs | dl | auto)
    ↓
strixonomy/runReasoner (LSP) → Ontologos
    ↓
ClassifyResult + unsatisfiable classes
    ↓
Diagnostics emission (problems panel target)
    ↓
Explanation cache (DL clash traces v0.12+)
    ↓
UI: hierarchy mode (asserted | inferred | combined)
```

## Implemented (v0.12)

- LSP `strixonomy/runReasoner`, `strixonomy/getExplanation`
- VS Code reasoner + explanation React panels
- CLI `strixonomy classify`
- EL/RL/RDFS/DL/auto profiles via Ontologos 1.0

## Compiler-like behaviors (target)

| Behavior | Status |
|----------|--------|
| Run on workspace open (configurable) | Planned |
| Incremental re-classify on edit | Planned |
| Problems panel for unsatisfiable classes | Planned v0.13 |
| Reasoning history in store | Planned v1.0 |
| Quick fixes linked to diagnostics | Partial (LSP codeAction) |

## Store integration (planned)

```ts
interface ReasoningState {
  profile: ReasonerProfile
  lastRun: ReasonerRunSummary | null
  unsatisfiable: string[]
  hierarchyMode: "asserted" | "inferred" | "combined"
  explanations: Record<string, Explanation>
}
```

Events: `ReasoningStarted`, `ReasoningCompleted`, `ReasoningFailed`.

## Links

- [design/REASONER_SPEC.md](../design/REASONER_SPEC.md)
- [ui/REASONING_EXPERIENCE.md](../ui/REASONING_EXPERIENCE.md)
- [guides/reasoner.md](../guides/reasoner.md)
- [cursor-prompts/12-reasoning-diagnostics-workflow.md](../cursor-prompts/12-reasoning-diagnostics-workflow.md)

## Evolution

"Reasoning-as-compiler" vision from [ui/REASONING_EXPERIENCE.md](../ui/REASONING_EXPERIENCE.md); this doc specifies implementation boundaries.
