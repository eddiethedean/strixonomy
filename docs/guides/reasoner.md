# Reasoner guide

Strixonomy ships OWL reasoning via [OntoLogos](https://github.com/eddiethedean/ontologos) **1.x** — **EL**, **RL**, **RDFS**, **DL**, and **auto** profile routing.

Results are **not** certified identical to Protégé + HermiT; use dual-tool checks for critical audits.

## Run in VS Code

1. Index the workspace (**Strixonomy: Index Workspace**).
2. Run **`Strixonomy: Run Reasoner`** — opens the Reasoner Results panel.
3. Use **`Strixonomy: Set Hierarchy Mode`** to switch the Classes tree between **asserted**, **inferred**, and **combined** hierarchies.
4. Click an unsatisfiable class in the panel to open **`Strixonomy: Show Explanation`** (DL-first on the DL profile; EL/RL/RDFS alternatives where OntoLogos supports).

### Explanation panel (v0.15)

- **Multiple justifications** — when available, use the justification dropdown to switch between alternative proofs.
- **Stale explanations** — after ontology edits or re-indexing, the panel warns when `indexed_at` / `content_hash` no longer match; re-run the reasoner and open the explanation again.
- **Navigate steps** — click entity IRIs in justification steps to open the Entity Inspector.

Settings:

| Setting | Default | Purpose |
|---------|---------|---------|
| `strixonomy.reasoner.default` | `el` | Profile for Run Reasoner |
| `strixonomy.reasoner.autoProfile` | `true` | Profile-detection warnings |
| `strixonomy.hierarchy.mode` | `asserted` | Explorer hierarchy display |

Use **`dl`** for full OWL 2 DL classification or **`auto`** to let OntoLogos route by detected profile.

## CLI / CI

```bash
strixonomy classify ./my-ontologies --profile el --format json
strixonomy classify ./my-ontologies --profile dl --format json
strixonomy classify ./my-ontologies --profile auto --format json
strixonomy explain ./my-ontologies --class 'http://example.org/onto#Invalid' --profile el
strixonomy realize ./my-ontologies --profile rl
strixonomy check-instance ./my-ontologies \
  --individual 'http://example.org/people#alice' \
  --class 'http://example.org/people#Person' \
  --profile rl
```

- `classify` exits non-zero when unsatisfiable classes are found — see [workspace-limits.md](../workspace-limits.md).
- `check-instance` exits non-zero when the instance is not entailed.

CI example: [ci-integration.md](../ci-integration.md).

## LSP

Custom methods: `strixonomy/runReasoner`, `strixonomy/getExplanation`, `strixonomy/checkInstance`, `strixonomy/listSwrlRules`. See [LSP API](../lsp-api.md).

## Profiles

| Profile | Engine | Typical use |
|---------|--------|-------------|
| `el` | `ontologos-el` | OWL EL ontologies (default) |
| `rl` | `ontologos-rl` | OWL RL materialization |
| `rdfs` | `ontologos-rl` (RDFS) | RDFS entailment |
| `dl` | `ontologos-dl` | Full OWL 2 DL (OntoLogos; not certified HermiT-identical) |
| `auto` | `ontologos-facade` | Profile auto-routing |

## Dual-stack note

Strixonomy keeps **two in-memory models** (dual-stack since early releases; still true in v0.23):

- **Oxigraph + Horned-OWL** — authoritative for indexing, SPARQL/SQL, Turtle write-back, asserted hierarchy.
- **OntoLogos** — loads workspace Turtle/RDF files separately for classification.

Axiom counts and some constructs may differ from Protégé until the Horned-OWL → OntoLogos bridge ships (v0.30 backlog). Profile warnings in the Reasoner Results panel flag constructs outside the selected profile.

## Known limitations

| Limitation | Notes |
|------------|-------|
| Explanations | **DL-first** on the DL profile; EL/RL/RDFS alternatives (v0.15+) where `ontologos-explain` supports; coverage depends on profile |
| Hierarchy toggle | **Inferred** / **combined** need a successful reasoner run first |
| Parse coverage | Partial OWL mapping — see [Protégé parity](../design/PROTEGE_PARITY.md) |

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| Explorer unchanged after classify | Run **Set Hierarchy Mode** → inferred or combined |
| Empty explanation | Class may not be unsatisfiable; run reasoner first; try `el` profile |
| Results differ from Protégé | Expected for partial mappings; check profile warnings |

More: [troubleshooting.md](../troubleshooting.md) · [FAQ](../faq.md).

## Related

- [What ships today](../SHIPPED.md)
- [CLI reference](../cli-reference.md)
- [Errors reference](../errors.md)
