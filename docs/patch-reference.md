# Patch reference (Strixonomy v0.26)

> **Status:** Documents behavior in **Strixonomy v0.26**. Pre-1.0 APIs may change.
> Canonical feature list: [What ships today](SHIPPED.md).

Patch write-back uses a JSON array of patch operations. The CLI (`strixonomy patch`) and LSP (`strixonomy/applyAxiomPatch`) accept the same envelope; operation sets differ by file extension.

Supported formats: **Turtle (`.ttl`)**, **OBO (`.obo`)**, **RDF/XML (`.owl`/`.rdf`)**, **OWL/XML (`.owx`)**. XML uses full-document re-serialize ([ADR-0021](design/adr/0021-deterministic-xml-serializers.md)). See [Supported formats](supported-formats.md).

**Apply path (v0.20):** inbound patch JSON is wrapped as an `strixonomy_edit::Transaction` and applied through format adapters (`TurtleAdapter` / `OboAdapter`) before the existing `apply_patches_to_text` engines run. Legacy patch arrays remain accepted; an optional forward envelope `{ "transaction": { "changes": [...] } }` is also supported:

```json
{
  "transaction": {
    "changes": [
      {
        "document_uri": "file:///path/to/ontology.ttl",
        "patches": [
          { "op": "add_label", "entity_iri": "http://ex#Person", "value": "Human" }
        ]
      }
    ]
  }
}
```

Most callers still pass a bare JSON **array** of ops (CLI `--file` / LSP `patches`).

**Source of truth:** [`patch.rs` on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/crates/strixonomy-owl/src/patch.rs)

## Format

- JSON **array** of operation objects
- Each object has an `"op"` field (snake_case)
- Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), and OWL/XML (`.owx`) documents (dispatch by extension)

## Turtle operations

| `op` | Required fields | Description |
|------|-----------------|-------------|
| `create_entity` | `entity_iri`, `kind` | Create class, property, individual, or datatype |
| `delete_entity` | `entity_iri` | Remove entity block from file |
| `set_label` | `entity_iri`, `value` | Replace all `rdfs:label` values with one |
| `add_label` | `entity_iri`, `value` | Add a label |
| `remove_label` | `entity_iri`, `value` | Remove a matching label |
| `set_comment` | `entity_iri`, `value` | Replace all `rdfs:comment` values |
| `add_comment` | `entity_iri`, `value` | Add a comment |
| `remove_comment` | `entity_iri`, `value` | Remove a matching comment |
| `add_sub_class_of` | `entity_iri`, `parent_iri` | Add `rdfs:subClassOf` parent (named class IRI) |
| `remove_sub_class_of` | `entity_iri`, `parent_iri` | Remove a `subClassOf` axiom |
| `add_complex_sub_class_of` | `entity_iri`, `manchester` | Add complex `SubClassOf` from Manchester expression |
| `remove_complex_sub_class_of` | `entity_iri`, `manchester` | Remove complex `SubClassOf` matching Manchester text |
| `add_equivalent_class` | `entity_iri`, `manchester` | Add `owl:equivalentClass` from Manchester expression |
| `remove_equivalent_class` | `entity_iri`, `manchester` | Remove equivalent class axiom |
| `set_equivalent_class` | `entity_iri`, `manchester` | Replace equivalent class axioms with one expression |
| `add_disjoint_class` | `entity_iri`, `other_iri` | Add `owl:disjointWith` to another named class |
| `remove_disjoint_class` | `entity_iri`, `other_iri` | Remove a `disjointWith` axiom |
| `set_deprecated` | `entity_iri`, `value` | Set `owl:deprecated` (`true` or `false`) |
| `add_import` | `ontology_iri`, `import_iri` | Add `owl:imports` to ontology header |
| `remove_import` | `ontology_iri`, `import_iri` | Remove matching `owl:imports` triple |
| `add_domain` | `entity_iri`, `class_iri` | Add `rdfs:domain` for object/data property |
| `remove_domain` | `entity_iri`, `class_iri` | Remove matching domain axiom |
| `add_range` | `entity_iri`, `range_iri` | Add `rdfs:range` (class or datatype IRI) |
| `remove_range` | `entity_iri`, `range_iri` | Remove matching range axiom |
| `set_functional` | `entity_iri`, `value` | Toggle `owl:FunctionalProperty` |
| `set_inverse_functional` | `entity_iri`, `value` | Toggle `owl:InverseFunctionalProperty` |
| `set_transitive` | `entity_iri`, `value` | Toggle `owl:TransitiveProperty` |
| `set_symmetric` | `entity_iri`, `value` | Toggle `owl:SymmetricProperty` |
| `set_asymmetric` | `entity_iri`, `value` | Toggle `owl:AsymmetricProperty` |
| `set_reflexive` | `entity_iri`, `value` | Toggle `owl:ReflexiveProperty` |
| `set_irreflexive` | `entity_iri`, `value` | Toggle `owl:IrreflexiveProperty` |
| `add_property_chain` | `entity_iri`, `properties` | Add `owl:propertyChainAxiom` (ordered IRI array) |
| `remove_property_chain` | `entity_iri`, `properties` | Remove matching property chain |
| `add_class_assertion` | `entity_iri`, `class_iri` | Add individual `rdf:type` |
| `remove_class_assertion` | `entity_iri`, `class_iri` | Remove class assertion |
| `add_object_property_assertion` | `entity_iri`, `property_iri`, `target_iri` | Add object property assertion |
| `remove_object_property_assertion` | `entity_iri`, `property_iri`, `target_iri` | Remove object property assertion |
| `add_data_property_assertion` | `entity_iri`, `property_iri`, `value` | Add data property assertion with literal |
| `remove_data_property_assertion` | `entity_iri`, `property_iri`, `value` | Remove data property assertion |
| `add_annotation` | `entity_iri`, `predicate`, `value` | Add generic annotation assertion |
| `remove_annotation` | `entity_iri`, `predicate`, `value` | Remove generic annotation assertion |
| `add_prefix` / `remove_prefix` / `set_prefix` | prefix fields | Prefix manager (Turtle only) |
| `add_ontology_annotation` / `remove_ontology_annotation` | `ontology_iri`, `predicate`, `value` | Ontology-level annotations |

