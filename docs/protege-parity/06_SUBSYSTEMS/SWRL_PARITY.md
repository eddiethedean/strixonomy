# SWRL Parity (v0.24)

Canonical product SWRL status for Strixonomy v0.23.

| Item | Status |
|------|--------|
| Rule IR (horned-aligned, BuiltIns retained) | Shipped (`strixonomy-swrl`) |
| Validate / list / parse LSP | Shipped |
| Turtle authoring via `strixonomy:swrlRule` JSON patch ops | Shipped |
| Rule Browser / Rule Editor | Shipped |
| DLSafe materialization via Ontologos | Shipped when rules present |
| Full RDF SWRL vocabulary round-trip for every blank-node shape | Partial — prefer RDF/XML/OWL/XML horned Rule components; Turtle uses Strixonomy JSON encoding |
| Every experimental `swrlb:` built-in | Out of scope; unsupported builtins warn |

See [BLOCKER_05_SWRL.md](../04_BLOCKERS/BLOCKER_05_SWRL.md) and [SWRL.md](SWRL.md).
