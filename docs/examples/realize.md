# Realize and instance check cookbook

ABox realization and instance checking for Strixonomy CLI (v0.23+). See [Reasoner guide](../guides/reasoner.md) and [CLI reference](../cli-reference.md).

## Realize individuals

```bash
strixonomy realize /path/to/ontologies --profile rl
strixonomy realize /path/to/ontologies --profile dl --format json
```

**Text output** lists each individual with inferred `types` and `most_specific` types.

```bash
# From a git clone
cargo run -- realize fixtures --profile rl
```

## Check instance

```bash
strixonomy check-instance fixtures \
  --individual 'http://example.org/people#alice' \
  --class 'http://example.org/people#Person' \
  --profile rl

strixonomy check-instance fixtures \
  --individual 'http://example.org/people#alice' \
  --class 'http://example.org/people#Person' \
  --format json
```

**Exit:** 0 when the individual is entailed to be an instance of the class; **non-zero** when not entailed (useful for CI assertions).

## Profiles

Realization and instance checking accept the same profiles as `classify`: `el`, `rl` (default), `rdfs`, `dl`, `auto`. Prefer `rl` or `dl` when ABox depth matters.

## Related

- [CLI reference — realize / check-instance](../cli-reference.md)
- [Classify cookbook](classify.md)
- [SWRL cookbook](swrl.md)
- [Migration v0.23](../migration/v0.23.md)
