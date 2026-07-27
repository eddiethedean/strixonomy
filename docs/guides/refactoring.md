# Workspace refactoring

Strixonomy provides **workspace-wide refactoring** with **preview before apply**: find usages, rename IRIs, merge/replace entities, migrate namespaces, move entities, and extract modules.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## Supported operations

| Operation | VS Code | CLI |
|-----------|---------|-----|
| Find usages of an IRI | Entity Inspector, explorer context menu | `strixonomy refactor usages` |
| Rename entity IRI | **Rename Entity IRI** command | `strixonomy refactor rename` |
| Migrate namespace base | **Migrate Namespace** command | `strixonomy refactor migrate-namespace` |
| Move entity to another file | **Move Entity** command | `strixonomy refactor move` |
| Extract module (subset of entities) | **Extract Module** command | `strixonomy refactor extract` |
| Merge entities | **Merge Entities** command | `strixonomy refactor merge` |
| Replace entity references | **Replace Entity** command | `strixonomy refactor replace` |

**Format policy:** **rename / merge / replace** rewrite Turtle, RDF/XML, OWL/XML, and OBO (format-specific remaps). **Move / extract / ontology merge / flatten / cleanup imports** remain **Turtle-first** (non-Turtle files skipped with warnings).

## VS Code workflow

### Find usages

1. Select an entity in the **Classes**, **Properties**, or **Individuals** view.
2. Right-click → **Find Entity Usages**, or use the **Find Usages** action in the Entity Inspector.
3. Review the usage list (file, line, kind, context).

### Merge or replace entities

1. Command Palette → **Strixonomy: Merge Entities** or **Strixonomy: Replace Entity**.
2. Enter survivor/duplicate IRIs (merge) or source/target IRIs (replace).
3. Review the **Refactor Preview** panel and click **Apply**.

Rename / merge / replace also run from the CLI (`strixonomy refactor …`). They rewrite Turtle plus RDF/XML, OWL/XML, and OBO where remaps apply (non-Turtle files may be skipped with warnings).

### Rename, migrate, move, or extract

1. Select an entity (for rename/move) or run the command from the Command Palette.
2. Enter parameters (new IRI, namespace base, target file, or entity set for extract).
3. The **Refactor Preview** panel opens with per-file diffs.
4. Review warnings (if any) and click **Apply** to write changes, or cancel.

Standard LSP **Rename** (`F2` on an IRI in a `.ttl` file) also triggers IRI rename when the symbol is a known entity.

| Command | When to use |
|---------|-------------|
| **Strixonomy: Find Entity Usages** | Locate all references before a manual edit |
| **Strixonomy: Rename Entity IRI** | Change one entity's IRI across the workspace |
| **Strixonomy: Migrate Namespace** | Replace a namespace base IRI (`@prefix` + term IRIs) |
| **Strixonomy: Move Entity** | Move an entity block to another `.ttl` file |
| **Strixonomy: Extract Module** | Copy selected entities into a new module file |
| **Strixonomy: Merge Entities** | Merge two entities (survivor + duplicate) across supported formats |
| **Strixonomy: Replace Entity** | Replace references to one IRI with another (preview + apply) |

Use **`--preview`** before apply in production repos.

After apply, the workspace re-indexes automatically. If re-index fails, you may see `reindex_warning` — run **Strixonomy: Index Workspace**.

## CLI examples

Replace `fixtures` with your ontology root (or use `cargo run --` from a git clone).

### Merge or replace entities

```bash
strixonomy refactor merge fixtures \
  --keep 'http://example.org/people#Person' \
  --merge 'http://example.org/people#Human' \
  --preview

strixonomy refactor replace fixtures \
  --from 'http://example.org/people#OldName' \
  --to 'http://example.org/people#NewName' \
  --preview
```

| Flag | Description |
|------|-------------|
| `--keep` / `--merge` | Survivor and duplicate IRIs for merge |
| `--from` / `--to` | Source and target IRIs for replace |
| `--preview` | Print plan without writing files |
| `--format` | `text` (default) or `json` |

### Find usages

```bash
strixonomy refactor usages fixtures 'http://example.org/people#Person'
strixonomy refactor usages fixtures 'http://example.org/people#Person' --format json
```

### Rename IRI (preview then apply)

```bash
strixonomy refactor rename fixtures \
  --from 'http://example.org/people#Person' \
  --to 'http://example.org/people#Human' \
  --preview

strixonomy refactor rename fixtures \
  --from 'http://example.org/people#Person' \
  --to 'http://example.org/people#Human'
```

### Migrate namespace

```bash
strixonomy refactor migrate-namespace fixtures \
  --from 'http://example.org/people#' \
  --to 'http://example.org/v2/people#' \
  --preview
```

### Move entity

```bash
strixonomy refactor move fixtures 'http://example.org/people#Student' \
  --to ./people/students.ttl \
  --preview
```

### Extract module

```bash
strixonomy refactor extract fixtures \
  --entities 'http://example.org/people#Person,http://example.org/people#Student' \
  --out ./people/core.ttl \
  --leave-stub \
  --preview
```

`--leave-stub` keeps import stubs in the original files. Extract uses direct-reference closure for related axioms.

Full flag reference: [CLI reference](../cli-reference.md#refactor).

## LSP integrators

| Method | Purpose |
|--------|---------|
| `strixonomy/findUsages` | List usages for an IRI |
| `strixonomy/previewRefactor` | Build a `RefactorPlan` without writing |
| `strixonomy/applyRefactor` | Apply a plan (re-previews and verifies match) |

Refactor requests use a tagged `kind` field including `rename_iri`, `migrate_namespace`, `move_entity`, `extract_module`, `merge_entities`, `replace_entity`, and related ontology ops. See [LSP API](../lsp-api.md) and [lsp-protocol.schema.json](../lsp-protocol.schema.json).

## Safety and limitations

| Topic | Notes |
|-------|-------|
| Preview | Always preview multi-file changes before apply in production repos |
| Open buffers | LSP applies to open editor buffers first, then disk (same as patches) |
| Multi-root workspace | All folders indexed (v0.10+); refactor applies across indexed roots |
| Format coverage | Rename / merge / replace: Turtle + RDF/XML + OWL/XML + OBO. Move / extract / ontology-merge / flatten / cleanup: Turtle-first |
| Extract module | Direct-reference closure; may not capture all indirect imports |
| Git | Review diffs before commit; use [semantic diff](../ide/semantic-diff.md) for PR summaries |

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| Preview shows no changes | Confirm entity exists in a `.ttl` file; run **Index Workspace** |
| Apply failed / plan mismatch | Re-run preview — files changed since preview was generated |
| `reindex_warning` after apply | File written but catalog stale — **Index Workspace** |
| Namespace migration overwrote renames | Migrate updates `@prefix` and IRIs under the base — preview carefully |

More: [Troubleshooting](../troubleshooting.md) · [Errors reference](../errors.md)

## Related

- [Authoring](../authoring.md) — single-entity Turtle patches
- [Patch reference](../patch-reference.md) — patch JSON for CI automation
- [CLI reference](../cli-reference.md)
- [Examples: refactoring](../examples/refactoring.md)
