# Semantic refactoring architecture

> **Status:** Partial (v0.13) — LSP preview/apply + `pendingRefactor` store slice shipped; unified transaction model planned v0.30

## Scope

Transaction model for semantic refactoring: preview → validate → apply → reindex → undo. Maps to existing `strixonomy/previewRefactor` and `strixonomy/applyRefactor` LSP methods.

## Transaction pipeline

```text
1. User initiates refactor (command or workspace action)
2. previewRefactor → impact summary + affected files
3. User reviews in Refactor Preview workspace
4. applyRefactor → disk write + workspace edit
5. Reindex (automatic or explicit Index Workspace)
6. Undo stack records inverse ops (planned)
```

## Implemented (v0.12)

| Op | LSP | UI |
|----|-----|-----|
| Rename IRI | `previewRefactor` / `applyRefactor` | RefactorPreview panel |
| Move entity | same | RefactorPreview panel |
| Extract module | same | RefactorPreview panel |
| Namespace migration | same | RefactorPreview panel |

Rust: `strixonomy-refactor` crate. See [guides/refactoring.md](../guides/refactoring.md).

## Planned enhancements

- Merge classes, batch label normalization (v0.30 UI track)
- WorkspaceStore holds `pendingRefactor` state shared across workspaces
- AI-assisted refactor proposals use same preview envelope ([AI_ORCHESTRATION.md](AI_ORCHESTRATION.md))
- Reasoning-aware validation before apply (unsatisfiability check)

## Store shape (planned)

```ts
interface RefactoringState {
  pending: RefactorPreview | null
  history: RefactorTransaction[]
  undoStack: RefactorTransaction[]
}
```

## Acceptance criteria (v0.13 integration)

- Preview required before every apply from UI
- Apply failure surfaces LSP diagnostics in store
- Refactor Preview workspace reads from WorkspaceStore, not isolated local state

## Links

- [ui/SEMANTIC_REFACTORING.md](../ui/SEMANTIC_REFACTORING.md)
- [design/SEMANTIC_DIFF_SPEC.md](../design/SEMANTIC_DIFF_SPEC.md)
- [cursor-prompts/11-semantic-refactoring-preview-workflow.md](../cursor-prompts/11-semantic-refactoring-preview-workflow.md)

## Evolution

Architecture consolidated from [ui/SEMANTIC_REFACTORING.md](../ui/SEMANTIC_REFACTORING.md); UX principles remain in ui/.
