# ADR-0019 — OBO Write-Back via fastobo

## Status

Accepted — **implemented in v0.12.0** (read path v0.11, write-back v0.12)

## Context

Strixonomy indexes OBO Format 1.4 (`.obo`) for biomedical workflows. v0.7 shipped read-only OBO support with a minimal line parser. v0.30 Protégé parity requires reliable read/write, rich metadata (synonyms, definitions, xrefs), and patch-based editing consistent with Turtle write-back ([ADR-0006](0006-patch-based-write-back.md)).

The canonical Rust stack for OBO is [`fastobo`](https://crates.io/crates/fastobo) + [`fastobo-owl`](https://crates.io/crates/fastobo-owl) per [OBO_ROBOT_SPEC.md](../OBO_ROBOT_SPEC.md).

## Decision (original)

### v0.11

1. **Read path:** Replace the minimal OBO line parser with `fastobo::from_str` → existing `ParsedOntology` / catalog model. Surface synonyms, definitions, and property values in catalog annotations and entity detail.
2. **Write-back:** Document patch schema; defer OBO inspector editing and disk serialize to v0.12.
3. **Boundaries:** Turtle patches (`strixonomy-owl::patch`) own Turtle write-back. OBO patches use a separate op namespace in `strixonomy-obo`.

### v0.30 (remaining)

- Optional `fastobo-validator` in CI / `strixonomy validate`
- Richer OBO metadata round-trip parity with Protégé / ROBOT

## Current implementation (v0.12)

- **`strixonomy-obo`** crate applies OBO patch ops to `.obo` files on disk
- **Entity Inspector** and **`strixonomy/applyAxiomPatch`** dispatch by file extension (`.ttl` vs `.obo`)
- **CLI:** `strixonomy patch path/to/terms.obo patches.json`
- **Documented ops:** [patch-reference.md](../../patch-reference.md) · [OBO authoring](../../ide/obo-authoring.md)

## OBO patch op schema

JSON array; each object has `"op"` (snake_case). Term subject is always an OBO id (e.g. `GO:0008150`) or resolved IRI.

| `op` | Required fields | Description |
|------|-----------------|-------------|
| `set_name` | `term_id`, `value` | Replace `name` clause |
| `add_synonym` | `term_id`, `value`, `scope` | Add synonym (`EXACT`, `RELATED`, `BROAD`, `NARROW`) |
| `remove_synonym` | `term_id`, `value` | Remove matching synonym |
| `add_def` | `term_id`, `value` | Add or replace `def` |
| `remove_def` | `term_id` | Remove definition |
| `add_xref` | `term_id`, `xref` | Add xref (`DB:ID` or structured) |
| `remove_xref` | `term_id`, `xref` | Remove xref |
| `set_namespace` | `term_id`, `namespace` | Set term namespace |
| `set_deprecated` | `term_id`, `value` | Set `is_obsolete` |
| `add_is_a` | `term_id`, `parent_id` | Add `is_a` parent |
| `remove_is_a` | `term_id`, `parent_id` | Remove `is_a` parent |

Example:

```json
[
  {
    "op": "add_synonym",
    "term_id": "EX:001",
    "value": "example term",
    "scope": "EXACT"
  },
  {
    "op": "add_def",
    "term_id": "EX:001",
    "value": "An example class."
  }
]
```

## Turtle vs OBO write-back

| Concern | Turtle | OBO |
|---------|--------|-----|
| Patch crate | `strixonomy-owl` | `strixonomy-obo` |
| LSP apply | `strixonomy/applyAxiomPatch` on `.ttl` | Same method, `.obo` dispatch |
| Subject key | `entity_iri` | `term_id` |
| Manchester | Yes | No (structural OBO clauses) |
| Imports | `owl:imports` patch ops | N/A (separate ontology files) |

## Consequences

**Positive:**

- Single canonical OBO parser (`fastobo`) reduces drift from Protégé / ROBOT
- Patch schema documented before implementation; shipped in v0.12
- Biomedical workflows can edit OBO in VS Code without Protégé

**Negative:**

- Two patch vocabularies (Turtle vs OBO ops) — unified LSP envelope only
- Full Protégé OBO feature parity (all xref types, logical definitions) remains v0.30 work

## References

- [OBO_ROBOT_SPEC.md](../OBO_ROBOT_SPEC.md)
- [PROTEGE_PARITY.md](../PROTEGE_PARITY.md)
- [patch-reference.md](../../patch-reference.md)
- [OBO authoring guide](../../ide/obo-authoring.md)
