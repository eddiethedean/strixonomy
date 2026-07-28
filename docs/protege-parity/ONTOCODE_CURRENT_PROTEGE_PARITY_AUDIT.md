# Strixonomy Protégé Parity Audit

**Repository audited:** `strixonomy-main (2)(1).zip`  
**Repository version:** Extension `0.18.2`  
**Audit type:** Static source, documentation, fixture, and test inventory audit  
**Target:** Full Protégé Desktop parity before Strixonomy `v0.30.0`

---

## 1. Executive Summary

Strixonomy is a substantial ontology engineering platform rather than an early editor prototype. The audited repository contains:

- 20 Rust workspace crates
- A VS Code/Cursor extension with 79 contributed commands
- A React webview interface
- A language server
- Turtle and OBO authoring
- OWL/XML and RDF/XML indexing
- EL, RL, RDFS, DL, and automatic reasoner adapters
- Unsatisfiability explanations
- SQL-like and SPARQL query support
- Semantic refactoring
- Semantic diff
- Plugin discovery and execution
- Extensive documentation and automated test source

The repository is already competitive with Protégé in several modern engineering workflows, especially Git integration, semantic diff, LSP tooling, CLI automation, and workspace indexing.

However, the current code does **not** support a defensible claim of complete Protégé Desktop parity.

### Audit conclusion

| Scope | Estimated parity |
|---|---:|
| Turtle/OBO daily ontology workflow | 88–92% |
| Agreed historical v0.18 reduced parity scope | Approximately 90% |
| Full clean-install Protégé Desktop capability parity | Approximately 65–72% |
| Full parity under the new v0.29–v0.30 requirement | Not yet achieved |

The estimate is intentionally conservative. It distinguishes between:

- A command being registered
- A UI shell existing
- A feature working for Turtle only
- A feature being semantically complete across required formats
- A feature being covered by executable tests

The most important conclusion is that the project’s previous `~88–92%` estimate applies to a narrowed **Turtle/OBO modeling loop**, not complete Protégé parity.

---

## 2. Audit Method

The audit examined:

- Root workspace configuration
- All Rust crates and source modules
- Extension command contributions
- LSP source
- React webview panels
- Workspace state and layout persistence
- Parser and serializer paths
- OWL authoring patch operations
- Reasoning adapters and explanation code
- Refactoring operations
- Plugin runtime and manifests
- Query implementation
- Test source and fixtures
- `SHIPPED.md`
- `known-limitations.md`
- Existing parity assessments
- Reverse-engineering documents
- Roadmaps and architecture decisions

### Verification limitation

The execution environment did not contain the Rust toolchain, so `cargo test`, `cargo clippy`, and `cargo fmt` could not be executed. The audit therefore confirms the **presence and apparent intent** of code and tests, but not their current runtime success.

The attempted command failed with:

```text
cargo: command not found
```

Node dependency installation and extension tests were not run because the repository was audited without modifying its dependency state.

---

## 3. Repository Evidence Summary

### Rust workspace

The workspace contains the following major subsystems:

- `strixonomy`
- `strixonomy-core`
- `strixonomy-parser`
- `strixonomy-owl`
- `strixonomy-obo`
- `strixonomy-catalog`
- `strixonomy-diagnostics`
- `strixonomy-query`
- `strixonomy-reasoner`
- `strixonomy-robot`
- `strixonomy-refactor`
- `strixonomy-diff`
- `strixonomy-docs`
- `strixonomy-cli`
- `strixonomy-lsp`
- `strixonomy-plugin`
- Reference plugin crates

### Extension

The extension declares **79 commands**, including workflows for:

- Ontology creation, opening, saving, save-as, export, and close
- Class, property, and individual creation
- Entity deletion and inspection
- Manchester authoring
- Querying
- Reasoning and explanations
- Graphs
- Imports
- Semantic diff
- Refactoring
- Workspace validation
- Perspectives
- Plugins
- Diagnostics and support

