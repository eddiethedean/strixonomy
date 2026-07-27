# Event Sequence Diagrams

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


## 1. Entity Selection

```text
User clicks entity
↓
Explorer emits EntitySelected
↓
WorkspaceStore updates CurrentFocus
↓
EventBus emits FocusChanged
↓
EntityEditor loads entity
↓
Inspector updates cards
↓
Graph centers node
↓
AI context refreshes
↓
Breadcrumbs update
```

## 2. Inline Rename

```text
User edits name
↓
InlineEditor validates locally
↓
RenameCommand preview requested
↓
Strixonomy computes impacted references
↓
Preview shown
↓
User confirms
↓
Command executes transaction
↓
Workspace index updates
↓
Diagnostics refresh
↓
Undo stack records semantic command
```

## 3. Query Execution

```text
User presses Run
↓
QueryWorkbench emits QueryRequested
↓
Strixonomy validates query
↓
Query engine executes
↓
Results stream to UI
↓
Table renders virtualized rows
↓
Graph view receives result graph
↓
History records execution
```

## 4. Reasoning Run

```text
Workspace changed
↓
Reasoning scheduler debounces
↓
Reasoner starts background run
↓
Progress events update status
↓
Classification completed
↓
Diagnostics generated
↓
Problems panel updates
↓
Graph overlays update
↓
Entity reasoning cards update
```

## 5. AI Suggestion

```text
Current focus changes
↓
AI context engine builds structured context
↓
Suggestion provider evaluates context
↓
SuggestionCard appears
↓
User opens preview
↓
AI produces proposed semantic changes
↓
User applies
↓
Semantic command executes
↓
Undo stack records change
```

## 6. Plugin Activation

```text
Workspace opened
↓
Plugin manager checks activation events
↓
Permissions validated
↓
Plugin sandbox starts
↓
Plugin registers capabilities
↓
UI extension slots update
↓
Commands appear in palette
```
