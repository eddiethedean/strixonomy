use crate::change::SemanticChange;
use crate::error::{EditError, Result};
use strixonomy_obo::OboPatchOp;
use strixonomy_owl::PatchOp;

pub fn invert_patch_op(op: &PatchOp) -> Result<PatchOp> {
    Ok(match op {
        PatchOp::AddPrefix { prefix, namespace_iri: _ } => {
            PatchOp::RemovePrefix { prefix: prefix.clone() }
        }
        PatchOp::RemovePrefix { .. } => {
            return Err(EditError::NotInvertible(
                "remove_prefix inverse requires prior namespace IRI".into(),
            ));
        }
        PatchOp::SetPrefix { .. } => {
            return Err(EditError::NotInvertible("set_prefix requires prior value".into()));
        }
        PatchOp::SetOntologyIri { .. } | PatchOp::SetVersionIri { .. } => {
            return Err(EditError::NotInvertible("ontology IRI set requires prior value".into()));
        }
        PatchOp::AddOntologyAnnotation { ontology_iri, predicate, value } => {
            PatchOp::RemoveOntologyAnnotation {
                ontology_iri: ontology_iri.clone(),
                predicate: predicate.clone(),
                value: value.clone(),
            }
        }
        PatchOp::RemoveOntologyAnnotation { ontology_iri, predicate, value } => {
            PatchOp::AddOntologyAnnotation {
                ontology_iri: ontology_iri.clone(),
                predicate: predicate.clone(),
                value: value.clone(),
            }
        }
        PatchOp::CreateEntity { entity_iri, kind: _ } => {
            PatchOp::DeleteEntity { entity_iri: entity_iri.clone() }
        }
        PatchOp::DeleteEntity { entity_iri: _ } => {
            return Err(EditError::NotInvertible(
                "delete_entity inverse requires entity kind".into(),
            ));
        }
        PatchOp::SetLabel { .. }
        | PatchOp::SetComment { .. }
        | PatchOp::SetEquivalentClass { .. } => {
            return Err(EditError::NotInvertible("set_* requires prior value".into()));
        }
        PatchOp::AddLabel { entity_iri, value } => {
            PatchOp::RemoveLabel { entity_iri: entity_iri.clone(), value: value.clone() }
        }
        PatchOp::RemoveLabel { entity_iri, value } => {
            PatchOp::AddLabel { entity_iri: entity_iri.clone(), value: value.clone() }
        }
        PatchOp::AddComment { entity_iri, value } => {
            PatchOp::RemoveComment { entity_iri: entity_iri.clone(), value: value.clone() }
        }
        PatchOp::RemoveComment { entity_iri, value } => {
            PatchOp::AddComment { entity_iri: entity_iri.clone(), value: value.clone() }
        }
        PatchOp::AddSubClassOf { entity_iri, parent_iri } => PatchOp::RemoveSubClassOf {
            entity_iri: entity_iri.clone(),
            parent_iri: parent_iri.clone(),
        },
        PatchOp::RemoveSubClassOf { entity_iri, parent_iri } => PatchOp::AddSubClassOf {
            entity_iri: entity_iri.clone(),
            parent_iri: parent_iri.clone(),
        },
        PatchOp::AddComplexSubClassOf { entity_iri, manchester } => {
            PatchOp::RemoveComplexSubClassOf {
                entity_iri: entity_iri.clone(),
                manchester: manchester.clone(),
            }
        }
        PatchOp::RemoveComplexSubClassOf { entity_iri, manchester } => {
            PatchOp::AddComplexSubClassOf {
                entity_iri: entity_iri.clone(),
                manchester: manchester.clone(),
            }
        }
        PatchOp::AddEquivalentClass { entity_iri, manchester } => PatchOp::RemoveEquivalentClass {
            entity_iri: entity_iri.clone(),
            manchester: manchester.clone(),
        },
        PatchOp::RemoveEquivalentClass { entity_iri, manchester } => PatchOp::AddEquivalentClass {
            entity_iri: entity_iri.clone(),
            manchester: manchester.clone(),
        },
        PatchOp::SetDeprecated { entity_iri, value } => {
            if !*value {
                return Err(EditError::NotInvertible(
                    "set_deprecated false inverse would invent deprecation without prior state"
                        .into(),
                ));
            }
            PatchOp::SetDeprecated { entity_iri: entity_iri.clone(), value: false }
        }
        PatchOp::AddDisjointClass { entity_iri, other_iri } => PatchOp::RemoveDisjointClass {
            entity_iri: entity_iri.clone(),
            other_iri: other_iri.clone(),
        },
        PatchOp::RemoveDisjointClass { entity_iri, other_iri } => PatchOp::AddDisjointClass {
            entity_iri: entity_iri.clone(),
            other_iri: other_iri.clone(),
        },
        PatchOp::AddImport { ontology_iri, import_iri } => PatchOp::RemoveImport {
            ontology_iri: ontology_iri.clone(),
            import_iri: import_iri.clone(),
        },
        PatchOp::RemoveImport { ontology_iri, import_iri } => PatchOp::AddImport {
            ontology_iri: ontology_iri.clone(),
            import_iri: import_iri.clone(),
        },
        PatchOp::AddDomain { entity_iri, class_iri } => {
            PatchOp::RemoveDomain { entity_iri: entity_iri.clone(), class_iri: class_iri.clone() }
        }
        PatchOp::RemoveDomain { entity_iri, class_iri } => {
            PatchOp::AddDomain { entity_iri: entity_iri.clone(), class_iri: class_iri.clone() }
        }
        PatchOp::AddRange { entity_iri, range_iri } => {
            PatchOp::RemoveRange { entity_iri: entity_iri.clone(), range_iri: range_iri.clone() }
        }
        PatchOp::RemoveRange { entity_iri, range_iri } => {
            PatchOp::AddRange { entity_iri: entity_iri.clone(), range_iri: range_iri.clone() }
        }
        PatchOp::SetFunctional { .. }
        | PatchOp::SetInverseFunctional { .. }
        | PatchOp::SetTransitive { .. }
        | PatchOp::SetSymmetric { .. }
        | PatchOp::SetAsymmetric { .. }
        | PatchOp::SetReflexive { .. }
        | PatchOp::SetIrreflexive { .. } => {
            let value = match op {
                PatchOp::SetFunctional { value, .. }
                | PatchOp::SetInverseFunctional { value, .. }
                | PatchOp::SetTransitive { value, .. }
                | PatchOp::SetSymmetric { value, .. }
                | PatchOp::SetAsymmetric { value, .. }
                | PatchOp::SetReflexive { value, .. }
                | PatchOp::SetIrreflexive { value, .. } => *value,
                _ => unreachable!(),
            };
            if !value {
                return Err(EditError::NotInvertible(
                    "clearing a property characteristic is not invertible without prior state"
                        .into(),
                ));
            }
            match op {
                PatchOp::SetFunctional { entity_iri, .. } => {
                    PatchOp::SetFunctional { entity_iri: entity_iri.clone(), value: false }
                }
                PatchOp::SetInverseFunctional { entity_iri, .. } => {
                    PatchOp::SetInverseFunctional { entity_iri: entity_iri.clone(), value: false }
                }
                PatchOp::SetTransitive { entity_iri, .. } => {
                    PatchOp::SetTransitive { entity_iri: entity_iri.clone(), value: false }
                }
                PatchOp::SetSymmetric { entity_iri, .. } => {
                    PatchOp::SetSymmetric { entity_iri: entity_iri.clone(), value: false }
                }
                PatchOp::SetAsymmetric { entity_iri, .. } => {
                    PatchOp::SetAsymmetric { entity_iri: entity_iri.clone(), value: false }
                }
                PatchOp::SetReflexive { entity_iri, .. } => {
                    PatchOp::SetReflexive { entity_iri: entity_iri.clone(), value: false }
                }
                PatchOp::SetIrreflexive { entity_iri, .. } => {
                    PatchOp::SetIrreflexive { entity_iri: entity_iri.clone(), value: false }
                }
                _ => unreachable!(),
            }
        }
        PatchOp::AddPropertyChain { entity_iri, properties } => PatchOp::RemovePropertyChain {
            entity_iri: entity_iri.clone(),
            properties: properties.clone(),
        },
        PatchOp::RemovePropertyChain { entity_iri, properties } => PatchOp::AddPropertyChain {
            entity_iri: entity_iri.clone(),
            properties: properties.clone(),
        },
        PatchOp::AddClassAssertion { entity_iri, class_iri } => PatchOp::RemoveClassAssertion {
            entity_iri: entity_iri.clone(),
            class_iri: class_iri.clone(),
        },
        PatchOp::RemoveClassAssertion { entity_iri, class_iri } => PatchOp::AddClassAssertion {
            entity_iri: entity_iri.clone(),
            class_iri: class_iri.clone(),
        },
        PatchOp::AddObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            PatchOp::RemoveObjectPropertyAssertion {
                entity_iri: entity_iri.clone(),
                property_iri: property_iri.clone(),
                target_iri: target_iri.clone(),
            }
        }
        PatchOp::RemoveObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            PatchOp::AddObjectPropertyAssertion {
                entity_iri: entity_iri.clone(),
                property_iri: property_iri.clone(),
                target_iri: target_iri.clone(),
            }
        }
        PatchOp::AddDataPropertyAssertion { entity_iri, property_iri, value } => {
            PatchOp::RemoveDataPropertyAssertion {
                entity_iri: entity_iri.clone(),
                property_iri: property_iri.clone(),
                value: value.clone(),
            }
        }
        PatchOp::RemoveDataPropertyAssertion { entity_iri, property_iri, value } => {
            PatchOp::AddDataPropertyAssertion {
                entity_iri: entity_iri.clone(),
                property_iri: property_iri.clone(),
                value: value.clone(),
            }
        }
        PatchOp::AddAnnotation { entity_iri, predicate, value } => PatchOp::RemoveAnnotation {
            entity_iri: entity_iri.clone(),
            predicate: predicate.clone(),
            value: value.clone(),
        },
        PatchOp::RemoveAnnotation { entity_iri, predicate, value } => PatchOp::AddAnnotation {
            entity_iri: entity_iri.clone(),
            predicate: predicate.clone(),
            value: value.clone(),
        },
        PatchOp::AddHasKey { class_iri, properties } => {
            PatchOp::RemoveHasKey { class_iri: class_iri.clone(), properties: properties.clone() }
        }
        PatchOp::RemoveHasKey { class_iri, properties } => {
            PatchOp::AddHasKey { class_iri: class_iri.clone(), properties: properties.clone() }
        }
        PatchOp::AddDisjointUnion { class_iri, members } => {
            PatchOp::RemoveDisjointUnion { class_iri: class_iri.clone(), members: members.clone() }
        }
        PatchOp::RemoveDisjointUnion { class_iri, members } => {
            PatchOp::AddDisjointUnion { class_iri: class_iri.clone(), members: members.clone() }
        }
        PatchOp::AddInverseObjectProperties { property_iri, inverse_iri } => {
            PatchOp::RemoveInverseObjectProperties {
                property_iri: property_iri.clone(),
                inverse_iri: inverse_iri.clone(),
            }
        }
        PatchOp::RemoveInverseObjectProperties { property_iri, inverse_iri } => {
            PatchOp::AddInverseObjectProperties {
                property_iri: property_iri.clone(),
                inverse_iri: inverse_iri.clone(),
            }
        }
        PatchOp::AddEquivalentObjectProperties { properties } => {
            PatchOp::RemoveEquivalentObjectProperties { properties: properties.clone() }
        }
        PatchOp::RemoveEquivalentObjectProperties { properties } => {
            PatchOp::AddEquivalentObjectProperties { properties: properties.clone() }
        }
        PatchOp::AddDisjointObjectProperties { properties } => {
            PatchOp::RemoveDisjointObjectProperties { properties: properties.clone() }
        }
        PatchOp::RemoveDisjointObjectProperties { properties } => {
            PatchOp::AddDisjointObjectProperties { properties: properties.clone() }
        }
        PatchOp::AddEquivalentDataProperties { properties } => {
            PatchOp::RemoveEquivalentDataProperties { properties: properties.clone() }
        }
        PatchOp::RemoveEquivalentDataProperties { properties } => {
            PatchOp::AddEquivalentDataProperties { properties: properties.clone() }
        }
        PatchOp::AddDisjointDataProperties { properties } => {
            PatchOp::RemoveDisjointDataProperties { properties: properties.clone() }
        }
        PatchOp::RemoveDisjointDataProperties { properties } => {
            PatchOp::AddDisjointDataProperties { properties: properties.clone() }
        }
        PatchOp::AddSubObjectPropertyOf { property_iri, parent_iri } => {
            PatchOp::RemoveSubObjectPropertyOf {
                property_iri: property_iri.clone(),
                parent_iri: parent_iri.clone(),
            }
        }
        PatchOp::RemoveSubObjectPropertyOf { property_iri, parent_iri } => {
            PatchOp::AddSubObjectPropertyOf {
                property_iri: property_iri.clone(),
                parent_iri: parent_iri.clone(),
            }
        }
        PatchOp::AddSubDataPropertyOf { property_iri, parent_iri } => {
            PatchOp::RemoveSubDataPropertyOf {
                property_iri: property_iri.clone(),
                parent_iri: parent_iri.clone(),
            }
        }
        PatchOp::RemoveSubDataPropertyOf { property_iri, parent_iri } => {
            PatchOp::AddSubDataPropertyOf {
                property_iri: property_iri.clone(),
                parent_iri: parent_iri.clone(),
            }
        }
        PatchOp::AddNegativeObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            PatchOp::RemoveNegativeObjectPropertyAssertion {
                entity_iri: entity_iri.clone(),
                property_iri: property_iri.clone(),
                target_iri: target_iri.clone(),
            }
        }
        PatchOp::RemoveNegativeObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            PatchOp::AddNegativeObjectPropertyAssertion {
                entity_iri: entity_iri.clone(),
                property_iri: property_iri.clone(),
                target_iri: target_iri.clone(),
            }
        }
        PatchOp::AddNegativeDataPropertyAssertion { entity_iri, property_iri, value } => {
            PatchOp::RemoveNegativeDataPropertyAssertion {
                entity_iri: entity_iri.clone(),
                property_iri: property_iri.clone(),
                value: value.clone(),
            }
        }
        PatchOp::RemoveNegativeDataPropertyAssertion { entity_iri, property_iri, value } => {
            PatchOp::AddNegativeDataPropertyAssertion {
                entity_iri: entity_iri.clone(),
                property_iri: property_iri.clone(),
                value: value.clone(),
            }
        }
        PatchOp::AddSameIndividual { individuals } => {
            PatchOp::RemoveSameIndividual { individuals: individuals.clone() }
        }
        PatchOp::RemoveSameIndividual { individuals } => {
            PatchOp::AddSameIndividual { individuals: individuals.clone() }
        }
        PatchOp::AddDifferentIndividuals { individuals } => {
            PatchOp::RemoveDifferentIndividuals { individuals: individuals.clone() }
        }
        PatchOp::RemoveDifferentIndividuals { individuals } => {
            PatchOp::AddDifferentIndividuals { individuals: individuals.clone() }
        }
        PatchOp::AddDatatypeDefinition { datatype_iri, manchester } => {
            PatchOp::RemoveDatatypeDefinition {
                datatype_iri: datatype_iri.clone(),
                manchester: manchester.clone(),
            }
        }
        PatchOp::RemoveDatatypeDefinition { datatype_iri, manchester } => {
            PatchOp::AddDatatypeDefinition {
                datatype_iri: datatype_iri.clone(),
                manchester: manchester.clone(),
            }
        }
        PatchOp::AddAxiomAnnotation { axiom_op, subject_iri, related_iri, predicate, value } => {
            PatchOp::RemoveAxiomAnnotation {
                axiom_op: axiom_op.clone(),
                subject_iri: subject_iri.clone(),
                related_iri: related_iri.clone(),
                predicate: predicate.clone(),
                value: value.clone(),
            }
        }
        PatchOp::RemoveAxiomAnnotation { axiom_op, subject_iri, related_iri, predicate, value } => {
            PatchOp::AddAxiomAnnotation {
                axiom_op: axiom_op.clone(),
                subject_iri: subject_iri.clone(),
                related_iri: related_iri.clone(),
                predicate: predicate.clone(),
                value: value.clone(),
            }
        }
        PatchOp::AddSwrlRule { ontology_iri, rule_json } => PatchOp::RemoveSwrlRule {
            ontology_iri: ontology_iri.clone(),
            rule_json: rule_json.clone(),
        },
        PatchOp::RemoveSwrlRule { ontology_iri, rule_json } => PatchOp::AddSwrlRule {
            ontology_iri: ontology_iri.clone(),
            rule_json: rule_json.clone(),
        },
        PatchOp::ReplaceSwrlRule { ontology_iri, old_rule_json, new_rule_json } => {
            PatchOp::ReplaceSwrlRule {
                ontology_iri: ontology_iri.clone(),
                old_rule_json: new_rule_json.clone(),
                new_rule_json: old_rule_json.clone(),
            }
        }
    })
}