### Turtle operations (v0.22 OWL 2 construct coverage)

| `op` | Required fields | TTL | XML | Description |
|------|-----------------|-----|-----|-------------|
| `add_has_key` / `remove_has_key` | `class_iri`, `properties` | Yes | Yes | `owl:hasKey` |
| `add_disjoint_union` / `remove_disjoint_union` | `class_iri`, `members` | Yes | Yes | `owl:disjointUnionOf` |
| `add_inverse_object_properties` / `remove_…` | `property_iri`, `inverse_iri` | Yes | Yes | `owl:inverseOf` |
| `add_equivalent_object_properties` / `remove_…` | `properties` | Yes | Yes | `owl:equivalentProperty` (object) |
| `add_disjoint_object_properties` / `remove_…` | `properties` | Yes | Yes | `owl:propertyDisjointWith` (object) |
| `add_equivalent_data_properties` / `remove_…` | `properties` | Yes | Yes | Equivalent data properties |
| `add_disjoint_data_properties` / `remove_…` | `properties` | Yes | Yes | Disjoint data properties |
| `add_sub_object_property_of` / `remove_…` | `property_iri`, `parent_iri` | Yes | Yes | Object property hierarchy |
| `add_sub_data_property_of` / `remove_…` | `property_iri`, `parent_iri` | Yes | Yes | Data property hierarchy |
| `add_negative_object_property_assertion` / `remove_…` | `entity_iri`, `property_iri`, `target_iri` | Yes | Yes | Negative object assertion |
| `add_negative_data_property_assertion` / `remove_…` | `entity_iri`, `property_iri`, `value` | Yes | Yes | Negative data assertion |
| `add_same_individual` / `remove_same_individual` | `individuals` | Yes | Yes | `owl:sameAs` |
| `add_different_individuals` / `remove_…` | `individuals` | Yes | Yes | `owl:differentFrom` / `AllDifferent` |
| `add_datatype_definition` / `remove_…` | `datatype_iri`, `manchester` | Yes | Yes | Datatype ≡ data range |
| `add_axiom_annotation` / `remove_…` | `axiom_op`, `subject_iri`, `related_iri?`, `predicate`, `value` | Yes | Yes* | Annotate an axiom (*XML: `sub_class_of`, `disjoint_with`) |

### SWRL operations (v0.24)

SWRL rules are stored as ontology annotations (`strixonomy:swrlRule` JSON). Prefer the Rule Browser / Rule Editor in VS Code for interactive authoring; use patches for CI and scripts.

