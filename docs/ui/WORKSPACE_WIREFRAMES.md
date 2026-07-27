# Workspace Wireframes

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


## 1. Application Shell

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ Strixonomy  Search anything…                            AI  Git  Account      │
├───────────────┬──────────────────────────────────────────────┬───────────────┤
│ Explorer      │ Workspace Tabs                               │ Inspector     │
│               ├──────────────────────────────────────────────┤               │
│ Classes       │ Active Workspace                             │ Entity Card   │
│ Properties    │                                              │ Relationships │
│ Individuals   │                                              │ Diagnostics   │
│ Imports       │                                              │ AI Suggestions│
│ Queries       │                                              │ Metadata      │
├───────────────┴──────────────────────────────────────────────┴───────────────┤
│ Problems | Query | Graph | AI | Git | Output | Terminal                     │
└──────────────────────────────────────────────────────────────────────────────┘
```

## 2. Entity Workspace

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ Healthcare › Person › Patient                  Rename Graph References AI    │
├──────────────────────────────────────────────────────────────────────────────┤
│ Patient                                                OWL Class  Healthy    │
│ A human receiving medical care.                                             │
├──────────────────────────────────────────────────────────────────────────────┤
│ Overview | Hierarchy | Relationships | Constraints | Docs | History | AI     │
├──────────────────────────────────────────────────────────────────────────────┤
│ Overview                                                                     │
│ ┌ Parents ─────────────┐ ┌ Children ────────────┐ ┌ Diagnostics ──────────┐ │
│ │ Person               │ │ AdultPatient          │ │ No active errors      │ │
│ │ Agent                │ │ PediatricPatient      │ │ 2 suggestions         │ │
│ └──────────────────────┘ └──────────────────────┘ └───────────────────────┘ │
└──────────────────────────────────────────────────────────────────────────────┘
```

## 3. Graph Workspace

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ Layout  Filters  Search graph…  Reasoning Overlay  Save View  AI Explain    │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│                         [Person]                                             │
│                            │                                                 │
│                         [Patient] ─── hasDiagnosis ─── [Disease]             │
│                            │                                                 │
│                       [AdultPatient]                                         │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│ Details | Minimap | Selection | Problems | History                           │
└──────────────────────────────────────────────────────────────────────────────┘
```

## 4. Query Workbench

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ SQL  Run  Explain  Save  AI Generate                         Schema Browser │
├─────────────────────────────────────────────────────────────┬────────────────┤
│ SELECT * FROM classes                                       │ Classes        │
│ WHERE deprecated = false;                                   │ Properties     │
│                                                             │ Individuals    │
├─────────────────────────────────────────────────────────────┴────────────────┤
│ Results | Graph | JSON | Explain | History | Diagnostics                    │
└──────────────────────────────────────────────────────────────────────────────┘
```

## 5. Review Workspace

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ Semantic Pull Request #42: Normalize Patient Hierarchy                       │
├──────────────────────────────────────┬───────────────────────────────────────┤
│ Semantic Diff                        │ Review Thread                         │
│ + 3 Classes                          │ Comment on Patient                    │
│ ~ 12 Relationships                   │ AI summary available                  │
│ ! 1 Reasoning Regression             │ Approve / Request Changes             │
├──────────────────────────────────────┴───────────────────────────────────────┤
│ Checks | Reasoning | Graph | AI Review | Approvals                          │
└──────────────────────────────────────────────────────────────────────────────┘
```
