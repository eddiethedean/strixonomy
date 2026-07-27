//! Remap an entity IRI through a Horned component-mapped ontology (v0.24 multi-format refactor).

use crate::error::{OwlError, Result};
use crate::serialize::{
    load_owl_xml_ontology, load_rdf_xml_ontology, serialize_owl_xml, serialize_rdf_xml,
};
use horned_owl::model::{
    AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationSubject, AnnotationValue, Atom,
    Build, Class, ClassAssertion, ClassExpression, Component, DArgument, DataPropertyAssertion,
    DataRange, DeclareClass, DeclareDataProperty, DeclareNamedIndividual, DeclareObjectProperty,
    FacetRestriction, IArgument, Individual, Literal, MutableOntology, NamedIndividual,
    ObjectPropertyAssertion, ObjectPropertyExpression, PropertyExpression, RcAnnotatedComponent,
    RcStr, SubClassOf, Variable,
};
use horned_owl::ontology::component_mapped::ComponentMappedOntology;
use std::collections::BTreeMap;

/// Remap every occurrence of `from_iri` to `to_iri` in `ont`.
///
/// Returns the number of annotated components rewritten.
pub fn remap_entity_iri(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    from_iri: &str,
    to_iri: &str,
) -> Result<usize> {
    if from_iri == to_iri {
        return Ok(0);
    }
    let build = Build::new_rc();
    let components: Vec<_> = ont.i().iter().cloned().collect();
    let mut rewritten = 0usize;
    for ac in components {
        let remapped = remap_annotated(&ac, from_iri, to_iri, &build);
        if remapped.component != ac.component || remapped.ann != ac.ann {
            let _ = ont.take(&ac);
            ont.insert(remapped);
            rewritten += 1;
        }
    }
    Ok(rewritten)
}