| `op` | Required fields | Description |
|------|-----------------|-------------|
| `add_swrl_rule` | `ontology_iri`, `rule_json` | Add a SWRL rule (JSON IR string) |
| `remove_swrl_rule` | `ontology_iri`, `rule_json` | Remove a matching rule JSON |
| `replace_swrl_rule` | `ontology_iri`, `old_rule_json`, `new_rule_json` | Replace one rule with another |

`rule_json` is a string containing a JSON object with `body` / `head` atoms (optional `id`, `enabled`). Example atom shape:

```json
{
  "op": "add_swrl_rule",
  "ontology_iri": "http://example.org/people",
  "rule_json": "{\"id\":\"person-is-human\",\"body\":[{\"kind\":\"class\",\"class\":\"http://example.org/people#Person\",\"arg\":{\"variable\":\"x\"}}],\"head\":[{\"kind\":\"class\",\"class\":\"http://example.org/people#Human\",\"arg\":{\"variable\":\"x\"}}],\"enabled\":true}"
}
```

See [SWRL cookbook](examples/swrl.md). LSP helpers: `strixonomy/listSwrlRules`, `strixonomy/validateSwrlRule`, `strixonomy/parseSwrlRule` ([LSP API](lsp-api.md)).

## OBO operations (`.obo`)

See [ADR-0019](design/adr/0019-obo-write-back.md). Ops use `term_id` (OBO id, e.g. `GO:0008150`).

| `op` | Required fields | Description |
|------|-----------------|-------------|
| `set_name` | `term_id`, `value` | Set term name |
| `add_synonym` | `term_id`, `value`, `scope` | Add synonym. Write **`scope` as OBO uppercase** (`EXACT`, `RELATED`, `BROAD`, `NARROW`). |
| `remove_synonym` | `term_id`, `value`, `scope?` | Remove synonym. Optional `scope` disambiguates when multiple synonyms share the same text; matching is **case-insensitive**. |
| `add_def` | `term_id`, `value` | Add definition |
| `remove_def` | `term_id` | Remove definition |
| `add_xref` | `term_id`, `xref` | Add xref |
| `remove_xref` | `term_id`, `xref` | Remove xref |
| `set_namespace` | `term_id`, `namespace` | Set namespace |
| `set_deprecated` | `term_id`, `value` | Set `is_obsolete` |
| `add_is_a` | `term_id`, `parent_id` | Add `is_a` parent |
| `remove_is_a` | `term_id`, `parent_id` | Remove `is_a` parent |

## RDF/XML and OWL/XML operations (`.owl` / `.rdf` / `.owx`)

Same Turtle-shaped `PatchOp` JSON applies via Horned load → mutate → full-document re-serialize (v0.21+). Prefixed ops and XML-unsupported edge cases return structured errors and **leave the file unchanged**. Details: [OWL/XML write-back](guides/owl-xml-workflow.md).

| Status | Operations |
|--------|------------|
| **Supported** | Entity create/delete (incl. annotation property + datatype), labels/comments/annotations, ontology IRI/version/imports/ontology annotations, named + Manchester `SubClassOf` / equivalents / disjoints, domain/range (incl. Manchester DataRanges), all property characteristics, property chains, class/object/data assertions, HasKey, DisjointUnion, inverse/equiv/disjoint object & data properties, sub-(object/data)-property, negative assertions, sameAs / differentFrom, datatype definitions (facets / oneOf / and/or/not), axiom annotations (Turtle `axiom_op` set) |
| **Unsupported (clear error)** | Prefix manager ops (`add_prefix` / `remove_prefix` / `set_prefix`) — Turtle-only; not part of the Horned ontology model |

### `kind` values for `create_entity`

`class`, `object_property`, `data_property`, `annotation_property`, `individual`, `datatype`

## Examples

### Create a class with label and parent

```json
[
  {
    "op": "create_entity",
    "entity_iri": "http://example.org/people#Employee",
    "kind": "class"
  },
  {
    "op": "add_label",
    "entity_iri": "http://example.org/people#Employee",
    "value": "Employee"
  },
  {
    "op": "add_sub_class_of",
    "entity_iri": "http://example.org/people#Employee",
    "parent_iri": "http://example.org/people#Person"
  }
]
```

### Add a HasKey

```json
[
  {
    "op": "add_has_key",
    "class_iri": "http://example.org/keys#Person",
    "properties": ["http://example.org/keys#hasSSN"]
  }
]
```

