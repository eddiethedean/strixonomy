# Capability providers

> **Historical architecture note.** Built-in capabilities ship in v0.14–v0.19 (reasoner, query, refactor, diagnostics, docs, plugin host). For **plugin authors**, start at **[Plugin authoring](../guides/plugins.md)** — not this page.

## Scope

**Capability Provider** is the plugin extensibility unit: a typed interface for reasoning, querying, AI, refactoring, diagnostics, import/export, and documentation generation. Third-party and first-party features register as providers — not raw VS Code commands.

## Provider catalog

| Capability | Interface (conceptual) | Built-in today | Plugin target |
|------------|------------------------|----------------|---------------|
| Reasoning | `ReasoningProvider` | Ontologos via LSP | v0.14 |
| Query | `QueryProvider` | SQL/SPARQL LSP | v0.14 |
| Refactoring | `RefactoringProvider` | LSP preview/apply | v0.14 |
| Diagnostics | `DiagnosticsProvider` | `strixonomy-diagnostics` | v0.14 |
| AI | `AIProvider` | — | v0.35 |
| Import/Export | `FormatProvider` | Turtle, OBO, ROBOT CLI | v0.14 |
| Documentation | `DocsProvider` | `strixonomy docs` | v0.14 |

## TypeScript interface sketch (planned)

```ts
interface CapabilityProvider {
  id: string
  version: string
  capabilities: CapabilityKind[]
}

interface ReasoningProvider extends CapabilityProvider {
  classify(profile: ReasonerProfile): Promise<ClassifyResult>
  explain(entityIri: string): Promise<Explanation | null>
}

interface RefactoringProvider extends CapabilityProvider {
  preview(op: RefactorOp): Promise<RefactorPreview>
  apply(op: RefactorOp): Promise<ApplyResult>
}
```

Rust-side host: see [design/PLUGIN_SPEC.md](../design/PLUGIN_SPEC.md) and [strixonomy/plugin-model.md](../strixonomy/plugin-model.md).

## Registration

```ts
interface CapabilityRegistry {
  register(provider: CapabilityProvider): void
  get<T extends CapabilityProvider>(kind: CapabilityKind, id?: string): T | undefined
  list(kind: CapabilityKind): CapabilityProvider[]
}
```

Strixonomy plugin host discovers providers from workspace config; OntoUI resolves UI contributions (inspector cards, commands) from provider metadata.

## Permissions

Providers declare required permissions (read workspace, modify ontology, network, AI). User approves at install/enable time. See [ui/PLUGIN_PLATFORM.md](../ui/PLUGIN_PLATFORM.md).

## Links

- [design/PLUGIN_SPEC.md](../design/PLUGIN_SPEC.md)
- [ui/PLUGIN_API_SPEC.md](../ui/PLUGIN_API_SPEC.md)
- [cursor-prompts/10-add-capability-provider-interfaces.md](../cursor-prompts/10-add-capability-provider-interfaces.md)

## Evolution

Unifies [ui/PLUGIN_PLATFORM.md](../ui/PLUGIN_PLATFORM.md) and [design/PLUGIN_SPEC.md](../design/PLUGIN_SPEC.md) under **Capability Provider** terminology.
