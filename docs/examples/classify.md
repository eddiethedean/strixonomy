# Classify cookbook

Run OWL classification from the CLI. See [Reasoner guide](../guides/reasoner.md) and [errors reference](../errors.md).

## EL profile (default)

```bash
strixonomy classify /path/to/ontologies --profile el --format json
```

**Exit:** 0 when consistent; **non-zero** when unsatisfiable classes exist (use in CI gates).

## Other profiles

```bash
strixonomy classify /path/to/ontologies --profile rl --format json
strixonomy classify /path/to/ontologies --profile rdfs --format json
strixonomy classify /path/to/ontologies --profile dl --format json
strixonomy classify /path/to/ontologies --profile auto --auto-profile --format json
```

## From a git clone

```bash
cargo run -- classify fixtures --profile el
```

## CI note

`strixonomy classify` exits non-zero on unsatisfiable classes. LSP `strixonomy/runReasoner` returns `consistent: false` as a **successful** JSON result — see [errors reference](../errors.md#classify-cli-vs-runreasoner-lsp).

## Related

- [CLI reference — classify](../cli-reference.md)
- [CI integration](../ci-integration.md)
