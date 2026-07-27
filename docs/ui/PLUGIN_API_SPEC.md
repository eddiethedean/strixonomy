# Plugin API Specification

> **Document type:** Product design specification (target state). **Not a shipped feature list.**
>
> **Do not implement from this page.** For the shipping VS Code / CLI plugin host, use **[Plugin authoring](../guides/plugins.md)** only.
>
> See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


## 1. Purpose

The plugin API allows third-party developers to extend Strixonomy while preserving performance, security, and UX consistency.

## 2. Manifest

```json
{
  "id": "example.obo-tools",
  "name": "OBO Tools",
  "version": "0.1.0",
  "publisher": "Example",
  "activationEvents": ["workspaceContains:*.obo"],
  "permissions": ["workspace.read", "workspace.write"],
  "contributes": {
    "commands": [],
    "inspectorCards": [],
    "reasoners": [],
    "aiProviders": []
  }
}
```

## 3. Capability Interfaces

```ts
interface PluginContext {
  workspace: WorkspaceApi
  commands: CommandRegistry
  events: EventBus
  ui: UIRegistry
  storage: PluginStorage
  ai: AIRegistry
}
```

## 4. Command Contribution

```ts
context.commands.register({
  id: "obo.normalize",
  title: "Normalize OBO Ontology",
  scope: "workspace",
  execute: async () => {}
})
```

## 5. Inspector Card Contribution

```ts
context.ui.registerInspectorCard({
  id: "obo-term-card",
  appliesTo: ["class"],
  render: OboTermCard
})
```

## 6. Reasoner Provider

```ts
interface ReasonerProvider {
  id: string
  label: string
  classify(workspace: WorkspaceSnapshot): Promise<ReasoningResult>
}
```

## 7. AI Provider

```ts
interface AIProvider {
  id: string
  label: string
  complete(request: AIRequest): AsyncIterable<AIChunk>
}
```

## 8. Refactoring Provider

```ts
interface RefactoringProvider {
  id: string
  analyze(target: SemanticRef): Promise<RefactoringPreview>
  apply(previewId: string): Promise<TransactionResult>
}
```

## 9. Permissions

Permissions are explicit:

- workspace.read
- workspace.write
- filesystem.read
- filesystem.write
- network
- ai.invoke
- git.read
- git.write

## 10. Security

Plugins run sandboxed. Unsafe APIs require user approval.

## 11. Testing

Plugin SDK includes:

- unit test harness
- fake workspace
- snapshot testing
- UI slot testing
- command testing
