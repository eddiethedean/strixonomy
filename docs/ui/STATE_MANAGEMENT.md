# State Management Specification

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


## 1. Purpose

The UI must behave as one coherent semantic workspace. This requires a formal state model.

> **Implementation architecture:** [platform/WORKSPACE_RUNTIME.md](../platform/WORKSPACE_RUNTIME.md) · **Status:** WorkspaceStore and event bus **shipped v0.13**

## 2. WorkspaceStore

`WorkspaceStore` is the single source of truth for global OntoUI state (**shipped v0.13**).

```ts
interface WorkspaceStore {
  workspace: WorkspaceState
  focus: FocusState
  selection: SelectionState
  tabs: TabState
  layout: LayoutState
  explorer: ExplorerState
  inspector: InspectorState
  dock: DockState
  graph: GraphState
  query: QueryState
  reasoning: ReasoningState
  diagnostics: DiagnosticsState
  ai: AIState
  plugins: PluginState
  navigation: NavigationState
  persistence: PersistenceState
}
```

## 3. Current Focus

```ts
type FocusKind =
  | "entity"
  | "axiom"
  | "query"
  | "diagnostic"
  | "graphNode"
  | "documentation"
  | "review"
  | "workspace"

interface CurrentFocus {
  kind: FocusKind
  id: string
  source: string
  timestamp: number
}
```

Changing focus emits `FocusChanged`.

## 4. Selection

Selection may be single or multi-object.

```ts
interface SelectionState {
  primary?: SemanticRef
  items: SemanticRef[]
  mode: "single" | "multi" | "range"
}
```

## 5. Event Bus

Typed event bus only.

```ts
type WorkspaceEvent =
  | EntitySelected
  | FocusChanged
  | TabOpened
  | QueryExecuted
  | ReasoningCompleted
  | DiagnosticsUpdated
  | RefactoringPreviewCreated
  | RefactoringApplied
  | AISuggestionCreated
  | PluginActivated
```

## 6. Synchronization Rules

- Components subscribe to state slices.
- Components do not manually synchronize with each other.
- Derived state is computed through selectors.
- Long-running operations publish lifecycle events.
- Every semantic mutation goes through a command.

## 7. Command Model

```ts
interface WorkspaceCommand {
  id: string
  label: string
  execute(ctx: CommandContext): Promise<CommandResult>
  undo?(ctx: CommandContext): Promise<void>
  preview?(ctx: CommandContext): Promise<Preview>
}
```

## 8. Undo/Redo

Undo/redo is semantic.

Examples:

- Rename class.
- Move property.
- Merge classes.
- Apply AI documentation.
- Accept refactoring.

Transactions group low-level changes into user-meaningful actions.

## 9. Persistence

Persist:

- Layout sizes.
- Open tabs.
- Recent entities.
- Favorites.
- Graph layouts.
- Saved queries.
- AI conversation summaries.
- Plugin settings.
- Workspace preferences.

## 10. Lifecycle

```text
Boot
↓
Load persisted shell state
↓
Initialize host adapter
↓
Load workspace
↓
Initialize Strixonomy session
↓
Activate required plugins
↓
Hydrate WorkspaceStore
↓
Render application
↓
Persist on change / shutdown
```

## 11. Error State

All async operations expose:

- idle
- loading
- success
- error
- cancelled

Never leave components in ambiguous states.

## 12. Testing Requirements

- Store reducers/selectors unit tested.
- Event ordering tested.
- Undo/redo transaction tests.
- Persistence migration tests.
- Plugin lifecycle tests.
