# Entity Inspector

The **Entity Inspector** is the Strixonomy React webview for viewing and editing ontology entities. It is powered by Strixonomy via LSP (`strixonomy/getEntity`, `strixonomy/applyAxiomPatch`).

## Open the inspector

1. Select a class, property, or individual in the **Strixonomy** explorer.
2. The inspector opens in the side panel with labels, parents, axioms, and source location.

Click **Go to source** to jump to the entity definition in the editor.

## Simple Turtle edits

For `.ttl` files with `editable: true`:

| Action | How |
|--------|-----|
| Edit label / comment | Inline fields in the inspector |
| Add / remove parent class | Parent class picker (named IRI) |
| Set deprecated | Toggle |
| Create entity | Explorer context menu → Create Class/Property/Individual |
| Delete entity | Inspector delete action |

Changes apply via patch write-back and trigger a workspace reindex.

**Read-only:** Non-Turtle formats are indexed but not editable in the inspector.

## Axiom catalog

The inspector groups axioms by kind:

- SubClassOf (named and complex)
- EquivalentClasses
- DisjointClasses
- Domain / range (view)
- Property chains (view)

Complex axioms link to the [Manchester editor](manchester-editor.md).

## Refactoring actions

From the inspector (v0.8+):

- **Find Usages** — workspace-wide reference search
- **Rename IRI** — preview and apply rename with refactor preview panel

See [Refactoring guide](../guides/refactoring.md).

## Patch operations

Inspector edits map to JSON patch ops documented in [patch reference](../patch-reference.md).

Full authoring workflow: [Authoring](../authoring.md).
