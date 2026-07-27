# CREATE_ONTOLOGY.md

# Create Ontology Workflow
## Reverse Engineering Specification for Protégé and Design Blueprint for Strixonomy

## Purpose

Creating a new ontology is the foundational workflow in an ontology engineering environment. This document reverse-engineers the workflow used in Protégé and defines an improved workflow for Strixonomy.

The workflow covers everything from project creation through the first successful reasoning cycle.

---

# Goals

The workflow should:

- Minimize setup friction
- Produce a standards-compliant OWL ontology
- Guide new users while remaining efficient for experts
- Support local and collaborative workspaces
- Create a reproducible project structure

---

# User Journey

```text
Launch Application
        │
        ▼
Create New Ontology
        │
        ▼
Choose Workspace
        │
        ▼
Configure Ontology Metadata
        │
        ▼
Create Initial Ontology File
        │
        ▼
Initialize Workspace
        │
        ▼
Begin Modeling
```

---

# Step 1 — Create Ontology

The user selects:

- File → New Ontology
- Toolbar action
- Command Palette
- Welcome screen

All entry points should invoke the same command.

---

# Step 2 — Ontology Configuration

Required fields:

- Ontology IRI
- Version IRI (optional but recommended)
- Display name
- Default namespace
- Serialization format

Optional metadata:

- Description
- Author
- Organization
- License
- Version
- Tags

Validation should occur as the user types.

---

# Step 3 — Workspace Selection

Users should be able to create the ontology in:

- Local workspace
- Existing workspace
- Git repository
- Shared collaborative workspace

Strixonomy should remember recent destinations.

---

# Step 4 — File Generation

Generate initial ontology with:

- Prefix declarations
- Base ontology declaration
- Metadata annotations
- Default serialization

Supported formats:

- RDF/XML
- Turtle
- OWL/XML
- Functional Syntax
- Manchester Syntax

---

# Step 5 — Initialize Workspace

Create:

- Ontology tree
- Class hierarchy
- Default views
- Undo history
- Event subscriptions
- Reasoning state

Workspace status:

- Clean
- No inferred knowledge yet
- Ready for editing

---

# Step 6 — Optional Wizard

A guided wizard may offer to:

- Create root classes
- Import common vocabularies
- Configure a default reasoner
- Create namespace prefixes
- Enable Git
- Enable AI assistance

Expert users should be able to skip the wizard.

---

# Initial Modeling Workflow

Recommended sequence:

1. Create top-level classes
2. Create object properties
3. Create data properties
4. Create annotation properties
5. Create individuals
6. Run first classification

---

# Validation

Before saving:

- Valid ontology IRI
- No duplicate prefixes
- Writable destination
- Valid serialization
- Metadata completeness (optional)

---

# Events

Typical events:

- WorkspaceCreated
- OntologyCreated
- MetadataSaved
- PrefixesInitialized
- WorkspaceReady

---

# Accessibility

Requirements:

- Full keyboard workflow
- Screen-reader support
- Accessible validation messages
- High-contrast compatibility

---

# Strixonomy Modernization

Recommended improvements:

- Project templates
- Domain-specific ontology starters
- AI-generated starter ontology
- Git initialization
- Dependency lockfile
- Live preview of generated ontology
- Team workspace creation

---

# Feature Parity Checklist

Creation

- [ ] New ontology dialog
- [ ] Ontology IRI
- [ ] Version IRI
- [ ] Prefix configuration

Workspace

- [ ] Workspace initialization
- [ ] Default layout
- [ ] Event registration

Validation

- [ ] IRI validation
- [ ] Prefix validation
- [ ] File validation

Modern Features

- [ ] Templates
- [ ] Git integration
- [ ] AI assistant
- [ ] Collaboration support

---

# Summary

Protégé provides a straightforward ontology creation workflow centered on creating an OWL document and beginning manual modeling. Strixonomy should extend this into a modern project creation experience with templates, Git integration, AI-assisted setup, collaborative workspaces, reproducible project configuration, and immediate readiness for ontology engineering.
