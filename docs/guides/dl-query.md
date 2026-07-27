# DL Query

!!! note "Honesty — Protégé-inspired, not HermiT-identical"
    Query Workbench **DL** mode is Protégé-*inspired*: Manchester class expression → **Instances** / **Subclasses** / **Superclasses** / **Equivalents**. It is **not** HermiT’s DL Query tab.

    - Classification uses **OntoLogos** profiles (`el` / `dl` / `rl` / `rdfs` / `auto`) — not certified HermiT-identical.
    - Some Protégé tab UX and edge cases differ; dual-tool check critical audits.
    - **Not Protégé DL Query** for Workbench **SQL** and **SPARQL** modes — use dedicated **DL** mode (or CLI / LSP) for class-expression queries.

    See [Known limitations](../known-limitations.md) and [Reasoner guide](reasoner.md).

Strixonomy ships DL Query in Query Workbench **DL** mode (**v0.24+**): Manchester class expressions with the four result tabs above (asserted or inferred).

## What ships

| Surface | Detail |
|---------|--------|
| Query Workbench **DL** mode | Manchester class expression → Instances / Subclasses / Superclasses / Equivalents |
| CLI | `strixonomy dl-query` |
| LSP | `strixonomy/dlQuery` |

Related query surfaces:

| Need | Use today |
|------|-----------|
| Catalog questions (`SELECT … FROM classes`) | Query Workbench **SQL** mode — [SQL reference](../sql-reference.md) |
| Graph patterns | Query Workbench **SPARQL** mode — [SPARQL reference](../sparql-reference.md) |
| Inferred types / instance checks | `strixonomy realize` / `strixonomy check-instance` or LSP `strixonomy/checkInstance` — [realize cookbook](../examples/realize.md) |
| Unsatisfiable classes | Reasoner panel / `strixonomy classify` — [Reasoner guide](reasoner.md) |

## Open DL Query in VS Code

1. **Command Palette** → **Strixonomy: Open Query Workbench**
2. Set **Mode** to **DL Query**
3. Enter a Manchester class expression (e.g. `Person and hasPet some Dog`)
4. Choose asserted or inferred, then run — results appear in the four tabs

**Asserted mode:** named-class instances come from asserted class assertions (including asserted subclasses). Anonymous expressions still need **inferred** mode for instances. Saved DL queries remember the asserted/inferred toggle.

## CLI

```bash
strixonomy dl-query "Person and hasPet some Dog" --workspace /path/to/ontologies --profile dl
```

See [CLI reference](../cli-reference.md) and [v0.24 migration](../migration/v0.24.md).

## Gaps vs Protégé DL Query

| Expectation | Strixonomy today |
|-------------|----------------|
| HermiT-identical classification | No — OntoLogos profiles; dual-tool check when required |
| Full Protégé DL Query tab UX | Partial — four result tabs; not a pixel clone |
| SQL/SPARQL as “DL Query” | No — use dedicated **DL** mode |

Keep Protégé when you need HermiT-identical behavior or other gaps in [Known limitations](../known-limitations.md). See [Protégé coexistence](protege-coexistence.md) and [Migrating from Protégé](protege-migration.md).

## Related

- [Query Workbench](../ide/query-workbench.md)
- [Known limitations](../known-limitations.md)
- [FAQ](../faq.md)