Command registration is strong evidence of product surface area, but it is not by itself proof of full workflow completion.

### Test inventory

Static inspection found approximately:

- 150 Rust source/test files under crates and integration tests
- 54 TypeScript/TSX test files
- 20 fixture files across primary fixture directories

This is a healthy testing foundation. It is not yet a full executable Protégé conformance corpus.

---

## 4. Capability Audit

## 4.1 Ontology Lifecycle

**Status: PARTIAL–STRONG**

### Evidence

Extension commands exist for:

- New ontology
- Open ontology
- Recent ontology
- Save
- Save all
- Save as
- Export
- Close project

The extension contains a dedicated `NewOntologyDialog`, prefix manager, imports panel, and workflow tests.

### Remaining gaps

- Lifecycle behavior is constrained by serialization support.
- Saving OWL/XML and RDF/XML in-place is not supported.
- Complete crash recovery and transaction journal behavior are not evident.
- “Save All” exists as a command, but multi-ontology semantic persistence requires deeper verification.
- Ontology project semantics remain partially tied to VS Code workspace/file behavior.

### Parity judgment

Protégé-equivalent lifecycle is available for the primary Turtle/OBO workflow, but not across the complete set of commonly used Protégé documents.

---

## 4.2 Ontology Formats and Write-Back

**Status: PARTIAL — P0 BLOCKER**

### Confirmed support

| Format | Parse/index | Query/browse | Write-back |
|---|---:|---:|---:|
| Turtle | Yes | Yes | Yes |
| OBO | Yes | Yes | Yes |
| OWL/XML | Yes | Yes | No |
| RDF/XML | Yes | Yes | No |
| JSON-LD | Yes | Yes | No |
| N-Triples | Yes | Yes | No |
| N-Quads | Yes | Yes | No |
| TriG | Yes | Yes | No |

### Code evidence

`crates/strixonomy-owl/src/patch.rs` explicitly limits OWL patch write-back to Turtle:

```rust
if format != OntologyFormat::Turtle {
    return Err(... "write-back supports Turtle (.ttl) only" ...)
}
```

OBO has a separate patch/write-back crate.

### Gap

A normal Protégé `.owl` file may be OWL/XML or RDF/XML. Strixonomy can index and inspect these files, but users cannot perform normal in-place authoring and save workflows.

### Required work

- Canonical semantic change model independent of syntax
- OWL/XML serializer/write-back
- RDF/XML serializer/write-back
- Axiom annotation preservation
- Ontology/version IRI preservation
- Imports preservation
- Blank-node and anonymous expression handling
- Semantic round-trip test corpus
- Conflict-safe save behavior

### Parity judgment

This is one of the largest blockers to full Protégé parity.

---

## 4.3 OWL 2 Authoring

**Status: PARTIAL — P0 BLOCKER**

### Implemented authoring operations

The Turtle patch model includes:

- Prefix operations
- Ontology and version IRIs
- Ontology annotations
- Entity creation and deletion
- Labels, comments, and deprecation
- Named and complex subclass axioms
- Equivalent and disjoint class axioms
- Imports
- Domains and ranges
- Object property characteristics
- Property chains
- Class assertions
- Object property assertions
- Data property assertions
- Entity annotations

Manchester parsing supports complex class expressions.

### Missing or insufficiently evidenced operations

The patch model does not demonstrate complete authoring support for:

- Disjoint union axioms
- HasKey axioms
- Datatype definitions
- Full datatype restriction authoring
- Negative object property assertions
- Negative data property assertions
- SameIndividual
- DifferentIndividuals
- Inverse object properties
- Equivalent object properties
- Disjoint object properties
- Equivalent data properties
- Disjoint data properties
- General n-ary axiom editing in all cases
- Axiom annotations as first-class editable annotations
- Anonymous individual workflows
- Complete property assertion annotation workflows
- Full OWL 2 structural specification coverage