pub fn invert_obo_patch_op(op: &OboPatchOp) -> Result<OboPatchOp> {
    Ok(match op {
        OboPatchOp::SetName { .. } | OboPatchOp::SetNamespace { .. } => {
            return Err(EditError::NotInvertible("obo set_* requires prior value".into()));
        }
        OboPatchOp::AddSynonym { term_id, value, scope } => OboPatchOp::RemoveSynonym {
            term_id: term_id.clone(),
            value: value.clone(),
            scope: Some(scope.clone()),
        },
        OboPatchOp::RemoveSynonym { term_id, value, scope: Some(scope) } => {
            OboPatchOp::AddSynonym {
                term_id: term_id.clone(),
                value: value.clone(),
                scope: scope.clone(),
            }
        }
        OboPatchOp::RemoveSynonym { scope: None, .. } => {
            return Err(EditError::NotInvertible(
                "remove_synonym inverse requires recorded prior scope".into(),
            ));
        }
        OboPatchOp::AddDef { .. } => {
            return Err(EditError::NotInvertible(
                "add_def may replace a prior def; inverse requires prior value".into(),
            ));
        }
        OboPatchOp::RemoveDef { term_id: _ } => {
            return Err(EditError::NotInvertible("remove_def inverse requires prior def".into()));
        }
        OboPatchOp::AddXref { term_id, xref } => {
            OboPatchOp::RemoveXref { term_id: term_id.clone(), xref: xref.clone() }
        }
        OboPatchOp::RemoveXref { term_id, xref } => {
            OboPatchOp::AddXref { term_id: term_id.clone(), xref: xref.clone() }
        }
        OboPatchOp::SetDeprecated { term_id, value } => {
            if !*value {
                return Err(EditError::NotInvertible(
                    "set_deprecated false inverse would invent obsolescence without prior state"
                        .into(),
                ));
            }
            OboPatchOp::SetDeprecated { term_id: term_id.clone(), value: false }
        }
        OboPatchOp::AddIsA { term_id, parent_id } => {
            OboPatchOp::RemoveIsA { term_id: term_id.clone(), parent_id: parent_id.clone() }
        }
        OboPatchOp::RemoveIsA { term_id, parent_id } => {
            OboPatchOp::AddIsA { term_id: term_id.clone(), parent_id: parent_id.clone() }
        }
    })
}

