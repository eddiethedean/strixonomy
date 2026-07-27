use crate::manchester::class_expression_to_manchester;
use crate::span::{annotate_spans, find_entity_block, short_name_from_iri};
use horned_owl::model::{
    AnnotatedComponent, AnnotationSubject, AnnotationValue, ClassExpression, Component,
    ComponentKind, Individual, ObjectPropertyExpression, PropertyExpression, RcAnnotatedComponent,
    RcStr, SubObjectPropertyExpression,
};
use horned_owl::ontology::component_mapped::{ComponentMappedIndex, ComponentMappedOntology};
use std::collections::BTreeMap;
use strixonomy_core::{
    Annotation, Axiom, AxiomAnnotation, Entity, EntityKind, Import, Namespace, SourceLocation,
    AXIOM_KIND_CLASS_ASSERTION, AXIOM_KIND_DATATYPE_DEFINITION, AXIOM_KIND_DATA_PROPERTY_ASSERTION,
    AXIOM_KIND_DIFFERENT_INDIVIDUALS, AXIOM_KIND_DISJOINT_CLASS,
    AXIOM_KIND_DISJOINT_DATA_PROPERTIES, AXIOM_KIND_DISJOINT_OBJECT_PROPERTIES,
    AXIOM_KIND_DISJOINT_UNION, AXIOM_KIND_DOMAIN, AXIOM_KIND_EQUIVALENT_CLASS,
    AXIOM_KIND_EQUIVALENT_DATA_PROPERTIES, AXIOM_KIND_EQUIVALENT_OBJECT_PROPERTIES,
    AXIOM_KIND_HAS_KEY, AXIOM_KIND_INVERSE_OBJECT_PROPERTIES,
    AXIOM_KIND_NEGATIVE_DATA_PROPERTY_ASSERTION, AXIOM_KIND_NEGATIVE_OBJECT_PROPERTY_ASSERTION,
    AXIOM_KIND_OBJECT_PROPERTY_ASSERTION, AXIOM_KIND_PROPERTY_CHAIN, AXIOM_KIND_RANGE,
    AXIOM_KIND_SAME_INDIVIDUAL, AXIOM_KIND_SUB_CLASS_OF, AXIOM_KIND_SUB_DATA_PROPERTY_OF,
    AXIOM_KIND_SUB_OBJECT_PROPERTY_OF,
};

const RDFS_LABEL: &str = "http://www.w3.org/2000/01/rdf-schema#label";
const RDFS_COMMENT: &str = "http://www.w3.org/2000/01/rdf-schema#comment";
const RDFS_SUB_CLASS_OF: &str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";
const RDFS_SUB_PROPERTY_OF: &str = "http://www.w3.org/2000/01/rdf-schema#subPropertyOf";
const OWL_DEPRECATED: &str = "http://www.w3.org/2002/07/owl#deprecated";
const OWL_SAME_AS: &str = "http://www.w3.org/2002/07/owl#sameAs";
const OWL_DIFFERENT_FROM: &str = "http://www.w3.org/2002/07/owl#differentFrom";
const OWL_HAS_KEY: &str = "http://www.w3.org/2002/07/owl#hasKey";
const OWL_DISJOINT_UNION_OF: &str = "http://www.w3.org/2002/07/owl#disjointUnionOf";
const OWL_INVERSE_OF: &str = "http://www.w3.org/2002/07/owl#inverseOf";
const OWL_EQUIVALENT_PROPERTY: &str = "http://www.w3.org/2002/07/owl#equivalentProperty";
const OWL_PROPERTY_DISJOINT_WITH: &str = "http://www.w3.org/2002/07/owl#propertyDisjointWith";
const OWL_EQUIVALENT_CLASS: &str = "http://www.w3.org/2002/07/owl#equivalentClass";
const MEMBER_SEP: &str = " | ";

/// Catalog-shaped extraction from a Horned-OWL ontology.
#[derive(Debug, Clone, Default)]
pub struct OwlBridgeResult {
    pub entities: Vec<Entity>,
    pub annotations: Vec<Annotation>,
    pub axioms: Vec<Axiom>,
    pub imports: Vec<Import>,
    pub namespace_rows: Vec<Namespace>,
    pub base_iri: Option<String>,
}