### UI gap

The entity inspector and Manchester editor provide meaningful authoring, but the repository does not contain dedicated structured editors for every OWL 2 axiom family.

### Parity judgment

Strixonomy has broad practical OWL authoring, but not complete OWL 2 authoring parity.

---

## 4.4 Workspace and Multi-Ontology Behavior

**Status: PARTIAL — P0 BLOCKER**

### Implemented

- Multi-root workspace indexing
- Incremental indexing
- Active ontology command
- Imports panel
- Import add/remove/reload
- WorkspaceStore in the webview layer
- Shared focus state
- Navigation history
- Selection synchronization
- Query, graph, inspector, reasoning, plugin, and refactor state
- Named perspectives
- Save all command
- Layout reset command

### Important limitation

`extension/src/webviews/layoutPersistence.ts` restores a recovery page that asks the user to reopen a panel. It does not restore the live semantic panel and complete UI state automatically.

The code itself states:

> “The previous session tab ... was recovered. Reopen to reload live ontology context.”

### Remaining gaps

- True Protégé-style workspace restoration
- Persistent live panel state
- Complete perspective serialization
- Dock/layout equivalence
- Clear editable versus read-only ontology model
- Per-ontology dirty-state model verified end-to-end
- Axiom target ontology selection throughout all editors
- Move axioms between ontologies
- Missing import mapping/catalog workflow depth
- Save conflict handling
- Full workspace transaction boundaries

### Parity judgment

The WorkspaceStore is a strong foundation, but workspace parity is not complete.

---

## 4.5 Reasoning

**Status: PARTIAL–STRONG — P0 BLOCKER**

### Implemented

- EL adapter
- RL adapter
- RDFS adapter
- DL adapter
- Automatic profile selection
- Classification
- Named unsatisfiable class reporting
- Inferred hierarchy
- Consistency command
- Explanation panel
- Explanation alternatives
- Reasoner cache
- Start/synchronize/classify/check commands
- Client cancellation and ignoring late results
- Asserted/inferred/combined hierarchy modes

### Critical semantic limitation

The reasoner result source explicitly documents that consistency means no named class is unsatisfiable:

```text
Does not detect all ontology inconsistencies, such as some ABox clashes.
```

This is not complete Protégé consistency parity.

### Explanation limitation

DL unsatisfiability explanations use EL/RL/RDFS traces as fallbacks. Native DL clash proof traces are not available.

### Missing or insufficiently evidenced

- Full ABox consistency
- Complete realization
- General instance checking
- Inferred individual types as a complete workflow
- Inferred object property assertion workflow
- Same-individual inference workflow
- Explanations for general entailments
- Native DL proof explanations
- Server-side CPU cancellation
- Reasoner lifecycle equivalent to multiple Protégé reasoner plugins
- Incremental reasoner semantics beyond reclassification/synchronization workflow

### Parity judgment

Reasoning is one of Strixonomy’s strongest subsystems, but complete Protégé reasoning parity has not been reached.

---

## 4.6 SWRL

**Status: MISSING — P0 if full Protégé parity is required**

### Evidence

Source search found no SWRL authoring UI, parser, serializer, rule browser, or validation subsystem.

The primary SWRL references are roadmap statements and an Ontologos engine enum mapping.

The existing parity document explicitly places:

- SWRL rule viewing as open
- Full SWRL authoring as v0.31+/P2

That conflicts with the newly stated requirement of full Protégé parity before v0.30.

### Required work

- SWRL object model
- Parse and serialize rules
- Rule list/browser
- Text or structured rule editor
- Variable and entity completion
- Built-in validation
- Rule annotations/names where applicable
- Undo/redo
- Rule search and usage
- Reasoner execution policy and unsupported-feature diagnostics
- Round-trip fixtures

### Parity judgment

SWRL is a major newly elevated scope item and requires a dedicated milestone.

---

## 4.7 Querying

