# Manchester editor

The **Manchester editor** is an Strixonomy React panel for editing **complex class expressions** in Turtle ontologies — restrictions, intersections, unions, and cardinality. Parsing and patch application run in **Strixonomy** (`strixonomy/parseManchester`, `strixonomy/applyAxiomPatch`).

Requires a **`.ttl`** file.

## Open the editor

| Entry point | How |
|-------------|-----|
| [Entity Inspector](inspector.md) | Select a class → **Edit in Manchester** on a complex axiom row |
| Add axiom | Inspector → **Add Manchester axiom** |
| Command Palette | **Strixonomy: Open Manchester Editor** |
| Command Palette | **Strixonomy: Add Manchester Axiom** |

## Workflow

1. Choose axiom type: **SubClassOf**, **EquivalentClasses**, or **DisjointClasses**.
2. Enter a Manchester expression, e.g. `ex:hasRecord some ex:MedicalRecord`.
3. Use **Insert** pickers for classes, object properties, data properties, and XSD datatypes (from the indexed catalog).
4. **Validate** — parse diagnostics and expression tree.
5. **Preview Turtle** — see the Turtle fragment before writing.
6. **Apply** — patch the `.ttl` file and re-index.

### Edit vs add

- **Edit** (existing complex axiom): removes the prior expression and adds the new one.
- **Add**: appends a new complex axiom to the entity block.

Simple named-parent `SubClassOf` (e.g. `ex:Person`) is edited in the [inspector](inspector.md) quick form.

## Expression support

| Construct | Example |
|-----------|---------|
| Named class | `ex:Person` |
| Intersection | `ex:A and ex:B` |
| Union | `ex:A or ex:B` |
| Existential restriction | `ex:hasRecord some ex:MedicalRecord` |
| Universal restriction | `ex:hasRecord only ex:MedicalRecord` |
| Cardinality | `ex:hasChild min 1 ex:Person` |
| DisjointClasses | Named class IRI via inspector / patch |

Domain, range, and property chains are editable via the Entity Inspector and patch JSON (v0.12).

## CLI

Manchester parsing is available via LSP. Use the VS Code panel or call `strixonomy/parseManchester` from an LSP client — see [LSP API](../lsp-api.md).
