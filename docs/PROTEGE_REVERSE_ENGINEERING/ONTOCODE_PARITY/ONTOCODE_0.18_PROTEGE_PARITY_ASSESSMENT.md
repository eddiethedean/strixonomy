# Strixonomy 0.18 Protégé Desktop Parity Assessment

## Executive summary

Audit date: 2026-07-10 · Baseline: **v0.17.0** · Method: docs/audit-first against reverse-engineering checklists + SHIPPED / known-limitations / code.

**Overall Protégé Desktop parity (corrected for false greens): ~88–92%** for the daily Turtle/OBO modeling loop. Remaining debt is concentrated in last-mile fidelity (layout restore, reasoner cancel/lifecycle, explanation staleness UX, workflow fixtures, a11y), not greenfield features.

WebProtégé / collaboration remain **out of scope** for this gate (post-1.0).

| Area | Corrected estimate | Notes |
|------|-------------------:|-------|
| Ontology editing (Turtle/OBO) | 95% | Write-back limits for RDF/XML / OWL/XML are known gaps |
| Navigation & search | 95% | |
| Refactoring | 90% | Turtle-only |
| Reasoning integration | 85% | Shared lifecycle path; Stop is UI-only |
| Explanations | 80% | Unsat MVP shipped; stale UX not wired; DL partial |
| Querying | 95% | |
| Validation | 90% | |
| Imports | 85% | Add/remove/reload shipped; inspector depth thin |
| Visualization | 75% | Asserted/inferred graphs shipped; OWLViz/OntoGraf layout/filter polish open |
| Workspace shell | 70% | Menus/dialogs shipped; layout restore Partial |
| Preferences | 40% | VS Code settings hub; not Protégé prefs dialog |
| Plugins | 55% | Host MVP; marketplace API → v1.0 |
| CLI / LSP / semantic diff | Beyond Protégé | Differentiators |
| Collaboration / WebProtégé | 10–15% | Explicit non-goal |

---

## Checklist honesty

| Doc | Honesty issue |
|-----|---------------|
| [UI/MENUS.md](../UI/MENUS.md) | Over-checked: Persist layout, Stop reasoner, Long-running task cancellation marked `[x]` while Partial/Missing in code |
| [UI/TOOLBARS.md](../UI/TOOLBARS.md) | Over-checked: Start/Stop/Classify/Open/Save on toolbar claimed; mostly palette-only |
| [UI/DIALOGS.md](../UI/DIALOGS.md) | Mild over-check on Preferences/Reasoner as dedicated DialogShell |
| [UI/VIEWS.md](../UI/VIEWS.md) | Under-checked (mostly `[ ]` despite shipped explorer/inspector/graphs) |
| [UI/WORKSPACE.md](../UI/WORKSPACE.md) | Under-checked |
| [UI/PREFERENCES.md](../UI/PREFERENCES.md) | Thin but honest empty checklist |

False greens reconciled in MENUS as part of v0.18 (see Must-ship).

---

## Gate blockers (v0.18 Must-ship)

These block an honest “Protégé not required for agreed Desktop scope” claim:

1. **True layout / perspective restore** — serializers keep tabs but restore recovery HTML only (`extension/src/webviews/layoutPersistence.ts`).
2. **Reasoner cancel + distinct lifecycle** — `stopReasoner` clears UI only; Start/Sync/Classify/Consistency share one classify path (`v017Commands.ts`).
3. **Explanation staleness UX** — `isExplanationStale` exists and is tested; panel hardcodes `stale = false`.
4. **Parity / workflow regression fixtures** — `examples/protege-roundtrip/` + thin `tests/protege_roundtrip.rs`; unused fixtures; no model→reason→explain→stale workflow tests.
5. **Import / serialization edge coverage** — broken-import lint exists; circular/OWX/RDFXML edge fixtures incomplete.
6. **Accessibility pass** on Explanation, Imports, Graph, Reasoner panels (keyboard/ARIA basics).
7. **Migration guide refresh** + honest desktop known-gap list (guide still framed as v0.14).
8. **Checklist honesty pass** on MENUS false greens.
9. **Large-ontology hardening (scoped)** — explorer/graph truncation messaging + incremental refresh where catalog already supports it (full virtualization library optional if truncation + lazy expand suffice).

---

## Documented known gaps (not gate blockers — v1.0+)

- OWL/XML and RDF/XML write-back
- Full multi-step semantic undo
- Persistent tabs + bottom dock / review workspace
- HermiT/Pellet JVM switch; incremental ELK-style reasoning
- Explain all inference kinds beyond unsat
- Full OntoGraf filter/layout suite; editable graphs
- Stable semver-1.0 plugin marketplace API
- WebProtégé / collaboration / OntoStudio / AI / SDKs / MCP

---

## Fixture inventory

### Exists

| Asset | Coverage |
|-------|----------|
| `examples/protege-roundtrip/*.ttl|.owl|.owx` | Sample ontologies |
| `tests/protege_roundtrip.rs` | 4 smoke tests (patch preview, individuals index, Horned OWL/OWX load) |
| `tests/reasoner_*.rs`, `cli_classify.rs`, `lsp_reasoner.rs` | Classify / unsat |
| `tests/cli_patch_manchester.rs` | Manchester subclass patch |
| UI unit tests | Graph, explanation logic, imports ontology helpers |

### Missing (gate)

| Workflow | Needed |
|----------|--------|
| Unused roundtrip fixtures asserted | `people`, `chains`, `annotations` |
| Unsat → explain fingerprint → reindex → stale | Engine +/or extension logic test |
| Import add/remove + broken import | Fixture + CLI/catalog test |
| Distinct consistency vs classify messaging | Extension unit or integration |
| OWX / RDFXML read-only honesty | Already partially covered; keep regression |

---

## Areas already beyond Protégé

Semantic diff, Git/CI workflows, SQL catalog queries, LSP, incremental indexing, namespace migration, batch refactor, docs export, VS Code integration.

---

## Exit criterion mapping

| ROADMAP exit | How this assessment satisfies it |
|--------------|----------------------------------|
| Desktop parity = 100% for agreed pre-1.0 scope | Agreed scope = **gate blockers above**; 100% means those are green; remaining items live in known-gap list |
| Protégé → Strixonomy migration path | Refresh `docs/guides/protege-migration.md` + `docs/migration/v0.18.md` + known-limitations |

See [v0.18_SCOPE.md](../../design/v0.18_SCOPE.md) for the locked Must-ship list derived from this assessment.