**Status: PARTIAL–STRONG**

### Implemented

- SPARQL query execution
- SQL-like virtual catalog queries
- Query workbench
- Query history in WorkspaceStore
- Schema browser
- Query tests
- Read-only SPARQL safeguards

### Missing or unclear

- Dedicated DL Query syntax/workflow equivalent to Protégé’s DL Query tab
- Full result navigation and explanation integration
- Saved named query management across workspaces
- Full SQL is explicitly not supported, though this is not a Protégé parity requirement

### Parity judgment

SPARQL is strong. Dedicated DL Query parity remains unclear or missing.

---

## 4.8 Refactoring

**Status: PARTIAL–STRONG**

### Implemented semantic request types

`strixonomy-refactor` includes:

- Rename IRI
- Merge entities
- Replace entity
- Namespace migration
- Move entity
- Extract module
- Preview plans
- Atomic file replacement infrastructure
- Usage analysis
- Integration tests

### Limitations

- Refactoring is documented as Turtle-only.
- Module extraction uses a direct-reference closure rather than complete locality module extraction.
- No explicit ontology merge request type was found.
- No import flatten operation was found.
- No general “move selected axioms” operation was found.
- Multi-format semantic refactoring is absent.
- Full transaction integration across several files and ontology formats requires verification.

### Parity judgment

Strixonomy is already competitive in several refactoring workflows, but full Protégé parity is not complete.

---

## 4.9 Visualization

**Status: PARTIAL**

### Implemented

- Class graph
- Property graph
- Import graph
- Neighborhood graph
- Asserted/inferred/combined modes
- Grid, circle, and stack layouts
- Search
- Truncation handling
- Graph panel tests

### Remaining gaps

- Full OntoGraf-like filtering and interaction depth
- OWLViz-equivalent inferred hierarchy visualization fidelity
- Editable graph workflows
- Rich layout configuration
- Persistent graph layouts
- Complete large ontology virtualization

### Parity judgment

Useful visualization exists, but it does not yet cover the full Protégé visualization experience.

---

## 4.10 Plugin Platform

**Status: PARTIAL — P1/P0 depending on parity definition**

### Implemented

- Manifest discovery
- In-process and subprocess hosts
- Permissions
- API version field
- Plugin views
- Plugin commands
- Preference pages
- Context actions
- LSP plugin execution
- Reference naming validator
- Markdown exporter
- SHACL scaffold
- Built-in plugin crate
- Path-jail hardening

### Remaining gaps

- Stable semver-guaranteed 1.0 SDK
- Complete extension point catalog
- Formal compatibility test suite
- Production plugin packaging/install workflow
- Plugin marketplace/distribution
- Stable migration policy
- Strong runtime isolation model
- Rich reasoner/provider integration verification

### Parity judgment

The plugin architecture is substantial, but the project’s own documentation correctly describes it as pre-stable.

---

## 4.11 UI and Accessibility

**Status: PARTIAL–STRONG**

### Implemented

- React panels for inspector, graph, imports, reasoner, explanations, query, semantic diff, and dialogs
- Shared design primitives
- Dialog shell
- Preview/apply workflow
- Keyboard-accessible command palette integration
- Many ARIA labels and component tests
- Focus synchronization
- VS Code end-to-end test source

### Remaining gaps

- Complete keyboard-only workflow audit
- Screen-reader audit
- Full panel focus restoration
- Persistent workspace layout
- Structured editors for all axiom types
- True Protégé-equivalent preferences surface
- Complete status/progress behavior for long-running semantic operations
- Cross-platform accessibility verification

### Parity judgment

The modern interface is a major strength, but accessibility and workflow completeness still require formal release-gate verification.

---

## 5. Documentation Claim Audit

## Accurate or well-qualified claims

The following documentation is appropriately candid:

- `docs/known-limitations.md`
- `docs/supported-formats.md`
- The current v0.18 assessment
- Reasoner documentation describing class-level consistency
- Plugin documentation identifying v0.29–v0.30 API stability