### Add an import

```json
[
  {
    "op": "add_import",
    "ontology_iri": "http://example.org/people",
    "import_iri": "http://purl.obolibrary.org/obo/ro.owl"
  }
]
```

### Property domain and characteristic

```json
[
  {
    "op": "add_domain",
    "entity_iri": "http://example.org/people#worksFor",
    "class_iri": "http://example.org/people#Person"
  },
  {
    "op": "set_functional",
    "entity_iri": "http://example.org/people#worksFor",
    "value": true
  }
]
```

### Rename via label (not IRI rename)

```json
[
  {
    "op": "set_label",
    "entity_iri": "http://example.org/people#Person",
    "value": "knows"
  }
]
```

### Mark deprecated and delete

```json
[
  {
    "op": "set_deprecated",
    "entity_iri": "http://example.org/people#LegacyClass",
    "value": true
  }
]
```

```json
[
  {
    "op": "delete_entity",
    "entity_iri": "http://example.org/people#LegacyClass"
  }
]
```

### Complex subclass (Manchester)

```json
[
  {
    "op": "add_complex_sub_class_of",
    "entity_iri": "http://example.org/clinic#Patient",
    "manchester": "ex:hasRecord some ex:MedicalRecord"
  }
]
```

### Equivalent class (Manchester)

```json
[
  {
    "op": "set_equivalent_class",
    "entity_iri": "http://example.org/clinic#Staff",
    "manchester": "ex:Employee"
  }
]
```

### Disjoint classes

```json
[
  {
    "op": "add_disjoint_class",
    "entity_iri": "http://example.org/org#Cat",
    "other_iri": "http://example.org/org#Dog"
  }
]
```

## CLI usage

```bash
# Preview changes (stdout JSON, no write)
strixonomy patch ./ontology.ttl patches.json --preview

# Apply patches
strixonomy patch ./ontology.ttl patches.json

# Validate after apply
strixonomy validate .
```

### Response shape (`ApplyPatchResult`)

```json
{
  "applied": true,
  "preview_text": "...",
  "diagnostics": [],
  "document_path": "/path/to/ontology.ttl"
}
```

- `applied`: `true` when changes were written (false for `--preview` or on error)
- `preview_text`: resulting Turtle text when content changed
- `diagnostics`: patch errors (non-empty means apply failed)

## LSP usage

Method: `strixonomy/applyAxiomPatch`

```json
{
  "document_uri": "file:///path/to/ontology.ttl",
  "patches": [
    { "op": "add_label", "entity_iri": "http://ex#Person", "value": "Human" }
  ],
  "preview_only": false
}
```

See [lsp-api.md](lsp-api.md) and [authoring.md](authoring.md).

## Limitations (v0.26)

- Write-back: **Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), OWL/XML (`.owx`)**; JSON-LD and line-oriented RDF are read-only. XML is semantic re-serialize — [OWL/XML write-back](guides/owl-xml-workflow.md)
- **OBO:** patch ops edit an existing `term_id` — there is **no** create-term / new-stanza op (create terms in the file or via other tools, then patch)
- Prefix manager ops are **Turtle-only**; XML write-back returns a clear error
- Simple `add_sub_class_of` parent must be a **named class IRI**; use Manchester ops (`add_complex_sub_class_of`, `add_equivalent_class`, etc.) for class expressions
- XML axiom annotations cover the same `axiom_op` set as Turtle (`sub_class_of`, `domain`/`range`, property hierarchy / inverse / equiv / disjoint, same/different individuals, …). Identity is named-entity based; supply `related_iri` when multiple axioms match. Annotation values may be Simple literals or IRIs (`<iri>` / absolute URL)
- Datatype definitions accept Manchester DataRanges (`xsd:integer[>= 0]`, `{…}`, `and`/`or`/`not`) on Turtle and XML
- Catalog `getEntity` lists HasKey, DisjointUnion, RBox/ABox families, datatype definitions, and nested axiom annotations on axiom cards
- Manchester coverage includes restriction constructors (`some`/`only`/`min`/`max`/`exactly`, `not`, `value`, `Self`, one-of, data facets) — see [SHIPPED Manchester scope](SHIPPED.md#manchester-scope-v022)
- Patch engine uses targeted text edits on Turtle/OBO; unusual formatting may need manual review. XML always full-document re-serialize.
