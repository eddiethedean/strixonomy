# Capabilities by format

> **Latest tagged: v0.26.2** — matrix truth: [Supported formats](../supported-formats.md) · [What ships today](../SHIPPED.md).

One-page view of what each format can do in Strixonomy / Strixonomy. Prefer this when deciding whether to keep Turtle, OBO, or Protégé-style XML.

## Matrix

| Capability | Turtle (`.ttl`) | OBO (`.obo`) | RDF/XML (`.owl`/`.rdf`) | OWL/XML (`.owx`) | JSON-LD / NT / TriG |
|------------|-----------------|--------------|-------------------------|------------------|---------------------|
| Index / browse / SQL / SPARQL | Yes | Yes | Yes | Yes | Yes (browse/query) |
| Entity Inspector edit | Yes | Yes | Yes (v0.21+) | Yes (v0.21+) | No |
| `strixonomy patch` | Yes | Yes | Yes (v0.21+) | Yes (v0.21+) | No |
| Create / delete entities | Yes | Limited OBO term ops | Yes (core ops) | Yes (core ops) | No |
| Manchester apply (rich class expressions) | Yes (richest) | No | Limited (named ops via patches) | Limited | No |
| HasKey / DisjointUnion / RBox / ABox (v0.22) | Yes | No | Yes (mutate / re-serialize) | Yes | No |
| Refactor apply (rename, merge, replace) | Yes | Yes¹ | Yes¹ | Yes¹ | No |
| Refactor apply (move, extract, ontology merge, flatten/cleanup) | Yes | No | No | No | No |
| Manage Imports (add/remove) | Yes | — | Yes | Yes | No |
| Semantic re-serialize on save | No (span surgery) | Stanza-preserving | **Yes** (whole document) | **Yes** | — |
| Protégé byte-identical layout | N/A | N/A | No | No | — |

¹ Semantic remap + re-serialize (XML ADR-0021) or OBO id/reference rewrite — not byte-identical for XML.

## When to pick which format

| Prefer | When |
|--------|------|
| **Turtle** | Team authoring, Git-readable diffs, full Manchester, refactor apply |
| **OBO** | Biomedical term workflows (name, synonym, def, is_a) with OBO-native files |
| **RDF/XML or OWL/XML** | Protégé corpora that must stay XML on disk; accept layout churn on save — [OWL/XML workflow](owl-xml-workflow.md) |
| **Keep Protégé** | Byte-identical XML, Protégé-only plugins, or axiom types still limited here — [known limitations](../known-limitations.md) |

## Related

- [First success](first-success.md) · [OBO workflows](obo-workflow.md) · [Patch reference](../patch-reference.md)