## Claims that require correction under the new scope

### “Protégé Desktop parity gate shipped”

The roadmap describes v0.18 as a shipped Protégé parity gate. This is defensible only for the previously narrowed gate, not full Protégé Desktop parity.

Recommended correction:

> v0.18 completed the first Protégé-parity assessment and the core Turtle/OBO desktop workflow gate; full parity remains the v0.30 target.

### “Authoring parity”

Earlier shipped-release headings use “authoring parity,” but authoring remains format-limited and lacks complete OWL 2 axiom coverage.

Recommended correction:

> broad Turtle/OBO authoring coverage

### “HermiT parity”

Reasoning documentation occasionally uses HermiT parity language, while the implementation itself acknowledges class-level consistency and incomplete ABox clash detection.

Recommended correction:

> broad OWL 2 DL classification support, with documented ABox and explanation limitations

### SWRL status conflict

Current project plans place SWRL authoring after v0.30, while the new full-parity requirement makes it a v0.29–v0.30 blocker.

This must be resolved in all roadmaps and parity matrices.

---

## 6. Corrected Parity Scorecard

| Area | Score | Status |
|---|---:|---|
| Ontology lifecycle | 80% | Partial–strong |
| Turtle authoring | 95% | Strong |
| OBO authoring | 80% | Strong for common workflows |
| OWL/XML/RDF/XML authoring | 20% | Browse-only |
| Complete OWL 2 axiom authoring | 65% | Partial |
| Imports | 75% | Partial–strong |
| Multi-ontology workspace | 65% | Partial |
| Navigation/search | 90% | Strong |
| Reasoning classification | 85% | Strong |
| ABox reasoning/realization | 35% | Major gap |
| Explanations | 60% | Partial |
| Query/SPARQL | 85% | Strong |
| DL Query workflow | 35% | Unclear/limited |
| SWRL | 5% | Missing |
| Refactoring | 75% | Strong but Turtle-only |
| Visualization | 65% | Partial |
| Plugin platform | 60% | Functional but unstable |
| UI workflow shell | 80% | Strong |
| Layout/perspectives | 55% | Partial |
| Accessibility evidence | 60% | Partial |
| Parity test corpus | 55% | Good foundation, incomplete |
| **Overall full Protégé parity** | **65–72%** | **Not release-ready** |

Scores are engineering estimates, not automatically computed metrics. The planned machine-readable parity manifest should replace manual scoring.

---

## 7. Critical Path to v0.30

## Phase A — Make parity executable

1. Replace the current high-level parity matrix with atomic requirements.
2. Add machine-readable parity data.
3. Link every complete item to source and tests.
4. Mark format-limited capabilities explicitly.
5. Add CI validation for parity evidence.
6. Freeze the exact Protégé baseline.

## Phase B — Canonical semantic editing

1. Introduce a format-independent ontology change model.
2. Add semantic transactions and inverse operations.
3. Route Turtle and OBO authoring through it.
4. Implement OWL/XML write-back.
5. Implement RDF/XML write-back.
6. Add semantic round-trip comparison.

## Phase C — Complete OWL 2 authoring

1. Inventory every OWL 2 structural axiom.
2. Fill missing model operations.
3. Add UI create/edit/delete workflows.
4. Add axiom annotation editing.
5. Add datatype and key workflows.
6. Add negative and same/different individual assertions.
7. Add full conformance fixtures.

## Phase D — Finish workspace parity

1. Formalize the loaded ontology set.
2. Add per-ontology dirty state.
3. Add editable/read-only ontology status.
4. Add target ontology selection to all editors.
5. Add move-axiom workflows.
6. Restore live panels and semantic state.
7. Add robust save conflict and recovery handling.

## Phase E — Complete reasoning parity

