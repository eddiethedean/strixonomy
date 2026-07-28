# Cursor implementation prompts

Step-by-step prompts for implementing the OntoUI platform (v0.13+). **Read [SHIPPED.md](../SHIPPED.md) and [glossary.md](../glossary.md) first.**

## Rules

1. Label **implemented vs planned** in code comments where behavior is stubbed.
2. Do not implement OntoStudio shell, full plugin host runtime, or AI provider backends unless the prompt says so.
3. Run tests after each prompt: `cd extension/webview-ui && npm test`
4. Match existing TypeScript/React patterns in `extension/webview-ui/`.
5. Ontology logic stays in Rust/LSP — webviews call LSP only.

## Prompt index

| # | Prompt | Target release |
|---|--------|----------------|
| 01 | [build-ontoui-workspace-platform](01-build-ontoui-workspace-platform.md) | v0.13 |
| 02 | [add-workspacestore](02-add-workspacestore.md) | v0.13 |
| 03 | [add-workspaceregistry](03-add-workspaceregistry.md) | v0.13 |
| 04 | [migrate-panels-to-workspaces](04-migrate-panels-to-workspaces.md) | v0.13 |
| 05 | [implement-current-focus](05-implement-current-focus.md) | v0.13 |
| 06 | [improve-entity-workspace](06-improve-entity-workspace.md) | v0.13–v0.30 |
| 07 | [improve-graph-workspace](07-improve-graph-workspace.md) | v0.13–v0.30 |
| 08 | [improve-query-workbench](08-improve-query-workbench.md) | v0.13 |
| 09 | [add-ai-action-lifecycle](09-add-ai-action-lifecycle.md) | v0.31+ |
| 10 | [add-capability-provider-interfaces](10-add-capability-provider-interfaces.md) | v0.14 |
| 11 | [semantic-refactoring-preview-workflow](11-semantic-refactoring-preview-workflow.md) | v0.13 |
| 12 | [reasoning-diagnostics-workflow](12-reasoning-diagnostics-workflow.md) | v0.13–v0.30 |
| 13 | [design-tokens-component-library](13-design-tokens-component-library.md) | v0.13 |

**Recommended order:** 01 → 02 → 05 → 03 → 04 → 06–08 → 11–13 → 09–10

## Architecture references

- [platform/OVERVIEW.md](../platform/OVERVIEW.md)
- [platform/ONTOUI.md](../platform/ONTOUI.md)
- [platform/WORKSPACE_RUNTIME.md](../platform/WORKSPACE_RUNTIME.md)
- [adr/README.md](../adr/README.md)