pub fn bridge_ontology(
    ontology: impl Into<ComponentMappedOntology<RcStr, RcAnnotatedComponent>>,
    ontology_id: &str,
    source_text: &str,
    namespaces: &BTreeMap<String, String>,
) -> OwlBridgeResult {
    let mapped = ontology.into();
    let idx: &ComponentMappedIndex<RcStr, RcAnnotatedComponent> = mapped.i();

    let mut result = OwlBridgeResult::default();
    let ont_id = idx
        .the_ontology_id()
        .and_then(|id| id.iri.as_ref().map(|iri| iri.to_string()))
        .unwrap_or_else(|| ontology_id.to_string());
    result.base_iri = Some(ont_id.clone());

    let mut entity_map: BTreeMap<String, Entity> = BTreeMap::new();

    for decl in idx.declare_class() {
        insert_entity(&mut entity_map, decl.0.to_string(), EntityKind::Class, &ont_id);
    }
    for decl in idx.declare_object_property() {
        insert_entity(&mut entity_map, decl.0.to_string(), EntityKind::ObjectProperty, &ont_id);
    }
    for decl in idx.declare_data_property() {
        insert_entity(&mut entity_map, decl.0.to_string(), EntityKind::DataProperty, &ont_id);
    }
    for decl in idx.declare_annotation_property() {
        insert_entity(&mut entity_map, decl.0.to_string(), EntityKind::AnnotationProperty, &ont_id);
    }
    for decl in idx.declare_named_individual() {
        insert_entity(&mut entity_map, decl.0.to_string(), EntityKind::Individual, &ont_id);
    }
    for decl in idx.declare_datatype() {
        insert_entity(&mut entity_map, decl.0.to_string(), EntityKind::Datatype, &ont_id);
    }

    for ann_ax in idx.annotation_assertion() {
        let subject = annotation_subject_iri(&ann_ax.subject);
        let predicate = ann_ax.ann.ap.to_string();
        let object = annotation_value_string(&ann_ax.ann.av);
        if predicate == RDFS_LABEL {
            entity_map.entry(subject.clone()).and_modify(|e| e.labels.push(object.clone()));
        } else if predicate == RDFS_COMMENT {
            entity_map.entry(subject.clone()).and_modify(|e| e.comments.push(object.clone()));
        } else if predicate == OWL_DEPRECATED
            && strixonomy_core::parse_boolean_literal(&object) == Some(true)
        {
            entity_map.entry(subject.clone()).and_modify(|e| e.deprecated = true);
        }
        result.annotations.push(Annotation {
            subject,
            predicate,
            object,
            ontology_id: ont_id.clone(),
            source_location: SourceLocation::default(),
        });
    }

    // Project SameIndividual as owl:sameAs annotations so semantic-diff rename detection works (#312).
    for si in idx.same_individual() {
        for a in &si.0 {
            let subject = individual_to_iri(a);
            for b in &si.0 {
                if a == b {
                    continue;
                }
                result.annotations.push(Annotation {
                    subject: subject.clone(),
                    predicate: OWL_SAME_AS.to_string(),
                    object: individual_to_iri(b),
                    ontology_id: ont_id.clone(),
                    source_location: SourceLocation::default(),
                });
            }
        }
    }

    let mut axiom_counter = 0usize;

    for ac in idx.component_for_kind(ComponentKind::SubClassOf) {
        let Component::SubClassOf(ax) = &ac.component else {
            continue;
        };
        if let ClassExpression::Class(sub) = &ax.sub {
            if let Some(sup) = class_expr_display(&ax.sup, namespaces) {
                push_axiom(
                    &mut result.axioms,
                    &mut axiom_counter,
                    ontology_id,
                    &ont_id,
                    sub.to_string(),
                    RDFS_SUB_CLASS_OF.to_string(),
                    sup,
                    AXIOM_KIND_SUB_CLASS_OF,
                    ann_bag(ac),
                );
            }
        }
    }

    for ac in idx.component_for_kind(ComponentKind::EquivalentClasses) {
        let Component::EquivalentClasses(eq) = &ac.component else {
            continue;
        };
        let anns = ann_bag(ac);
        for ce in &eq.0 {
            if let ClassExpression::Class(sub) = ce {
                for other in &eq.0 {
                    if other == ce {
                        continue;
                    }
                    if let Some(obj) = class_expr_display(other, namespaces) {
                        push_axiom(
                            &mut result.axioms,
                            &mut axiom_counter,
                            ontology_id,
                            &ont_id,
                            sub.to_string(),
                            OWL_EQUIVALENT_CLASS.to_string(),
                            obj,
                            AXIOM_KIND_EQUIVALENT_CLASS,
                            anns.clone(),
                        );
                    }
                }
            }
        }
    }

    for ac in idx.component_for_kind(ComponentKind::DisjointClasses) {
        let Component::DisjointClasses(disj) = &ac.component else {
            continue;
        };
        let anns = ann_bag(ac);
        for ce in &disj.0 {
            if let ClassExpression::Class(sub) = ce {
                for other in &disj.0 {
                    if other == ce {
                        continue;
                    }
                    if let Some(obj) = class_expr_display(other, namespaces) {
                        push_axiom(
                            &mut result.axioms,
                            &mut axiom_counter,
                            ontology_id,
                            &ont_id,
                            sub.to_string(),
                            "http://www.w3.org/2002/07/owl#disjointWith".to_string(),
                            obj,
                            AXIOM_KIND_DISJOINT_CLASS,
                            anns.clone(),
                        );
                    }
                }
            }
        }
    }

    for ac in idx.component_for_kind(ComponentKind::DisjointUnion) {
        let Component::DisjointUnion(du) = &ac.component else {
            continue;
        };
        let members: Vec<String> =
            du.1.iter().filter_map(|ce| class_expr_display(ce, namespaces)).collect();
        push_axiom(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            du.0.to_string(),
            OWL_DISJOINT_UNION_OF.to_string(),
            members.join(MEMBER_SEP),
            AXIOM_KIND_DISJOINT_UNION,
            ann_bag(ac),
        );
    }

    for ac in idx.component_for_kind(ComponentKind::HasKey) {
        let Component::HasKey(hk) = &ac.component else {
            continue;
        };
        let ClassExpression::Class(cls) = &hk.ce else {
            continue;
        };
        let props: Vec<String> = hk.vpe.iter().map(property_expr_to_iri).collect();
        push_axiom(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            cls.to_string(),
            OWL_HAS_KEY.to_string(),
            props.join(MEMBER_SEP),
            AXIOM_KIND_HAS_KEY,
            ann_bag(ac),
        );
    }

    for ac in idx.component_for_kind(ComponentKind::InverseObjectProperties) {
        let Component::InverseObjectProperties(inv) = &ac.component else {
            continue;
        };
        let anns = ann_bag(ac);
        let a = inv.0.to_string();
        let b = inv.1.to_string();
        push_axiom(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            a.clone(),
            OWL_INVERSE_OF.to_string(),
            b.clone(),
            AXIOM_KIND_INVERSE_OBJECT_PROPERTIES,
            anns.clone(),
        );
        push_axiom(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            b,
            OWL_INVERSE_OF.to_string(),
            a,
            AXIOM_KIND_INVERSE_OBJECT_PROPERTIES,
            anns,
        );
    }

    for ac in idx.component_for_kind(ComponentKind::EquivalentObjectProperties) {
        let Component::EquivalentObjectProperties(eq) = &ac.component else {
            continue;
        };
        let anns = ann_bag(ac);
        let iris: Vec<String> = eq.0.iter().map(ope_to_iri).collect();
        push_pairwise_property_axioms(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            &iris,
            OWL_EQUIVALENT_PROPERTY,
            AXIOM_KIND_EQUIVALENT_OBJECT_PROPERTIES,
            anns,
        );
    }

    for ac in idx.component_for_kind(ComponentKind::DisjointObjectProperties) {
        let Component::DisjointObjectProperties(dj) = &ac.component else {
            continue;
        };
        let anns = ann_bag(ac);
        let iris: Vec<String> = dj.0.iter().map(ope_to_iri).collect();
        push_pairwise_property_axioms(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            &iris,
            OWL_PROPERTY_DISJOINT_WITH,
            AXIOM_KIND_DISJOINT_OBJECT_PROPERTIES,
            anns,
        );
    }

    for ac in idx.component_for_kind(ComponentKind::EquivalentDataProperties) {
        let Component::EquivalentDataProperties(eq) = &ac.component else {
            continue;
        };
        let anns = ann_bag(ac);
        let iris: Vec<String> = eq.0.iter().map(|dp| dp.to_string()).collect();
        push_pairwise_property_axioms(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            &iris,
            OWL_EQUIVALENT_PROPERTY,
            AXIOM_KIND_EQUIVALENT_DATA_PROPERTIES,
            anns,
        );
    }

    for ac in idx.component_for_kind(ComponentKind::DisjointDataProperties) {
        let Component::DisjointDataProperties(dj) = &ac.component else {
            continue;
        };
        let anns = ann_bag(ac);
        let iris: Vec<String> = dj.0.iter().map(|dp| dp.to_string()).collect();
        push_pairwise_property_axioms(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            &iris,
            OWL_PROPERTY_DISJOINT_WITH,
            AXIOM_KIND_DISJOINT_DATA_PROPERTIES,
            anns,
        );
    }

    for ac in idx.component_for_kind(ComponentKind::ObjectPropertyDomain) {
        let Component::ObjectPropertyDomain(dom) = &ac.component else {
            continue;
        };
        let prop = ope_to_iri(&dom.ope);
        if let ClassExpression::Class(cls) = &dom.ce {
            push_axiom(
                &mut result.axioms,
                &mut axiom_counter,
                ontology_id,
                &ont_id,
                prop,
                "http://www.w3.org/2000/01/rdf-schema#domain".to_string(),
                cls.to_string(),
                AXIOM_KIND_DOMAIN,
                ann_bag(ac),
            );
        }
    }
    for ac in idx.component_for_kind(ComponentKind::ObjectPropertyRange) {
        let Component::ObjectPropertyRange(rng) = &ac.component else {
            continue;
        };
        let prop = ope_to_iri(&rng.ope);
        if let ClassExpression::Class(cls) = &rng.ce {
            push_axiom(
                &mut result.axioms,
                &mut axiom_counter,
                ontology_id,
                &ont_id,
                prop,
                "http://www.w3.org/2000/01/rdf-schema#range".to_string(),
                cls.to_string(),
                AXIOM_KIND_RANGE,
                ann_bag(ac),
            );
        }
    }
    for ac in idx.component_for_kind(ComponentKind::DataPropertyDomain) {
        let Component::DataPropertyDomain(dom) = &ac.component else {
            continue;
        };
        let prop = dom.dp.to_string();
        if let ClassExpression::Class(cls) = &dom.ce {
            push_axiom(
                &mut result.axioms,
                &mut axiom_counter,
                ontology_id,
                &ont_id,
                prop,
                "http://www.w3.org/2000/01/rdf-schema#domain".to_string(),
                cls.to_string(),
                AXIOM_KIND_DOMAIN,
                ann_bag(ac),
            );
        }
    }
    for ac in idx.component_for_kind(ComponentKind::DataPropertyRange) {
        let Component::DataPropertyRange(rng) = &ac.component else {
            continue;
        };
        push_axiom(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            rng.dp.to_string(),
            "http://www.w3.org/2000/01/rdf-schema#range".to_string(),
            data_range_display(&rng.dr),
            AXIOM_KIND_RANGE,
            ann_bag(ac),
        );
    }

    for ac in idx.component_for_kind(ComponentKind::SubObjectPropertyOf) {
        let Component::SubObjectPropertyOf(sub_prop) = &ac.component else {
            continue;
        };
        match &sub_prop.sub {
            SubObjectPropertyExpression::ObjectPropertyChain(chain) => {
                let chain_display = chain.iter().map(ope_to_iri).collect::<Vec<_>>().join(" o ");
                push_axiom(
                    &mut result.axioms,
                    &mut axiom_counter,
                    ontology_id,
                    &ont_id,
                    ope_to_iri(&sub_prop.sup),
                    "http://www.w3.org/2002/07/owl#propertyChainAxiom".to_string(),
                    chain_display,
                    AXIOM_KIND_PROPERTY_CHAIN,
                    ann_bag(ac),
                );
            }
            SubObjectPropertyExpression::ObjectPropertyExpression(sub) => {
                push_axiom(
                    &mut result.axioms,
                    &mut axiom_counter,
                    ontology_id,
                    &ont_id,
                    ope_to_iri(sub),
                    RDFS_SUB_PROPERTY_OF.to_string(),
                    ope_to_iri(&sub_prop.sup),
                    AXIOM_KIND_SUB_OBJECT_PROPERTY_OF,
                    ann_bag(ac),
                );
            }
        }
    }

    for ac in idx.component_for_kind(ComponentKind::SubDataPropertyOf) {
        let Component::SubDataPropertyOf(sub_prop) = &ac.component else {
            continue;
        };
        push_axiom(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            sub_prop.sub.to_string(),
            RDFS_SUB_PROPERTY_OF.to_string(),
            sub_prop.sup.to_string(),
            AXIOM_KIND_SUB_DATA_PROPERTY_OF,
            ann_bag(ac),
        );
    }

    for ac in idx.component_for_kind(ComponentKind::ClassAssertion) {
        let Component::ClassAssertion(ca) = &ac.component else {
            continue;
        };
        if let ClassExpression::Class(cls) = &ca.ce {
            push_axiom(
                &mut result.axioms,
                &mut axiom_counter,
                ontology_id,
                &ont_id,
                individual_to_iri(&ca.i),
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_string(),
                cls.to_string(),
                AXIOM_KIND_CLASS_ASSERTION,
                ann_bag(ac),
            );
        }
    }

    for ac in idx.component_for_kind(ComponentKind::ObjectPropertyAssertion) {
        let Component::ObjectPropertyAssertion(opa) = &ac.component else {
            continue;
        };
        push_axiom(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            individual_to_iri(&opa.from),
            ope_to_iri(&opa.ope),
            individual_to_iri(&opa.to),
            AXIOM_KIND_OBJECT_PROPERTY_ASSERTION,
            ann_bag(ac),
        );
    }

    for ac in idx.component_for_kind(ComponentKind::DataPropertyAssertion) {
        let Component::DataPropertyAssertion(dpa) = &ac.component else {
            continue;
        };
        push_axiom(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            individual_to_iri(&dpa.from),
            dpa.dp.to_string(),
            dpa.to.literal().clone(),
            AXIOM_KIND_DATA_PROPERTY_ASSERTION,
            ann_bag(ac),
        );
    }

    for ac in idx.component_for_kind(ComponentKind::NegativeObjectPropertyAssertion) {
        let Component::NegativeObjectPropertyAssertion(nopa) = &ac.component else {
            continue;
        };
        push_axiom(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            individual_to_iri(&nopa.from),
            ope_to_iri(&nopa.ope),
            individual_to_iri(&nopa.to),
            AXIOM_KIND_NEGATIVE_OBJECT_PROPERTY_ASSERTION,
            ann_bag(ac),
        );
    }

    for ac in idx.component_for_kind(ComponentKind::NegativeDataPropertyAssertion) {
        let Component::NegativeDataPropertyAssertion(ndpa) = &ac.component else {
            continue;
        };
        push_axiom(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            individual_to_iri(&ndpa.from),
            ndpa.dp.to_string(),
            ndpa.to.literal().clone(),
            AXIOM_KIND_NEGATIVE_DATA_PROPERTY_ASSERTION,
            ann_bag(ac),
        );
    }

    for ac in idx.component_for_kind(ComponentKind::SameIndividual) {
        let Component::SameIndividual(si) = &ac.component else {
            continue;
        };
        let anns = ann_bag(ac);
        let iris: Vec<String> = si.0.iter().map(individual_to_iri).collect();
        for (i, a) in iris.iter().enumerate() {
            for (j, b) in iris.iter().enumerate() {
                if i == j {
                    continue;
                }
                push_axiom(
                    &mut result.axioms,
                    &mut axiom_counter,
                    ontology_id,
                    &ont_id,
                    a.clone(),
                    OWL_SAME_AS.to_string(),
                    b.clone(),
                    AXIOM_KIND_SAME_INDIVIDUAL,
                    anns.clone(),
                );
            }
        }
    }

    for ac in idx.component_for_kind(ComponentKind::DifferentIndividuals) {
        let Component::DifferentIndividuals(di) = &ac.component else {
            continue;
        };
        let anns = ann_bag(ac);
        let iris: Vec<String> = di.0.iter().map(individual_to_iri).collect();
        for (i, a) in iris.iter().enumerate() {
            for (j, b) in iris.iter().enumerate() {
                if i >= j {
                    continue;
                }
                push_axiom(
                    &mut result.axioms,
                    &mut axiom_counter,
                    ontology_id,
                    &ont_id,
                    a.clone(),
                    OWL_DIFFERENT_FROM.to_string(),
                    b.clone(),
                    AXIOM_KIND_DIFFERENT_INDIVIDUALS,
                    anns.clone(),
                );
                push_axiom(
                    &mut result.axioms,
                    &mut axiom_counter,
                    ontology_id,
                    &ont_id,
                    b.clone(),
                    OWL_DIFFERENT_FROM.to_string(),
                    a.clone(),
                    AXIOM_KIND_DIFFERENT_INDIVIDUALS,
                    anns.clone(),
                );
            }
        }
    }

    for ac in idx.component_for_kind(ComponentKind::DatatypeDefinition) {
        let Component::DatatypeDefinition(dd) = &ac.component else {
            continue;
        };
        insert_entity(&mut entity_map, dd.kind.to_string(), EntityKind::Datatype, &ont_id);
        push_axiom(
            &mut result.axioms,
            &mut axiom_counter,
            ontology_id,
            &ont_id,
            dd.kind.to_string(),
            OWL_EQUIVALENT_CLASS.to_string(),
            data_range_display(&dd.range),
            AXIOM_KIND_DATATYPE_DEFINITION,
            ann_bag(ac),
        );
    }

    mark_functional_properties(&mut entity_map, idx);

    for imp in idx.import() {
        result.imports.push(Import { ontology_id: ont_id.clone(), import_iri: imp.0.to_string() });
    }

    for (prefix, iri) in namespaces {
        result.namespace_rows.push(Namespace {
            prefix: prefix.clone(),
            iri: iri.clone(),
            ontology_id: ont_id.clone(),
        });
    }

    result.entities = entity_map.into_values().collect();
    for entity in &mut result.entities {
        entity.source_location =
            find_entity_block(source_text, &entity.iri, &entity.short_name, namespaces);
    }
    annotate_spans(source_text, &mut result.entities, &mut result.annotations, &mut result.axioms);
    result
}

