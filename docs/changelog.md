# Changelog

Canonical source: [CHANGELOG.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/CHANGELOG.md) (update both on release).

Migration guides: [Migration index](migration/README.md)

## [Unreleased]

## [0.27.0] - 2026-07-27

**v0.27.0** — **Strixonomy rename:** OntoCore / OntoCode → Strixonomy (crates, CLI, LSP, extension, docs) with compatibility shims through ≥1.0. See [CHANGELOG.md](https://github.com/eddiethedean/strixonomy/blob/main/CHANGELOG.md) and [migration/v0.27.md](migration/v0.27.md).

## [0.26.2] - 2026-07-17

**v0.26.2** — host focus/transaction integrity, semantic-diff/edit/query correctness, exhaustive XML/OWL IRI remap, aligned merge semantics across formats, and residual invert / SPARQL / orphan-diagnostic fixes. See [CHANGELOG.md](https://github.com/eddiethedean/strixonomy/blob/main/CHANGELOG.md) and [migration/v0.26.2.md](migration/v0.26.2.md).

## [0.26.1] - 2026-07-16

**v0.26.1** — OWL Inspector write-back fixes plus reasoner/SWRL/DL Query honesty, plugin + ROBOT path-jail hardening, and LSP/editor/catalog bugfixes. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.26.1.md](migration/v0.26.1.md).

## [0.26.0] - 2026-07-16

**v0.26.0** — Protégé Desktop JUnit behavioral test port (Waves 1–4): edit oracles, annotation linkification + property order, `catalog-v001.xml` redirects, IdPolicy, Foundry registry parse, ontology `version_iri`; OBO merge/replace duplicate-`id:` fix. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.26.md](migration/v0.26.md).

## [0.25.0] - 2026-07-15

**v0.25.0** — **Adopters:** expanded graphs, Plugin SDK 1.0 (author against frozen wire; marketplace still product 1.0), a11y for owned webviews, parity CI gates. See [migration/v0.25.md](migration/v0.25.md). Engineering: `PAR-VIS-001` / `PAR-PLG-001` / `PAR-ACC-001` / `PAR-TST-001` — full [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md).

## [0.24.0] - 2026-07-14

**v0.24.0** — Semantic services: DL Query + refactoring parity (`PAR-QRY-002` / `PAR-REF-001`). Query Workbench **DL** mode, CLI `ontocore dl-query`, LSP `ontocore/dlQuery` and `ontocore/search`; CLI `refactor merge` / `replace` plus ontology merge, import flatten/cleanup, and locality extract; multi-format rename/merge/replace remaps; refactor impact metrics. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.24.md](migration/v0.24.md).

## [0.23.0] - 2026-07-14

**v0.23.0** — Reasoning parity + SWRL (`PAR-RSN-002`/`PAR-RSN-003`/`PAR-SWRL-001`): full consistency, realization, instance check, inferred ABox assertions; native DL explanations with weaker-engine fallback; engine cancel through enrich; `ontocore-swrl` + Rule Browser/Editor + authored-rule injection for classify. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.23.md](migration/v0.23.md).

## [0.22.0] - 2026-07-14

**v0.22.0** — Complete OWL 2 authoring (`PAR-OWL-001`): HasKey, DisjointUnion, RBox/ABox expansions, datatype defs, axiom annotations, Manchester `not`/`value`/`Self`/OneOf/data restrictions, XML mutate parity, Inspector UI; AllDifferent Turtle write-back; dangling-leaf symlink path-jail fix. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.22.md](migration/v0.22.md).

## [0.21.0] - 2026-07-13

**v0.21.0** — RDF/XML and OWL/XML write-back (Horned re-serialize); semantic comparator; editable gates for `.owl`/`.rdf`/`.owx`; session/TM and write-back bug-fix cluster. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.21.md](migration/v0.21.md).

## [0.20.0] - 2026-07-13

**v0.20.0** — Workspace runtime (registry, save, transactions, session); Turtle patch matching for Protégé/ROBOT-style files. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.20.md](migration/v0.20.md).

## [0.19.0] - 2026-07-13

**v0.19.0** — Semantic transaction apply path (`ontocore-edit`); Protégé parity program baseline (manifest, CI validator, epics). See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.19.md](migration/v0.19.md).

## [0.18.2] - 2026-07-13

**v0.18.2** — Patch: Windows path normalization; reasoner cancel/`ops_lock`; Manchester cardinality; Find Usages/prefixes; Turtle literals; authoring and panel restore fixes. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.18.2.md](migration/v0.18.2.md).

## [0.18.1] - 2026-07-12

**v0.18.1** — Patch: named unsatisfiable expansion; composed explanations for expansion-only unsats; Horned patch oracles; dialog e2e; stronger tests. See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.18.1.md](migration/v0.18.1.md).

## [0.18.0] - 2026-07-11

**v0.18.0** — Protégé Desktop parity gate: distinct reasoner lifecycle + client cancel, explanation stale UX, layout reopen-with-context, expanded protege-roundtrip fixtures, migration guide + honest known gaps; plus extension UI polish (React Reasoner/Explanation) and docs adoption (version freshness, MkDocs IA, Trust). See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.18.md](migration/v0.18.md).

## [0.17.0] - 2026-07-10

**v0.17.0** — Protégé shell parity: menus/toolbars/keybindings, command registry with enablement, DialogShell flows (new ontology, prefixes, metrics, about), webview tab persistence and named perspectives, help/error-log surfaces, plus OntoCore APIs for create/export, prefix/metadata patches, merge/replace, delete impact, and reasoner classify (Start/Sync/Classify/Consistency share one path; Stop is UI-only). Includes the v0.17 bugfix cluster and LSP honesty fixes ([#209](https://github.com/eddiethedean/ontocode/pull/209)). See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.17.md](migration/v0.17.md).

## [0.16.0] - 2026-07-09

**v0.16.0** — Plugin preferences pages + context actions wired in the extension, plugin command execution via `ontocore/runPlugin`, imports reload + layout reset. Includes OBO idspace IRI normalization ([#111](https://github.com/eddiethedean/ontocode/issues/111)) and OBO patch newline/token validation ([#112](https://github.com/eddiethedean/ontocode/issues/112)). See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.16.md](migration/v0.16.md).

## [0.15.0] - 2026-07-08

See [CHANGELOG.md](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) and [migration/v0.15.md](migration/v0.15.md).

## Earlier releases

Summaries for v0.14 and earlier: [CHANGELOG.md on GitHub](https://github.com/eddiethedean/ontocode/blob/main/CHANGELOG.md) · [Migration index](migration/README.md).
