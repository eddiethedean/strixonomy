# OWL/XML and RDF/XML write-back

> **Status:** Write-back **shipped in v0.21**; v0.22 expands OWL 2 mutate/authoring ops. Open → edit → save → reload without semantic loss for `.owl`/`.rdf` (RDF/XML) and `.owx` (OWL/XML). Write-back uses Horned full-document re-serialize (not byte-identical formatting). See [ADR-0021](../design/adr/0021-deterministic-xml-serializers.md).

Strixonomy indexes and edits RDF/XML and OWL/XML via [Horned-OWL](https://crates.io/crates/horned-owl). The Entity Inspector marks these documents **editable** when parse status is OK.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## What works today

| Capability | `.owl` / `.rdf` (RDF/XML) | `.owx` (OWL/XML) |
|------------|---------------------------|------------------|
| Workspace indexing | Yes | Yes |
| Explorer / SQL / SPARQL | Yes | Yes |
| Entity Inspector (view) | Yes | Yes |
| Entity Inspector (edit) | Yes (core ops) | Yes (core ops) |
| Patch / `strixonomy patch` write-back | Yes | Yes |
| Manchester editor apply | Limited (named ops via patches) | Limited |
| Refactoring apply (rename / merge / replace) | Yes (remap + re-serialize) | Yes (remap + re-serialize) |
| Refactoring apply (move / extract / …) | No (Turtle-first) | No (Turtle-first) |

**Supported XML patch ops (first cut):** create/delete entity, labels/comments/annotations, SubClassOf, imports, ontology/version IRI, class assertions. Unsupported ops return structured errors and leave the file unchanged.

## Recommended workflows

### Edit in place

1. Open a workspace containing `.owl`, `.rdf`, or `.owx` files.
2. Index the workspace; select an entity in Entity Inspector.
3. Edit labels, parents, create/delete entities, or manage imports as you would for Turtle.
4. Save; Strixonomy re-serializes the whole document and reindexes.

### Protégé coexistence

Protégé may rewrite XML layout on save. Strixonomy verifies **semantic** round-trips (entities, labels, parents, imports), not byte-identical XML. See [Protégé coexistence](protege-coexistence.md) and ADR-0021.

### When to prefer Turtle

Keep Turtle (span surgery) when you need byte-stable formatting, property characteristics, complex Manchester expressions, or Turtle-first refactor ops (move / extract / ontology merge / flatten / cleanup). Rename / merge / replace also work on RDF/XML and OWL/XML.

## VS Code behavior

1. Open a workspace containing `.owl` / `.rdf` / `.owx`.
2. Run **Strixonomy: Index Workspace**.
3. Select an entity — edit controls are enabled for supported operations.
4. Unsupported ops surface clear diagnostics (no silent truncate).

## CLI

```bash
strixonomy validate /path/to/ontologies
strixonomy patch path/to/file.owl patches.json
strixonomy patch path/to/file.owx patches.json --preview
```

Patch JSON uses the same Turtle-shaped `PatchOp` wire format for XML documents.

## Migration from earlier releases

- v0.12–v0.20: XML formats were browse-only.
- v0.21: write-back enabled — see [Migration v0.20 → v0.21](../migration/v0.21.md).
- v0.22: expanded OWL 2 patch ops and Manchester on XML-capable documents — see [Migration v0.21 → v0.22](../migration/v0.22.md).

## Related

| Topic | Guide |
|-------|-------|
| Format policy | [Authoring](../authoring.md#format-policy) |
| OBO write-back | [OBO authoring](../ide/obo-authoring.md) |
| Protégé gaps | [Protégé decision guide](protege-decision.md) |
| Serializer policy | [ADR-0021](../design/adr/0021-deterministic-xml-serializers.md) |
