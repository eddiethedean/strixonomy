use crate::OntologyCatalog;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use strixonomy_core::{
    document_for_entity, read_to_string_capped, Entity, EntityKind, PropertyCharacteristics,
    AXIOM_KIND_CLASS_ASSERTION, AXIOM_KIND_DATATYPE_DEFINITION, AXIOM_KIND_DATA_PROPERTY_ASSERTION,
    AXIOM_KIND_DIFFERENT_INDIVIDUALS, AXIOM_KIND_DISJOINT_CLASS,
    AXIOM_KIND_DISJOINT_DATA_PROPERTIES, AXIOM_KIND_DISJOINT_OBJECT_PROPERTIES,
    AXIOM_KIND_DISJOINT_UNION, AXIOM_KIND_DOMAIN, AXIOM_KIND_EQUIVALENT_CLASS,
    AXIOM_KIND_EQUIVALENT_DATA_PROPERTIES, AXIOM_KIND_EQUIVALENT_OBJECT_PROPERTIES,
    AXIOM_KIND_HAS_KEY, AXIOM_KIND_INVERSE_OBJECT_PROPERTIES,
    AXIOM_KIND_NEGATIVE_DATA_PROPERTY_ASSERTION, AXIOM_KIND_NEGATIVE_OBJECT_PROPERTY_ASSERTION,
    AXIOM_KIND_OBJECT_PROPERTY_ASSERTION, AXIOM_KIND_PROPERTY_CHAIN, AXIOM_KIND_RANGE,
    AXIOM_KIND_SAME_INDIVIDUAL, AXIOM_KIND_SUB_CLASS_OF, AXIOM_KIND_SUB_DATA_PROPERTY_OF,
    AXIOM_KIND_SUB_OBJECT_PROPERTY_OF, MAX_FILE_BYTES,
};
use strixonomy_diagnostics::{entity_needles, find_in_source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubclassEdge {
    pub child: String,
    pub parent: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClassHierarchy {
    pub edges: Vec<SubclassEdge>,
    pub parents: BTreeMap<String, Vec<String>>,
    pub children: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceHint {
    pub path: PathBuf,
    pub line: u64,
    pub column: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct EntityAxiomSummary {
    pub kind: String,
    pub display: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manchester: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_iri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_iri: Option<String>,
    /// Property / relation IRI for assertion-like axioms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<String>,
    /// Ordered member property IRIs for `property_chain` / `has_key` / `disjoint_union`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<String>,
    /// Annotations attached to this axiom (not entity annotation assertions).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<EntityAnnotationSummary>,
    pub editable: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct EntityAnnotationSummary {
    pub predicate: String,
    pub value: String,
    pub editable: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct EntityDetail {
    pub entity: Entity,
    pub parents: Vec<String>,
    pub children: Vec<String>,
    pub axioms: Vec<EntityAxiomSummary>,
    pub annotations: Vec<EntityAnnotationSummary>,
    #[serde(skip_serializing_if = "PropertyCharacteristics::is_empty")]
    pub characteristics: PropertyCharacteristics,
    pub source: Option<SourceHint>,
    pub editable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_path: Option<String>,
}

impl OntologyCatalog {
    pub fn find_entity(&self, iri: &str) -> Option<&Entity> {
        self.data().entities.iter().find(|e| e.iri == iri)
    }

    pub fn entity_document(&self, iri: &str) -> Option<&strixonomy_core::OntologyDocument> {
        if let Some(&doc_idx) = self.entity_to_document.get(iri) {
            return self.data().documents.get(doc_idx);
        }

        let entity = self.find_entity(iri)?;
        document_for_entity(&self.data().documents, entity)
    }

    pub fn class_hierarchy(&self) -> ClassHierarchy {
        let mut edges = Vec::new();
        let mut parents: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut children: BTreeMap<String, Vec<String>> = BTreeMap::new();

        let class_iris: BTreeSet<&str> = self
            .data()
            .entities
            .iter()
            .filter(|e| e.kind == EntityKind::Class)
            .map(|e| e.iri.as_str())
            .collect();

        for axiom in &self.data().axioms {
            if axiom.axiom_kind != AXIOM_KIND_SUB_CLASS_OF {
                continue;
            }
            // Keep edges when the child is a known class, even if the parent is external
            // (common for OBO is_a and imports).
            if !class_iris.contains(axiom.subject.as_str()) {
                continue;
            }
            let edge = SubclassEdge { child: axiom.subject.clone(), parent: axiom.object.clone() };
            edges.push(edge.clone());
            parents.entry(edge.child.clone()).or_default().push(edge.parent.clone());
            children.entry(edge.parent.clone()).or_default().push(edge.child.clone());
        }

        for list in parents.values_mut().chain(children.values_mut()) {
            list.sort();
            list.dedup();
        }

        ClassHierarchy { edges, parents, children }
    }

    pub fn entity_detail(&self, iri: &str) -> Option<EntityDetail> {
        let hierarchy = self.class_hierarchy();
        self.entity_detail_with_hierarchy(iri, &hierarchy)
    }

    pub fn entity_detail_with_hierarchy(
        &self,
        iri: &str,
        hierarchy: &ClassHierarchy,
    ) -> Option<EntityDetail> {
        let entity = self.find_entity(iri)?.clone();

        let parents = hierarchy.parents.get(iri).cloned().unwrap_or_default();
        let children = hierarchy.children.get(iri).cloned().unwrap_or_default();

        let source = self.find_source_location(iri);
        let doc = self.entity_document(iri);
        let editable = doc.is_some_and(|d| {
            matches!(
                d.format,
                strixonomy_core::OntologyFormat::Turtle
                    | strixonomy_core::OntologyFormat::Obo
                    | strixonomy_core::OntologyFormat::Owl
                    | strixonomy_core::OntologyFormat::RdfXml
                    | strixonomy_core::OntologyFormat::OwlXml
            ) && d.parse_status == strixonomy_core::ParseStatus::Ok
        });
        let document_path = doc.map(|d| d.path.display().to_string());

        let axioms: Vec<EntityAxiomSummary> = self
            .data()
            .axioms
            .iter()
            .filter(|a| a.subject == iri)
            .map(|a| axiom_summary(a, editable))
            .collect();

        const PROMOTED: &[&str] = &[
            "http://www.w3.org/2000/01/rdf-schema#label",
            "http://www.w3.org/2000/01/rdf-schema#comment",
            "http://www.w3.org/2002/07/owl#deprecated",
        ];
        let annotations: Vec<EntityAnnotationSummary> = self
            .data()
            .annotations
            .iter()
            .filter(|a| a.subject == iri && !PROMOTED.contains(&a.predicate.as_str()))
            .map(|a| EntityAnnotationSummary {
                predicate: a.predicate.clone(),
                value: a.object.clone(),
                editable,
            })
            .collect();

        Some(EntityDetail {
            entity: entity.clone(),
            parents,
            children,
            axioms,
            annotations,
            characteristics: entity.characteristics.clone(),
            source,
            editable,
            document_path,
        })
    }

    pub fn find_source_location(&self, iri: &str) -> Option<SourceHint> {
        let entity = self.find_entity(iri)?;
        let doc = self.entity_document(iri)?;

        if let Some(loc) = entity.source_location.line {
            return Some(SourceHint {
                path: doc.path.clone(),
                line: loc,
                column: entity.source_location.column.unwrap_or(0),
            });
        }

        scan_file_for_iri(&doc.path, iri, &entity.short_name, &doc.namespaces)
    }

    pub fn entities_in_document(&self, doc_path: &std::path::Path) -> Vec<&Entity> {
        let doc_path = doc_path.canonicalize().unwrap_or_else(|_| doc_path.to_path_buf());
        let Some(doc_idx) = self
            .data()
            .documents
            .iter()
            .position(|d| d.path.canonicalize().unwrap_or_else(|_| d.path.clone()) == doc_path)
        else {
            return Vec::new();
        };
        self.document_entity_iris
            .get(doc_idx)
            .into_iter()
            .flatten()
            .filter_map(|iri| self.find_entity(iri))
            .collect()
    }
}

fn axiom_summary(a: &strixonomy_core::Axiom, editable: bool) -> EntityAxiomSummary {
    let is_named_iri = a.object.starts_with("http://") || a.object.starts_with("https://");
    let manchester = if is_named_iri
        || a.axiom_kind == AXIOM_KIND_HAS_KEY
        || a.axiom_kind == AXIOM_KIND_DISJOINT_UNION
        || a.axiom_kind == AXIOM_KIND_PROPERTY_CHAIN
    {
        None
    } else if a.axiom_kind == AXIOM_KIND_DATATYPE_DEFINITION
        || (!is_named_iri && !a.object.is_empty())
    {
        Some(a.object.clone())
    } else {
        None
    };
    let parent_iri = if (a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF
        || a.axiom_kind == AXIOM_KIND_CLASS_ASSERTION
        || a.axiom_kind == AXIOM_KIND_SUB_OBJECT_PROPERTY_OF
        || a.axiom_kind == AXIOM_KIND_SUB_DATA_PROPERTY_OF
        || a.axiom_kind == AXIOM_KIND_DOMAIN
        || a.axiom_kind == AXIOM_KIND_RANGE)
        && is_named_iri
    {
        Some(a.object.clone())
    } else {
        None
    };
    let other_iri = if matches!(
        a.axiom_kind.as_str(),
        AXIOM_KIND_DISJOINT_CLASS
            | AXIOM_KIND_INVERSE_OBJECT_PROPERTIES
            | AXIOM_KIND_EQUIVALENT_OBJECT_PROPERTIES
            | AXIOM_KIND_DISJOINT_OBJECT_PROPERTIES
            | AXIOM_KIND_EQUIVALENT_DATA_PROPERTIES
            | AXIOM_KIND_DISJOINT_DATA_PROPERTIES
            | AXIOM_KIND_SAME_INDIVIDUAL
            | AXIOM_KIND_DIFFERENT_INDIVIDUALS
            | AXIOM_KIND_EQUIVALENT_CLASS
            | AXIOM_KIND_NEGATIVE_OBJECT_PROPERTY_ASSERTION
            | AXIOM_KIND_OBJECT_PROPERTY_ASSERTION
    ) && is_named_iri
    {
        Some(a.object.clone())
    } else {
        None
    };
    let properties = if a.axiom_kind == AXIOM_KIND_PROPERTY_CHAIN {
        a.object.split(" o ").map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    } else if a.axiom_kind == AXIOM_KIND_HAS_KEY || a.axiom_kind == AXIOM_KIND_DISJOINT_UNION {
        a.object.split(" | ").map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    } else {
        Vec::new()
    };
    let kind_label = match a.axiom_kind.as_str() {
        AXIOM_KIND_EQUIVALENT_CLASS => "EquivalentClasses",
        AXIOM_KIND_DISJOINT_CLASS => "DisjointClasses",
        AXIOM_KIND_DOMAIN => "Domain",
        AXIOM_KIND_RANGE => "Range",
        AXIOM_KIND_PROPERTY_CHAIN => "PropertyChain",
        AXIOM_KIND_CLASS_ASSERTION => "ClassAssertion",
        AXIOM_KIND_OBJECT_PROPERTY_ASSERTION => "ObjectPropertyAssertion",
        AXIOM_KIND_DATA_PROPERTY_ASSERTION => "DataPropertyAssertion",
        AXIOM_KIND_HAS_KEY => "HasKey",
        AXIOM_KIND_DISJOINT_UNION => "DisjointUnion",
        AXIOM_KIND_INVERSE_OBJECT_PROPERTIES => "InverseObjectProperties",
        AXIOM_KIND_EQUIVALENT_OBJECT_PROPERTIES => "EquivalentObjectProperties",
        AXIOM_KIND_DISJOINT_OBJECT_PROPERTIES => "DisjointObjectProperties",
        AXIOM_KIND_EQUIVALENT_DATA_PROPERTIES => "EquivalentDataProperties",
        AXIOM_KIND_DISJOINT_DATA_PROPERTIES => "DisjointDataProperties",
        AXIOM_KIND_SUB_OBJECT_PROPERTY_OF => "SubObjectPropertyOf",
        AXIOM_KIND_SUB_DATA_PROPERTY_OF => "SubDataPropertyOf",
        AXIOM_KIND_NEGATIVE_OBJECT_PROPERTY_ASSERTION => "NegativeObjectPropertyAssertion",
        AXIOM_KIND_NEGATIVE_DATA_PROPERTY_ASSERTION => "NegativeDataPropertyAssertion",
        AXIOM_KIND_SAME_INDIVIDUAL => "SameIndividual",
        AXIOM_KIND_DIFFERENT_INDIVIDUALS => "DifferentIndividuals",
        AXIOM_KIND_DATATYPE_DEFINITION => "DatatypeDefinition",
        _ => "SubClassOf",
    };
    let display = match a.axiom_kind.as_str() {
        AXIOM_KIND_NEGATIVE_OBJECT_PROPERTY_ASSERTION
        | AXIOM_KIND_NEGATIVE_DATA_PROPERTY_ASSERTION
        | AXIOM_KIND_OBJECT_PROPERTY_ASSERTION
        | AXIOM_KIND_DATA_PROPERTY_ASSERTION => {
            format!("{} {} {}", kind_label, short_tail(&a.predicate), short_tail(&a.object))
        }
        _ => format!("{} {}", kind_label, short_tail(&a.object)),
    };
    let annotations: Vec<EntityAnnotationSummary> = a
        .annotations
        .iter()
        .map(|ann| EntityAnnotationSummary {
            predicate: ann.predicate.clone(),
            value: ann.value.clone(),
            editable,
        })
        .collect();
    let predicate = if matches!(
        a.axiom_kind.as_str(),
        AXIOM_KIND_OBJECT_PROPERTY_ASSERTION
            | AXIOM_KIND_DATA_PROPERTY_ASSERTION
            | AXIOM_KIND_NEGATIVE_OBJECT_PROPERTY_ASSERTION
            | AXIOM_KIND_NEGATIVE_DATA_PROPERTY_ASSERTION
    ) {
        Some(a.predicate.clone())
    } else {
        None
    };
    EntityAxiomSummary {
        kind: a.axiom_kind.clone(),
        display,
        manchester,
        parent_iri,
        other_iri,
        predicate,
        properties,
        annotations,
        editable,
    }
}

fn short_tail(iri_or_text: &str) -> &str {
    iri_or_text.rsplit(['#', '/']).next().unwrap_or(iri_or_text)
}

fn scan_file_for_iri(
    path: &std::path::Path,
    iri: &str,
    short_name: &str,
    namespaces: &BTreeMap<String, String>,
) -> Option<SourceHint> {
    if path.symlink_metadata().ok()?.file_type().is_symlink() {
        return None;
    }
    let content = read_to_string_capped(path, MAX_FILE_BYTES).ok()?;
    let loc = find_in_source(&content, &entity_needles(iri, short_name, namespaces));
    loc.line.map(|line| SourceHint {
        path: path.to_path_buf(),
        line,
        column: loc.column.unwrap_or(0),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IndexBuilder;
    use std::path::PathBuf;

    fn fixture_workspace() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
    }

    fn fixture_catalog() -> OntologyCatalog {
        IndexBuilder::new().workspace(fixture_workspace()).build().expect("build catalog")
    }

    #[test]
    fn find_entity_by_iri() {
        let catalog = fixture_catalog();
        let entity =
            catalog.find_entity("http://example.org/people#Person").expect("Person entity");
        assert_eq!(entity.short_name, "Person");
        assert_eq!(entity.kind, EntityKind::Class);
    }

    #[test]
    fn class_hierarchy_includes_subclass_axiom() {
        let catalog = fixture_catalog();
        let hierarchy = catalog.class_hierarchy();
        assert!(!hierarchy.edges.is_empty());
        assert!(hierarchy
            .parents
            .get("http://example.org/people#Person")
            .is_some_and(|p| p.contains(&"http://example.org/people#Thing".to_string())));
    }

    #[test]
    fn entity_detail_includes_labels_and_parents() {
        let catalog = fixture_catalog();
        let detail =
            catalog.entity_detail("http://example.org/people#Person").expect("Person detail");
        assert!(!detail.entity.labels.is_empty());
        assert!(!detail.parents.is_empty());
    }

    #[test]
    fn find_source_location_in_fixture() {
        let catalog = fixture_catalog();
        let source = catalog
            .find_source_location("http://example.org/people#Person")
            .expect("source location");
        assert!(source.path.ends_with("example.ttl"));
        assert!(source.line > 0);
    }

    #[test]
    fn entities_in_document_uses_build_time_index() {
        let catalog = fixture_catalog();
        let doc_path = fixture_workspace().join("example.ttl");
        let entities = catalog.entities_in_document(&doc_path);
        assert!(entities.iter().any(|e| e.short_name == "Person"));
    }

    #[test]
    fn find_source_location_skips_person_colon_in_comments() {
        let dir = tempfile::tempdir().expect("tempdir");
        let ttl_path = dir.path().join("test.ttl");
        std::fs::write(
            &ttl_path,
            concat!(
                "@prefix ex: <http://ex#> .\n",
                "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\n",
                "# Note: Person: see documentation\n\n",
                "ex:Person a owl:Class .\n"
            ),
        )
        .expect("write ttl");

        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build catalog");
        let source = catalog.find_source_location("http://ex#Person").expect("source location");
        let entity_line = std::fs::read_to_string(&ttl_path)
            .expect("read ttl")
            .lines()
            .position(|line| line.contains("ex:Person"))
            .expect("entity line")
            + 1;
        assert_eq!(source.line, entity_line as u64);
        assert_eq!(source.column, 0);
    }

    #[test]
    fn obo_entity_detail_marks_axioms_editable() {
        let dir = tempfile::tempdir().expect("tempdir");
        let obo_path = dir.path().join("demo.obo");
        std::fs::write(
            &obo_path,
            concat!(
                "format-version: 1.2\n",
                "ontology: demo\n\n",
                "[Term]\n",
                "id: DEMO:0001\n",
                "name: child\n",
                "is_a: DEMO:0002\n\n",
                "[Term]\n",
                "id: DEMO:0002\n",
                "name: parent\n",
            ),
        )
        .expect("write obo");

        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build catalog");
        let child = catalog
            .data()
            .entities
            .iter()
            .find(|e| e.obo_id.as_deref() == Some("DEMO:0001"))
            .expect("child term");
        let detail = catalog.entity_detail(&child.iri).expect("child detail");
        assert!(detail.editable);
        assert!(!detail.axioms.is_empty());
        assert!(detail.axioms.iter().all(|a| a.editable));
    }

    #[test]
    fn property_chain_axiom_summary_includes_member_iris() {
        let dir = tempfile::tempdir().expect("tempdir");
        let ttl_path = dir.path().join("chains.ttl");
        std::fs::write(
            &ttl_path,
            concat!(
                "@prefix ex: <http://example.org/org#> .\n",
                "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n",
                "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n",
                "ex:chases a owl:ObjectProperty .\n",
                "ex:composed a owl:ObjectProperty ;\n",
                "    owl:propertyChainAxiom ( ex:chases ex:chases ) .\n"
            ),
        )
        .expect("write ttl");
        let catalog = IndexBuilder::new().workspace(dir.path()).build().expect("build");
        let detail =
            catalog.entity_detail("http://example.org/org#composed").expect("composed detail");
        let chain = detail
            .axioms
            .iter()
            .find(|a| a.kind == AXIOM_KIND_PROPERTY_CHAIN)
            .expect("property_chain axiom");
        assert_eq!(
            chain.properties,
            vec![
                "http://example.org/org#chases".to_string(),
                "http://example.org/org#chases".to_string()
            ]
        );
    }
}