pub fn invert_change(change: &SemanticChange) -> Result<SemanticChange> {
    Ok(match change {
        SemanticChange::Turtle { change } => {
            SemanticChange::Turtle { change: invert_patch_op(change)? }
        }
        SemanticChange::Obo { change } => {
            SemanticChange::Obo { change: invert_obo_patch_op(change)? }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_prefix_is_not_invertible() {
        let err = invert_patch_op(&PatchOp::RemovePrefix { prefix: "ex".into() }).unwrap_err();
        assert!(matches!(err, EditError::NotInvertible(_)));
    }

    #[test]
    fn add_synonym_inverts_to_scoped_remove() {
        let inverted = invert_obo_patch_op(&OboPatchOp::AddSynonym {
            term_id: "EX:1".into(),
            value: "alias".into(),
            scope: "RELATED".into(),
        })
        .expect("invert");
        assert_eq!(
            inverted,
            OboPatchOp::RemoveSynonym {
                term_id: "EX:1".into(),
                value: "alias".into(),
                scope: Some("RELATED".into()),
            }
        );
    }

    #[test]
    fn remove_synonym_with_scope_inverts_to_add() {
        let inverted = invert_obo_patch_op(&OboPatchOp::RemoveSynonym {
            term_id: "EX:1".into(),
            value: "alias".into(),
            scope: Some("RELATED".into()),
        })
        .expect("invert");
        assert_eq!(
            inverted,
            OboPatchOp::AddSynonym {
                term_id: "EX:1".into(),
                value: "alias".into(),
                scope: "RELATED".into(),
            }
        );
    }

    #[test]
    fn remove_synonym_without_scope_and_add_def_are_not_invertible() {
        assert!(matches!(
            invert_obo_patch_op(&OboPatchOp::RemoveSynonym {
                term_id: "EX:1".into(),
                value: "alias".into(),
                scope: None,
            }),
            Err(EditError::NotInvertible(_))
        ));
        assert!(matches!(
            invert_obo_patch_op(&OboPatchOp::AddDef {
                term_id: "EX:1".into(),
                value: "new".into(),
            }),
            Err(EditError::NotInvertible(_))
        ));
    }

    #[test]
    fn set_deprecated_false_is_not_invertible() {
        assert!(matches!(
            invert_patch_op(&PatchOp::SetDeprecated {
                entity_iri: "http://example.org/A".into(),
                value: false,
            }),
            Err(EditError::NotInvertible(_))
        ));
        assert!(matches!(
            invert_obo_patch_op(&OboPatchOp::SetDeprecated {
                term_id: "EX:1".into(),
                value: false,
            }),
            Err(EditError::NotInvertible(_))
        ));
    }

    #[test]
    fn set_deprecated_true_inverts_to_false() {
        let inverted = invert_patch_op(&PatchOp::SetDeprecated {
            entity_iri: "http://example.org/A".into(),
            value: true,
        })
        .expect("invert");
        assert_eq!(
            inverted,
            PatchOp::SetDeprecated { entity_iri: "http://example.org/A".into(), value: false }
        );
    }

    #[test]
    fn clearing_boolean_characteristic_is_not_invertible() {
        assert!(matches!(
            invert_patch_op(&PatchOp::SetFunctional {
                entity_iri: "http://example.org/p".into(),
                value: false,
            }),
            Err(EditError::NotInvertible(_))
        ));
        let inverted = invert_patch_op(&PatchOp::SetFunctional {
            entity_iri: "http://example.org/p".into(),
            value: true,
        })
        .expect("invert set true");
        assert_eq!(
            inverted,
            PatchOp::SetFunctional { entity_iri: "http://example.org/p".into(), value: false }
        );
    }

    #[test]
    fn has_key_and_inverse_invert() {
        let inverted = invert_patch_op(&PatchOp::AddHasKey {
            class_iri: "http://example.org/C".into(),
            properties: vec!["http://example.org/p".into()],
        })
        .expect("invert has key");
        assert_eq!(
            inverted,
            PatchOp::RemoveHasKey {
                class_iri: "http://example.org/C".into(),
                properties: vec!["http://example.org/p".into()],
            }
        );
        let inverted = invert_patch_op(&PatchOp::AddInverseObjectProperties {
            property_iri: "http://example.org/p".into(),
            inverse_iri: "http://example.org/q".into(),
        })
        .expect("invert inverse");
        assert_eq!(
            inverted,
            PatchOp::RemoveInverseObjectProperties {
                property_iri: "http://example.org/p".into(),
                inverse_iri: "http://example.org/q".into(),
            }
        );
    }

    #[test]
    fn negative_assertion_and_same_individual_invert() {
        let inverted = invert_patch_op(&PatchOp::AddNegativeObjectPropertyAssertion {
            entity_iri: "http://example.org/a".into(),
            property_iri: "http://example.org/p".into(),
            target_iri: "http://example.org/b".into(),
        })
        .expect("invert nopa");
        assert!(matches!(inverted, PatchOp::RemoveNegativeObjectPropertyAssertion { .. }));
        let inverted = invert_patch_op(&PatchOp::AddSameIndividual {
            individuals: vec!["http://example.org/a".into(), "http://example.org/b".into()],
        })
        .expect("invert same");
        assert_eq!(
            inverted,
            PatchOp::RemoveSameIndividual {
                individuals: vec!["http://example.org/a".into(), "http://example.org/b".into()],
            }
        );
    }
}
