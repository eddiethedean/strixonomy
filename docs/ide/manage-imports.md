# Manage Imports

The **Manage Imports** panel lets you add and remove `owl:imports` declarations in **Turtle (`.ttl`)** ontology headers without hand-editing import lines.

Requires Strixonomy **v0.11.3+** and a trusted workspace.

## Open the panel

| Entry point | How |
|-------------|-----|
| Explorer | **Ontologies** view → right-click a `.ttl` file → **Manage Imports** |
| Command Palette | **Strixonomy: Manage Imports** (when a Turtle file is active or selected) |

## Workflow

1. Select a Turtle ontology file in the workspace.
2. Open **Manage Imports**.
3. Review current `owl:imports` IRIs listed for that document.
4. **Add import** — enter the ontology IRI to import; preview the Turtle fragment.
5. **Remove import** — select an existing import and confirm removal.
6. **Apply** — writes the `.ttl` file on disk and re-indexes the workspace.

Import changes use patch operations `add_import` and `remove_import`. See [Patch reference](../patch-reference.md).

## CLI equivalent

```bash
# Preview
strixonomy patch ./ontology.ttl imports.json --preview

# Apply
strixonomy patch ./ontology.ttl imports.json
```

Example `imports.json`:

```json
[
  {
    "op": "add_import",
    "ontology_iri": "http://example.org/people",
    "import_iri": "http://example.org/vocab/core"
  }
]
```

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| Panel empty or disabled | File must be **`.ttl`**; other formats are read-only |
| Import not resolved after add | Confirm the imported ontology is in the workspace or reachable; run **Index Workspace** |
| Broken import diagnostic | Use the lightbulb quick fix or remove via Manage Imports — see [Troubleshooting](../troubleshooting.md) |

## Related

- [Authoring](../authoring.md) — Turtle write-back overview
- [Patch reference](../patch-reference.md) — `add_import` / `remove_import` JSON
- [Migration v0.11](../migration/v0.11.md) — release notes
