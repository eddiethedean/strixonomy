# DL Query cookbook

Manchester class-expression queries via CLI (Query Workbench **DL** mode and LSP `strixonomy/dlQuery` share the same engine).

Honesty and limits: [DL Query guide](../guides/dl-query.md).

!!! note "Requires a clone (or your own files)"
    Samples below use repo [`fixtures/`](https://github.com/eddiethedean/strixonomy/tree/v0.28.1/fixtures). After `cargo install`, replace `fixtures` with your ontology directory or the [First success](../guides/first-success.md) tutorial folder.

**Usage:** `strixonomy dl-query <expression> [--workspace PATH]` — expression is the only positional; workspace defaults to `.` via `--workspace`.

Expressions accept **prefix:local** QNames (resolved from indexed ontology prefixes) or angle-bracket IRIs `<http://…>`. There is **no** `--prefix` CLI flag. Prefer `<…>` when several files bind the same short prefix (for example both tutorial fixtures use `ex:`). Flags: `--workspace`, `--profile`, `--mode`, `--format` only — [CLI reference](../cli-reference.md).

## Named class (inferred)

```bash
strixonomy dl-query 'ex:ClinicPerson' --workspace fixtures \
  --profile rl \
  --mode inferred

strixonomy dl-query '<http://example.org/clinic#ClinicPerson>' --workspace fixtures \
  --format json
```

(`ex:` comes from [`complex-classes.ttl`](https://github.com/eddiethedean/strixonomy/blob/v0.28.1/fixtures/complex-classes.ttl).)

## Asserted instances

```bash
strixonomy dl-query '<http://example.org/people#Person>' --workspace fixtures \
  --mode asserted \
  --format json
```

Use the absolute IRI here so `Person` resolves to the people ontology even when other fixtures also bind `ex:`.

## Anonymous expression

```bash
strixonomy dl-query \
  'ex:Patient and ex:hasRecord some ex:MedicalRecord' \
  --workspace fixtures \
  --profile dl
```

## CI tip

Pin CLI with `--version 0.28.1` (or the release tarball for Linux x64). Treat results as OntoLogos-backed — dual-check critical audits against Protégé when required.
