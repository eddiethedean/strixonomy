# SWRL cookbook

Author and validate SWRL rules in Strixonomy / Strixonomy (v0.23+). Interactive path: **Rule Browser** / **Rule Editor** in VS Code. Script path: patch ops + LSP validate/parse.

Rules are stored as ontology annotations (`strixonomy:swrlRule` JSON). DL/Auto classify may materialize DLSafe rules via Ontologos when present. See [What ships today](../SHIPPED.md) and [patch reference](../patch-reference.md).

## Patch: add a simple rule

```json
[
  {
    "op": "add_swrl_rule",
    "ontology_iri": "http://example.org/people",
    "rule_json": "{\"id\":\"person-is-human\",\"body\":[{\"kind\":\"class\",\"class\":\"http://example.org/people#Person\",\"arg\":{\"variable\":\"x\"}}],\"head\":[{\"kind\":\"class\",\"class\":\"http://example.org/people#Human\",\"arg\":{\"variable\":\"x\"}}],\"enabled\":true}"
  }
]
```

```bash
strixonomy patch ./example.ttl patches-swrl.json --preview
strixonomy patch ./example.ttl patches-swrl.json
```

Download: [patches-swrl.json](patches-swrl.json)

`remove_swrl_rule` / `replace_swrl_rule` use the same `rule_json` shape (exact JSON string match for remove/replace).

## Validate via LSP

Custom methods (see [LSP API](../lsp-api.md)):

| Method | Purpose |
|--------|---------|
| `strixonomy/listSwrlRules` | List rules in the indexed workspace |
| `strixonomy/validateSwrlRule` | Validate rule JSON (builtins / DLSafe diagnostics) |
| `strixonomy/parseSwrlRule` | Parse rule JSON into the Strixonomy SWRL IR |

There is **no** dedicated `strixonomy swrl` CLI subcommand — use `patch` for write-back and the LSP methods (or Rule Editor) for validate/list.

## Classify with rules present

```bash
strixonomy classify ./ontologies --profile dl --format json
```

When SWRL rules are present, DL/Auto classify may materialize them before taxonomy. Prefer dual-tool checks when you need Protégé/HermiT-identical rule entailments.

## Related

- [Patch reference — SWRL](../patch-reference.md#swrl-operations-v024)
- [Reasoner guide](../guides/reasoner.md)
- [Realize cookbook](realize.md)
- [Migration v0.23](../migration/v0.23.md)
