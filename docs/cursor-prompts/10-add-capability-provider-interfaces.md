# Cursor prompt: Add Capability Provider TypeScript interfaces

## Prerequisites

Read:

- [ ] `platform/CAPABILITY_PROVIDERS.md`
- [ ] [0005-capability-provider-plugin-model.md](../adr/0005-capability-provider-plugin-model.md)

## Non-goals

- Rust plugin host
- Dynamic plugin loading

## Current state

- No capability registry

## Tasks

1. Create extension/webview-ui/src/capabilities/types.ts with CapabilityProvider base + ReasoningProvider, QueryProvider stubs
2. Create extension/webview-ui/src/capabilities/registry.ts with register/list
3. Register built-in 'strixonomy' stubs that delegate to existing LSP calls
4. Add capabilities/registry.test.ts

## Acceptance criteria

- [ ] Built-in providers registered
- [ ] Types export cleanly

## Tests

- `cd extension/webview-ui && npm test`

## Do not

- Do not implement WASM/IPC sandbox

## References

- [Cursor prompts index](README.md)
