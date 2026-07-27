# DL_QUERY.md

# Protégé DL Query Reverse Engineering Specification

## Purpose

The DL Query view allows ontology engineers to execute Description Logic (DL) expressions against the active ontology and inspect the inferred results produced by the currently selected reasoner. It provides an interactive way to explore an ontology without writing SPARQL or modifying ontology axioms.

For Strixonomy, the DL Query editor should preserve Protégé's capabilities while adding modern editing, visualization, AI assistance, and richer result exploration.

---

# Responsibilities

The DL Query editor supports:

- Writing Description Logic expressions
- Executing queries against the active reasoner
- Displaying inferred classes and individuals
- Validating query syntax
- Autocompleting ontology entities
- Navigating from results back to editors

---

# Workspace Layout

```text
+----------------------------------------------------------------------------+
| Toolbar                                                                    |
+----------------------------------------------------------------------------+
| Query Editor                                                               |
|----------------------------------------------------------------------------|
| Results Tabs                                                               |
|  • Subclasses                                                              |
|  • Superclasses                                                            |
|  • Equivalent Classes                                                      |
|  • Instances                                                               |
|  • Unsatisfiable Classes                                                   |
+----------------------------------------------------------------------------+
| Status / Diagnostics                                                       |
+----------------------------------------------------------------------------+
```

---

# Query Language

Protégé primarily supports **Manchester OWL Syntax** for DL queries.

Examples:

```text
Person
```

```text
Person and (hasPet some Dog)
```

```text
Animal and not Mammal
```

Supported constructs include:

- Class names
- `and`
- `or`
- `not`
- `some`
- `only`
- `value`
- `min`
- `max`
- `exactly`
- Parenthesized expressions

---

# Query Execution

Typical workflow:

1. Select an active reasoner.
2. Enter a DL expression.
3. Validate syntax.
4. Execute query.
5. Display inferred results.
6. Navigate from results into ontology editors.

Execution should always use the current synchronized ontology state.

---

# Result Types

The interface should allow viewing:

- Inferred subclasses
- Inferred superclasses
- Equivalent classes
- Matching individuals
- Unsatisfiable classes (where applicable)

Results should clearly distinguish asserted information from inferred information.

---

# Editor Features

Recommended capabilities:

- Syntax highlighting
- Auto-complete
- Entity documentation on hover
- Parenthesis matching
- Inline validation
- Error squiggles
- Query history

---

# Search & Completion

Auto-complete should suggest:

- Classes
- Object properties
- Data properties
- Individuals
- Annotation properties (where relevant)

Suggestions should support:

- Prefix matching
- Fuzzy matching
- Namespace-aware completion

---

# Validation

Detect:

- Invalid Manchester syntax
- Unknown entities
- Unsupported constructs
- Missing parentheses
- Reasoner unavailable
- Unsynchronized ontology

Validation should occur while typing.

---

# Navigation

Every result should support:

- Open in Classes editor
- Open in Properties editor
- Open in Individuals editor
- Show usages
- Highlight in graph view

---

# Events

Typical events:

- QueryEdited
- QueryValidated
- QueryExecuted
- ResultsUpdated
- ReasonerChanged
- OntologySynchronized

---

# Accessibility

Requirements:

- Keyboard-first editor
- Screen-reader accessible results
- Searchable result lists
- High contrast support
- Scalable fonts

---

# Plugin Extension Points

Plugins should be able to contribute:

- Additional query languages
- Result renderers
- Visualization panels
- Export formats
- Query history providers

---

# Strixonomy Modernization

Recommended enhancements:

- Monaco-based editor
- AI-assisted query generation
- Natural-language to DL translation
- Saved queries
- Query sharing
- Graph visualization of results
- Explain-result integration
- Performance metrics
- Side-by-side query comparison

---

# Command Architecture

Suggested commands:

- dlQuery.execute
- dlQuery.clear
- dlQuery.save
- dlQuery.openHistory
- dlQuery.exportResults
- dlQuery.explainSelection

---

# Feature Parity Checklist

Editor

- [ ] Manchester syntax editor
- [ ] Syntax highlighting
- [ ] Auto-complete
- [ ] Validation

Execution

- [ ] Execute query
- [ ] Cancel query
- [ ] Reasoner integration

Results

- [ ] Subclasses
- [ ] Superclasses
- [ ] Equivalent classes
- [ ] Instances
- [ ] Navigation

Platform

- [ ] History
- [ ] Export
- [ ] Accessibility
- [ ] Plugin support

---

# Beyond Protégé

Strixonomy should evolve the DL Query experience into a semantic query workbench supporting reusable queries, notebooks, natural-language prompting, visualization, AI-assisted query authoring, and integration with SPARQL and graph exploration.

---

# Summary

The DL Query view is an essential exploratory tool in Protégé, allowing ontology engineers to interrogate inferred knowledge using Description Logic expressions. Strixonomy should retain compatibility with Manchester Syntax while providing a modern editor, richer diagnostics, AI assistance, reusable query workflows, and deep integration with visualization and reasoning services.