fn ann_bag(ac: &AnnotatedComponent<RcStr>) -> Vec<AxiomAnnotation> {
    ac.ann
        .iter()
        .map(|a| AxiomAnnotation {
            predicate: a.ap.to_string(),
            value: annotation_value_string(&a.av),
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn push_axiom(
    axioms: &mut Vec<Axiom>,
    counter: &mut usize,
    doc_id: &str,
    ontology_id: &str,
    subject: String,
    predicate: String,
    object: String,
    kind: &str,
    annotations: Vec<AxiomAnnotation>,
) {
    *counter += 1;
    axioms.push(Axiom {
        id: format!("{doc_id}#axiom-{counter}"),
        ontology_id: ontology_id.to_string(),
        subject,
        predicate,
        object,
        axiom_kind: kind.to_string(),
        source_location: SourceLocation::default(),
        annotations,
    });
}

#[allow(clippy::too_many_arguments)]
fn push_pairwise_property_axioms(
    axioms: &mut Vec<Axiom>,
    counter: &mut usize,
    doc_id: &str,
    ontology_id: &str,
    iris: &[String],
    predicate: &str,
    kind: &str,
    annotations: Vec<AxiomAnnotation>,
) {
    for (i, a) in iris.iter().enumerate() {
        for (j, b) in iris.iter().enumerate() {
            if i == j {
                continue;
            }
            push_axiom(
                axioms,
                counter,
                doc_id,
                ontology_id,
                a.clone(),
                predicate.to_string(),
                b.clone(),
                kind,
                annotations.clone(),
            );
        }
    }
}

fn insert_entity(
    map: &mut BTreeMap<String, Entity>,
    iri: String,
    kind: EntityKind,
    ontology_id: &str,
) {
    let short_name = short_name_from_iri(&iri);
    map.entry(iri.clone()).or_insert_with(|| Entity {
        iri,
        short_name,
        kind,
        ontology_id: ontology_id.to_string(),
        source_location: SourceLocation::default(),
        labels: Vec::new(),
        comments: Vec::new(),
        deprecated: false,
        obo_id: None,
        characteristics: strixonomy_core::PropertyCharacteristics::default(),
    });
}

fn individual_to_iri(individual: &Individual<RcStr>) -> String {
    individual.to_string()
}

fn mark_functional_properties(
    entity_map: &mut BTreeMap<String, Entity>,
    idx: &ComponentMappedIndex<RcStr, RcAnnotatedComponent>,
) {
    for f in idx.functional_object_property() {
        let iri = ope_to_iri(&f.0);
        if let Some(e) = entity_map.get_mut(&iri) {
            e.characteristics.functional = true;
        }
    }
    for f in idx.inverse_functional_object_property() {
        let iri = ope_to_iri(&f.0);
        if let Some(e) = entity_map.get_mut(&iri) {
            e.characteristics.inverse_functional = true;
        }
    }
    for f in idx.transitive_object_property() {
        let iri = ope_to_iri(&f.0);
        if let Some(e) = entity_map.get_mut(&iri) {
            e.characteristics.transitive = true;
        }
    }
    for f in idx.symmetric_object_property() {
        let iri = ope_to_iri(&f.0);
        if let Some(e) = entity_map.get_mut(&iri) {
            e.characteristics.symmetric = true;
        }
    }
    for f in idx.asymmetric_object_property() {
        let iri = ope_to_iri(&f.0);
        if let Some(e) = entity_map.get_mut(&iri) {
            e.characteristics.asymmetric = true;
        }
    }
    for f in idx.reflexive_object_property() {
        let iri = ope_to_iri(&f.0);
        if let Some(e) = entity_map.get_mut(&iri) {
            e.characteristics.reflexive = true;
        }
    }
    for f in idx.irreflexive_object_property() {
        let iri = ope_to_iri(&f.0);
        if let Some(e) = entity_map.get_mut(&iri) {
            e.characteristics.irreflexive = true;
        }
    }
    for f in idx.functional_data_property() {
        let iri = f.0.to_string();
        if let Some(e) = entity_map.get_mut(&iri) {
            e.characteristics.functional = true;
        }
    }
}

fn class_expr_display(
    expr: &ClassExpression<RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> Option<String> {
    match expr {
        ClassExpression::Class(c) => Some(c.to_string()),
        _ => Some(class_expression_to_manchester(expr, namespaces)),
    }
}

fn ope_to_iri(ope: &ObjectPropertyExpression<RcStr>) -> String {
    match ope {
        ObjectPropertyExpression::ObjectProperty(p) => p.to_string(),
        ObjectPropertyExpression::InverseObjectProperty(p) => {
            format!("inverse({})", p.0)
        }
    }
}

fn property_expr_to_iri(pe: &PropertyExpression<RcStr>) -> String {
    match pe {
        PropertyExpression::ObjectPropertyExpression(ope) => ope_to_iri(ope),
        PropertyExpression::DataProperty(dp) => dp.to_string(),
        PropertyExpression::AnnotationProperty(ap) => ap.to_string(),
    }
}

fn data_range_display(dr: &horned_owl::model::DataRange<RcStr>) -> String {
    crate::manchester::data_range_to_manchester(dr, &BTreeMap::new())
}

fn annotation_subject_iri(subject: &AnnotationSubject<RcStr>) -> String {
    match subject {
        AnnotationSubject::IRI(iri) => iri.to_string(),
        AnnotationSubject::AnonymousIndividual(a) => format!("_:{}", a.as_ref()),
    }
}

fn annotation_value_string(value: &AnnotationValue<RcStr>) -> String {
    match value {
        AnnotationValue::Literal(l) => l.literal().clone(),
        AnnotationValue::IRI(iri) => iri.to_string(),
        AnnotationValue::AnonymousIndividual(a) => format!("_:{}", a.as_ref()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load::load_turtle_text;
    use oxigraph::io::RdfParser;
    use oxigraph::model::Quad;
    use std::path::Path;

    #[test]
    fn bridge_extracts_subclass_axioms() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let parser = RdfParser::from_format(oxigraph::io::RdfFormat::Turtle);
        let quads: Vec<Quad> =
            parser.for_reader(ttl.as_bytes()).collect::<std::result::Result<Vec<_>, _>>().unwrap();
        let namespaces =
            BTreeMap::from([("ex".to_string(), "http://example.org/people#".to_string())]);
        let loaded = load_turtle_text(Path::new("example.ttl"), "doc-1", ttl, &quads, &namespaces)
            .expect("load");
        assert!(loaded.bridge.axioms.iter().any(|a| a.axiom_kind == AXIOM_KIND_SUB_CLASS_OF));
    }

    #[test]
    fn bridge_extracts_disjoint_and_property_chain() {
        let ttl = include_str!("../../../fixtures/disjoint-classes.ttl");
        let parser = RdfParser::from_format(oxigraph::io::RdfFormat::Turtle);
        let quads: Vec<Quad> =
            parser.for_reader(ttl.as_bytes()).collect::<std::result::Result<Vec<_>, _>>().unwrap();
        let namespaces =
            BTreeMap::from([("ex".to_string(), "http://example.org/org#".to_string())]);
        let loaded =
            load_turtle_text(Path::new("disjoint-classes.ttl"), "doc-1", ttl, &quads, &namespaces)
                .expect("load");
        assert!(loaded.bridge.axioms.iter().any(|a| a.axiom_kind == AXIOM_KIND_DISJOINT_CLASS));
        assert!(loaded.bridge.axioms.iter().any(|a| a.axiom_kind == AXIOM_KIND_PROPERTY_CHAIN));
    }

    #[test]
    fn bridge_extracts_owl2_keys_and_inverse() {
        let ttl = include_str!("../../../examples/protege-roundtrip/owl2-keys.ttl");
        let parser = RdfParser::from_format(oxigraph::io::RdfFormat::Turtle);
        let quads: Vec<Quad> =
            parser.for_reader(ttl.as_bytes()).collect::<std::result::Result<Vec<_>, _>>().unwrap();
        let namespaces =
            BTreeMap::from([("ex".to_string(), "http://example.org/keys#".to_string())]);
        let loaded =
            load_turtle_text(Path::new("owl2-keys.ttl"), "doc-keys", ttl, &quads, &namespaces)
                .expect("load");
        assert!(loaded
            .bridge
            .axioms
            .iter()
            .any(|a| a.axiom_kind == AXIOM_KIND_HAS_KEY
                && a.subject == "http://example.org/keys#Person"));
        assert!(loaded.bridge.axioms.iter().any(|a| a.axiom_kind == AXIOM_KIND_DISJOINT_UNION
            && a.subject == "http://example.org/keys#Sex"));
        assert!(loaded.bridge.axioms.iter().any(|a| {
            a.axiom_kind == AXIOM_KIND_INVERSE_OBJECT_PROPERTIES
                && a.subject == "http://example.org/keys#hasParent"
                && a.object == "http://example.org/keys#hasChild"
        }));
    }

    #[test]
    fn bridge_extracts_owl2_abox() {
        let ttl = include_str!("../../../examples/protege-roundtrip/owl2-abox.ttl");
        let parser = RdfParser::from_format(oxigraph::io::RdfFormat::Turtle);
        let quads: Vec<Quad> =
            parser.for_reader(ttl.as_bytes()).collect::<std::result::Result<Vec<_>, _>>().unwrap();
        let namespaces =
            BTreeMap::from([("ex".to_string(), "http://example.org/abox#".to_string())]);
        let loaded =
            load_turtle_text(Path::new("owl2-abox.ttl"), "doc-abox", ttl, &quads, &namespaces)
                .expect("load");
        assert!(loaded.bridge.axioms.iter().any(|a| {
            a.axiom_kind == AXIOM_KIND_SAME_INDIVIDUAL
                && a.subject == "http://example.org/abox#alice"
        }));
        assert!(loaded.bridge.axioms.iter().any(|a| {
            a.axiom_kind == AXIOM_KIND_DIFFERENT_INDIVIDUALS
                && a.subject == "http://example.org/abox#alice"
        }));
        assert!(loaded.bridge.axioms.iter().any(|a| {
            a.axiom_kind == AXIOM_KIND_NEGATIVE_OBJECT_PROPERTY_ASSERTION
                && a.subject == "http://example.org/abox#alice"
                && a.object == "http://example.org/abox#bob"
        }));
    }
}
