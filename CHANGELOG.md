# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **Strixonomy rename (v0.27):** OntoCore / OntoCode product identity → **Strixonomy** across Rust crates (`strixonomy` / `strixonomy-*`), CLI (`strixonomy`, deprecated `ontocore` alias), LSP (`strixonomy-lsp`, deprecated `ontocore-lsp` alias; methods `strixonomy/*` with legacy `ontocore/*`), VS Code extension (`strixonomy.strixonomy`), workspace config (`.strixonomy/` with dual-read of `.ontocore/` / `.ontocode/`), docs, and CI. See [migration/v0.27.md](docs/migration/v0.27.md) and [ADR-0022](docs/design/adr/0022-strixonomy-identity.md).

## [0.26.2] - 2026-07-17

**For pilots / adopters:** host focus/transaction integrity, semantic-diff/edit/query correctness, exhaustive XML/OWL IRI remap, aligned merge semantics across formats, and residual invert / SPARQL / orphan-diagnostic fixes. No format write-back regression vs v0.26.1 — [migration/v0.26.2.md](docs/migration/v0.26.2.md).

### Fixed

- Host `focusRelay.setFocus` rejects older timestamps (aligns with webview hydrate) ([#402](https://github.com/eddiethedean/ontocode/issues/402), [#417](https://github.com/eddiethedean/ontocode/pull/417))
- WorkspaceStore navigation back/forward syncs inspector / graph / explorer slices ([#352](https://github.com/eddiethedean/ontocode/issues/352))
- `applyRefactor` registers self-writes and marks reasoning dirty like patches ([#396](https://github.com/eddiethedean/ontocode/issues/396))
- `WorkspaceTransactionManager.begin` reserves pending before await (concurrent begin TOCTOU) ([#386](https://github.com/eddiethedean/ontocode/issues/386))
- `RemoveSynonym` with recorded scope inverts to `AddSynonym` ([#400](https://github.com/eddiethedean/ontocode/issues/400))
- SPARQL update detector classifies `USING` / `USING NAMED` forms ([#399](https://github.com/eddiethedean/ontocode/issues/399))
- `orphan_classes` no longer false-positives classes whose only parents are restrictions ([#401](https://github.com/eddiethedean/ontocode/issues/401))
- Semantic diff axiom/annotation keys include `ontology_id`; rename detection requires added `owl:sameAs` / `skos:exactMatch` ([#395](https://github.com/eddiethedean/ontocode/issues/395), [#389](https://github.com/eddiethedean/ontocode/issues/389), [#419](https://github.com/eddiethedean/ontocode/pull/419))
- AllDifferent rewrite fails closed when a list member CURIE cannot expand ([#350](https://github.com/eddiethedean/ontocode/issues/350))
- `atomic_write` best-effort removes `.tmp` on mid-write I/O failure (OWL + OBO) ([#343](https://github.com/eddiethedean/ontocode/issues/343))
- `createOntology` rejects format/extension mismatches (e.g. `ttl` under `.owl`) ([#342](https://github.com/eddiethedean/ontocode/issues/342))
- `SELECT *` with zero rows still returns table schema columns ([#385](https://github.com/eddiethedean/ontocode/issues/385))
- SWRL rule list dedupes on document + canonical rule JSON ([#362](https://github.com/eddiethedean/ontocode/issues/362))
- XML/OWL IRI remap covers all Horned `Component` kinds and recursive `DataRange` ([#368](https://github.com/eddiethedean/ontocode/issues/368), [#376](https://github.com/eddiethedean/ontocode/issues/376), [#420](https://github.com/eddiethedean/ontocode/pull/420))
- Entity merge uses delete-then-remap across Turtle, OBO, and XML (merge-owned axioms no longer absorbed onto Keep in RDF/XML) ([#369](https://github.com/eddiethedean/ontocode/issues/369), [#421](https://github.com/eddiethedean/ontocode/pull/421))

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.26.2**; extension and webview UI **0.26.2**

## [0.26.1] - 2026-07-16

**For pilots / adopters:** OWL Inspector write-back reliability, reasoner/SWRL/DL Query honesty, plugin + ROBOT path-jail hardening, and LSP/editor/catalog fixes. No format write-back regression vs v0.26.0 — [migration/v0.26.1.md](docs/migration/v0.26.1.md).

### Fixed

- XML/Horned `Remove*` takes full `AnnotatedComponent`s (axiom annotations) so annotated removals no longer silently no-op ([#382](https://github.com/eddiethedean/ontocode/issues/382), [#408](https://github.com/eddiethedean/ontocode/pull/408))
- SameIndividual / InverseObjectProperties / HasKey / DisjointUnion set- and order-insensitive matching for Inspector pairwise cards ([#331](https://github.com/eddiethedean/ontocode/issues/331), [#334](https://github.com/eddiethedean/ontocode/issues/334), [#351](https://github.com/eddiethedean/ontocode/issues/351))
- `AddHasKey` uses `DataProperty` when declared; Turtle `Disjoint*Properties` emits full pairwise closure ([#332](https://github.com/eddiethedean/ontocode/issues/332), [#394](https://github.com/eddiethedean/ontocode/issues/394))
- `RemoveRange` matches Manchester/faceted DataRanges; `AddRange` errors on parse failure ([#333](https://github.com/eddiethedean/ontocode/issues/333), [#409](https://github.com/eddiethedean/ontocode/pull/409))
- Known DeclareDatatype / DatatypeDefinition IRIs as data fillers; facet errors not swallowed ([#335](https://github.com/eddiethedean/ontocode/issues/335))
- XML `applied` only when serialize output changes; unsafe IRIs rejected; incomplete OWL/XML parses refused ([#337](https://github.com/eddiethedean/ontocode/issues/337), [#338](https://github.com/eddiethedean/ontocode/issues/338), [#383](https://github.com/eddiethedean/ontocode/issues/383))
- IRI-valued annotation remove; ambiguous axiom annotation always errors; Turtle axiom-annotation exact lexical match ([#339](https://github.com/eddiethedean/ontocode/issues/339), [#340](https://github.com/eddiethedean/ontocode/issues/340), [#349](https://github.com/eddiethedean/ontocode/issues/349))
- SWRL/reasoner honesty: inject skips, consistency signals, and realization reporting ([#410](https://github.com/eddiethedean/ontocode/pull/410))
- DL Query equivalents / asserted instances under equivalence; realize errors as warnings; CLI `--workspace` clap layout ([#370](https://github.com/eddiethedean/ontocode/issues/370)–[#373](https://github.com/eddiethedean/ontocode/issues/373), [#375](https://github.com/eddiethedean/ontocode/issues/375), [#411](https://github.com/eddiethedean/ontocode/pull/411))
- Plugin host: `ui_view` requires `workspace.write`; jail violations → diagnostics + `success: false`; truncated stdio; exec TOCTOU ([#347](https://github.com/eddiethedean/ontocode/issues/347), [#348](https://github.com/eddiethedean/ontocode/issues/348), [#388](https://github.com/eddiethedean/ontocode/issues/388), [#404](https://github.com/eddiethedean/ontocode/issues/404), [#412](https://github.com/eddiethedean/ontocode/pull/412))
- XML catalog `nextCatalog` cycle/depth jail + size-capped reads; OBO `inverse_of` / `domain` / `range` remap; CRLF preserve ([#374](https://github.com/eddiethedean/ontocode/issues/374), [#387](https://github.com/eddiethedean/ontocode/issues/387), [#392](https://github.com/eddiethedean/ontocode/issues/392), [#393](https://github.com/eddiethedean/ontocode/issues/393), [#413](https://github.com/eddiethedean/ontocode/pull/413))
- LSP/editor: `.owx` document sync; IRI tokens allow `.`; annotation/axiom predicate usages; semantic tokens UTF-16 ([#384](https://github.com/eddiethedean/ontocode/issues/384), [#397](https://github.com/eddiethedean/ontocode/issues/397), [#398](https://github.com/eddiethedean/ontocode/issues/398), [#403](https://github.com/eddiethedean/ontocode/issues/403), [#414](https://github.com/eddiethedean/ontocode/pull/414))
- ROBOT subprocess timeout; rewrite jailed path args to absolute; create/export refuse symlink/TOCTOU escapes ([#341](https://github.com/eddiethedean/ontocode/issues/341), [#346](https://github.com/eddiethedean/ontocode/issues/346), [#353](https://github.com/eddiethedean/ontocode/issues/353), [#354](https://github.com/eddiethedean/ontocode/issues/354), [#415](https://github.com/eddiethedean/ontocode/pull/415))

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.26.1**; extension and webview UI **0.26.1**

## [0.26.0] - 2026-07-16

**For pilots / adopters:** Protégé Desktop **behavioral** parity corpus (Waves 1–4) — hierarchy/merge/deprecation/history oracles, annotation linkification + property order in the IDE, `catalog-v001.xml` import redirects, IdPolicy parse, ontology `version_iri`. No format write-back regression vs v0.25 — [migration/v0.26.md](docs/migration/v0.26.md).

### Added

- Protégé Desktop test-port inventory ([`parity/protege-test-port.yaml`](parity/protege-test-port.yaml)) with `PORT_W1`–`PORT_W4` / `SKIP` / `COVERED` tags and `scripts/validate-protege-test-port.py` CI wiring
- Wave 1 oracle suites: hierarchy, merge, deprecation, history/change algebra, axiom location, refs/defined-class, parsers/IDs, IdPolicy (`tests/protege_port_*.rs` + fixtures under `examples/protege-roundtrip/ported/`)
- Wave 2 presentation helpers: render/escape/prefix/IRI (`ontocore-owl` `render`) and annotation link extractors (`ontocore-owl` `links`); LSP hover linkification; Entity Inspector annotation hyperlinks
- Wave 3 utils: abbreviate, ISO8601, lexical replace, markdown entity links, annotation-property order (`ontocore-owl` `util` + Inspector sort); OBO Foundry registry JSON parse (`ontocore-obo` `obofoundry`, vendored fixtures)
- Wave 4: `catalog-v001.xml` import redirects (`ontocore-catalog` `xml_catalog` + `resolve_import_document`)
- `OntologyDocument.version_iri` and related parser oracles
- Migration guide: [docs/migration/v0.26.md](docs/migration/v0.26.md)

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.26.0**; extension and webview UI **0.26.0**
- Parity manifest `test_ids` linkage for Protégé-ported suites; Gate 3 hard-fail remains `--strict-release` at **1.0.0-rc**

### Fixed

- OBO merge/replace no longer leaves duplicate `id:` stanzas (merge removes the merge stanza then remaps refs; replace-when-target-exists remaps refs only) ([#367](https://github.com/eddiethedean/ontocode/issues/367))

## [0.25.0] - 2026-07-15

**For pilots / adopters:** richer graphs (more kinds, filters, keyboard nav), frozen **Plugin SDK 1.0** (lifecycle + providers + `plugins info|enable|disable`), accessibility improvements for OntoCode-owned webviews, and parity CI release gates. No format write-back regression vs v0.24 — [migration/v0.25.md](docs/migration/v0.25.md).

v0.25.0 — UX completion + executable verification (viz, Plugin SDK 1.0, a11y, parity CI).

### Added

- EPIC-008 / visualization parity (`PAR-VIS-001`): expanded `ontocore/getGraph` kinds (`object_property`, `data_property`, `individual`, `dependency`, `query_result`, `refactor_preview`); richer node/filter model; Graph panel filters, search dimming, unsatisfiable overlay, Graph\|List alternate, keyboard nav, context menus, history, virtualized React Flow render; Query Workbench **Open as graph** and Refactor Preview **Show graph**
- Graph truncation caps documented in workspace limits; scale regression test for class-graph build
- EPIC-009 / Plugin SDK 1.0 (`PAR-PLG-001`): frozen extension-point matrix; `depends_on` / `activation` lifecycle with topo-sorted activate and cascade disable; provider actions `reasoner.classify` / `query.run` / `refactor.preview` / `graph.build`; CLI `plugins info|enable|disable`; compat harness `tests/plugin_sdk_compat.rs`; reference providers in `examples/plugin-workspace/`
- EPIC-010 / accessibility parity (`PAR-ACC-001`): shared webview a11y layer (focus trap/restore, live announcer, reduced motion); DialogShell trap + labelled dialogs; P0 panel landmarks/announcements; axe-core Vitest harness (serious/critical); filled ACCESSIBILITY_REPORT + P0 audit inventory
- EPIC-011 / parity verification (`PAR-TST-001`): evidence path validation, release-gate metrics report (`check-parity-release-gate.py`), YAML→matrix/metrics sync (`generate-parity-docs.py`), CI wiring; Gate 3 hard-fail reserved for `--strict-release` at 1.0.0-rc
- Migration guide: [docs/migration/v0.25.md](docs/migration/v0.25.md)

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.25.0**; extension and webview UI **0.25.0**
- `PAR-VIS-001` marked **VERIFIED** in the parity manifest
- `PAR-PLG-001` marked **VERIFIED**; plugin author guide and policy retargeted to SDK 1.0 compatibility
- `PAR-ACC-001` marked **VERIFIED** for OntoCode-owned webviews (VS Code host chrome remains N/A)
- `PAR-TST-001` marked **VERIFIED**; parity matrix statuses synced from YAML

## [0.24.0] - 2026-07-14

v0.24.0 — Semantic services (refactoring + DL Query).

### Added

- DL Query (`PAR-QRY-002` / EPIC-007): Manchester class expressions via temp equivalent class + classify/realize; CLI `ontocore dl-query`; LSP `ontocore/dlQuery`; Query Workbench **DL** mode with Instances / Subclasses / Superclasses / Equivalents tabs
- Asserted-mode DL Query instance path polish; Query Workbench DL mode history
- Workspace search LSP `ontocore/search` (QuickPick live search)
- Refactor plan impact metrics (`affected_entity_count` / `affected_axiom_count`); CLI `refactor merge` / `replace`
- Multi-format rename/merge/replace remaps (`ontocore-owl` / `ontocore-obo` `remap.rs`) for RDF/XML, OWL/XML, and OBO (plus Turtle)
- Move selected axioms; ontology merge; flatten imports; import cleanup; locality-based module extraction (`--locality`)
- SWRL-aware rename/merge/replace (rewrites `ontocore:swrlRule` JSON literals); `UsageKind::SwrlReference`
- Migration guide: [docs/migration/v0.24.md](docs/migration/v0.24.md)

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.24.0**; extension and webview UI **0.24.0**
- Refactor preview subtitle surfaces entity/axiom impact counts
- Release publish workflow trusts green `ci.yml` on the tagged SHA and publishes crates without fixed inter-crate delays
- `PAR-REF-001` / `PAR-QRY-002` marked **VERIFIED** in the parity manifest (rename/merge/replace multi-format; move/extract/ontology-merge/flatten/cleanup remain Turtle-first)

## [0.23.0] - 2026-07-14

v0.23.0 — Reasoning parity + SWRL.

### Added

- Reasoning parity (`PAR-RSN-002` / EPIC-004): full consistency (TBox + ABox), realization, instance checking, inferred class/object-property assertions and sameAs clusters; CLI `ontocore realize` / `ontocore check-instance`; LSP `ontocore/checkInstance` and enriched `runReasoner` snapshot
- Native DL explanations (`PAR-RSN-003`): DL-first axiom-contribution + clash diagnostics; EL/RL/RDFS traces remain secondary alternatives
- Engine-level reasoner cancel via Ontologos `set_current_cancel`; workspace dirty/synchronize contract on edits
- SWRL subsystem (`PAR-SWRL-001` / EPIC-005): new `ontocore-swrl` crate; `AddSwrlRule` / `RemoveSwrlRule` / `ReplaceSwrlRule` patches; Rule Browser/Editor; LSP `listSwrlRules` / `validateSwrlRule` / `parseSwrlRule`; Ontologos SWRL materialization on DL/Auto classify when rules are present
- Ontologos workspace deps bumped to **1.1.4** (incl. `ontologos-swrl`)
- Fixtures `tests/fixtures/reasoner/abox/`, `tests/fixtures/swrl/`; tests `reasoner_abox`, `swrl_validate`, `swrl_patch`
- Migration guide: [docs/migration/v0.23.md](docs/migration/v0.23.md)
- Release publish order includes `ontocore-swrl` before `ontocore-reasoner` / `ontocore-lsp`

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.23.0**; extension and webview UI **0.23.0**
- Classification `consistent` incorporates ontology/ABox consistency detail when available
- Docs adoption sync: version-truth hygiene, HermiT wording softened, CLI/examples for realize/SWRL, Protégé/enterprise guides refreshed for v0.23

## [0.22.0] - 2026-07-14

### Added

- Complete OWL 2 authoring (`PAR-OWL-001` / EPIC-002): HasKey, DisjointUnion, inverse/equivalent/disjoint object and data properties, sub-property hierarchy, negative assertions, Same/DifferentIndividual, datatype entities and definitions, axiom annotations
- Manchester: `not`, `value`, `Self`, ObjectOneOf `{…}`, xsd data restrictions / facets
- XML (RDF/XML / OWL/XML) mutate coverage for the expanded `PatchOp` catalog
- Entity Inspector sections for HasKey, DisjointUnion, Inverse, Same/Different, NegativeOPA
- Fixtures `owl2-keys.ttl` / `owl2-abox.ttl` and `tests/owl2_authoring.rs`
- Inventory: [OWL2_AUTHORING_GAPS.md](docs/protege-parity/06_SUBSYSTEMS/OWL2_AUTHORING_GAPS.md)
- Migration guide: [docs/migration/v0.22.md](docs/migration/v0.22.md)

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.22.0**; extension and webview UI **0.22.0**

### Fixed

- Turtle `DifferentIndividuals` write-back now emits/matches Protégé-style `owl:AllDifferent` + `owl:distinctMembers` (pairwise `owl:differentFrom` no longer drops multi-member axioms) ([#330](https://github.com/eddiethedean/ontocode/issues/330), [#344](https://github.com/eddiethedean/ontocode/pull/344))
- Workspace path jail rejects dangling leaf symlinks (and symlink create escape); `createOntology` / CLI `new` use exclusive create ([#336](https://github.com/eddiethedean/ontocode/issues/336), [#345](https://github.com/eddiethedean/ontocode/pull/345))

## [0.21.0] - 2026-07-13

### Added

- RDF/XML (`.owl` / `.rdf`) and OWL/XML (`.owx`) write-back via Horned full-document re-serialize (ADR-0021)
- `ontocore-owl` serialize / mutate / apply_xml APIs and semantic comparator
- CLI / LSP / catalog / extension editable gates for required XML formats
- Protégé round-trip edit → save → reload tests and malformed / unsupported-op fail-closed cases
- Migration guide: [docs/migration/v0.21.md](docs/migration/v0.21.md)
- Parity closeout: PAR-FMT-003 / PAR-FMT-004 ([#247](https://github.com/eddiethedean/ontocode/issues/247))

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.21.0**; extension and webview UI **0.21.0**

### Fixed

- Session restore: command allowlist, workspace URI jail, open-editors capture, `NOT_INDEXED` retry, TransactionManager sync, panel serializers ([#309](https://github.com/eddiethedean/ontocode/issues/309), [#311](https://github.com/eddiethedean/ontocode/issues/311), [#295](https://github.com/eddiethedean/ontocode/issues/295), [#294](https://github.com/eddiethedean/ontocode/issues/294), [#300](https://github.com/eddiethedean/ontocode/issues/300), [#301](https://github.com/eddiethedean/ontocode/issues/301); [#317](https://github.com/eddiethedean/ontocode/pull/317), [#318](https://github.com/eddiethedean/ontocode/pull/318))
- TransactionManager integrity: self-write FS guard, Sync-cancel bookkeeping, route UI edits through TM, honest Save when dirty ([#293](https://github.com/eddiethedean/ontocode/issues/293), [#296](https://github.com/eddiethedean/ontocode/issues/296)–[#299](https://github.com/eddiethedean/ontocode/issues/299); [#319](https://github.com/eddiethedean/ontocode/pull/319))
- OBO catalog absolute IRIs / `property_value` / def-vs-comment; XML `SetOntologyIri` + `DeleteEntity` ([#303](https://github.com/eddiethedean/ontocode/issues/303)–[#306](https://github.com/eddiethedean/ontocode/issues/306), [#308](https://github.com/eddiethedean/ontocode/issues/308); [#320](https://github.com/eddiethedean/ontocode/pull/320))
- Focus/nav races: history suppress, hydrateFocus timestamps, explanation fetch generation ([#276](https://github.com/eddiethedean/ontocode/issues/276), [#277](https://github.com/eddiethedean/ontocode/issues/277), [#292](https://github.com/eddiethedean/ontocode/issues/292); [#321](https://github.com/eddiethedean/ontocode/pull/321))
- Nested SQL WHERE, bare-column rejection, text/CSV truncation honesty ([#238](https://github.com/eddiethedean/ontocode/issues/238), [#307](https://github.com/eddiethedean/ontocode/issues/307), [#313](https://github.com/eddiethedean/ontocode/issues/313); [#322](https://github.com/eddiethedean/ontocode/pull/322))
- Cyclic class roots, `entityBelongsToDocument` exact match, Inspector `ontology_iri` rejection ([#222](https://github.com/eddiethedean/ontocode/issues/222), [#234](https://github.com/eddiethedean/ontocode/issues/234), [#310](https://github.com/eddiethedean/ontocode/issues/310); [#323](https://github.com/eddiethedean/ontocode/pull/323))
- Semantic-diff `owl:sameAs` rename detection; move-entity `@prefix` merge into non-empty targets; subprocess validate requires `workspace.write` ([#312](https://github.com/eddiethedean/ontocode/issues/312), [#314](https://github.com/eddiethedean/ontocode/issues/314), [#315](https://github.com/eddiethedean/ontocode/issues/315); [#324](https://github.com/eddiethedean/ontocode/pull/324))
- ROBOT PATH discovery without Unix-only `which`; Windows-safe `file://` URIs; plugin timeout process-group kill; silent index debounce coalesces distinct paths ([#316](https://github.com/eddiethedean/ontocode/issues/316), [#218](https://github.com/eddiethedean/ontocode/issues/218), [#217](https://github.com/eddiethedean/ontocode/issues/217), [#215](https://github.com/eddiethedean/ontocode/issues/215); [#325](https://github.com/eddiethedean/ontocode/pull/325))
- Pre-tag hardening: jail ROBOT short-flag `=` forms (`-i=/abs`); `default_base_iri` / LSP `path_to_uri` use `file_uri_for_path`; path identity rejects dual-failed canonicalize; Find Usages lists distinct annotation objects; session capture skips DEFAULT_REOPEN and forgets closed panels; deferred catalog restore keeps retrying; Windows path identity keys for self-write/registry

## [0.20.0] - 2026-07-13

### Added

- Workspace runtime (EPIC-003 / BLOCKER_03): host-owned ontology registry with active targeting and editability rules; per-ontology dirty tracking; `SaveCoordinator` for Save / Save All; `WorkspaceTransactionManager` with semantic undo/redo stacks (`undo_patches` from LSP); host event bus; selection and navigation managers; session persistence (workspace state + `.ontocode/session.json`); external-change recovery (reload / keep / compare); panel restore with semantic args ([#249](https://github.com/eddiethedean/ontocode/issues/249))

### Fixed

- Turtle patch matching for Protégé/ROBOT-style files: lang-tagged/typed literal removes, angle-bracket IRI object removes, `rdf:type` ontology IRI rewrite, comment-safe type/characteristic detection, and property-chain subject needles ([#261](https://github.com/eddiethedean/ontocode/issues/261), [#262](https://github.com/eddiethedean/ontocode/issues/262), [#270](https://github.com/eddiethedean/ontocode/issues/270), [#271](https://github.com/eddiethedean/ontocode/issues/271), [#272](https://github.com/eddiethedean/ontocode/issues/272), [#273](https://github.com/eddiethedean/ontocode/issues/273), [#278](https://github.com/eddiethedean/ontocode/issues/278); [#286](https://github.com/eddiethedean/ontocode/pull/286))
- OBO apply + transaction invert: Typedef/Instance stanza start bounds, scoped `RemoveSynonym`, CRLF-preserving rewrites, and refuse lossy invert for `RemovePrefix` / OBO synonym-def / boolean clears ([#216](https://github.com/eddiethedean/ontocode/issues/216), [#259](https://github.com/eddiethedean/ontocode/issues/259), [#260](https://github.com/eddiethedean/ontocode/issues/260), [#263](https://github.com/eddiethedean/ontocode/issues/263), [#264](https://github.com/eddiethedean/ontocode/issues/264), [#275](https://github.com/eddiethedean/ontocode/issues/275))
- Reasoner cancel wait no longer drops concurrent LSP messages; non-`$/cancelRequest` traffic is deferred for the main dispatcher ([#268](https://github.com/eddiethedean/ontocode/issues/268))
- Prefix/CURIE correctness: Manchester complex patches merge document `@prefix`, unqualified cardinality does not require `owl:`, IRI rename avoids wrong-prefix CURIEs, move-to-new-file copies `@prefix` headers, and `undefined_prefixes` ignores colons inside `<IRI>`s ([#274](https://github.com/eddiethedean/ontocode/issues/274), [#279](https://github.com/eddiethedean/ontocode/issues/279), [#280](https://github.com/eddiethedean/ontocode/issues/280), [#281](https://github.com/eddiethedean/ontocode/issues/281), [#282](https://github.com/eddiethedean/ontocode/issues/282))
- LSP/edit tooling: transaction envelopes reject cross-format apply, `entity_detail` comes from parsed transactions, reindex invalidates in-flight reasoner runs, code-action full-document ranges cover trailing newlines, ROBOT jails `--prefixes`/`--inputs`/`--add-prefixes`, and `duplicate_labels` ignores same-entity case variants ([#265](https://github.com/eddiethedean/ontocode/issues/265), [#266](https://github.com/eddiethedean/ontocode/issues/266), [#267](https://github.com/eddiethedean/ontocode/issues/267), [#283](https://github.com/eddiethedean/ontocode/issues/283), [#284](https://github.com/eddiethedean/ontocode/issues/284), [#285](https://github.com/eddiethedean/ontocode/issues/285))
- Stop Reasoner posts `reasonerRunCancelled` so the React panel clears `running` without reintroducing start-flicker sync clears ([#269](https://github.com/eddiethedean/ontocode/issues/269))

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.20.0**; extension and webview UI **0.20.0**
- Migration notes: [docs/migration/v0.20.md](docs/migration/v0.20.md)

## [0.19.0] - 2026-07-13

### Added

- `ontocore-edit` crate: `SemanticChange`, `Transaction` (compose, validate, invert, serde), Turtle/OBO format adapters
- ADR 0020: semantic transaction edit model (`docs/design/adr/0020-semantic-transaction-edit-model.md`)
- Parity manifest `parity/protege-desktop-parity.yaml` with 19 `PAR-*` requirements and CI validator (`scripts/validate-parity-manifest.py`)
- GitHub epics [#247](https://github.com/eddiethedean/ontocode/issues/247)–[#257](https://github.com/eddiethedean/ontocode/issues/257) for EPIC-001…011
- Migration guide: [docs/migration/v0.19.md](docs/migration/v0.19.md)

### Changed

- LSP `applyAxiomPatch` and CLI `ontocore patch` apply Turtle/OBO edits through `Transaction` (patch JSON arrays still accepted)
- Parity scope frozen; protege-parity docs link to `ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md`
- Workspace package and all `ontocore-*` crates bumped to **0.19.0**; extension marketplace and webview UI **0.19.0**

## [0.18.2] - 2026-07-13

### Fixed

- Extension normalizes Windows `\\?\` paths from LSP canonicalize so Prefix Manager and inspector patches resolve workspace documents
- Duplicate `ontocode.showImportsPanel` registration no longer breaks extension activation
- Reasoner ops_lock release test no longer deadlocks by holding the mutex across reasoner join
- Reasoner cancel releases `ops_lock` during classify so index/patch ops are not blocked; cancelled runs skip snapshot updates ([#211](https://github.com/eddiethedean/ontocode/issues/211))
- Manchester parser accepts Protégé-style property-first cardinality (`ex:hasPart min 1 ex:Organ`) and normalizes to the same form ([#226](https://github.com/eddiethedean/ontocode/issues/226))
- Manchester Turtle emit writes qualified cardinalities as `"N"^^xsd:nonNegativeInteger` per OWL 2 RDF mapping ([#231](https://github.com/eddiethedean/ontocode/issues/231))
- Find Usages jump guards out-of-range line numbers instead of throwing from `lineAt` ([#228](https://github.com/eddiethedean/ontocode/issues/228))
- Find Usages text scan matches default-prefix CURIEs (`:Local`) under `@prefix : <…>` ([#232](https://github.com/eddiethedean/ontocode/issues/232))
- Find Usages lists each annotation subject separately instead of collapsing to one hit per entity ([#237](https://github.com/eddiethedean/ontocode/issues/237))
- `prefixes_from_turtle` recognizes `PREFIX`, `@PREFIX`, and default-prefix declarations for LSP rename and text scans ([#225](https://github.com/eddiethedean/ontocode/issues/225))
- Reasoner panel keeps running state during in-flight classify; `reasonerSyncRunId` no longer clears the spinner at run start ([#212](https://github.com/eddiethedean/ontocode/issues/212))
- Reasoner `lastRunAt` and `dirty` update only on successful completion; cancel restores pre-run snapshot ([#221](https://github.com/eddiethedean/ontocode/issues/221))
- Focus sync treats `reasoningState` with `running: true` as in-progress without emitting completion ([#220](https://github.com/eddiethedean/ontocode/issues/220))
- Explanation panel sequences catalog fingerprint with displayed content; reindex no longer reuses stale content hash ([#219](https://github.com/eddiethedean/ontocode/issues/219))
- Reasoner checkbox label matches `autoDetect` setting ("Auto-detect profile") ([#223](https://github.com/eddiethedean/ontocode/issues/223))
- Turtle literal scanning: honor `\` escapes in long `"""` / `'''` strings for statement bounds ([#210](https://github.com/eddiethedean/ontocode/issues/210))
- Turtle patch scanners recognize all four string literal forms; RemoveLabel/RemoveComment match lexical values regardless of quoting ([#224](https://github.com/eddiethedean/ontocode/issues/224), [#233](https://github.com/eddiethedean/ontocode/issues/233))
- Undefined-prefix diagnostic stripper handles `'''` long single-quoted strings and escaped long-string closures ([#214](https://github.com/eddiethedean/ontocode/issues/214))
- Extension authoring safety: workspace path guards on create entity, Manchester apply, and inspector/imports apply; honest error toasts when `documentUri` is outside the workspace ([#213](https://github.com/eddiethedean/ontocode/issues/213), [#230](https://github.com/eddiethedean/ontocode/issues/230), [#239](https://github.com/eddiethedean/ontocode/issues/239))
- Create/delete entity refreshes the explorer when disk apply succeeds but editor sync is cancelled ([#236](https://github.com/eddiethedean/ontocode/issues/236))
- Imports panel opens from the command palette and Review perspective; session restore persists `filePath` ([#227](https://github.com/eddiethedean/ontocode/issues/227))
- Graph panel updates title and restores the correct graph kind after reload ([#229](https://github.com/eddiethedean/ontocode/issues/229))
- Manchester editor session restore persists `iri` and `documentUri` ([#235](https://github.com/eddiethedean/ontocode/issues/235))

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.18.2**; extension marketplace and webview UI **0.18.2**

## [0.18.1] - 2026-07-12

### Fixed

- Expand named unsatisfiable classes that are ⊑ `owl:Nothing` (and their descendants) so reasoner `unsatisfiable` / `consistent` report classes such as `Invalid` and `B`, not only `owl:Nothing` itself
- Compose honest explanations for expansion-only unsatisfiable classes: subclass chain to an ancestor with a bottom proof (or to `owl:Nothing`), marked `composed_subclass_chain`, when Ontologos has no direct `C ⊑ ⊥` trace

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.18.1**; extension marketplace and webview UI **0.18.1**
- Stronger behavioral test oracles for reasoner hierarchy, Protégé round-trip, semantic diff breaking changes, path jail, and OBO patch round-trip
- OWL patch success paths: Horned/`IndexBuilder` reparse oracles (`tests/owl_patch_oracles.rs`) for the top ten write-back ops
- Extension: real LSP write-back and reasoner workflow tests; New Ontology / Prefix Manager / Property Chain VS Code e2e; ImportsPanel Vitest; remove source-regex client startup guards
- Optional `cargo-mutants` baseline for `path_jail` + OWL `patch` (manual/nightly; see `docs/mutants-baseline.md`)

## [0.18.0] - 2026-07-11

### Added

- Protégé Desktop parity gate: distinct reasoner Start / Synchronize / Classify / Consistency workflows; Stop cancels the in-flight client request and ignores late results
- Explanation stale banner wired to catalog fingerprint (`content_hash` / `indexed_at`) after reindex while the panel stays open
- Layout restore: recovered webview tabs offer **Reopen panel** using the last saved command + context
- Expanded `examples/protege-roundtrip/` workflow fixtures and `tests/protege_roundtrip.rs` coverage
- Desktop parity assessment, refreshed Protégé migration guide, and honest known-gap list ([migration/v0.18.md](docs/migration/v0.18.md))

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.18.0**; extension marketplace and webview UI **0.18.0**
- MENUS checklist honesty: Persist layout / Stop reasoner / cancellation marked Partial until v0.18 closeout
- Graph truncation messaging and basic ARIA/focus affordances on Explanation, Reasoner, and restore chrome
- Extension UI polish: React Reasoner and Explanation panels on the shared `oc-*` shell, explorer ThemeIcons, unified Loading/Empty states
- Docs adoption pass: version freshness, MkDocs IA (exclude engineering corpus, redirect maps), Trust messaging, known-limitations honesty

### Fixed

- Remap blank nodes per document when merging into the shared SPARQL store so multi-file workspaces no longer fuse unrelated restrictions ([#160](https://github.com/eddiethedean/ontocode/issues/160))
- Graph **Expand** refreshes after depth changes; Preferences hub covers reasoner/query/keys/plugins categories
- Graph **Export JSON/CSV** via save dialog (v0.15 visualization remainder)

## [0.17.0] - 2026-07-10

### Added

- Protégé-style menus, toolbar actions, context commands, and platform-aware keybindings backed by the centralized command registry
- React dialogs for New Ontology, Prefix Manager, Ontology Metrics, and About, using the shared `DialogShell` with keyboard handling and live IRI/prefix validation
- Panel serializers that keep OntoCode webview tabs across reloads, named Modeling/Reasoning/Review perspectives, help/support surfaces, error logging, and diagnostic export (transient panel state is not restored)
- Engine and LSP support for ontology creation/export, prefix and metadata patches, active ontology state, merge/replace refactors, and reasoner Start/Synchronize/Classify/Consistency (shared classify path; Stop clears UI only)

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.17.0**; extension marketplace and webview UI **0.17.0**
- Protégé parity checklists now record the v0.17 menu, toolbar, and dialog closeout

### Fixed

- Accept peer multi-root folder adds after open, surface `OwlBridgeFailed` when OWX Horned load fails, implement `textDocument/prepareRename`, and harden the undefined-prefix stripper for empty/escaped prefixes ([#209](https://github.com/eddiethedean/ontocode/pull/209))
- Scope Turtle QName completions to the typed prefix namespace, return no IRI when the cursor is past the line end, drop the open buffer and show an error on invalid `didChange` ranges, and harden LSP client start/stop against races ([#7](https://github.com/eddiethedean/ontocode/issues/7), [#13](https://github.com/eddiethedean/ontocode/issues/13), [#90](https://github.com/eddiethedean/ontocode/issues/90), [#91](https://github.com/eddiethedean/ontocode/issues/91))
- Require `owl:sameAs` (not shared labels alone) for semantic-diff renames, enrich reasoner diffs with the actual left/right catalogs, and copy full Markdown sections from Semantic Diff ([#20](https://github.com/eddiethedean/ontocode/issues/20), [#56](https://github.com/eddiethedean/ontocode/issues/56), [#149](https://github.com/eddiethedean/ontocode/issues/149))
- Guard graph refresh with a generation counter, exclude unknown IRIs from `ontology_iri` graph filters, keep class/property graphs from switching to neighborhood on entity focus, and ignore older `hydrateFocus` timestamps ([#33](https://github.com/eddiethedean/ontocode/issues/33), [#117](https://github.com/eddiethedean/ontocode/issues/117), [#154](https://github.com/eddiethedean/ontocode/issues/154), [#161](https://github.com/eddiethedean/ontocode/issues/161))
- Invalidate cancelled reasoner runs so late results are ignored, prefer last-run profile for explanations, stop marking fresh explanations as stale, and label DL justifications with the actual EL/RL/RDFS fallback profile ([#141](https://github.com/eddiethedean/ontocode/issues/141), [#142](https://github.com/eddiethedean/ontocode/issues/142), [#148](https://github.com/eddiethedean/ontocode/issues/148), [#66](https://github.com/eddiethedean/ontocode/issues/66))
- Serialize Entity Inspector patches, post failures to the webview error channel, refresh after cancelled editor sync, reset form state on navigation, gate characteristics on editable with preview/apply, use case-insensitive `.ttl`/`.obo` detection, keep form text until successful reload, and mark OBO axioms editable consistently with the entity ([#40](https://github.com/eddiethedean/ontocode/issues/40), [#60](https://github.com/eddiethedean/ontocode/issues/60), [#119](https://github.com/eddiethedean/ontocode/issues/119), [#58](https://github.com/eddiethedean/ontocode/issues/58), [#59](https://github.com/eddiethedean/ontocode/issues/59), [#68](https://github.com/eddiethedean/ontocode/issues/68), [#41](https://github.com/eddiethedean/ontocode/issues/41), [#65](https://github.com/eddiethedean/ontocode/issues/65))
- Put required `plugins run` plugin id before optional workspace (clap debug panic), join/kill plugin subprocess readers on timeout and wait errors, clear `active_ontology_id` and explanation cache on root/workspace clear, publish empty diagnostics for stale URIs, and cap the explanation cache at 8 entries ([#105](https://github.com/eddiethedean/ontocode/issues/105), [#164](https://github.com/eddiethedean/ontocode/issues/164), [#143](https://github.com/eddiethedean/ontocode/issues/143), [#132](https://github.com/eddiethedean/ontocode/issues/132), [#163](https://github.com/eddiethedean/ontocode/issues/163), [#113](https://github.com/eddiethedean/ontocode/issues/113))
- Reject unsupported `createOntology` formats, surface export failures, use `set_prefix` for Prefix Manager updates, align prefix validation (no hyphens), fix byte→UTF-16 conversion and out-of-range diagnostic jumps, and quote newlines in query CSV export ([#130](https://github.com/eddiethedean/ontocode/issues/130), [#131](https://github.com/eddiethedean/ontocode/issues/131), [#129](https://github.com/eddiethedean/ontocode/issues/129), [#17](https://github.com/eddiethedean/ontocode/issues/17), [#32](https://github.com/eddiethedean/ontocode/issues/32), [#152](https://github.com/eddiethedean/ontocode/issues/152))
- OR-merge property characteristics across duplicate IRIs, escape OBO synonym quotes, propagate Horned incomplete-load warnings, cap disk-cache snapshot reads, surface cache write failures, and reject invalid UTF-8 ontology files ([#74](https://github.com/eddiethedean/ontocode/issues/74), [#57](https://github.com/eddiethedean/ontocode/issues/57), [#61](https://github.com/eddiethedean/ontocode/issues/61), [#35](https://github.com/eddiethedean/ontocode/issues/35), [#82](https://github.com/eddiethedean/ontocode/issues/82), [#36](https://github.com/eddiethedean/ontocode/issues/36))
- Token-bound diagnostic ranges, skip unresolved `"."` paths, prefer `owl:imports` for broken-import fixes, preserve CRLF in RemoveLine, canonicalize code-action namespace lookup, and convert byte columns from disk when the editor buffer is closed ([#78](https://github.com/eddiethedean/ontocode/issues/78), [#14](https://github.com/eddiethedean/ontocode/issues/14), [#8](https://github.com/eddiethedean/ontocode/issues/8), [#6](https://github.com/eddiethedean/ontocode/issues/6), [#84](https://github.com/eddiethedean/ontocode/issues/84), [#15](https://github.com/eddiethedean/ontocode/issues/15))
- Detect `WITH … INSERT/DELETE` SPARQL updates, fail CLI patch on index errors, cap patch JSON reads, and reject partial ApplyPatch code actions ([#114](https://github.com/eddiethedean/ontocode/issues/114), [#115](https://github.com/eddiethedean/ontocode/issues/115), [#34](https://github.com/eddiethedean/ontocode/issues/34), [#116](https://github.com/eddiethedean/ontocode/issues/116))
- Always include `obo_id` in SQL entity rows and detect `Self` restrictions by token, not substring ([#83](https://github.com/eddiethedean/ontocode/issues/83), [#140](https://github.com/eddiethedean/ontocode/issues/140))
- Preserve the query engine `truncated` flag in CLI SQL/SPARQL output ([#77](https://github.com/eddiethedean/ontocode/issues/77))
- Update `@PREFIX` / SPARQL-style `PREFIX` declarations during namespace migrate and copy them into extract-module headers ([#79](https://github.com/eddiethedean/ontocode/issues/79), [#80](https://github.com/eddiethedean/ontocode/issues/80))
- Escape extract-module stub path comments for Turtle and type Ontology stubs as `owl:Ontology` ([#25](https://github.com/eddiethedean/ontocode/issues/25), [#147](https://github.com/eddiethedean/ontocode/issues/147))
- Make OBO `atomic_write` Windows-safe with temp cleanup on failure, matching Turtle replace behavior ([#64](https://github.com/eddiethedean/ontocode/issues/64), [#165](https://github.com/eddiethedean/ontocode/issues/165))
- Use format-aware writers for refactor disk rollback and surface rollback I/O errors from axiom-patch and refactor apply ([#63](https://github.com/eddiethedean/ontocode/issues/63), [#76](https://github.com/eddiethedean/ontocode/issues/76), [#93](https://github.com/eddiethedean/ontocode/issues/93))
- Encode LSP semantic-token `delta_line`/`delta_start` from absolute previous positions so highlighting no longer drifts on multi-line Turtle/OBO ([#137](https://github.com/eddiethedean/ontocode/issues/137))
- Preserve `obo_id` when merging duplicate entity IRIs across documents so OBO write-back keeps `term_id` ([#138](https://github.com/eddiethedean/ontocode/issues/138))
- Align Auto explain backends: report the concrete classify engine in `profile_used` and route CLI/LSP explanations through that engine ([#139](https://github.com/eddiethedean/ontocode/issues/139))
- Use jailed resolved paths for `createOntology` / `exportOntology` I/O so relative paths write under the workspace, not the LSP process CWD ([#123](https://github.com/eddiethedean/ontocode/issues/123))
- Reject unsafe ontology/version/prefix IRIs in `createOntology` and `ontocore new`, and refuse CLI overwrite unless `--force` ([#124](https://github.com/eddiethedean/ontocode/issues/124))
- Replace existing `owl:versionIRI` in `SetVersionIri` instead of appending another statement ([#125](https://github.com/eddiethedean/ontocode/issues/125))
- Rewrite CURIE-form `owl:Ontology` subjects in `SetOntologyIri` instead of appending a second ontology declaration ([#126](https://github.com/eddiethedean/ontocode/issues/126))
- Validate annotation predicate CURIEs against known prefixes and PN_LOCAL rules so Turtle injection via `AddAnnotation` / `AddOntologyAnnotation` is rejected ([#127](https://github.com/eddiethedean/ontocode/issues/127))
- Remap blank nodes per document when merging into the shared SPARQL store so multi-file workspaces no longer fuse unrelated restrictions ([#160](https://github.com/eddiethedean/ontocode/issues/160))
- Reject unknown SQL column identifiers instead of coercing them to empty strings ([#159](https://github.com/eddiethedean/ontocode/issues/159))
- Recognize `@PREFIX` and SPARQL-style `PREFIX` in prefix patch ops so Prefix Manager updates replace existing declarations ([#158](https://github.com/eddiethedean/ontocode/issues/158))
- Write URL-shaped annotation values as string literals unless explicitly marked as IRIs (`<…>` or a known CURIE) ([#157](https://github.com/eddiethedean/ontocode/issues/157))
- Preserve typed RDF literal datatypes across disk-cache round-trips (`^^<datatype>` serialization) ([#156](https://github.com/eddiethedean/ontocode/issues/156))
- Jail ROBOT `query`/`update` path flags (`--query`, `--update`, `--output-dir`, …) in `runRobot` so `--query=/outside/...` cannot escape the workspace ([#155](https://github.com/eddiethedean/ontocode/issues/155))
- Treat cancelled editor sync as failure in v0.17 Prefix Manager / metadata `applyDocumentPatches` ([#150](https://github.com/eddiethedean/ontocode/issues/150))
- Emit Turtle subject CURIE needles from the longest namespace match + PN_LOCAL suffix (not entity `short_name`) ([#146](https://github.com/eddiethedean/ontocode/issues/146))
- Resolve entity→document ownership by exact ontology id first, then longest matching `base_iri` (not first prefix match) ([#145](https://github.com/eddiethedean/ontocode/issues/145))
- End OBO term blocks at the next stanza header (`[Typedef]`, `[Instance]`, …), not only at the next `[Term]` ([#144](https://github.com/eddiethedean/ontocode/issues/144))
- Drop the catalog `RwLock` before incremental reindex so document sync is not blocked for the whole build ([#162](https://github.com/eddiethedean/ontocode/issues/162))
- Jail in-process `runPlugin` / `plugins run` export output under the workspace (default `.ontocore/plugin-out`) instead of process CWD ([#136](https://github.com/eddiethedean/ontocode/issues/136))
- Target ApplyPatch code-action WorkspaceEdits at the open document path, not diagnostic `document_path` ([#135](https://github.com/eddiethedean/ontocode/issues/135))
- Reject SQL `HAVING` as unsupported instead of silently returning unfiltered rows ([#134](https://github.com/eddiethedean/ontocode/issues/134))
- Reject SQL `JOIN` (including `FROM a JOIN b`) instead of silently returning the left table only ([#133](https://github.com/eddiethedean/ontocode/issues/133))
- Populate `deleteImpact.referencing_entities` from usage referencers (and resolve axiom docs by ontology IRI) so delete confirmation lists dependents ([#128](https://github.com/eddiethedean/ontocode/issues/128))

## [0.16.0] - 2026-07-09

### Added

- **Plugin preferences pages** — extension command **Plugins: Open Preferences…** hosts `ui.preferences_pages` contributions
- **Plugin context actions** — extension command **Plugins: Run Context Action…** runs `ui.context_actions` against the focused entity
- **Imports reload** — command **OntoCode: Reload Imports** re-indexes and refreshes the imports panel
- **Layout reset** — command **OntoCode: Reset Layout** closes key OntoCode panels

### Changed

- Plugin `ui.commands` contributions execute via LSP `ontocore/runPlugin` (validator/export/workflow dispatch)
- Workspace package and all `ontocore-*` crates at **0.16.0**; extension marketplace **0.16.0**

### Fixed

- **OBO idspace IRI expansion** — standard `idspace:` headers now produce canonical PURLs (`GO:0000001` → `…/GO_0000001`) ([#111](https://github.com/eddiethedean/ontocode/issues/111))
- **OBO patch input validation** — reject patch values with embedded newlines or invalid tokens before writing to disk ([#112](https://github.com/eddiethedean/ontocode/issues/112))

## [0.15.0] - 2026-07-08

### Added

- **Plugin API v0.15** — manifest `permissions`, `api_version = "1"`, UI views and commands; LSP `ontocore/runPlugin` `ui_view` action
- **Plugin UI views** — dockable webview panels via **Plugins: Open View…**; `PluginViewPanel` host
- **Explanation alternatives** — multiple unsatisfiability justifications via `explain_alternatives`; explanation panel justification selector
- **Explanation staleness** — `indexed_at` and `content_hash` on explanation payloads; stale warning in explanation panel
- **Graph panel upgrades** — asserted / inferred / combined modes, grid/circle/stack layouts, search filter
- **Subprocess plugin hardening** — path-jail improvements and security regression tests (`tests/cli_plugins_security.rs`)
- **Examples** — `demo-ui-view.toml` plugin fixture and `demo_ui_view.sh`

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.15.0**
- Extension marketplace version **0.15.0**
- Plugin host enforces declared permissions; backward-compatible defaults for manifests omitting `permissions`

### Fixed

- **Multi-root workspaces** — `ontocore/indexWorkspace` respects requested root when indexing multi-root workspaces

## [0.14.0] - 2026-07-09

### Added

- **Plugin host MVP** — `PluginHost` registry, manifest discovery from `.ontocore/plugins/`, in-process reference plugins, subprocess workflow runner
- **Reference plugins** — naming convention validator, Markdown exporter, SHACL scaffold (`ontocore-plugin-naming`, `ontocore-plugin-markdown-export`, `ontocore-plugin-shacl`)
- **CLI** — `ontocore plugins list`, `ontocore plugins run`, `ontocore workflow run --plugin owlmake --step qc`, plugin diagnostics in `validate`, `docs --plugin`
- **LSP** — `ontocore/listPlugins`, `ontocore/runPlugin`; plugin diagnostics merged on index
- **OntoCode** — plugin commands, workflow output panel scaffold, OntoUI capability registry + inspector plugin cards
- **Examples** — `examples/plugin-workspace/` fixture; [Plugin authoring guide](docs/guides/plugins.md)

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.14.0**
- Extension marketplace version **0.14.0**
- `Diagnostic` model extended with `PluginViolation` and plugin metadata

## [0.13.0] - 2026-07-08

### Added

- **OntoUI platform** — `WorkspaceHost`, Zustand `WorkspaceStore`, event bus, `WorkspaceRegistry`, design tokens, shared primitives
- **Focus relay** — extension-host `FocusRelayService` syncs Current Focus across Inspector, Graph, and Query webviews
- **Schema browser** — Query Workbench sidebar backed by LSP `ontocore/listSqlSchema`
- **Horned-OWL SQL tables** — `restrictions`, `equivalent_class_axioms`, `disjoint_class_axioms`, `domain_axioms`, `range_axioms`
- **`ontocore diff --pr-summary`** — PR-ready Markdown diff format; LSP `format: "pr-summary"`
- **Configurable diagnostics** — `.ontocore/diagnostics.toml` and `ontocode.diagnostics.rules` setting
- **LSP semantic tokens** — Turtle and OBO (`namespace`, `iri`, `keyword`, `comment`, `string`)
- **Docs export** — class hierarchy tree and property index in `ontocore docs` Markdown index
- **Benchmark smoke tests** — `tests/bench_index.rs` and `scripts/fetch-bench-fixtures.sh`

### Changed

- Entity Inspector, Graph, Query Workbench, and Refactor Preview read state from WorkspaceStore
- Workspace package and all `ontocore-*` crates bumped to **0.13.0**
- Extension marketplace version **0.13.0**

### Fixed

- **Refactor rename** — do not rewrite IRIs inside Turtle single-quoted string literals
- **Axiom patch feedback** — report failure when editor buffer sync is cancelled instead of false success
- **Manchester editor** — discard stale parse results when switching entities during bootstrap
- **Git worktree diff** — include untracked ontology files in `HEAD..WORKTREE` catalog comparison
- **Reasoner** — derive asserted hierarchy from loaded ontology (including open buffers) so buffer subclass axioms are not misreported as inferences
- **OBO patches** — exact `id:`, `is_a:`, `xref:`, and `synonym:` matching (no prefix collisions such as `EX:001` vs `EX:0010`)
- **Property chain editor** — offer object property IRIs only; reject class IRIs in `add_property_chain`

## [0.12.0] - 2026-07-06

### Added

- **Turtle authoring parity** — patch ops for domain, range, property characteristics, property chains, individual assertions, and generic annotations
- **`ontocore-obo` crate** — OBO Format 1.4 patch write-back (`set_name`, synonyms, definitions, xrefs, `is_a`, deprecated) per ADR-0019
- **OBO Entity Inspector** — edit forms with preview-before-apply for `.obo` documents
- **OWL/XML read path** — Horned-OWL catalog for `.owl` RDF/XML and native `.owx` parsing; read-only inspector for non-Turtle OWL formats
- **DL unsatisfiability explanations** — `explain_unsatisfiable_dl` with profile label in explanation panel
- **Protégé round-trip fixtures** — `examples/protege-roundtrip/` corpus and `cargo test protege_roundtrip` CI gate
- **`PreviewApplyBar`** — reusable preview-then-apply component for all Turtle/OBO inspector edits
- **Property chain editor** — ordered property list with patch preview in Entity Inspector

### Fixed

- **OBO Entity Inspector** — `parseApplyPatchMessage` now accepts `term_id`-based OBO patches (fixes non-functional OBO edit UI)
- **Individual class assertions** — Entity Inspector wires Preview/Apply for `add_class_assertion` and Remove for existing types

### Changed

- LSP and CLI `patch` dispatch by file extension: `.ttl` → `ontocore-owl`, `.obo` → `ontocore-obo`
- Entity Inspector shows domain/range/characteristics, annotations, and property chains for Turtle entities
- Workspace package and all `ontocore-*` crates bumped to **0.12.0**
- Extension marketplace version **0.12.0**

## [0.11.3] - 2026-07-06

### Fixed

- **Entity Inspector navigation** — opening a new entity while an inspector is already open now reuses the panel and loads the new entity (stale `requestId` guard no longer blocks newer navigation)

### Added

- **VS Code e2e tests** — inspector entity switching, workspace index/refresh commands, and smoke-panel regression coverage

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.11.3**
- Extension marketplace version **0.11.3** (VS Code Marketplace + Open VSX)

## [0.11.2] - 2026-07-06

### Fixed

- **React webview panel routing (follow-up)** — bootstrap now merges `panel=` into existing `window.location.search` (VS Code/Cursor webviews that already have query params no longer fall back to the Smoke panel); Entity Inspector recreates the panel if the webview never reported ready

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.11.2**
- Extension marketplace version **0.11.2** (VS Code Marketplace + Open VSX)
- Documentation adoption audit fixes (CLI examples, onboarding, contributor debugging guide)
- Remove explorer preview screenshots from docs and extension marketplace metadata

## [0.11.1] - 2026-07-06

### Fixed

- **React webview panel routing** — Entity Inspector and other panels showed the SmokePanel fallback because `?panel=` was on the script URL instead of the page location; host HTML now bootstraps `window.location.search` before React loads

### Added

- **Webview regression tests** — `webviewBootstrap` unit tests, React App bootstrap routing test, VS Code E2E hooks for inspector and Query Workbench HTML/ready checks

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.11.1**
- Extension marketplace version **0.11.1** (VS Code Marketplace + Open VSX)
- VS Code Marketplace badges use [vsmarketplacebadges.dev](https://vsmarketplacebadges.dev/) (shields.io `visual-studio-marketplace` endpoints retired)
- Explorer preview screenshot and marketplace hero image updated to match the React Entity Inspector UI (`./scripts/render-explorer-preview.sh`)
- User-facing docs: Open VSX install paths and badges; remove "Git-native" wording

## [0.11.0] - 2026-07-05

### Added

- **Open VSX publishing** — release workflow publishes VSIX to Open VSX for Cursor marketplace discoverability (`OVSX_PAT` secret)
- **LSP `textDocument/completion`** — Turtle prefix, QName, and IRI bracket completions from indexed catalog
- **Diagnostic quick fixes** — `undefined_prefix`, `missing_label`, and `broken_import` rules populate `quick_fix`; LSP `textDocument/codeAction` applies edits
- **`ontocore-docs` crate** — Markdown and HTML documentation export from indexed workspaces
- **`ontocore docs` CLI** — `--output`, `--format markdown|html`, optional `--ontology-id` filter
- **Import patch ops** — `add_import` and `remove_import` for Turtle `owl:imports`
- **Imports management UI** — Ontologies tree context menu **Manage Imports** with React panel and patch preview
- **OBO read path via `fastobo`** — richer synonyms, definitions, and property values in catalog; ADR-0019 documents v1.0 OBO write-back patch schema
- Migration guide [docs/migration/v0.11.md](docs/migration/v0.11.md); user guide [docs/guides/docs-export.md](docs/guides/docs-export.md)

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.11.0**
- Extension marketplace version **0.11.0** (VS Code Marketplace + Open VSX)

## [0.10.0] - 2026-07-04

### Added

- **Incremental workspace indexing** — content-hash reuse in `ontocore-catalog`; LSP debounced reindex avoids reparsing unchanged files
- **Multi-root LSP workspaces** — all VS Code folders indexed; `path_jail` and `didChangeWorkspaceFolders` support
- **Stable `ontocore::Workspace` API** — `open_with_options`, `reindex` / `reindex_incremental`, `import_graph`, `diff`, `stats`
- **`ontocore-diff` crate** — catalog semantic diff, breaking-change heuristics, git ref compare
- **`ontocore diff` CLI** — text/json/markdown output; directory and git range modes
- **LSP `ontocore/semanticDiff`** and VS Code **Semantic Diff** React panel
- **Optional disk cache** — `.ontocore/cache/` keyed by content hash (`ontocode.indexCache` / `WorkspaceOptions::disk_cache`)
- Migration guide [docs/migration/v0.10.md](docs/migration/v0.10.md); example `semantic_diff`

### Changed

- Workspace package and all `ontocore-*` crates bumped to **0.10.0**
- Extension marketplace version **0.10.0**

## [0.9.0] - 2026-07-03

### Added

- **`ontocore` crate** — public façade with `Workspace::open`, module re-exports, and `lsp` feature
- **OntoCore documentation** — `docs/ontocore/` and `docs/ontocode/` trees; ADR-0018; platform architecture (`VISION.md`, `ARCHITECTURE.md`, `ROADMAP.md`)
- Example `ontocore_workspace` using `Workspace` API
- Diagnostic codes `owl_bridge_failed` and `io_read_error`
- Release pipeline publishes `ontocore` façade; extended `check-doc-versions.sh`
- **OntoLogos 1.0.0** integration — real `dl` and `auto` reasoner adapters (`ontocore-reasoner`)
- DL/auto classification tests (library, CLI, LSP) and reasoner panel enablement in VS Code extension
- Plugin platform design — [PLUGIN_SPEC.md](docs/design/PLUGIN_SPEC.md) with build/workflow/release plugin categories; [owlmake](https://github.com/INCATools/owlmake) as reference external workflow plugin
- OBO/ROBOT spec — owlmake integration path and ODK workflow goals ([OBO_ROBOT_SPEC.md](docs/design/OBO_ROBOT_SPEC.md))

### Changed

- **Breaking:** rename all `ontoindex-*` crates to **`ontocore-*`**
- **Breaking:** CLI binary `ontoindex` → **`ontocore`** (`ontocore-cli` crate)
- **Breaking:** LSP binary `ontoindex-lsp` → **`ontocore-lsp`**; custom methods `ontoindex/*` → **`ontocore/*`**
- **Breaking:** `OntoIndexError` → **`OntoCoreError`**
- **OntoCore** platform branding across README, docs, extension output channel, and GitHub templates
- `apply_refactor_plan` requires `workspace_root`; diagnostic engine surfaces IO read failures
- Horned-OWL bridge failures emit catalog diagnostics instead of silent fallback
- OntoLogos workspace dependencies bumped from 0.9.0 → **1.0.0**
- Enterprise adoption docs reconciled with shipped DL/auto classification capability
- Extension marketplace metadata — OntoCore-powered description and expanded keywords
- `ontocore` crate README repositioned as public façade API

### Fixed

- LSP reasoner integration test updated for shipped DL/auto profiles
- MkDocs strict-mode documentation link fixes (ADR rename, concepts, contributing)
- Release packaging: license files for `ontocore` and `ontocore-robot`; crate READMEs and include lists for crates.io; release dry-run only on leaf crates

### Notes

- See [migration/v0.9.md](docs/migration/v0.9.md) for upgrade steps from v0.8
- `Workspace` API remains experimental until v0.10
- First crates.io publish under `ontocore-*` names (prior releases used `ontoindex-*`)

## [0.8.0] - 2026-06-26

### Added

- **`ontoindex-refactor` crate** — workspace-wide usage index; rename IRI, namespace migration, move entity, extract module with preview/apply
- CLI **`ontoindex refactor`** subcommands: `usages`, `rename`, `migrate-namespace`, `move`, `extract`
- LSP refactoring: `ontoindex/findUsages`, `ontoindex/previewRefactor`, `ontoindex/applyRefactor`
- Standard LSP **`textDocument/references`**, **`textDocument/rename`** (with `prepareRename`)
- VS Code refactor commands and **Refactor Preview** React panel
- Inspector **Find Usages** and **Rename IRI** actions
- Full Manchester catalog extensions: **disjoint classes** (author + patch), **domain/range** and **property chains** (view in axiom catalog)
- Patch ops: `add_disjoint_class`, `remove_disjoint_class`
- **React Query Workbench** and **React Manchester Editor** panels (legacy HTML webviews removed)
- Fixture: `fixtures/disjoint-classes.ttl`

### Changed

- Workspace and extension version **0.8.0**
- Axiom catalog groups axioms by kind in React inspector
- Manchester editor supports `disjoint_class` axiom kind with validation UI

### Fixed

- Query Workbench dropped successful results (runId stale-guard never updated)
- Namespace migration overwrote per-IRI renames when updating `@prefix` declarations
- Multi-entity extract module used stale byte offsets in the same file
- LSP rename/references: prefixed rename targets, error reporting, and reference range width
- Explorer refreshed before refactor apply; disjoint axiom edit now passes `other_iri`
- Manchester editor: restored data property/datatype pickers; panel CSS for v0.8 React panels
- EL `classify` false negatives on unsatisfiable ontologies (reasoner ontology merge via triple bridge)
- Extract/move to new files failed path validation when target file did not exist yet
- LSP axiom patch uses atomic disk writes; buffer updated before disk
- Refactor rollback errors propagated when restore fails mid-apply
- Reasoner panel runId synchronization between host and webview
- RL/RDFS profiles report unsatisfiable classes (EL post-check)
- Catalog indexes orphan LSP buffer overrides not returned by workspace scan
- LSP patch/refactor require indexed catalog; `APPLIED_NOT_INDEXED` when reindex fails after apply
- SPARQL update guard bypass after `PREFIX` or comment lines
- Capped file reads in parser, catalog semantics, and refactor preview/backup paths

[0.18.2]: https://github.com/eddiethedean/ontocode/releases/tag/v0.18.2
[0.18.1]: https://github.com/eddiethedean/ontocode/releases/tag/v0.18.1
[0.18.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.18.0
[0.17.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.17.0
[0.16.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.16.0
[0.15.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.15.0
[0.14.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.14.0
[0.13.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.13.0
[0.12.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.12.0
[0.11.3]: https://github.com/eddiethedean/ontocode/releases/tag/v0.11.3
[0.11.2]: https://github.com/eddiethedean/ontocode/releases/tag/v0.11.2
[0.11.1]: https://github.com/eddiethedean/ontocode/releases/tag/v0.11.1
[0.11.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.11.0
[0.10.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.10.0
[0.9.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.9.0
[0.8.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.8.0

## [0.7.0] - 2026-06-25

### Added

- **React webview foundation** (`extension/webview-ui/`) — Vite + React, typed message protocol, CSP panel host
- **Graph visualization** — class, property, import, and neighborhood graphs via LSP `ontoindex/getGraph` and React `@xyflow/react` panels
- **React Entity Inspector** — migrated from legacy HTML webview with edit/patch parity
- **OBO format** — `.obo` scanning, parsing, `obo_id` in catalog/SQL, explorer labels
- **`ontoindex-robot` crate** — ROBOT CLI wrappers; CLI `ontoindex robot validate|merge|report`; LSP `ontoindex/runRobot`
- Extension commands: `openClassGraph`, `openPropertyGraph`, `openImportGraph`, `openNeighborhoodGraph`, `openGraph`
- OBO TextMate grammar, `ontocode.robotPath` setting, `examples/obo-workflow/`
- Docs: [webview-protocol.md](docs/webview-protocol.md)

### Changed

- Workspace and extension version **0.7.0**
- Entity inspector and graph panels use React bundle in VSIX

### Fixed

- Webview ready/init races (graph, reasoner, inspector panels buffer messages until `ready`)
- LSP `effective_index_root()` for consistent reindex and patch paths
- `applyAxiomPatch` result contract (`reindex_warning`, disk-before-buffer write, diagnostics on partial apply)
- Reasoner vs index worker serialization; `getExplanation` content-hash cache staleness
- Patch engine: safer removal, subject-boundary entity detection, batch preview on failure
- Manchester: unclosed IRI errors, unknown-prefix validation
- OBO files no longer marked editable in catalog; OBO file size cap enforced
- Diagnostics: orphan import roots, undefined-prefix false positives
- SQL/SPARQL: reject top-level bare `WHERE` columns and SPARQL UPDATE forms
- Extension: multi-root folder picker, `obo`/`json-ld` document selectors, diagnostic UTF-16 columns, hierarchy `hasChildren`

[0.7.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.7.0

## [0.6.0] - 2026-06-24

### Added

- **`ontoindex-reasoner` crate** — thin OntoLogos 0.9.0 facade (`el`, `rl`, `rdfs` adapters; `dl`/`auto` stubbed until OntoLogos 1.0)
- CLI **`ontocore classify`** and **`ontocore explain`**
- LSP **`ontoindex/runReasoner`** and **`ontoindex/getExplanation`**
- VS Code **Reasoner Results** panel, **Explanation** panel, hierarchy mode toggle (`asserted` / `inferred` / `combined`)
- Settings: `ontocode.reasoner.default`, `ontocode.reasoner.autoProfile`, `ontocode.hierarchy.mode`
- Fixtures: `fixtures/reasoner-el.ttl`, `fixtures/reasoner-unsat.ttl`
- User guide: [docs/guides/reasoner.md](docs/guides/reasoner.md)
- **Enterprise documentation pack** — production-readiness, enterprise-deployment, performance-sizing, LGPL compliance guides
- **Documentation adoption** — concepts, best-practices, Protégé coexistence, Rust library guide, migration index, crate READMEs
- **`read_file_capped`** / **`parse_boolean_literal`** helpers in `ontoindex-core`

### Changed

- `CatalogSnapshot` includes optional `reasoner` metadata after classification
- Workspace crates bumped to **0.6.0**
- SQL JSON export uses column-ordered row arrays
- Inspector webview loads entity data via `postMessage` (no inline `<script>` embedding)

### Fixed

- **Turtle patch write-back:** multi-statement subject blocks, CRLF byte spans, predicate-object removal (not line deletion), literal-safe separator cleanup
- **Resource limits:** capped file reads (scanner, LSP disk fallback, patch apply); `MAX_ENTITIES` fail-fast during catalog build; filesystem walk entry cap; index worker job coalescing
- **`ontology_id` mismatch** between entities and axioms/annotations when `owl:Ontology` is declared
- **SQL:** row cap during iteration (not after full materialization); `SELECT col AS alias` projection
- **`owl:deprecated` false positives** from substring `.contains("true")`
- **Extension:** inspector/query/reasoner stale-async guards; stricter LSP protocol validation

[0.6.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.6.0

## [0.5.0] - 2026-06-24

### Added

- **Query workbench (VS Code):** SQL and SPARQL modes, result table, CSV/JSON export, saved queries, query history, starter templates
- LSP **`ontoindex/query`** and **`ontoindex/sparql`** — tabular results against indexed workspace catalog
- **Manchester MVP editor (VS Code):** complex `SubClassOf` and `EquivalentClasses` authoring with validate, expression tree, Turtle preview
- LSP **`ontoindex/parseManchester`** — parse/validate Manchester expressions with catalog-based completion lists
- **`ontoindex-owl` Manchester module** — parse, serialize, Turtle fragment generation, expression tree JSON
- New patch ops: `add_complex_sub_class_of`, `remove_complex_sub_class_of`, `add_equivalent_class`, `remove_equivalent_class`, `set_equivalent_class`
- **`EntityDetail.axioms`** — structured `EntityAxiomSummary` rows (kind, display, manchester, parent_iri, editable)
- `fixtures/complex-classes.ttl` for Manchester and consistency tests
- Extension integration tests for SQL, SPARQL, Manchester parse, and structured axioms

### Changed

- Inspector shows **Edit in Manchester** for complex axioms and **Add Manchester axiom**
- README capability table: SQL + SPARQL in VS Code → **Yes**

### Fixed

- **Turtle span/patch engine:** structural subject/block detection; complex axiom removal via blank-node spans; transactional patch apply; Turtle literal escaping
- **Manchester:** `and` / `or` serialization emits full operand lists; SubClassOf edit uses remove+add (no duplicate axioms)
- **LSP:** patches apply to open document buffer before disk write; `APPLIED_NOT_INDEXED` when reindex fails after apply
- **Query layer:** SPARQL truncates at row cap instead of hard-failing; SQL filter errors propagate; correct `truncated` flag at exactly 100k rows
- **Extension:** query result table uses safe DOM APIs (XSS); validate/run sequence IDs ignore stale responses; `@prefix` fallback when catalog namespaces are unavailable

[0.5.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.5.0

## [0.4.0] - 2026-06-24

### Added

- **Simple write-back (v0.4a):** create/edit/delete entities; labels, comments, simple `SubClassOf` in Turtle
- New **`ontoindex-owl`** crate — Horned-OWL facade for axiom modeling and patch write-back
- LSP **`ontoindex/applyAxiomPatch`** — preview and apply patch operations
- CLI **`ontocore patch`** — apply patches from JSON
- Editable **Entity Inspector** and explorer create/delete commands in VS Code
- **`EntityDetail.editable`** and `document_path` for authoring UI
- Oxigraph ↔ Horned-OWL **consistency tests** and `examples/protege-roundtrip/` fixtures
- [docs/authoring.md](docs/authoring.md)

### Changed

- Turtle catalog entities/axioms sourced from Horned-OWL when parse succeeds (dual stack per ADR-0013)
- Workspace MSRV bumped to **1.88** (Horned-OWL 1.4)
- Label strings in catalog normalized (no extra RDF literal quotes from Horned-OWL bridge)
- Read the Docs site, first-success tutorial, errors reference, and enterprise evaluation guide

[0.4.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.4.0

## [0.3.0] - 2026-06-23

### Added

- **Ontology diagnostics (v0.3):** parse errors, broken imports, undefined prefixes, duplicate/missing labels, orphan classes
- New `ontoindex-diagnostics` crate with catalog lint rules
- `diagnostics` SQL virtual table (`SELECT * FROM diagnostics`)
- LSP `textDocument/publishDiagnostics` after workspace reindex (VS Code Problems panel)
- Diagnostics explorer tree grouped by severity
- `ontocore validate` prints all diagnostics; non-zero exit on errors
- Open-buffer parsing for inline diagnostics on unsaved edits

### Fixed

- Diagnostic file paths when entity `ontology_id` is an ontology IRI (not `doc-N`)
- LSP always responds to hover/definition/symbol requests (`null` when no result)
- LSP advertises `textDocumentSync` so unsaved-buffer diagnostics work in VS Code
- RDF/XML `xmlns` prefix extraction; fewer false `undefined_prefix` reports
- Orphan-class heuristic skips taxonomy roots with in-workspace children
- Import IRI normalization (trailing slash); stale Problems panel entries cleared after reindex

[0.3.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.3.0

## [0.2.3] - 2026-06-11

### Fixed

- **VS Code extension:** **Open Entity Inspector** and **Jump to Source** from the explorer context menu (tree items pass an object, not an IRI string)

### Changed

- Extension Marketplace README — step-by-step usage guide, troubleshooting, and preview image
- Marketplace listing description and search keywords

[0.2.3]: https://github.com/eddiethedean/ontocode/releases/tag/v0.2.3

## [0.2.2] - 2026-06-11

### Fixed

- **VS Code extension:** await `LanguageClient.start()` (v9 removed `onReady()` — startup was broken)
- **macOS:** clear `com.apple.quarantine` on bundled `ontoindex-lsp` when present

### Added

- Extension startup regression guards (`clientStartup.guard.test.ts`) — block `onReady()` and non-awaited `start()` from shipping again
- VS Code integration tests (`@vscode/test-electron`) and CI matrix across Ubuntu, macOS, and Windows on VS Code 1.85.0 and stable

[0.2.2]: https://github.com/eddiethedean/ontocode/releases/tag/v0.2.2

## [0.2.1] - 2026-06-11

### Fixed

- **VS Code extension:** set executable permission on bundled `ontoindex-lsp` before spawn (fixes `EACCES` on macOS/Linux after Marketplace or VSIX install)

### Added

- Extension e2e tests: simulate Marketplace `chmod 644` on bundled LSP and verify spawn after fix; CI VSIX unpack regression test

[0.2.1]: https://github.com/eddiethedean/ontocode/releases/tag/v0.2.1

## [0.2.0] - 2026-06-11

### Added

- **OntoCode Explorer** — VS Code extension (`extension/`) with ontology tree views and entity inspector
- `ontoindex-lsp` — language server with custom methods (`indexWorkspace`, `getCatalogSnapshot`, `getEntity`)
- LSP browsing aids — hover, document/workspace symbols, go-to-definition, debounced re-index on file changes
- Catalog entity APIs — hierarchy, entity detail, and jump-to-source helpers in `ontoindex-catalog`
- LSP smoke integration test and CI jobs for LSP + extension builds
- Release workflow assets for `ontoindex-lsp` binary and multi-platform extension VSIX (Linux, macOS, Windows)
- User docs: `docs/lsp-api.md`, `docs/vscode-install.md`, `docs/release-integrity.md`
- Design docs under `docs/design/` including v1.0 Protégé parity matrix and Rust-native reasoner strategy (ADR-0014)

### Fixed

- LSP JSON wire format uses snake_case enums (`class`, `ok`, …) — aligned with extension, tests, and `docs/lsp-api.md` (ADR-0012)
- Workspace reindex debouncing and notifications when open documents change
- Jump-to-source for prefixed Turtle entity names
- Explorer shows classes whose parents are declared in another ontology file
- Structured `LspErrorPayload` for custom LSP error responses

[0.2.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.2.0

## [0.1.0] - 2026-06-10

### Added

- **OntoCore foundation** — Rust workspace for local-first ontology indexing
- `ontoindex-core` — workspace scanner, shared types, content hashing
- `ontoindex-parser` — RDF/OWL parsing and entity extraction via Oxigraph
- `ontoindex-catalog` — semantic catalog and triple store
- `ontoindex-query` — SQL-like virtual tables and SPARQL queries
- `ontoindex-cli` — `ontoindex` binary with `index`, `query`, `sparql`, `validate`, and `inspect` commands
- Fixture ontology and integration/golden snapshot tests
- CI and crates.io release workflows

[0.1.0]: https://github.com/eddiethedean/ontocode/releases/tag/v0.1.0
