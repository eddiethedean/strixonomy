# UI_WIREFRAMES.md

> **Target UI stack:** Panels below ship as **React webviews** from v0.7a onward ([ADR-0017](adr/0017-react-webview-ui.md), [OntoCode_React_UI_Integration_Plan.md](OntoCode_React_UI_Integration_Plan.md)). Tree views remain native VS Code `TreeDataProvider` components in the extension host.

## 1. VS Code Activity Bar

```text
[ Explorer ]
[ Search   ]
[ Source   ]
[ Run      ]
[ Strixonomy ]  <-- new icon
```

## 2. Ontology Explorer Sidebar

```text
ONTOCODE

Workspace: clinical-ontology

▾ Ontologies
  ▸ patient.ttl
  ▸ billing.owl
  ▸ common.ttl

▾ Classes
  ▾ Patient
    ▸ Inpatient
    ▸ Outpatient
  ▾ Encounter
  ▾ Medication

▾ Object Properties
  ▸ hasDiagnosis
  ▸ hasProvider
  ▸ memberOf

▾ Data Properties
  ▸ dateOfBirth
  ▸ encounterDate

▾ Annotation Properties
  ▸ rdfs:label
  ▸ skos:definition

▾ Individuals
  ▸ ExamplePatient001

▾ Diagnostics
  ⚠ 4 missing labels
  ✖ 1 broken import
```

## 3. Entity Inspector

```text
CLASS: Patient

IRI:
http://example.org/ontology/Patient

Labels:
- Patient@en

Comments:
- A human receiving care.

Parents:
- Person

Children:
- Inpatient
- Outpatient

Axioms:
- Patient SubClassOf Person
- Patient SubClassOf hasRecord some MedicalRecord

Annotations:
- skos:definition
- dc:created
- owl:deprecated false

Actions:
[Add Label] [Add Axiom] [Find Usages] [Rename IRI] [Open Graph]
```

## 4. Query Workbench

```text
QUERY WORKBENCH

Mode: [SQL v] [SPARQL]

SELECT iri, label
FROM classes
WHERE deprecated = true;

[Run] [Save Query] [Export CSV]

Results:
| iri | label |
|-----|-------|
| ... | ...   |
```

## 5. Graph View

```text
GRAPH: Patient Neighborhood

          Person
            |
         Patient
        /       \
 Inpatient     Outpatient

Side Panel:
- selected node
- relationship filters
- depth slider
- show inferred edges
```

## 6. Semantic Diff Panel

```text
SEMANTIC DIFF: main..feature

Summary:
- 12 classes added
- 2 classes removed
- 4 domains changed
- 1 broken import introduced

Breaking Changes:
✖ Removed class: LegacyPatient
✖ Changed range: hasProvider

Entity Changes:
+ Added class: TelehealthEncounter
~ Changed label: Provider
- Removed property: oldIdentifier

[Export Markdown] [Copy PR Summary]
```

## 7. Reasoner Results Panel

```text
REASONER: el (OWL EL)  [Switch to DL — requires OntoLogos 1.0]

Status: Completed

Unsatisfiable Classes:
✖ InvalidEncounter

Inferred Changes:
+ TelehealthEncounter SubClassOf Encounter
+ PediatricPatient SubClassOf Patient

[Show Inferred Hierarchy] [Explain Selected]

--- Explanation panel (P0 v0.30) ---

EXPLANATION: InvalidEncounter

Unsatisfiable class: ex:InvalidEncounter

Justification:
1. ex:InvalidEncounter EquivalentTo ex:Foo and ex:Bar
2. ex:Foo SubClassOf ex:Baz
3. ex:Bar SubClassOf owl:Nothing
4. ex:Baz SubClassOf ex:Bar

[Jump to axiom] [Copy] [Re-run DL reasoner]
```

## 8. Manchester Axiom Editor

```text
MANCHESTER AXIOM EDITOR — Patient

Axiom type: [SubClassOf v]

Parent class:
  [ Person                    ] (picker)

Expression (Manchester):
  hasRecord some MedicalRecord

Expression tree:
  └ SubClassOf
      ├ Person
      └ ObjectSomeValuesFrom
          ├ hasRecord
          └ MedicalRecord

Turtle preview:
  ex:Patient a owl:Class ;
      rdfs:subClassOf [
          a owl:Restriction ;
          owl:onProperty ex:hasRecord ;
          owl:someValuesFrom ex:MedicalRecord
      ] .

[Validate] [Apply] [Cancel]
```

## 9. Quick Axiom Forms

```text
ADD AXIOM — Patient

Type: [SubClassOf v]

Parent: [ Person          ] [Browse...]

--- or ---

Type: [Object property domain v]

Property: [ hasRecord     ]
Domain:   [ Patient       ]

Characteristics (object property):
[ ] Functional  [ ] Inverse functional
[ ] Transitive  [ ] Symmetric
[ ] Asymmetric  [ ] Reflexive  [ ] Irreflexive

[Preview Turtle] [Add] [Cancel]
```
