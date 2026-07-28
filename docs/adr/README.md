# Product & platform ADRs

Architecture decisions for **OntoUI**, workspace model, plugins, AI, and OntoStudio — product/platform layer.

**Engineering ADRs** (Strixonomy crates, parsers, reasoners): [design/adr/README.md](../design/adr/README.md)

## Index

| ADR | Title | Status |
|-----|-------|--------|
| [0001](0001-ontoui-shared-react-platform.md) | OntoUI as shared React platform | Accepted (v0.13 foundation shipped) |
| [0002](0002-workspace-over-panel-model.md) | Workspace model over panel model | Accepted (implemented v0.13) |
| [0003](0003-current-focus-central-ux.md) | Current Focus as central UX concept | Accepted (implemented v0.13) |
| [0004](0004-workspacestore-ui-source-of-truth.md) | WorkspaceStore as UI source of truth | Accepted (implemented v0.13) |
| [0005](0005-capability-provider-plugin-model.md) | Capability Provider plugin model | Accepted (shipped v0.14+) |
| [0006](0006-ai-preview-before-apply.md) | AI changes require preview before apply | Accepted (proposed v0.35) |
| [0007](0007-ontostudio-shares-platform.md) | OntoStudio shares UI/platform with Strixonomy | Accepted (planned post v0.30) |

## When to write a product ADR

- Cross-cutting OntoUI / Strixonomy / OntoStudio decisions
- UX architecture that affects multiple workspaces
- Plugin or AI safety boundaries

Strixonomy-only decisions (parser choice, crate split) → [design/adr/README.md](../design/adr/README.md)
