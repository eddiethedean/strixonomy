# Modernizing the Ontology Ecosystem

> **Long-term vision.** For what ships in **v0.27**, see [What ships today](https://strixonomy-vs.readthedocs.io/en/latest/SHIPPED/). **Protégé replacement is the 1.0 goal — not supported for org-wide retirement today.** Plugin **SDK 1.0** freezes the TOML + subprocess JSON wire (`api_version = "1"`) today; a curated marketplace and production owlmake integration remain **product 1.0** goals.

## Mission

Build the modern open-source platform for ontology engineering.

The current ontology ecosystem is powerful but fragmented, heavily JVM-centric, and centered around desktop-era workflows. Our goal is not to replace W3C standards—it is to modernize how developers, researchers, and organizations build, validate, query, reason over, and maintain ontologies.

## Long-Term Vision

Four projects work together:

- **Ontologos** — Rust-native reasoning engine.
- **Strixonomy** — Semantic workspace engine and reusable platform.
- **Strixonomy** — Flagship VS Code IDE powered by Strixonomy.
- **OntoUI** — Shared React UI platform ([platform/ONTOUI.md](docs/platform/ONTOUI.md)) — **v0.13 foundation shipped** (WorkspaceStore, focus relay, design tokens); **OntoStudio** reuses it (planned post-1.0).

Together they enable modern workflows including CI/CD, team collaboration on version-controlled ontology files, and high-performance local tooling. **Post-1.0 / planned (not shipped today):** AI-assisted development surfaces, Python and TypeScript SDKs, and OntoStudio desktop — see [What ships today](https://strixonomy-vs.readthedocs.io/en/latest/SHIPPED/) and [Protégé decision](https://strixonomy-vs.readthedocs.io/en/latest/guides/protege-decision/) for current non-goals.

## Ecosystem Collaboration

Strixonomy is the **platform** — workspace indexing, query, diagnostics, refactoring, and **plugin hosting**. It does not absorb every tool in the ontology stack.

**External workflow tools** integrate through Strixonomy's plugin APIs. [owlmake](https://github.com/INCATools/owlmake) is the first reference workflow plugin: it demonstrates ROBOT/ODK-style build, validation, and release automation **outside** Strixonomy, while Strixonomy surfaces those workflows in the IDE. ROBOT and ODK remain the semantic standards for many operations; Strixonomy integrates with them rather than rewriting them.

- **Ontologos** — reasoning (classification, consistency, explanations).
- **Strixonomy** — semantic workspace engine and plugin platform.
- **owlmake** (and future plugins) — workflow, build, and release automation.
- **Strixonomy** — presents workspace editing, reasoning, and toolchain workflows in one modern IDE.

## Guiding Principles

- Standards-first (OWL, RDF, SHACL, SPARQL, OBO)
- Developer-first APIs
- Local-first architecture
- AI-native tooling
- Cross-language support
- Extensible plugin ecosystem — external tools integrate; Strixonomy does not monolith
- Open-source and community driven

## Success

The ecosystem succeeds when developers build new tools on Strixonomy, Ontologos becomes a trusted reasoning engine, Strixonomy is a production-ready alternative to Protégé, and workflow tools like owlmake integrate as first-class citizens without becoming core dependencies.

See also [ARCHITECTURE.md](ARCHITECTURE.md) and [ROADMAP.md](ROADMAP.md).
