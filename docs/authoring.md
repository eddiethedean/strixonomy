# Ontology authoring (Strixonomy v0.28)

> **Status:** Documents behavior in **Strixonomy v0.28.1**. v0.29–v0.30 APIs may change.
> Canonical feature list: [What ships today](SHIPPED.md).

Strixonomy provides **Turtle, OBO, RDF/XML, and OWL/XML write-back** for simple edits and **Manchester** for complex class expressions. It is **not** a full Protégé replacement — see [Known limitations](known-limitations.md) and [Protégé coexistence](guides/protege-coexistence.md).

## Supported operations

| Operation | LSP / patch `op` |
|-----------|------------------|
| Create class, property, individual | `create_entity` |
| Delete entity | `delete_entity` |
| Add / set / remove label | `add_label`, `set_label`, `remove_label` |
| Add / set / remove comment | `add_comment`, `set_comment`, `remove_comment` |
| Add / remove parent class (named IRI) | `add_sub_class_of`, `remove_sub_class_of` |
| Add / remove complex `SubClassOf` (Manchester) | `add_complex_sub_class_of`, `remove_complex_sub_class_of` |
| Add / remove / set equivalent class (Manchester) | `add_equivalent_class`, `remove_equivalent_class`, `set_equivalent_class` |
| Add / remove disjoint class (named IRI) | `add_disjoint_class`, `remove_disjoint_class` |
| Add / remove ontology import | `add_import`, `remove_import` |
| Add / remove domain and range | `add_domain`, `remove_domain`, `add_range`, `remove_range` |
| Property characteristics (functional, transitive, …) | `set_functional`, `set_transitive`, … |
| Property chains | `add_property_chain`, `remove_property_chain` |
| Individual assertions | `add_class_assertion`, `add_object_property_assertion`, `add_data_property_assertion` |
| Generic annotations | `add_annotation`, `remove_annotation` |
| Set deprecated flag | `set_deprecated` |

Full JSON reference: [patch-reference.md](patch-reference.md).

## Format policy

- **Write-back:** Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), OWL/XML (`.owx`)
- **Read/index:** Turtle, OBO, RDF/XML (`.owl` / `.rdf`), OWL/XML (`.owx`), JSON-LD, N-Triples, TriG
- OWL/XML and RDF/XML support inspector/`strixonomy patch` write-back via Horned re-serialize (v0.21; not byte-identical layout)

## VS Code workflow (simple edits)

### Turtle (`.ttl`)

1. Open a `.ttl` ontology and select an entity in the Strixonomy explorer.
2. Use the **Entity Inspector** edit section: add labels, comments, parents, or delete.
3. Use context menu **Create Class/Property/Individual** on explorer views.
4. Changes apply via `strixonomy/applyAxiomPatch` and trigger a workspace reindex.
5. VS Code undo works on saved file changes.

### OBO (`.obo`)

1. Open a `.obo` file and select a term in the Strixonomy explorer.
2. Edit name, definition, synonyms, `is_a`, and related fields in the Entity Inspector.
3. Preview and apply — see [OBO authoring](ide/obo-authoring.md).

## Manchester editor

For complex class expressions (restrictions, `and`/`or`, cardinality):

1. Select a class in a `.ttl` file.
2. In the Entity Inspector, click **Edit in Manchester** on a complex axiom row, or **Add Manchester axiom**.
3. Choose axiom type: **SubClassOf**, **EquivalentClasses**, or **DisjointClasses** (named class IRI for disjoint).
4. Enter a Manchester expression (e.g. `ex:hasRecord some ex:MedicalRecord`).
5. Use **Insert** pickers for classes, object properties, data properties, and XSD datatypes.
6. **Validate** shows parse diagnostics and an expression tree.
7. **Preview** shows the Turtle fragment; **Apply** writes the patch.

Manchester covers OWL 2 class expressions used in complex `SubClassOf` / `EquivalentClasses` on Turtle — including `and` / `or` / `not`, `some` / `only` / `value` / `Self`, OneOf `{…}`, `min` / `max` / `exact` cardinality, nested restrictions, and data restrictions on XSD types (see [SHIPPED — Manchester scope](SHIPPED.md#manchester-scope-v022)). **DisjointClasses**, domain/range, and **property chains** are editable via inspector and patch JSON. XML write-back supports core ops; richest Manchester authoring remains **Turtle-first**.

## Manage Imports (v0.11)

Add or remove `owl:imports` in Turtle ontology headers:

1. **Ontologies** view → right-click a `.ttl` file → **Manage Imports**
2. Add an import IRI or remove an existing one; preview and apply

Guide: [Manage Imports](ide/manage-imports.md)

## Editor completion and quick fixes (v0.11)

In `.ttl` files, Strixonomy offers:

- **Completion** — triggered on `:`, `<`, and `@` for prefixes, QNames, and IRIs
- **Quick fixes** — lightbulb actions for `undefined_prefix`, `missing_label`, and `broken_import` diagnostics

See [LSP API](lsp-api.md#textdocumentcompletion-v011) and [Troubleshooting](troubleshooting.md).

## Workspace refactoring

For multi-file changes (rename IRI, namespace migration, move entity, extract module), use the refactor commands and **Refactor Preview** panel — see [Refactoring guide](guides/refactoring.md).

## Query workbench

Run **Strixonomy: Open Query Workbench** from the Command Palette.

- Toggle **SQL**, **SPARQL**, or **DL Query** mode
- Run queries against the indexed workspace (DL mode: Manchester class expressions — [DL Query honesty](guides/dl-query.md))
- Export results as CSV or JSON
- Save named queries and reload from history
- Use the virtual table dropdown for SQL table names

## CLI

### Example `patches.json`

```json
[
  {
    "op": "create_entity",
    "entity_iri": "http://example.org/people#Student",
    "kind": "class"
  },
  {
    "op": "add_label",
    "entity_iri": "http://example.org/people#Student",
    "value": "Student"
  },
  {
    "op": "add_sub_class_of",
    "entity_iri": "http://example.org/people#Student",
    "parent_iri": "http://example.org/people#Person"
  }
]
```

```bash
# From a folder containing example.ttl (e.g. strixonomy-tutorial from first-success)
strixonomy patch ./example.ttl patches.json --preview
strixonomy patch ./example.ttl patches.json
strixonomy validate .
```

More examples: [patch-reference.md](patch-reference.md).

## Horned-OWL dual stack

For Turtle files, catalog **entities and axioms** come from [Horned-OWL](https://github.com/phillord/horned-owl) via `strixonomy-owl`. Oxigraph remains authoritative for triple counts and SPARQL. CI runs `owl_oxigraph_consistency` tests on fixtures.

## Unsaved buffers

When you apply a patch or Manchester edit in VS Code, the language server uses the **open editor buffer** as the source of truth (not only the file on disk). Unsaved edits in the buffer are preserved and patched in place. If reindex fails after a successful write, you may see `APPLIED_NOT_INDEXED` — run **Index Workspace**. See [errors.md](errors.md).

## Limitations

- Write-back is **Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), and OWL/XML (`.owx`)**; JSON-LD and line-oriented RDF remain read-only
- Complex axioms appear in the inspector and Manchester editor but **not** as edges in the class hierarchy tree (named-parent edges only; use reasoner for inferred hierarchy)
- Manchester autocomplete uses catalog **Insert** pickers (no inline buffer autocomplete yet)
- No SQL/SPARQL editor autocomplete in the workbench (v0.8+)