/// Load RDF/XML or OWL/XML text, remap an entity IRI, and re-serialize.
///
/// `format` is `"rdfxml"` or `"owlxml"`. Incomplete RDF/XML loads return an error
/// (same contract as patch XML write-back).
pub fn remap_entity_iri_in_xml_text(
    source: &str,
    format: &str,
    from_iri: &str,
    to_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<String> {
    match format {
        "rdfxml" | "rdf" | "owl" => {
            let (mut ont, incomplete) = load_rdf_xml_ontology(source)?;
            if incomplete {
                return Err(OwlError::LoadFailed(
                    "RDF/XML parse incomplete; refusing IRI remap write-back".into(),
                ));
            }
            remap_entity_iri(&mut ont, from_iri, to_iri)?;
            serialize_rdf_xml(&ont)
        }
        "owlxml" | "owx" => {
            let (mut ont, mut ns, incomplete) = load_owl_xml_ontology(source)?;
            if incomplete {
                return Err(OwlError::LoadFailed(
                    "OWL/XML parse incomplete; refusing IRI remap write-back".into(),
                ));
            }
            for (k, v) in namespaces {
                ns.entry(k.clone()).or_insert_with(|| v.clone());
            }
            remap_entity_iri(&mut ont, from_iri, to_iri)?;
            serialize_owl_xml(&ont, &ns)
        }
        other => Err(OwlError::SerializeFailed(format!("unsupported XML format: {other}"))),
    }
}

/// Merge `merge_iri` into `keep_iri`: drop subject-owned components for the merge
/// entity, then remap remaining mentions (Turtle/OBO-aligned; #369).
///
/// Returns `(removed, remapped)` component counts.
pub fn merge_entity_iri(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    keep_iri: &str,
    merge_iri: &str,
) -> Result<(usize, usize)> {
    if keep_iri == merge_iri {
        return Ok((0, 0));
    }
    let removed = remove_subject_owned_components(ont, merge_iri);
    let remapped = remap_entity_iri(ont, merge_iri, keep_iri)?;
    Ok((removed, remapped))
}

/// Load RDF/XML or OWL/XML, merge entities with Turtle delete-then-remap semantics, re-serialize.
pub fn merge_entity_iri_in_xml_text(
    source: &str,
    format: &str,
    keep_iri: &str,
    merge_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<String> {
    match format {
        "rdfxml" | "rdf" | "owl" => {
            let (mut ont, incomplete) = load_rdf_xml_ontology(source)?;
            if incomplete {
                return Err(OwlError::LoadFailed(
                    "RDF/XML parse incomplete; refusing entity merge write-back".into(),
                ));
            }
            merge_entity_iri(&mut ont, keep_iri, merge_iri)?;
            serialize_rdf_xml(&ont)
        }
        "owlxml" | "owx" => {
            let (mut ont, mut ns, incomplete) = load_owl_xml_ontology(source)?;
            if incomplete {
                return Err(OwlError::LoadFailed(
                    "OWL/XML parse incomplete; refusing entity merge write-back".into(),
                ));
            }
            for (k, v) in namespaces {
                ns.entry(k.clone()).or_insert_with(|| v.clone());
            }
            merge_entity_iri(&mut ont, keep_iri, merge_iri)?;
            serialize_owl_xml(&ont, &ns)
        }
        other => Err(OwlError::SerializeFailed(format!("unsupported XML format: {other}"))),
    }
}

/// Drop Horned components that correspond to Turtle statements *about* `entity_iri`
/// (declaration + axioms with that entity as primary subject), leaving object-position
/// mentions for a subsequent remap.
fn remove_subject_owned_components(
    ont: &mut ComponentMappedOntology<RcStr, RcAnnotatedComponent>,
    entity_iri: &str,
) -> usize {
    let to_remove: Vec<_> = ont
        .i()
        .iter()
        .filter(|ac| component_is_subject_owned(&ac.component, entity_iri))
        .cloned()
        .collect();
    let n = to_remove.len();
    for ac in to_remove {
        let _ = ont.take(&ac);
    }
    n
}

fn named_class_is(ce: &ClassExpression<RcStr>, iri: &str) -> bool {
    matches!(ce, ClassExpression::Class(Class(c)) if c.to_string() == iri)
}

fn named_individual_is(ind: &Individual<RcStr>, iri: &str) -> bool {
    matches!(ind, Individual::Named(NamedIndividual(n)) if n.to_string() == iri)
}

fn ope_is_named(ope: &ObjectPropertyExpression<RcStr>, iri: &str) -> bool {
    matches!(ope, ObjectPropertyExpression::ObjectProperty(p) if p.to_string() == iri)
}

fn component_is_subject_owned(c: &Component<RcStr>, entity_iri: &str) -> bool {
    match c {
        Component::DeclareClass(d) => d.0.to_string() == entity_iri,
        Component::DeclareObjectProperty(d) => d.0.to_string() == entity_iri,
        Component::DeclareDataProperty(d) => d.0.to_string() == entity_iri,
        Component::DeclareNamedIndividual(d) => d.0.to_string() == entity_iri,
        Component::DeclareAnnotationProperty(d) => d.0.to_string() == entity_iri,
        Component::DeclareDatatype(d) => d.0.to_string() == entity_iri,
        Component::AnnotationAssertion(ax) => {
            matches!(&ax.subject, AnnotationSubject::IRI(i) if i.to_string() == entity_iri)
        }
        // Binary LHS / primary subject only — object-position mentions stay for remap.
        Component::SubClassOf(ax) => named_class_is(&ax.sub, entity_iri),
        Component::SubObjectPropertyOf(ax) => match &ax.sub {
            horned_owl::model::SubObjectPropertyExpression::ObjectPropertyExpression(ope) => {
                ope_is_named(ope, entity_iri)
            }
            horned_owl::model::SubObjectPropertyExpression::ObjectPropertyChain(_) => false,
        },
        Component::SubDataPropertyOf(ax) => ax.sub.to_string() == entity_iri,
        Component::SubAnnotationPropertyOf(ax) => ax.sub.to_string() == entity_iri,
        Component::ObjectPropertyDomain(ax) => ope_is_named(&ax.ope, entity_iri),
        Component::ObjectPropertyRange(ax) => ope_is_named(&ax.ope, entity_iri),
        Component::DataPropertyDomain(ax) => ax.dp.to_string() == entity_iri,
        Component::DataPropertyRange(ax) => ax.dp.to_string() == entity_iri,
        Component::AnnotationPropertyDomain(ax) => ax.ap.to_string() == entity_iri,
        Component::AnnotationPropertyRange(ax) => ax.ap.to_string() == entity_iri,
        Component::FunctionalObjectProperty(ax) => ope_is_named(&ax.0, entity_iri),
        Component::InverseFunctionalObjectProperty(ax) => ope_is_named(&ax.0, entity_iri),
        Component::ReflexiveObjectProperty(ax) => ope_is_named(&ax.0, entity_iri),
        Component::IrreflexiveObjectProperty(ax) => ope_is_named(&ax.0, entity_iri),
        Component::SymmetricObjectProperty(ax) => ope_is_named(&ax.0, entity_iri),
        Component::AsymmetricObjectProperty(ax) => ope_is_named(&ax.0, entity_iri),
        Component::TransitiveObjectProperty(ax) => ope_is_named(&ax.0, entity_iri),
        Component::FunctionalDataProperty(ax) => ax.0.to_string() == entity_iri,
        Component::DatatypeDefinition(ax) => ax.kind.to_string() == entity_iri,
        Component::HasKey(ax) => named_class_is(&ax.ce, entity_iri),
        Component::DisjointUnion(ax) => ax.0.to_string() == entity_iri,
        Component::ClassAssertion(ax) => named_individual_is(&ax.i, entity_iri),
        Component::ObjectPropertyAssertion(ax) => named_individual_is(&ax.from, entity_iri),
        Component::NegativeObjectPropertyAssertion(ax) => named_individual_is(&ax.from, entity_iri),
        Component::DataPropertyAssertion(ax) => named_individual_is(&ax.from, entity_iri),
        Component::NegativeDataPropertyAssertion(ax) => named_individual_is(&ax.from, entity_iri),
        // InverseOf is subject-owned when the left property is the merge entity.
        Component::InverseObjectProperties(ax) => ax.0.to_string() == entity_iri,
        // N-ary / object-position-only axioms are remapped, not deleted.
        _ => false,
    }
}

fn remap_annotated(
    ac: &AnnotatedComponent<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> AnnotatedComponent<RcStr> {
    let component = remap_component(&ac.component, from, to, build);
    let ann = ac.ann.iter().map(|a| remap_annotation(a, from, to, build)).collect();
    AnnotatedComponent { component, ann }
}

fn remap_iri(
    iri: &horned_owl::model::IRI<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> horned_owl::model::IRI<RcStr> {
    if iri.to_string() == from {
        build.iri(to)
    } else {
        iri.clone()
    }
}

fn remap_component(
    c: &Component<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> Component<RcStr> {
    match c {
        Component::OntologyID(id) => Component::OntologyID(horned_owl::model::OntologyID {
            iri: id.iri.as_ref().map(|i| remap_iri(i, from, to, build)),
            viri: id.viri.as_ref().map(|i| remap_iri(i, from, to, build)),
        }),
        Component::DocIRI(doc) => {
            Component::DocIRI(horned_owl::model::DocIRI(remap_iri(&doc.0, from, to, build)))
        }
        Component::OntologyAnnotation(ann) => Component::OntologyAnnotation(
            horned_owl::model::OntologyAnnotation(remap_annotation(&ann.0, from, to, build)),
        ),
        Component::Import(imp) => {
            if imp.0.to_string() == from {
                Component::Import(horned_owl::model::Import(build.iri(to)))
            } else {
                c.clone()
            }
        }
        Component::DeclareClass(DeclareClass(cls)) => {
            Component::DeclareClass(DeclareClass(remap_class(cls, from, to, build)))
        }
        Component::DeclareObjectProperty(d) => Component::DeclareObjectProperty(
            DeclareObjectProperty(remap_object_property(&d.0, from, to, build)),
        ),
        Component::DeclareAnnotationProperty(d) => {
            Component::DeclareAnnotationProperty(horned_owl::model::DeclareAnnotationProperty(
                remap_annotation_property(&d.0, from, to, build),
            ))
        }
        Component::DeclareDataProperty(d) => Component::DeclareDataProperty(DeclareDataProperty(
            remap_data_property(&d.0, from, to, build),
        )),
        Component::DeclareNamedIndividual(d) => Component::DeclareNamedIndividual(
            DeclareNamedIndividual(remap_named_individual(&d.0, from, to, build)),
        ),
        Component::DeclareDatatype(d) => Component::DeclareDatatype(
            horned_owl::model::DeclareDatatype(remap_datatype(&d.0, from, to, build)),
        ),
        Component::SubClassOf(ax) => Component::SubClassOf(SubClassOf {
            sub: remap_class_expression(&ax.sub, from, to, build),
            sup: remap_class_expression(&ax.sup, from, to, build),
        }),
        Component::EquivalentClasses(ax) => {
            Component::EquivalentClasses(horned_owl::model::EquivalentClasses(
                ax.0.iter().map(|ce| remap_class_expression(ce, from, to, build)).collect(),
            ))
        }
        Component::DisjointClasses(ax) => {
            Component::DisjointClasses(horned_owl::model::DisjointClasses(
                ax.0.iter().map(|ce| remap_class_expression(ce, from, to, build)).collect(),
            ))
        }
        Component::DisjointUnion(ax) => Component::DisjointUnion(horned_owl::model::DisjointUnion(
            remap_class(&ax.0, from, to, build),
            ax.1.iter().map(|ce| remap_class_expression(ce, from, to, build)).collect(),
        )),
        Component::SubObjectPropertyOf(ax) => {
            Component::SubObjectPropertyOf(horned_owl::model::SubObjectPropertyOf {
                sub: match &ax.sub {
                    horned_owl::model::SubObjectPropertyExpression::ObjectPropertyChain(chain) => {
                        horned_owl::model::SubObjectPropertyExpression::ObjectPropertyChain(
                            chain.iter().map(|ope| remap_ope(ope, from, to, build)).collect(),
                        )
                    }
                    horned_owl::model::SubObjectPropertyExpression::ObjectPropertyExpression(
                        ope,
                    ) => horned_owl::model::SubObjectPropertyExpression::ObjectPropertyExpression(
                        remap_ope(ope, from, to, build),
                    ),
                },
                sup: remap_ope(&ax.sup, from, to, build),
            })
        }
        Component::EquivalentObjectProperties(ax) => {
            Component::EquivalentObjectProperties(horned_owl::model::EquivalentObjectProperties(
                ax.0.iter().map(|ope| remap_ope(ope, from, to, build)).collect(),
            ))
        }
        Component::DisjointObjectProperties(ax) => {
            Component::DisjointObjectProperties(horned_owl::model::DisjointObjectProperties(
                ax.0.iter().map(|ope| remap_ope(ope, from, to, build)).collect(),
            ))
        }
        Component::InverseObjectProperties(ax) => {
            Component::InverseObjectProperties(horned_owl::model::InverseObjectProperties(
                remap_object_property(&ax.0, from, to, build),
                remap_object_property(&ax.1, from, to, build),
            ))
        }
        Component::ObjectPropertyDomain(ax) => {
            Component::ObjectPropertyDomain(horned_owl::model::ObjectPropertyDomain {
                ope: remap_ope(&ax.ope, from, to, build),
                ce: remap_class_expression(&ax.ce, from, to, build),
            })
        }
        Component::ObjectPropertyRange(ax) => {
            Component::ObjectPropertyRange(horned_owl::model::ObjectPropertyRange {
                ope: remap_ope(&ax.ope, from, to, build),
                ce: remap_class_expression(&ax.ce, from, to, build),
            })
        }
        Component::FunctionalObjectProperty(ax) => Component::FunctionalObjectProperty(
            horned_owl::model::FunctionalObjectProperty(remap_ope(&ax.0, from, to, build)),
        ),
        Component::InverseFunctionalObjectProperty(ax) => {
            Component::InverseFunctionalObjectProperty(
                horned_owl::model::InverseFunctionalObjectProperty(remap_ope(
                    &ax.0, from, to, build,
                )),
            )
        }
        Component::ReflexiveObjectProperty(ax) => Component::ReflexiveObjectProperty(
            horned_owl::model::ReflexiveObjectProperty(remap_ope(&ax.0, from, to, build)),
        ),
        Component::IrreflexiveObjectProperty(ax) => Component::IrreflexiveObjectProperty(
            horned_owl::model::IrreflexiveObjectProperty(remap_ope(&ax.0, from, to, build)),
        ),
        Component::SymmetricObjectProperty(ax) => Component::SymmetricObjectProperty(
            horned_owl::model::SymmetricObjectProperty(remap_ope(&ax.0, from, to, build)),
        ),
        Component::AsymmetricObjectProperty(ax) => Component::AsymmetricObjectProperty(
            horned_owl::model::AsymmetricObjectProperty(remap_ope(&ax.0, from, to, build)),
        ),
        Component::TransitiveObjectProperty(ax) => Component::TransitiveObjectProperty(
            horned_owl::model::TransitiveObjectProperty(remap_ope(&ax.0, from, to, build)),
        ),
        Component::SubDataPropertyOf(ax) => {
            Component::SubDataPropertyOf(horned_owl::model::SubDataPropertyOf {
                sub: remap_data_property(&ax.sub, from, to, build),
                sup: remap_data_property(&ax.sup, from, to, build),
            })
        }
        Component::EquivalentDataProperties(ax) => {
            Component::EquivalentDataProperties(horned_owl::model::EquivalentDataProperties(
                ax.0.iter().map(|dp| remap_data_property(dp, from, to, build)).collect(),
            ))
        }
        Component::DisjointDataProperties(ax) => {
            Component::DisjointDataProperties(horned_owl::model::DisjointDataProperties(
                ax.0.iter().map(|dp| remap_data_property(dp, from, to, build)).collect(),
            ))
        }
        Component::DataPropertyDomain(ax) => {
            Component::DataPropertyDomain(horned_owl::model::DataPropertyDomain {
                dp: remap_data_property(&ax.dp, from, to, build),
                ce: remap_class_expression(&ax.ce, from, to, build),
            })
        }
        Component::DataPropertyRange(ax) => {
            Component::DataPropertyRange(horned_owl::model::DataPropertyRange {
                dp: remap_data_property(&ax.dp, from, to, build),
                dr: remap_data_range(&ax.dr, from, to, build),
            })
        }
        Component::FunctionalDataProperty(ax) => Component::FunctionalDataProperty(
            horned_owl::model::FunctionalDataProperty(remap_data_property(&ax.0, from, to, build)),
        ),
        Component::DatatypeDefinition(ax) => {
            Component::DatatypeDefinition(horned_owl::model::DatatypeDefinition {
                kind: remap_datatype(&ax.kind, from, to, build),
                range: remap_data_range(&ax.range, from, to, build),
            })
        }
        Component::HasKey(ax) => Component::HasKey(horned_owl::model::HasKey {
            ce: remap_class_expression(&ax.ce, from, to, build),
            vpe: ax.vpe.iter().map(|pe| remap_property_expression(pe, from, to, build)).collect(),
        }),
        Component::SameIndividual(ax) => {
            Component::SameIndividual(horned_owl::model::SameIndividual(
                ax.0.iter().map(|i| remap_individual(i, from, to, build)).collect(),
            ))
        }
        Component::DifferentIndividuals(ax) => {
            Component::DifferentIndividuals(horned_owl::model::DifferentIndividuals(
                ax.0.iter().map(|i| remap_individual(i, from, to, build)).collect(),
            ))
        }
        Component::ClassAssertion(ax) => Component::ClassAssertion(ClassAssertion {
            ce: remap_class_expression(&ax.ce, from, to, build),
            i: remap_individual(&ax.i, from, to, build),
        }),
        Component::ObjectPropertyAssertion(ax) => {
            Component::ObjectPropertyAssertion(ObjectPropertyAssertion {
                ope: remap_ope(&ax.ope, from, to, build),
                from: remap_individual(&ax.from, from, to, build),
                to: remap_individual(&ax.to, from, to, build),
            })
        }
        Component::NegativeObjectPropertyAssertion(ax) => {
            Component::NegativeObjectPropertyAssertion(
                horned_owl::model::NegativeObjectPropertyAssertion {
                    ope: remap_ope(&ax.ope, from, to, build),
                    from: remap_individual(&ax.from, from, to, build),
                    to: remap_individual(&ax.to, from, to, build),
                },
            )
        }
        Component::DataPropertyAssertion(ax) => {
            Component::DataPropertyAssertion(DataPropertyAssertion {
                dp: remap_data_property(&ax.dp, from, to, build),
                from: remap_individual(&ax.from, from, to, build),
                to: remap_literal(&ax.to, from, to, build),
            })
        }
        Component::NegativeDataPropertyAssertion(ax) => Component::NegativeDataPropertyAssertion(
            horned_owl::model::NegativeDataPropertyAssertion {
                dp: remap_data_property(&ax.dp, from, to, build),
                from: remap_individual(&ax.from, from, to, build),
                to: remap_literal(&ax.to, from, to, build),
            },
        ),
        Component::AnnotationAssertion(ax) => Component::AnnotationAssertion(AnnotationAssertion {
            subject: remap_annotation_subject(&ax.subject, from, to, build),
            ann: remap_annotation(&ax.ann, from, to, build),
        }),
        Component::SubAnnotationPropertyOf(ax) => {
            Component::SubAnnotationPropertyOf(horned_owl::model::SubAnnotationPropertyOf {
                sub: remap_annotation_property(&ax.sub, from, to, build),
                sup: remap_annotation_property(&ax.sup, from, to, build),
            })
        }
        Component::AnnotationPropertyDomain(ax) => {
            Component::AnnotationPropertyDomain(horned_owl::model::AnnotationPropertyDomain {
                ap: remap_annotation_property(&ax.ap, from, to, build),
                iri: remap_iri(&ax.iri, from, to, build),
            })
        }
        Component::AnnotationPropertyRange(ax) => {
            Component::AnnotationPropertyRange(horned_owl::model::AnnotationPropertyRange {
                ap: remap_annotation_property(&ax.ap, from, to, build),
                iri: remap_iri(&ax.iri, from, to, build),
            })
        }
        Component::Rule(rule) => Component::Rule(horned_owl::model::Rule {
            head: rule.head.iter().map(|a| remap_atom(a, from, to, build)).collect(),
            body: rule.body.iter().map(|a| remap_atom(a, from, to, build)).collect(),
        }),
    }
}

fn remap_annotation(
    ann: &Annotation<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> Annotation<RcStr> {
    Annotation {
        ap: remap_annotation_property(&ann.ap, from, to, build),
        av: match &ann.av {
            AnnotationValue::IRI(iri) if iri.to_string() == from => {
                AnnotationValue::IRI(build.iri(to))
            }
            AnnotationValue::Literal(lit) => {
                AnnotationValue::Literal(remap_literal(lit, from, to, build))
            }
            other => other.clone(),
        },
    }
}

fn remap_annotation_subject(
    subject: &AnnotationSubject<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> AnnotationSubject<RcStr> {
    match subject {
        AnnotationSubject::IRI(iri) if iri.to_string() == from => {
            AnnotationSubject::IRI(build.iri(to))
        }
        other => other.clone(),
    }
}

fn remap_annotation_property(
    ap: &horned_owl::model::AnnotationProperty<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> horned_owl::model::AnnotationProperty<RcStr> {
    if ap.to_string() == from {
        build.annotation_property(to)
    } else {
        ap.clone()
    }
}

fn remap_class(cls: &Class<RcStr>, from: &str, to: &str, build: &Build<RcStr>) -> Class<RcStr> {
    if cls.0.to_string() == from {
        build.class(to)
    } else {
        cls.clone()
    }
}

fn remap_object_property(
    p: &horned_owl::model::ObjectProperty<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> horned_owl::model::ObjectProperty<RcStr> {
    if p.to_string() == from {
        build.object_property(to)
    } else {
        p.clone()
    }
}

fn remap_data_property(
    p: &horned_owl::model::DataProperty<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> horned_owl::model::DataProperty<RcStr> {
    if p.to_string() == from {
        build.data_property(to)
    } else {
        p.clone()
    }
}

fn remap_datatype(
    dt: &horned_owl::model::Datatype<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> horned_owl::model::Datatype<RcStr> {
    if dt.to_string() == from {
        build.datatype(to)
    } else {
        dt.clone()
    }
}

fn remap_named_individual(
    i: &NamedIndividual<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> NamedIndividual<RcStr> {
    if i.0.to_string() == from {
        build.named_individual(to)
    } else {
        i.clone()
    }
}

fn remap_individual(
    i: &Individual<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> Individual<RcStr> {
    match i {
        Individual::Named(n) => Individual::Named(remap_named_individual(n, from, to, build)),
        other => other.clone(),
    }
}

fn remap_ope(
    ope: &ObjectPropertyExpression<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> ObjectPropertyExpression<RcStr> {
    match ope {
        ObjectPropertyExpression::ObjectProperty(p) => {
            ObjectPropertyExpression::ObjectProperty(remap_object_property(p, from, to, build))
        }
        ObjectPropertyExpression::InverseObjectProperty(p) => {
            ObjectPropertyExpression::InverseObjectProperty(remap_object_property(
                p, from, to, build,
            ))
        }
    }
}

fn remap_property_expression(
    pe: &PropertyExpression<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> PropertyExpression<RcStr> {
    match pe {
        PropertyExpression::ObjectPropertyExpression(ope) => {
            PropertyExpression::ObjectPropertyExpression(remap_ope(ope, from, to, build))
        }
        PropertyExpression::DataProperty(dp) => {
            PropertyExpression::DataProperty(remap_data_property(dp, from, to, build))
        }
        PropertyExpression::AnnotationProperty(ap) => {
            PropertyExpression::AnnotationProperty(remap_annotation_property(ap, from, to, build))
        }
    }
}

fn remap_literal(
    lit: &Literal<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> Literal<RcStr> {
    match lit {
        Literal::Datatype { literal, datatype_iri } => Literal::Datatype {
            literal: literal.clone(),
            datatype_iri: remap_iri(datatype_iri, from, to, build),
        },
        other => other.clone(),
    }
}

fn remap_variable(
    v: &Variable<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> Variable<RcStr> {
    if v.0.to_string() == from {
        build.variable(to)
    } else {
        v.clone()
    }
}

fn remap_iargument(
    arg: &IArgument<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> IArgument<RcStr> {
    match arg {
        IArgument::Individual(i) => IArgument::Individual(remap_individual(i, from, to, build)),
        IArgument::Variable(v) => IArgument::Variable(remap_variable(v, from, to, build)),
    }
}

fn remap_dargument(
    arg: &DArgument<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> DArgument<RcStr> {
    match arg {
        DArgument::Literal(l) => DArgument::Literal(remap_literal(l, from, to, build)),
        DArgument::Variable(v) => DArgument::Variable(remap_variable(v, from, to, build)),
    }
}

fn remap_atom(atom: &Atom<RcStr>, from: &str, to: &str, build: &Build<RcStr>) -> Atom<RcStr> {
    match atom {
        Atom::BuiltInAtom { pred, args } => Atom::BuiltInAtom {
            pred: remap_iri(pred, from, to, build),
            args: args.iter().map(|a| remap_dargument(a, from, to, build)).collect(),
        },
        Atom::ClassAtom { pred, arg } => Atom::ClassAtom {
            pred: remap_class_expression(pred, from, to, build),
            arg: remap_iargument(arg, from, to, build),
        },
        Atom::DataPropertyAtom { pred, args } => Atom::DataPropertyAtom {
            pred: remap_data_property(pred, from, to, build),
            args: (
                remap_dargument(&args.0, from, to, build),
                remap_dargument(&args.1, from, to, build),
            ),
        },
        Atom::DataRangeAtom { pred, arg } => Atom::DataRangeAtom {
            pred: remap_data_range(pred, from, to, build),
            arg: remap_dargument(arg, from, to, build),
        },
        Atom::DifferentIndividualsAtom(a, b) => Atom::DifferentIndividualsAtom(
            remap_iargument(a, from, to, build),
            remap_iargument(b, from, to, build),
        ),
        Atom::ObjectPropertyAtom { pred, args } => Atom::ObjectPropertyAtom {
            pred: remap_ope(pred, from, to, build),
            args: (
                remap_iargument(&args.0, from, to, build),
                remap_iargument(&args.1, from, to, build),
            ),
        },
        Atom::SameIndividualAtom(a, b) => Atom::SameIndividualAtom(
            remap_iargument(a, from, to, build),
            remap_iargument(b, from, to, build),
        ),
    }
}

fn remap_class_expression(
    ce: &ClassExpression<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> ClassExpression<RcStr> {
    match ce {
        ClassExpression::Class(c) => ClassExpression::Class(remap_class(c, from, to, build)),
        ClassExpression::ObjectIntersectionOf(v) => ClassExpression::ObjectIntersectionOf(
            v.iter().map(|e| remap_class_expression(e, from, to, build)).collect(),
        ),
        ClassExpression::ObjectUnionOf(v) => ClassExpression::ObjectUnionOf(
            v.iter().map(|e| remap_class_expression(e, from, to, build)).collect(),
        ),
        ClassExpression::ObjectComplementOf(inner) => ClassExpression::ObjectComplementOf(
            Box::new(remap_class_expression(inner, from, to, build)),
        ),
        ClassExpression::ObjectSomeValuesFrom { ope, bce } => {
            ClassExpression::ObjectSomeValuesFrom {
                ope: remap_ope(ope, from, to, build),
                bce: Box::new(remap_class_expression(bce, from, to, build)),
            }
        }
        ClassExpression::ObjectAllValuesFrom { ope, bce } => ClassExpression::ObjectAllValuesFrom {
            ope: remap_ope(ope, from, to, build),
            bce: Box::new(remap_class_expression(bce, from, to, build)),
        },
        ClassExpression::ObjectHasValue { ope, i } => ClassExpression::ObjectHasValue {
            ope: remap_ope(ope, from, to, build),
            i: remap_individual(i, from, to, build),
        },
        ClassExpression::ObjectHasSelf(ope) => {
            ClassExpression::ObjectHasSelf(remap_ope(ope, from, to, build))
        }
        ClassExpression::ObjectOneOf(inds) => ClassExpression::ObjectOneOf(
            inds.iter().map(|i| remap_individual(i, from, to, build)).collect(),
        ),
        ClassExpression::ObjectMinCardinality { n, ope, bce } => {
            ClassExpression::ObjectMinCardinality {
                n: *n,
                ope: remap_ope(ope, from, to, build),
                bce: Box::new(remap_class_expression(bce, from, to, build)),
            }
        }
        ClassExpression::ObjectMaxCardinality { n, ope, bce } => {
            ClassExpression::ObjectMaxCardinality {
                n: *n,
                ope: remap_ope(ope, from, to, build),
                bce: Box::new(remap_class_expression(bce, from, to, build)),
            }
        }
        ClassExpression::ObjectExactCardinality { n, ope, bce } => {
            ClassExpression::ObjectExactCardinality {
                n: *n,
                ope: remap_ope(ope, from, to, build),
                bce: Box::new(remap_class_expression(bce, from, to, build)),
            }
        }
        ClassExpression::DataSomeValuesFrom { dp, dr } => ClassExpression::DataSomeValuesFrom {
            dp: remap_data_property(dp, from, to, build),
            dr: remap_data_range(dr, from, to, build),
        },
        ClassExpression::DataAllValuesFrom { dp, dr } => ClassExpression::DataAllValuesFrom {
            dp: remap_data_property(dp, from, to, build),
            dr: remap_data_range(dr, from, to, build),
        },
        ClassExpression::DataHasValue { dp, l } => ClassExpression::DataHasValue {
            dp: remap_data_property(dp, from, to, build),
            l: remap_literal(l, from, to, build),
        },
        ClassExpression::DataMinCardinality { n, dp, dr } => ClassExpression::DataMinCardinality {
            n: *n,
            dp: remap_data_property(dp, from, to, build),
            dr: remap_data_range(dr, from, to, build),
        },
        ClassExpression::DataMaxCardinality { n, dp, dr } => ClassExpression::DataMaxCardinality {
            n: *n,
            dp: remap_data_property(dp, from, to, build),
            dr: remap_data_range(dr, from, to, build),
        },
        ClassExpression::DataExactCardinality { n, dp, dr } => {
            ClassExpression::DataExactCardinality {
                n: *n,
                dp: remap_data_property(dp, from, to, build),
                dr: remap_data_range(dr, from, to, build),
            }
        }
    }
}

fn remap_data_range(
    dr: &DataRange<RcStr>,
    from: &str,
    to: &str,
    build: &Build<RcStr>,
) -> DataRange<RcStr> {
    match dr {
        DataRange::Datatype(dt) => DataRange::Datatype(remap_datatype(dt, from, to, build)),
        DataRange::DataIntersectionOf(v) => DataRange::DataIntersectionOf(
            v.iter().map(|d| remap_data_range(d, from, to, build)).collect(),
        ),
        DataRange::DataUnionOf(v) => {
            DataRange::DataUnionOf(v.iter().map(|d| remap_data_range(d, from, to, build)).collect())
        }
        DataRange::DataComplementOf(inner) => {
            DataRange::DataComplementOf(Box::new(remap_data_range(inner, from, to, build)))
        }
        DataRange::DataOneOf(lits) => {
            DataRange::DataOneOf(lits.iter().map(|l| remap_literal(l, from, to, build)).collect())
        }
        DataRange::DatatypeRestriction(dt, facets) => DataRange::DatatypeRestriction(
            remap_datatype(dt, from, to, build),
            facets
                .iter()
                .map(|f| FacetRestriction {
                    f: f.f.clone(),
                    l: remap_literal(&f.l, from, to, build),
                })
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remaps_class_in_rdf_xml() {
        let src = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:owl="http://www.w3.org/2002/07/owl#"
         xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#">
  <owl:Class rdf:about="http://example.org#Person"/>
  <owl:Class rdf:about="http://example.org#Patient">
    <rdfs:subClassOf rdf:resource="http://example.org#Person"/>
  </owl:Class>
</rdf:RDF>
"#;
        let out = remap_entity_iri_in_xml_text(
            src,
            "rdfxml",
            "http://example.org#Person",
            "http://example.org#Human",
            &BTreeMap::new(),
        )
        .expect("remap");
        assert!(out.contains("http://example.org#Human"), "{out}");
        assert!(!out.contains("http://example.org#Person"), "{out}");
        assert!(out.contains("http://example.org#Patient"), "{out}");
    }

    #[test]
    fn merge_deletes_subject_axioms_then_remaps_refs() {
        // Keep + Merge(with label + parent) + Child⊑Merge — merge must not absorb
        // Merge's label/parent onto Keep (#369).
        let src = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:owl="http://www.w3.org/2002/07/owl#"
         xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#">
  <owl:Class rdf:about="http://example.org#Keep">
    <rdfs:label>Keep</rdfs:label>
  </owl:Class>
  <owl:Class rdf:about="http://example.org#Merge">
    <rdfs:label>MergeLabel</rdfs:label>
    <rdfs:subClassOf rdf:resource="http://example.org#Parent"/>
  </owl:Class>
  <owl:Class rdf:about="http://example.org#Parent"/>
  <owl:Class rdf:about="http://example.org#Child">
    <rdfs:subClassOf rdf:resource="http://example.org#Merge"/>
  </owl:Class>
</rdf:RDF>
"#;
        let out = merge_entity_iri_in_xml_text(
            src,
            "rdfxml",
            "http://example.org#Keep",
            "http://example.org#Merge",
            &BTreeMap::new(),
        )
        .expect("merge");
        assert!(out.contains("http://example.org#Keep"), "{out}");
        assert!(out.contains("http://example.org#Child"), "{out}");
        assert!(!out.contains("http://example.org#Merge"), "merge IRI must be gone: {out}");
        assert!(!out.contains("MergeLabel"), "merge-owned label must not absorb onto Keep: {out}");
        // Child⊑Merge remapped to Child⊑Keep; Keep must not gain Parent via absorb.
        let keep_block_has_parent = out.contains("rdf:about=\"http://example.org#Keep\"")
            && out
                .split("rdf:about=\"http://example.org#Keep\"")
                .nth(1)
                .map(|rest| {
                    let end = rest.find("<owl:Class").unwrap_or(rest.len());
                    rest[..end].contains("http://example.org#Parent")
                })
                .unwrap_or(false);
        assert!(!keep_block_has_parent, "Keep must not absorb Merge⊑Parent: {out}");
        assert!(
            out.contains("http://example.org#Child")
                && out.contains("rdf:resource=\"http://example.org#Keep\""),
            "Child must reference Keep: {out}"
        );
    }

    #[test]
    fn remaps_inverse_functional_sameas_haskey() {
        let src = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns:owl="http://www.w3.org/2002/07/owl#"
         xmlns:rdfs="http://www.w3.org/2000/01/rdf-schema#">
  <owl:ObjectProperty rdf:about="http://example.org#P">
    <rdf:type rdf:resource="http://www.w3.org/2002/07/owl#FunctionalProperty"/>
    <owl:inverseOf rdf:resource="http://example.org#Q"/>
  </owl:ObjectProperty>
  <owl:ObjectProperty rdf:about="http://example.org#Q"/>
  <owl:Class rdf:about="http://example.org#C">
    <owl:hasKey rdf:parseType="Collection">
      <rdf:Description rdf:about="http://example.org#P"/>
    </owl:hasKey>
  </owl:Class>
  <owl:NamedIndividual rdf:about="http://example.org#a">
    <owl:sameAs rdf:resource="http://example.org#b"/>
  </owl:NamedIndividual>
  <owl:NamedIndividual rdf:about="http://example.org#b"/>
</rdf:RDF>
"#;
        let out = remap_entity_iri_in_xml_text(
            src,
            "rdfxml",
            "http://example.org#P",
            "http://example.org#RenamedP",
            &BTreeMap::new(),
        )
        .expect("remap P");
        assert!(out.contains("http://example.org#RenamedP"), "{out}");
        assert!(
            !out.contains("rdf:about=\"http://example.org#P\"")
                && !out.contains("rdf:resource=\"http://example.org#P\""),
            "old P IRI must not remain as about/resource: {out}"
        );
    }

    #[test]
    fn remaps_nested_datatype_restriction() {
        // Build ontology in-memory so we do not depend on RDF/XML DatatypeRestriction parsing.
        let build = Build::new_rc();
        let mut ont = ComponentMappedOntology::<RcStr, RcAnnotatedComponent>::default();
        let my_dt = build.datatype("http://example.org#MyDt");
        ont.insert(Component::DeclareDatatype(horned_owl::model::DeclareDatatype(my_dt.clone())));
        ont.insert(Component::SubClassOf(SubClassOf {
            sub: ClassExpression::Class(build.class("http://example.org#Restricted")),
            sup: ClassExpression::DataSomeValuesFrom {
                dp: build.data_property("http://example.org#dp"),
                dr: DataRange::DatatypeRestriction(my_dt, Vec::new()),
            },
        }));
        let n = remap_entity_iri(&mut ont, "http://example.org#MyDt", "http://example.org#NewDt")
            .expect("remap");
        assert!(n >= 1, "expected at least one component rewritten");
        let mut saw_new = false;
        let mut saw_old = false;
        for ac in ont.i() {
            match &ac.component {
                Component::DeclareDatatype(d) => {
                    let s = d.0.to_string();
                    if s.contains("NewDt") {
                        saw_new = true;
                    }
                    if s.contains("MyDt") {
                        saw_old = true;
                    }
                }
                Component::SubClassOf(ax) => {
                    let ClassExpression::DataSomeValuesFrom { dr, .. } = &ax.sup else {
                        continue;
                    };
                    let DataRange::DatatypeRestriction(dt, _) = dr else {
                        continue;
                    };
                    let s = dt.to_string();
                    if s.contains("NewDt") {
                        saw_new = true;
                    }
                    if s.contains("MyDt") {
                        saw_old = true;
                    }
                }
                _ => {}
            }
        }
        assert!(saw_new, "expected NewDt after remap");
        assert!(!saw_old, "MyDt must not remain");
    }
}