1. Implement full consistency semantics.
2. Add realization and instance checking.
3. Add inferred individual assertions.
4. Add native DL explanation traces.
5. Improve cancellation at engine/server level.
6. Add cross-reasoner conformance tests.

## Phase F — Implement SWRL

1. Rule model and parser.
2. Rule browser/editor.
3. Validation and completion.
4. Serialization.
5. Undo/redo.
6. Reasoner integration.
7. Test corpus.

## Phase G — Close advanced workflow gaps

1. Locality-based module extraction.
2. Ontology merge.
3. Flatten imports.
4. Move selected axioms.
5. Dedicated DL Query.
6. Visualization polish.
7. Stable Plugin SDK.
8. Accessibility audit.

## Phase H — Release validation

1. Run all Rust and extension tests.
2. Run full Protégé round-trip corpus.
3. Complete cross-platform testing.
4. Run performance and memory benchmarks.
5. Validate migration with real Protégé users.
6. Freeze APIs.
7. Clear all P0 defects.

---

## 8. Recommended Repository Changes

Create a dedicated directory such as:

```text
docs/protege-parity/
```

Place the parity planning documents there and add:

```text
README.md
PROTEGE_PARITY_SCOPE.md
PROTEGE_CAPABILITY_INVENTORY.md
PROTEGE_PARITY_MATRIX.md
PROTEGE_GAP_ANALYSIS.md
PARITY_ACCEPTANCE_CRITERIA.md
PARITY_TEST_PLAN.md
OWL2_AUTHORING_GAPS.md
FORMAT_PARITY.md
WORKSPACE_PARITY.md
REASONING_PARITY.md
SWRL_PARITY.md
REFACTORING_PARITY.md
UI_WORKFLOW_PARITY.md
PLUGIN_PARITY.md
PROTEGE_PARITY_ROADMAP.md
V1_PARITY_BACKLOG.md
PARITY_RELEASE_GATE.md
CURRENT_REPOSITORY_AUDIT.md
```

Also add:

```text
parity/protege-desktop-parity.yaml
scripts/validate-parity-manifest.*
tests/parity/
tests/fixtures/protege/
```

---

## 9. First Cursor Implementation Sequence

The first Cursor sessions should not begin with random UI additions.

### Session 1 — Atomic parity manifest

- Convert capability inventory into atomic requirements.
- Record current status conservatively.
- Add path-based evidence fields.
- Add schema validation.
- Add CI validation.

### Session 2 — OWL 2 structural coverage inventory

- Generate a complete axiom/expression checklist.
- Map each construct to model, parser, serializer, UI, and tests.
- Do not implement features yet.

### Session 3 — Canonical change model ADR and skeleton

- Define semantic change types.
- Define transaction and inverse-change APIs.
- Add tests for change application.
- Avoid serializer-specific assumptions.

### Session 4 — RDF/XML and OWL/XML write-back architecture

- Evaluate Horned-OWL serialization capabilities.
- Define deterministic output policy.
- Build semantic comparison tests.
- Implement the smallest end-to-end edit.

### Session 5 — Workspace ontology registry

- Separate loaded ontology state from panel UI state.
- Add active ontology and editability.
- Add per-ontology dirty state.
- Add persistence tests.

This sequence creates the foundation required for the largest parity gaps.

---

## 10. Final Assessment

Strixonomy is much closer to Protégé replacement status than most new ontology tools. It already has a credible native engine, modern IDE shell, reasoning adapters, semantic refactoring, plugins, graphs, queries, and a large documentation surface.

The project should not restart or perform a broad rewrite.

The correct strategy is to preserve the existing architecture and close a finite set of deep gaps:

1. Format-independent write-back
2. Complete OWL 2 authoring
3. Full workspace semantics
4. ABox reasoning and native explanations
5. SWRL
6. Advanced ontology operations
7. Executable parity verification

Once these are complete and demonstrated through a real Protégé-generated conformance corpus, Strixonomy can credibly ship v0.30 as a full Protégé Desktop replacement.
