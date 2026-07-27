use crate::error::{OwlError, Result};
use crate::manchester::{class_expression_to_turtle_fragment, parse_class_expression};
use crate::span::{
    all_entity_statement_ranges, entity_primary_block_range, namespaces_for_text,
    short_name_from_iri, statement_end_byte, ByteRange,
};
use crate::turtle_lex::{advance_turtle_scan, turtle_literal_lexical_value, TurtleScanState};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use strixonomy_core::{read_to_string_capped, OntologyFormat, MAX_FILE_BYTES};

/// A single authoring patch operation (v0.4 Turtle scope).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum PatchOp {
    AddPrefix {
        prefix: String,
        namespace_iri: String,
    },
    RemovePrefix {
        prefix: String,
    },
    SetPrefix {
        prefix: String,
        namespace_iri: String,
    },
    SetOntologyIri {
        ontology_iri: String,
    },
    SetVersionIri {
        ontology_iri: String,
        version_iri: String,
    },
    AddOntologyAnnotation {
        ontology_iri: String,
        predicate: String,
        value: String,
    },
    RemoveOntologyAnnotation {
        ontology_iri: String,
        predicate: String,
        value: String,
    },
    CreateEntity {
        entity_iri: String,
        kind: PatchEntityKind,
    },
    DeleteEntity {
        entity_iri: String,
    },
    SetLabel {
        entity_iri: String,
        value: String,
    },
    AddLabel {
        entity_iri: String,
        value: String,
    },
    RemoveLabel {
        entity_iri: String,
        value: String,
    },
    SetComment {
        entity_iri: String,
        value: String,
    },
    AddComment {
        entity_iri: String,
        value: String,
    },
    RemoveComment {
        entity_iri: String,
        value: String,
    },
    AddSubClassOf {
        entity_iri: String,
        parent_iri: String,
    },
    RemoveSubClassOf {
        entity_iri: String,
        parent_iri: String,
    },
    AddComplexSubClassOf {
        entity_iri: String,
        manchester: String,
    },
    RemoveComplexSubClassOf {
        entity_iri: String,
        manchester: String,
    },
    AddEquivalentClass {
        entity_iri: String,
        manchester: String,
    },
    RemoveEquivalentClass {
        entity_iri: String,
        manchester: String,
    },
    SetEquivalentClass {
        entity_iri: String,
        manchester: String,
    },
    SetDeprecated {
        entity_iri: String,
        value: bool,
    },
    AddDisjointClass {
        entity_iri: String,
        other_iri: String,
    },
    RemoveDisjointClass {
        entity_iri: String,
        other_iri: String,
    },
    AddImport {
        ontology_iri: String,
        import_iri: String,
    },
    RemoveImport {
        ontology_iri: String,
        import_iri: String,
    },
    AddDomain {
        entity_iri: String,
        class_iri: String,
    },
    RemoveDomain {
        entity_iri: String,
        class_iri: String,
    },
    AddRange {
        entity_iri: String,
        range_iri: String,
    },
    RemoveRange {
        entity_iri: String,
        range_iri: String,
    },
    SetFunctional {
        entity_iri: String,
        value: bool,
    },
    SetInverseFunctional {
        entity_iri: String,
        value: bool,
    },
    SetTransitive {
        entity_iri: String,
        value: bool,
    },
    SetSymmetric {
        entity_iri: String,
        value: bool,
    },
    SetAsymmetric {
        entity_iri: String,
        value: bool,
    },
    SetReflexive {
        entity_iri: String,
        value: bool,
    },
    SetIrreflexive {
        entity_iri: String,
        value: bool,
    },
    AddPropertyChain {
        entity_iri: String,
        properties: Vec<String>,
    },
    RemovePropertyChain {
        entity_iri: String,
        properties: Vec<String>,
    },
    AddClassAssertion {
        entity_iri: String,
        class_iri: String,
    },
    RemoveClassAssertion {
        entity_iri: String,
        class_iri: String,
    },
    AddObjectPropertyAssertion {
        entity_iri: String,
        property_iri: String,
        target_iri: String,
    },
    RemoveObjectPropertyAssertion {
        entity_iri: String,
        property_iri: String,
        target_iri: String,
    },
    AddDataPropertyAssertion {
        entity_iri: String,
        property_iri: String,
        value: String,
    },
    RemoveDataPropertyAssertion {
        entity_iri: String,
        property_iri: String,
        value: String,
    },
    AddAnnotation {
        entity_iri: String,
        predicate: String,
        value: String,
    },
    RemoveAnnotation {
        entity_iri: String,
        predicate: String,
        value: String,
    },
    // TBox / Keys / RBox (v0.22)
    AddHasKey {
        class_iri: String,
        properties: Vec<String>,
    },
    RemoveHasKey {
        class_iri: String,
        properties: Vec<String>,
    },
    AddDisjointUnion {
        class_iri: String,
        members: Vec<String>,
    },
    RemoveDisjointUnion {
        class_iri: String,
        members: Vec<String>,
    },
    AddInverseObjectProperties {
        property_iri: String,
        inverse_iri: String,
    },
    RemoveInverseObjectProperties {
        property_iri: String,
        inverse_iri: String,
    },
    AddEquivalentObjectProperties {
        properties: Vec<String>,
    },
    RemoveEquivalentObjectProperties {
        properties: Vec<String>,
    },
    AddDisjointObjectProperties {
        properties: Vec<String>,
    },
    RemoveDisjointObjectProperties {
        properties: Vec<String>,
    },
    AddEquivalentDataProperties {
        properties: Vec<String>,
    },
    RemoveEquivalentDataProperties {
        properties: Vec<String>,
    },
    AddDisjointDataProperties {
        properties: Vec<String>,
    },
    RemoveDisjointDataProperties {
        properties: Vec<String>,
    },
    AddSubObjectPropertyOf {
        property_iri: String,
        parent_iri: String,
    },
    RemoveSubObjectPropertyOf {
        property_iri: String,
        parent_iri: String,
    },
    AddSubDataPropertyOf {
        property_iri: String,
        parent_iri: String,
    },
    RemoveSubDataPropertyOf {
        property_iri: String,
        parent_iri: String,
    },
    // ABox (v0.22)
    AddNegativeObjectPropertyAssertion {
        entity_iri: String,
        property_iri: String,
        target_iri: String,
    },
    RemoveNegativeObjectPropertyAssertion {
        entity_iri: String,
        property_iri: String,
        target_iri: String,
    },
    AddNegativeDataPropertyAssertion {
        entity_iri: String,
        property_iri: String,
        value: String,
    },
    RemoveNegativeDataPropertyAssertion {
        entity_iri: String,
        property_iri: String,
        value: String,
    },
    AddSameIndividual {
        individuals: Vec<String>,
    },
    RemoveSameIndividual {
        individuals: Vec<String>,
    },
    AddDifferentIndividuals {
        individuals: Vec<String>,
    },
    RemoveDifferentIndividuals {
        individuals: Vec<String>,
    },
    // Datatype (v0.22)
    AddDatatypeDefinition {
        datatype_iri: String,
        manchester: String,
    },
    RemoveDatatypeDefinition {
        datatype_iri: String,
        manchester: String,
    },
    // Axiom annotations (v0.22)
    AddAxiomAnnotation {
        axiom_op: String,
        subject_iri: String,
        related_iri: Option<String>,
        predicate: String,
        value: String,
    },
    RemoveAxiomAnnotation {
        axiom_op: String,
        subject_iri: String,
        related_iri: Option<String>,
        predicate: String,
        value: String,
    },
    /// SWRL rule authoring (v0.23) — stores rule as JSON via strixonomy:swrlRule.
    AddSwrlRule {
        ontology_iri: String,
        rule_json: String,
    },
    RemoveSwrlRule {
        ontology_iri: String,
        rule_json: String,
    },
    ReplaceSwrlRule {
        ontology_iri: String,
        old_rule_json: String,
        new_rule_json: String,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PatchEntityKind {
    Class,
    ObjectProperty,
    DataProperty,
    AnnotationProperty,
    Individual,
    Datatype,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDiagnostic {
    pub severity: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyPatchResult {
    pub applied: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<PatchDiagnostic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_path: Option<String>,
}

/// Apply patches to a document on disk (Turtle span surgery or XML re-serialize).
pub fn apply_patches(
    document_path: &Path,
    patches: &[PatchOp],
    preview_only: bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<ApplyPatchResult> {
    let format = OntologyFormat::from_extension(
        document_path.extension().and_then(|e| e.to_str()).unwrap_or(""),
    );
    match format {
        OntologyFormat::Turtle => {}
        OntologyFormat::Owl | OntologyFormat::RdfXml | OntologyFormat::OwlXml => {
            return crate::apply_xml::apply_xml_patches(
                document_path,
                patches,
                preview_only,
                namespaces,
            );
        }
        other => {
            return Err(OwlError::UnsupportedFormat(format!(
                "write-back supports Turtle (.ttl), RDF/XML (.owl/.rdf), OWL/XML (.owx); got {}",
                other.as_str()
            )));
        }
    }

    let source = read_to_string_capped(document_path, MAX_FILE_BYTES).map_err(OwlError::Core)?;
    let mut result = apply_patches_to_text(&source, patches, preview_only, namespaces)?;
    result.document_path = Some(document_path.display().to_string());

    if result.applied && !preview_only {
        if let Some(text) = &result.preview_text {
            atomic_write(document_path, text)?;
        }
    }
    Ok(result)
}

pub fn atomic_write(path: &Path, contents: &str) -> Result<()> {
    let parent =
        path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    let stem = path.file_name().and_then(|s| s.to_str()).unwrap_or("file");
    let tmp_path = parent.join(format!(".strixonomy-{stem}-{nanos}.tmp"));
    // Best-effort remove temp on mid-write failure (#343).
    let write_result = (|| -> Result<()> {
        let mut file = fs::File::create(&tmp_path)?;
        file.write_all(contents.as_bytes())?;
        file.sync_all()?;
        Ok(())
    })();
    if let Err(e) = write_result {
        let _ = fs::remove_file(&tmp_path);
        return Err(e);
    }
    replace_file(&tmp_path, path)?;
    Ok(())
}

/// Replace `path` with `tmp_path` (tmp is consumed). Works on Windows where `rename` cannot
/// overwrite an existing destination.
fn replace_file(tmp_path: &Path, path: &Path) -> std::io::Result<()> {
    match fs::rename(tmp_path, path) {
        Ok(()) => Ok(()),
        Err(_) if path.exists() => {
            // Windows (and some network FS): rename refuses to replace. Move the existing
            // file aside, then rename; restore on failure.
            let bak_path = tmp_path.with_extension("bak");
            fs::rename(path, &bak_path)?;
            match fs::rename(tmp_path, path) {
                Ok(()) => {
                    let _ = fs::remove_file(&bak_path);
                    Ok(())
                }
                Err(rename_err) => {
                    let _ = fs::rename(&bak_path, path);
                    let _ = fs::remove_file(tmp_path);
                    Err(rename_err)
                }
            }
        }
        Err(e) => {
            let _ = fs::remove_file(tmp_path);
            Err(e)
        }
    }
}

/// Apply patches to in-memory Turtle text.
pub fn apply_patches_to_text(
    source: &str,
    patches: &[PatchOp],
    preview_only: bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<ApplyPatchResult> {
    let mut working = source.to_string();
    let mut diagnostics = Vec::new();

    for patch in patches {
        match apply_one_patch(&mut working, patch, namespaces) {
            Ok(()) => {}
            Err(e) => {
                diagnostics.push(PatchDiagnostic {
                    severity: "error".to_string(),
                    message: e.to_string(),
                });
                return Ok(ApplyPatchResult {
                    applied: false,
                    preview_text: Some(source.to_string()),
                    diagnostics,
                    document_path: None,
                });
            }
        }
    }

    let changed = working != source;
    Ok(ApplyPatchResult {
        applied: changed && !preview_only,
        preview_text: if changed { Some(working) } else { None },
        diagnostics,
        document_path: None,
    })
}

fn apply_one_patch(
    text: &mut String,
    patch: &PatchOp,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    match patch {
        PatchOp::AddPrefix { prefix, namespace_iri } => add_prefix(text, prefix, namespace_iri),
        PatchOp::RemovePrefix { prefix } => remove_prefix(text, prefix),
        PatchOp::SetPrefix { prefix, namespace_iri } => set_prefix(text, prefix, namespace_iri),
        PatchOp::SetOntologyIri { ontology_iri } => set_ontology_iri(text, ontology_iri),
        PatchOp::SetVersionIri { ontology_iri, version_iri } => {
            remove_all_predicate_any_statement(text, ontology_iri, "owl:versionIRI", namespaces)?;
            let version_term = iri_to_turtle_term(version_iri, namespaces)?;
            add_object_triple(text, ontology_iri, "owl:versionIRI", &version_term, namespaces)
        }
        PatchOp::AddOntologyAnnotation { ontology_iri, predicate, value } => {
            add_annotation_value(text, ontology_iri, predicate, value, namespaces)
        }
        PatchOp::RemoveOntologyAnnotation { ontology_iri, predicate, value } => {
            remove_annotation_value(text, ontology_iri, predicate, value, namespaces)
        }
        PatchOp::CreateEntity { entity_iri, kind } => {
            create_entity(text, entity_iri, *kind, namespaces)
        }
        PatchOp::DeleteEntity { entity_iri } => delete_entity(text, entity_iri, namespaces),
        PatchOp::SetLabel { entity_iri, value } => {
            remove_all_predicate_any_statement(text, entity_iri, "rdfs:label", namespaces)?;
            add_annotation_triple(text, entity_iri, "rdfs:label", value, namespaces)
        }
        PatchOp::AddLabel { entity_iri, value } => {
            add_annotation_triple(text, entity_iri, "rdfs:label", value, namespaces)
        }
        PatchOp::RemoveLabel { entity_iri, value } => {
            remove_matching_predicate_any(text, entity_iri, "rdfs:label", value, namespaces)
        }
        PatchOp::SetComment { entity_iri, value } => {
            remove_all_predicate_any_statement(text, entity_iri, "rdfs:comment", namespaces)?;
            add_annotation_triple(text, entity_iri, "rdfs:comment", value, namespaces)
        }
        PatchOp::AddComment { entity_iri, value } => {
            add_annotation_triple(text, entity_iri, "rdfs:comment", value, namespaces)
        }
        PatchOp::RemoveComment { entity_iri, value } => {
            remove_matching_predicate_any(text, entity_iri, "rdfs:comment", value, namespaces)
        }
        PatchOp::AddSubClassOf { entity_iri, parent_iri } => {
            add_subclass_triple(text, entity_iri, parent_iri, namespaces)
        }
        PatchOp::RemoveSubClassOf { entity_iri, parent_iri } => {
            remove_predicate_iri_object(text, entity_iri, "rdfs:subClassOf", parent_iri, namespaces)
        }
        PatchOp::AddComplexSubClassOf { entity_iri, manchester } => {
            add_complex_axiom(text, entity_iri, manchester, "rdfs:subClassOf", namespaces)
        }
        PatchOp::RemoveComplexSubClassOf { entity_iri, manchester } => {
            remove_complex_axiom(text, entity_iri, manchester, "rdfs:subClassOf", namespaces)
        }
        PatchOp::AddEquivalentClass { entity_iri, manchester } => {
            add_complex_axiom(text, entity_iri, manchester, "owl:equivalentClass", namespaces)
        }
        PatchOp::RemoveEquivalentClass { entity_iri, manchester } => {
            remove_complex_axiom(text, entity_iri, manchester, "owl:equivalentClass", namespaces)
        }
        PatchOp::SetEquivalentClass { entity_iri, manchester } => {
            remove_predicate_triples(text, entity_iri, "owl:equivalentClass", namespaces)?;
            add_complex_axiom(text, entity_iri, manchester, "owl:equivalentClass", namespaces)
        }
        PatchOp::SetDeprecated { entity_iri, value } => {
            if *value {
                add_object_triple(text, entity_iri, "owl:deprecated", "true", namespaces)
            } else {
                remove_predicate_triples(text, entity_iri, "owl:deprecated", namespaces)
            }
        }
        PatchOp::AddDisjointClass { entity_iri, other_iri } => {
            let other = iri_to_turtle_term(other_iri, namespaces)?;
            add_object_triple(text, entity_iri, "owl:disjointWith", &other, namespaces)
        }
        PatchOp::RemoveDisjointClass { entity_iri, other_iri } => {
            remove_predicate_iri_object(text, entity_iri, "owl:disjointWith", other_iri, namespaces)
        }
        PatchOp::AddImport { ontology_iri, import_iri } => {
            let import_term = iri_to_turtle_term(import_iri, namespaces)?;
            add_object_triple(text, ontology_iri, "owl:imports", &import_term, namespaces)
        }
        PatchOp::RemoveImport { ontology_iri, import_iri } => {
            remove_predicate_iri_object(text, ontology_iri, "owl:imports", import_iri, namespaces)
        }
        PatchOp::AddDomain { entity_iri, class_iri } => {
            let class = iri_to_turtle_term(class_iri, namespaces)?;
            add_object_triple(text, entity_iri, "rdfs:domain", &class, namespaces)
        }
        PatchOp::RemoveDomain { entity_iri, class_iri } => {
            remove_predicate_iri_object(text, entity_iri, "rdfs:domain", class_iri, namespaces)
        }
        PatchOp::AddRange { entity_iri, range_iri } => {
            let range = iri_to_turtle_term(range_iri, namespaces)?;
            add_object_triple(text, entity_iri, "rdfs:range", &range, namespaces)
        }
        PatchOp::RemoveRange { entity_iri, range_iri } => {
            remove_predicate_iri_object(text, entity_iri, "rdfs:range", range_iri, namespaces)
        }
        PatchOp::SetFunctional { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:FunctionalProperty",
            *value,
            namespaces,
        ),
        PatchOp::SetInverseFunctional { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:InverseFunctionalProperty",
            *value,
            namespaces,
        ),
        PatchOp::SetTransitive { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:TransitiveProperty",
            *value,
            namespaces,
        ),
        PatchOp::SetSymmetric { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:SymmetricProperty",
            *value,
            namespaces,
        ),
        PatchOp::SetAsymmetric { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:AsymmetricProperty",
            *value,
            namespaces,
        ),
        PatchOp::SetReflexive { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:ReflexiveProperty",
            *value,
            namespaces,
        ),
        PatchOp::SetIrreflexive { entity_iri, value } => set_property_characteristic(
            text,
            entity_iri,
            "owl:IrreflexiveProperty",
            *value,
            namespaces,
        ),
        PatchOp::AddPropertyChain { entity_iri, properties } => {
            add_property_chain(text, entity_iri, properties, namespaces)
        }
        PatchOp::RemovePropertyChain { entity_iri, properties } => {
            remove_property_chain(text, entity_iri, properties, namespaces)
        }
        PatchOp::AddClassAssertion { entity_iri, class_iri } => {
            let class = iri_to_turtle_term(class_iri, namespaces)?;
            add_type_triple(text, entity_iri, &class, namespaces)
        }
        PatchOp::RemoveClassAssertion { entity_iri, class_iri } => {
            remove_rdf_type_iri(text, entity_iri, class_iri, namespaces)
        }
        PatchOp::AddObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            let prop = iri_to_turtle_term(property_iri, namespaces)?;
            let target = iri_to_turtle_term(target_iri, namespaces)?;
            add_property_assertion_triple(text, entity_iri, &prop, &target, namespaces)
        }
        PatchOp::RemoveObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            let prop = iri_to_turtle_term(property_iri, namespaces)?;
            remove_predicate_iri_object(text, entity_iri, &prop, target_iri, namespaces)
        }
        PatchOp::AddDataPropertyAssertion { entity_iri, property_iri, value } => {
            let prop = iri_to_turtle_term(property_iri, namespaces)?;
            add_data_property_assertion(text, entity_iri, &prop, value, namespaces)
        }
        PatchOp::RemoveDataPropertyAssertion { entity_iri, property_iri, value } => {
            let prop = iri_to_turtle_term(property_iri, namespaces)?;
            remove_matching_predicate_any(text, entity_iri, &prop, value, namespaces)
        }
        PatchOp::AddAnnotation { entity_iri, predicate, value } => {
            add_annotation_value(text, entity_iri, predicate, value, namespaces)
        }
        PatchOp::RemoveAnnotation { entity_iri, predicate, value } => {
            remove_annotation_value(text, entity_iri, predicate, value, namespaces)
        }
        PatchOp::AddHasKey { class_iri, properties } => {
            add_iri_list_axiom(text, class_iri, "owl:hasKey", properties, namespaces)
        }
        PatchOp::RemoveHasKey { class_iri, properties } => {
            remove_iri_list_axiom(text, class_iri, "owl:hasKey", properties, namespaces)
        }
        PatchOp::AddDisjointUnion { class_iri, members } => {
            add_iri_list_axiom(text, class_iri, "owl:disjointUnionOf", members, namespaces)
        }
        PatchOp::RemoveDisjointUnion { class_iri, members } => {
            remove_iri_list_axiom(text, class_iri, "owl:disjointUnionOf", members, namespaces)
        }
        PatchOp::AddInverseObjectProperties { property_iri, inverse_iri } => {
            let inv = iri_to_turtle_term(inverse_iri, namespaces)?;
            add_object_triple(text, property_iri, "owl:inverseOf", &inv, namespaces)
        }
        PatchOp::RemoveInverseObjectProperties { property_iri, inverse_iri } => {
            remove_predicate_iri_object(
                text,
                property_iri,
                "owl:inverseOf",
                inverse_iri,
                namespaces,
            )
        }
        PatchOp::AddEquivalentObjectProperties { properties } => {
            add_pairwise_property_axioms(text, properties, "owl:equivalentProperty", namespaces)
        }
        PatchOp::RemoveEquivalentObjectProperties { properties } => {
            remove_pairwise_property_axioms(text, properties, "owl:equivalentProperty", namespaces)
        }
        PatchOp::AddDisjointObjectProperties { properties } => {
            add_pairwise_property_axioms(text, properties, "owl:propertyDisjointWith", namespaces)
        }
        PatchOp::RemoveDisjointObjectProperties { properties } => remove_pairwise_property_axioms(
            text,
            properties,
            "owl:propertyDisjointWith",
            namespaces,
        ),
        PatchOp::AddEquivalentDataProperties { properties } => {
            add_pairwise_property_axioms(text, properties, "owl:equivalentProperty", namespaces)
        }
        PatchOp::RemoveEquivalentDataProperties { properties } => {
            remove_pairwise_property_axioms(text, properties, "owl:equivalentProperty", namespaces)
        }
        PatchOp::AddDisjointDataProperties { properties } => {
            add_pairwise_property_axioms(text, properties, "owl:propertyDisjointWith", namespaces)
        }
        PatchOp::RemoveDisjointDataProperties { properties } => remove_pairwise_property_axioms(
            text,
            properties,
            "owl:propertyDisjointWith",
            namespaces,
        ),
        PatchOp::AddSubObjectPropertyOf { property_iri, parent_iri } => {
            let parent = iri_to_turtle_term(parent_iri, namespaces)?;
            add_object_triple(text, property_iri, "rdfs:subPropertyOf", &parent, namespaces)
        }
        PatchOp::RemoveSubObjectPropertyOf { property_iri, parent_iri } => {
            remove_predicate_iri_object(
                text,
                property_iri,
                "rdfs:subPropertyOf",
                parent_iri,
                namespaces,
            )
        }
        PatchOp::AddSubDataPropertyOf { property_iri, parent_iri } => {
            let parent = iri_to_turtle_term(parent_iri, namespaces)?;
            add_object_triple(text, property_iri, "rdfs:subPropertyOf", &parent, namespaces)
        }
        PatchOp::RemoveSubDataPropertyOf { property_iri, parent_iri } => {
            remove_predicate_iri_object(
                text,
                property_iri,
                "rdfs:subPropertyOf",
                parent_iri,
                namespaces,
            )
        }
        PatchOp::AddNegativeObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            add_negative_object_property_assertion(
                text,
                entity_iri,
                property_iri,
                target_iri,
                namespaces,
            )
        }
        PatchOp::RemoveNegativeObjectPropertyAssertion { entity_iri, property_iri, target_iri } => {
            remove_negative_object_property_assertion(
                text,
                entity_iri,
                property_iri,
                target_iri,
                namespaces,
            )
        }
        PatchOp::AddNegativeDataPropertyAssertion { entity_iri, property_iri, value } => {
            add_negative_data_property_assertion(text, entity_iri, property_iri, value, namespaces)
        }
        PatchOp::RemoveNegativeDataPropertyAssertion { entity_iri, property_iri, value } => {
            remove_negative_data_property_assertion(
                text,
                entity_iri,
                property_iri,
                value,
                namespaces,
            )
        }
        PatchOp::AddSameIndividual { individuals } => {
            add_pairwise_individual_axioms(text, individuals, "owl:sameAs", namespaces)
        }
        PatchOp::RemoveSameIndividual { individuals } => {
            remove_pairwise_individual_axioms(text, individuals, "owl:sameAs", namespaces)
        }
        PatchOp::AddDifferentIndividuals { individuals } => {
            add_different_individuals(text, individuals, namespaces)
        }
        PatchOp::RemoveDifferentIndividuals { individuals } => {
            remove_different_individuals(text, individuals, namespaces)
        }
        PatchOp::AddDatatypeDefinition { datatype_iri, manchester } => {
            add_datatype_definition(text, datatype_iri, manchester, namespaces)
        }
        PatchOp::RemoveDatatypeDefinition { datatype_iri, manchester } => {
            remove_datatype_definition(text, datatype_iri, manchester, namespaces)
        }
        PatchOp::AddAxiomAnnotation { axiom_op, subject_iri, related_iri, predicate, value } => {
            add_axiom_annotation(
                text,
                axiom_op,
                subject_iri,
                related_iri.as_deref(),
                predicate,
                value,
                namespaces,
            )
        }
        PatchOp::RemoveAxiomAnnotation { axiom_op, subject_iri, related_iri, predicate, value } => {
            remove_axiom_annotation(
                text,
                axiom_op,
                subject_iri,
                related_iri.as_deref(),
                predicate,
                value,
                namespaces,
            )
        }
        PatchOp::AddSwrlRule { ontology_iri, rule_json } => {
            add_swrl_rule_json(text, ontology_iri, rule_json)
        }
        PatchOp::RemoveSwrlRule { ontology_iri, rule_json } => {
            remove_swrl_rule_json(text, ontology_iri, rule_json)
        }
        PatchOp::ReplaceSwrlRule { ontology_iri, old_rule_json, new_rule_json } => {
            remove_swrl_rule_json(text, ontology_iri, old_rule_json)?;
            add_swrl_rule_json(text, ontology_iri, new_rule_json)
        }
    }
}

const ONTOCORE_SWRL_PRED: &str = "http://ontocode.dev/ns#swrlRule";

fn add_swrl_rule_json(text: &mut String, ontology_iri: &str, rule_json: &str) -> Result<()> {
    if !is_safe_iri(ontology_iri) {
        return Err(OwlError::PatchInvalid(format!(
            "SWRL ontology IRI is unsafe for Turtle write-back: {ontology_iri:?}"
        )));
    }
    // Compact JSON so the literal stays on one Turtle line.
    let compact = serde_json::from_str::<serde_json::Value>(rule_json)
        .map_err(|e| OwlError::PatchInvalid(format!("invalid SWRL rule JSON: {e}")))
        .and_then(|v| {
            serde_json::to_string(&v)
                .map_err(|e| OwlError::PatchInvalid(format!("serialize SWRL rule JSON: {e}")))
        })?;
    let escaped = escape_turtle_string(&compact);
    let triple = format!(
        "<{ontology_iri}> <{ONTOCORE_SWRL_PRED}> \"{escaped}\"^^<http://www.w3.org/2001/XMLSchema#string> .\n"
    );
    let needle = format!("<{ontology_iri}> <{ONTOCORE_SWRL_PRED}> \"{escaped}\"");
    if text.lines().any(|line| line.contains(&needle)) {
        return Ok(());
    }
    if !text.ends_with('\n') {
        text.push('\n');
    }
    text.push_str(&triple);
    Ok(())
}

fn remove_swrl_rule_json(text: &mut String, ontology_iri: &str, rule_json: &str) -> Result<()> {
    if !is_safe_iri(ontology_iri) {
        return Err(OwlError::PatchInvalid(format!(
            "SWRL ontology IRI is unsafe for Turtle write-back: {ontology_iri:?}"
        )));
    }
    let target = serde_json::from_str::<serde_json::Value>(rule_json).ok();
    let mut out = String::new();
    let mut removed = false;
    for line in text.lines() {
        if !removed {
            if let Some(literal) = swrl_rule_literal_on_line(line, ontology_iri) {
                let matches = match (&target, serde_json::from_str::<serde_json::Value>(&literal)) {
                    (Some(want), Ok(got)) => want == &got,
                    _ => {
                        // Fall back to exact / compacted string match for non-JSON literals.
                        let compact = serde_json::from_str::<serde_json::Value>(rule_json)
                            .map(|v| {
                                serde_json::to_string(&v).unwrap_or_else(|_| rule_json.to_string())
                            })
                            .unwrap_or_else(|_| rule_json.to_string());
                        literal == rule_json || literal == compact
                    }
                };
                if matches {
                    removed = true;
                    continue;
                }
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    if !removed {
        return Err(OwlError::PatchInvalid(format!(
            "SWRL rule not found for ontology {ontology_iri}"
        )));
    }
    *text = out;
    Ok(())
}

/// Extract the JSON string literal from an `strixonomy:swrlRule` triple line for `ontology_iri`.
fn swrl_rule_literal_on_line(line: &str, ontology_iri: &str) -> Option<String> {
    let subj = format!("<{ontology_iri}>");
    let pred = format!("<{ONTOCORE_SWRL_PRED}>");
    let trimmed = line.trim();
    if !(trimmed.contains(&subj) && trimmed.contains(&pred)) {
        return None;
    }
    let after_pred = trimmed.split(&pred).nth(1)?.trim_start();
    if !after_pred.starts_with('"') {
        return None;
    }
    let mut out = String::new();
    let bytes = after_pred.as_bytes();
    let mut i = 1usize;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' if i + 1 < bytes.len() => {
                match bytes[i + 1] {
                    b'"' => out.push('"'),
                    b'\\' => out.push('\\'),
                    b'n' => out.push('\n'),
                    b't' => out.push('\t'),
                    c => {
                        out.push('\\');
                        out.push(c as char);
                    }
                }
                i += 2;
            }
            b'"' => return Some(out),
            c => {
                out.push(c as char);
                i += 1;
            }
        }
    }
    None
}

fn prefix_declaration_name(line: &str) -> Option<&str> {
    let mut parts = line.split_whitespace();
    let keyword = parts.next()?;
    if !(keyword.eq_ignore_ascii_case("@prefix") || keyword.eq_ignore_ascii_case("PREFIX")) {
        return None;
    }
    parts.next()?.strip_suffix(':')
}

fn prefix_declaration_keyword(line: &str) -> Option<&str> {
    let keyword = line.split_whitespace().next()?;
    if keyword.eq_ignore_ascii_case("@prefix") || keyword.eq_ignore_ascii_case("PREFIX") {
        Some(keyword)
    } else {
        None
    }
}

fn format_prefix_declaration(keyword: &str, prefix: &str, namespace_iri: &str) -> String {
    if keyword.eq_ignore_ascii_case("PREFIX") && !keyword.starts_with('@') {
        format!("PREFIX {prefix}: <{namespace_iri}>")
    } else {
        format!("{keyword} {prefix}: <{namespace_iri}> .")
    }
}

pub fn validate_prefix(prefix: &str, namespace_iri: &str) -> Result<()> {
    if prefix.is_empty() || !prefix.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(OwlError::PatchInvalid(format!(
            "prefix must contain only letters, numbers, or underscores: {prefix:?}"
        )));
    }
    if !(namespace_iri.starts_with("http://") || namespace_iri.starts_with("https://"))
        || !is_safe_iri(namespace_iri)
    {
        return Err(OwlError::PatchInvalid(format!(
            "prefix namespace IRI must be a valid http(s) IRI: {namespace_iri:?}"
        )));
    }
    Ok(())
}

fn add_prefix(text: &mut String, prefix: &str, namespace_iri: &str) -> Result<()> {
    validate_prefix(prefix, namespace_iri)?;
    if text.lines().any(|line| prefix_declaration_name(line) == Some(prefix)) {
        return Err(OwlError::PatchInvalid(format!("duplicate prefix already present: {prefix}")));
    }

    let mut offset = 0;
    let mut insertion_at = 0;
    for line in text.split_inclusive('\n') {
        offset += line.len();
        if prefix_declaration_name(line).is_some() {
            insertion_at = offset;
        }
    }

    let declaration = format!("@prefix {prefix}: <{namespace_iri}> .\n");
    if insertion_at > 0 && !text[..insertion_at].ends_with('\n') {
        text.insert_str(insertion_at, &format!("\n{declaration}"));
    } else {
        text.insert_str(insertion_at, &declaration);
    }
    Ok(())
}

fn remove_prefix(text: &mut String, prefix: &str) -> Result<()> {
    if prefix.is_empty() || !prefix.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(OwlError::PatchInvalid(format!(
            "prefix must contain only letters, numbers, or underscores: {prefix:?}"
        )));
    }
    let mut rewritten = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        if prefix_declaration_name(line) != Some(prefix) {
            rewritten.push_str(line);
        }
    }
    *text = rewritten;
    Ok(())
}

fn set_prefix(text: &mut String, prefix: &str, namespace_iri: &str) -> Result<()> {
    validate_prefix(prefix, namespace_iri)?;
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        if prefix_declaration_name(line) == Some(prefix) {
            let keyword = prefix_declaration_keyword(line).unwrap_or("@prefix");
            let mut replacement = format_prefix_declaration(keyword, prefix, namespace_iri);
            if line.ends_with('\n') {
                replacement.push('\n');
            }
            text.replace_range(offset..offset + line.len(), &replacement);
            return Ok(());
        }
        offset += line.len();
    }
    add_prefix(text, prefix, namespace_iri)
}

fn set_ontology_iri(text: &mut String, ontology_iri: &str) -> Result<()> {
    if !is_safe_iri(ontology_iri) {
        return Err(OwlError::PatchInvalid(format!(
            "IRI contains characters that cannot be safely written to Turtle: {ontology_iri:?}"
        )));
    }

    let mut offset = 0;
    let mut declaration_subject = None;
    for line in text.split_inclusive('\n') {
        let leading = line.len() - line.trim_start().len();
        let trimmed = line.trim_start();
        if let Some(len) = ontology_declaration_subject_len(trimmed) {
            declaration_subject = Some((offset + leading, len));
            break;
        }
        offset += line.len();
    }
    if let Some((start, len)) = declaration_subject {
        text.replace_range(start..start + len, &format!("<{ontology_iri}>"));
        return Ok(());
    }

    if !text.is_empty() && !text.ends_with('\n') {
        text.push('\n');
    }
    if !text.is_empty() && !text.ends_with("\n\n") {
        text.push('\n');
    }
    text.push_str(&format!("<{ontology_iri}> a owl:Ontology .\n"));
    Ok(())
}

/// Byte length of the subject token on a line that declares an ontology type.
///
/// Accepts absolute IRI subjects (`<…>`) and prefixed names (`ex:ont`, `:ont`).
/// Recognizes `a` / `rdf:type` with `owl:Ontology` or the absolute Ontology IRI.
fn ontology_declaration_subject_len(trimmed: &str) -> Option<usize> {
    if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('@') {
        return None;
    }
    // SPARQL-style PREFIX lines (no leading @).
    if trimmed.len() >= 6 && trimmed[..6].eq_ignore_ascii_case("prefix") {
        return None;
    }

    let (subject, remainder) = if trimmed.starts_with('<') {
        let end = trimmed.find('>')?;
        (&trimmed[..=end], &trimmed[end + 1..])
    } else {
        let end = trimmed.find(char::is_whitespace)?;
        let subject = &trimmed[..end];
        // Prefixed name / default-prefix CURIE / blank-node label.
        if !subject.contains(':') {
            return None;
        }
        (subject, &trimmed[end..])
    };

    if subject.is_empty() {
        return None;
    }
    if remainder_declares_owl_ontology(remainder) {
        Some(subject.len())
    } else {
        None
    }
}

fn remainder_declares_owl_ontology(remainder: &str) -> bool {
    let r = remainder.trim_start();
    for pred in ["rdf:type", "a"] {
        let Some(after_pred) = r.strip_prefix(pred) else {
            continue;
        };
        if !after_pred.starts_with(|c: char| c.is_whitespace()) {
            continue;
        }
        let obj = after_pred.trim_start();
        for ontology_type in ["owl:Ontology", "<http://www.w3.org/2002/07/owl#Ontology>"] {
            if let Some(after) = obj.strip_prefix(ontology_type) {
                if after.is_empty()
                    || after
                        .starts_with(|c: char| c.is_whitespace() || matches!(c, ';' | '.' | ','))
                {
                    return true;
                }
            }
        }
    }
    false
}

fn add_annotation_value(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if predicate == "rdfs:label" || predicate.ends_with("#label") {
        add_annotation_triple(text, entity_iri, "rdfs:label", value, namespaces)
    } else if predicate == "rdfs:comment" || predicate.ends_with("#comment") {
        add_annotation_triple(text, entity_iri, "rdfs:comment", value, namespaces)
    } else if let Some(obj) = explicit_iri_annotation_term(value, namespaces)? {
        let pred = predicate_to_term(predicate, namespaces)?;
        add_object_triple(text, entity_iri, &pred, &obj, namespaces)
    } else {
        let pred = predicate_to_term(predicate, namespaces)?;
        add_annotation_triple(text, entity_iri, &pred, value, namespaces)
    }
}

fn remove_annotation_value(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if predicate == "rdfs:label" || predicate.ends_with("#label") {
        remove_matching_predicate_any(text, entity_iri, "rdfs:label", value, namespaces)
    } else if predicate == "rdfs:comment" || predicate.ends_with("#comment") {
        remove_matching_predicate_any(text, entity_iri, "rdfs:comment", value, namespaces)
    } else if let Some(obj) = explicit_iri_annotation_term(value, namespaces)? {
        let pred = predicate_to_term(predicate, namespaces)?;
        remove_predicate_object_any_statement(text, entity_iri, &pred, &obj, namespaces)
    } else {
        let pred = predicate_to_term(predicate, namespaces)?;
        remove_matching_predicate_any(text, entity_iri, &pred, value, namespaces)
    }
}

/// Treat annotation values as IRI objects only when explicitly marked (`<iri>`) or a known CURIE.
/// URL-shaped plain strings default to quoted literals.
fn explicit_iri_annotation_term(
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<Option<String>> {
    let trimmed = value.trim();
    if let Some(inner) = trimmed.strip_prefix('<').and_then(|s| s.strip_suffix('>')) {
        let iri = inner.trim();
        if iri.is_empty() {
            return Err(OwlError::PatchInvalid("empty IRI in annotation value <>".to_string()));
        }
        return Ok(Some(iri_to_turtle_term(iri, namespaces)?));
    }
    if let Some(curie) = known_curie_term(trimmed, namespaces) {
        return Ok(Some(curie.to_string()));
    }
    Ok(None)
}

fn create_entity(
    text: &mut String,
    entity_iri: &str,
    kind: PatchEntityKind,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if text_contains_entity(text, entity_iri, namespaces) {
        return Err(OwlError::EntityExists(entity_iri.to_string()));
    }
    let subject = iri_to_turtle_term(entity_iri, namespaces)?;
    let type_term = match kind {
        PatchEntityKind::Class => "owl:Class",
        PatchEntityKind::ObjectProperty => "owl:ObjectProperty",
        PatchEntityKind::DataProperty => "owl:DatatypeProperty",
        PatchEntityKind::AnnotationProperty => "owl:AnnotationProperty",
        PatchEntityKind::Individual => "owl:NamedIndividual",
        PatchEntityKind::Datatype => "rdfs:Datatype",
    };
    let block = format!("\n{subject} a {type_term} .\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
    text.push_str(&block);
    Ok(())
}

fn delete_entity(
    text: &mut String,
    entity_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let namespaces = crate::span::namespaces_for_text(text, namespaces);
    let short = short_name_from_iri(entity_iri);
    let mut ranges = all_entity_statement_ranges(text, entity_iri, &short, &namespaces);
    if ranges.is_empty() {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    ranges.sort_by_key(|r| r.start);
    for range in ranges.into_iter().rev() {
        replace_range(text, range, "");
    }
    Ok(())
}

fn add_annotation_triple(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if !text_contains_entity(text, entity_iri, namespaces) {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    let escaped = escape_turtle_string(value);
    let triple = format!("    {predicate} \"{escaped}\" ;\n");
    insert_into_entity_block(text, entity_iri, &triple, namespaces, true)
}

fn add_object_triple(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    object: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if !text_contains_entity(text, entity_iri, namespaces) {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    let triple = format!("    {predicate} {object} ;\n");
    insert_into_entity_block(text, entity_iri, &triple, namespaces, false)
}

fn add_complex_axiom(
    text: &mut String,
    entity_iri: &str,
    manchester: &str,
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let ns = crate::span::namespaces_for_text(text, namespaces);
    let parsed = parse_class_expression(manchester, &ns)?;
    let triple = class_expression_to_turtle_fragment(&parsed.expression, predicate, &ns)?;
    insert_multiline_into_entity_block(text, entity_iri, &triple, namespaces)
}

fn remove_complex_axiom(
    text: &mut String,
    entity_iri: &str,
    manchester: &str,
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let ns = crate::span::namespaces_for_text(text, namespaces);
    let parsed = parse_class_expression(manchester, &ns)?;
    let object_value =
        crate::manchester::class_expression_to_turtle_value(&parsed.expression, &ns, 0)?;
    remove_predicate_object(text, entity_iri, predicate, &object_value, namespaces)
}

fn insert_multiline_into_entity_block(
    text: &mut String,
    entity_iri: &str,
    insertion: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let range = entity_primary_range(text, entity_iri, namespaces)?;
    let block = &text[range.start as usize..range.end as usize];
    let trimmed = block.trim_end();
    let mut new_block = block.to_string();
    if trimmed.ends_with('.') {
        if let Some(pos) = new_block.trim_end().rfind('.') {
            let base = new_block[..pos].trim_end();
            new_block = format!("{base} ;\n{insertion}.");
        }
    } else if !trimmed.ends_with(';') {
        new_block.push_str(" ;\n");
        new_block.push_str(insertion);
    } else {
        new_block.push_str(insertion);
    }
    replace_range(text, range, &new_block);
    Ok(())
}

fn add_subclass_triple(
    text: &mut String,
    entity_iri: &str,
    parent_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let parent = iri_to_turtle_term(parent_iri, namespaces)?;
    add_object_triple(text, entity_iri, "rdfs:subClassOf", &parent, namespaces)
}

fn predicate_to_term(predicate: &str, namespaces: &BTreeMap<String, String>) -> Result<String> {
    if let Some(curie) = known_curie_term(predicate, namespaces) {
        Ok(curie.to_string())
    } else {
        // Full IRIs and non-CURIE terms go through IRI safety checks. Malformed
        // CURIE-shaped strings (injection payloads) fail `is_safe_iri` here instead
        // of being emitted verbatim.
        iri_to_turtle_term(predicate, namespaces)
    }
}

/// Return `term` when it is a known-prefix CURIE with a safe Turtle PN_LOCAL.
///
/// Rejects `http:`/`https:` (those are IRIs), unknown prefixes, and locals with
/// characters that would break out of a Turtle predicate position.
fn known_curie_term<'a>(term: &'a str, namespaces: &BTreeMap<String, String>) -> Option<&'a str> {
    let (prefix, local) = term.split_once(':')?;
    if prefix.is_empty()
        || local.is_empty()
        || prefix.eq_ignore_ascii_case("http")
        || prefix.eq_ignore_ascii_case("https")
        || !namespaces.contains_key(prefix)
        || !prefix.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        || !is_valid_pn_local(local)
    {
        return None;
    }
    Some(term)
}

fn set_property_characteristic(
    text: &mut String,
    entity_iri: &str,
    characteristic: &str,
    value: bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if value {
        add_type_triple(text, entity_iri, characteristic, namespaces)
    } else {
        remove_type_triple(text, entity_iri, characteristic, namespaces)
    }
}

fn add_type_triple(
    text: &mut String,
    entity_iri: &str,
    type_term: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if !text_contains_entity(text, entity_iri, namespaces) {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    let subject = iri_to_turtle_term(entity_iri, namespaces)?;
    let ns = crate::span::namespaces_for_text(text, namespaces);
    if let Some(range) = entity_primary_block_range(text, entity_iri, &ns) {
        let block = &text[range.start as usize..range.end as usize];
        if block_has_type_term(block, type_term) {
            return Ok(());
        }
        let insertion = format!("    a {type_term} ;\n");
        return insert_into_entity_block(text, entity_iri, &insertion, namespaces, false);
    }
    // Trailing `subject a type .` form
    let triple = format!("\n{subject} a {type_term} .\n");
    if !text.ends_with('\n') {
        text.push('\n');
    }
    text.push_str(&triple);
    Ok(())
}

/// True when `block` already has `type_term` as an `a` / `rdf:type` object (token-aware).
fn block_has_type_term(block: &str, type_term: &str) -> bool {
    let norm = normalize_ws(type_term.trim());
    for predicate in ["a", "rdf:type"] {
        let mut search_from = 0;
        while let Some(pred_pos) = find_predicate_token(block, search_from, predicate) {
            for (obj_start, obj_end) in objects_in_predicate_value(block, pred_pos, predicate) {
                if normalize_ws(block[obj_start..obj_end].trim()) == norm {
                    return true;
                }
            }
            search_from = pred_pos + 1;
        }
    }
    false
}

fn remove_type_triple(
    text: &mut String,
    entity_iri: &str,
    type_term: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let ns = crate::span::namespaces_for_text(text, namespaces);
    if let Some((prefix, local)) = type_term.split_once(':') {
        if !prefix.is_empty() {
            if let Some(base) = ns.get(prefix) {
                let iri = format!("{base}{local}");
                return remove_rdf_type_iri(text, entity_iri, &iri, namespaces);
            }
        }
    }
    if let Some(iri) = type_term.strip_prefix('<').and_then(|s| s.strip_suffix('>')) {
        return remove_rdf_type_iri(text, entity_iri, iri, namespaces);
    }
    let mut last_err = None;
    for pred in ["a", "rdf:type"] {
        match remove_predicate_object_any_statement(text, entity_iri, pred, type_term, namespaces) {
            Ok(()) => return Ok(()),
            Err(e @ OwlError::EntityNotFound(_)) => return Err(e),
            Err(e) => last_err = Some(e),
        }
    }
    Err(last_err.unwrap_or_else(|| {
        OwlError::ManchesterInvalid(format!("no matching type axiom for {type_term}"))
    }))
}

fn remove_rdf_type_iri(
    text: &mut String,
    entity_iri: &str,
    type_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let mut last_err = None;
    for pred in ["a", "rdf:type"] {
        match remove_predicate_iri_object(text, entity_iri, pred, type_iri, namespaces) {
            Ok(()) => return Ok(()),
            Err(e @ OwlError::EntityNotFound(_)) => return Err(e),
            Err(e) => last_err = Some(e),
        }
    }
    Err(last_err.unwrap_or_else(|| {
        OwlError::ManchesterInvalid(format!("no matching type axiom for {type_iri}"))
    }))
}

fn add_property_assertion_triple(
    text: &mut String,
    entity_iri: &str,
    property_term: &str,
    target_term: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if !text_contains_entity(text, entity_iri, namespaces) {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    let triple = format!("    {property_term} {target_term} ;\n");
    insert_into_entity_block(text, entity_iri, &triple, namespaces, false)
}

fn add_data_property_assertion(
    text: &mut String,
    entity_iri: &str,
    property_term: &str,
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if !text_contains_entity(text, entity_iri, namespaces) {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    let escaped = escape_turtle_string(value);
    let triple = format!("    {property_term} \"{escaped}\" ;\n");
    insert_into_entity_block(text, entity_iri, &triple, namespaces, false)
}

fn entity_declared_as(
    text: &str,
    entity_iri: &str,
    owl_type: &str,
    namespaces: &BTreeMap<String, String>,
) -> bool {
    if !text_contains_entity(text, entity_iri, namespaces) {
        return false;
    }
    let namespaces = crate::span::namespaces_for_text(text, namespaces);
    let short = short_name_from_iri(entity_iri);
    all_entity_statement_ranges(text, entity_iri, &short, &namespaces).into_iter().any(|range| {
        let block = &text[range.start as usize..range.end as usize];
        block_has_type_term(block, owl_type)
            || ({
                // Also accept absolute Ontology-style type IRIs for common owl:* terms.
                if let Some((prefix, local)) = owl_type.split_once(':') {
                    if let Some(base) = namespaces.get(prefix) {
                        let abs = format!("<{base}{local}>");
                        block_has_type_term(block, &abs)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
    })
}

fn validate_property_chain_members(
    text: &str,
    properties: &[String],
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    const INVALID_TYPES: &[&str] =
        &["owl:Class", "owl:NamedIndividual", "owl:DatatypeProperty", "owl:AnnotationProperty"];
    for iri in properties {
        for owl_type in INVALID_TYPES {
            if entity_declared_as(text, iri, owl_type, namespaces) {
                return Err(OwlError::PatchInvalid(format!(
                    "property chain member {iri} is declared as {owl_type}, expected owl:ObjectProperty"
                )));
            }
        }
    }
    Ok(())
}

fn add_property_chain(
    text: &mut String,
    entity_iri: &str,
    properties: &[String],
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if properties.is_empty() {
        return Err(OwlError::PatchInvalid(
            "property chain must have at least one property".into(),
        ));
    }
    validate_property_chain_members(text, properties, namespaces)?;
    let terms: Vec<String> =
        properties.iter().map(|p| iri_to_turtle_term(p, namespaces)).collect::<Result<Vec<_>>>()?;
    let chain_obj = format!("( {} )", terms.join(" "));
    add_object_triple(text, entity_iri, "owl:propertyChainAxiom", &chain_obj, namespaces)
}

fn remove_property_chain(
    text: &mut String,
    entity_iri: &str,
    properties: &[String],
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let terms: Vec<String> =
        properties.iter().map(|p| iri_to_turtle_term(p, namespaces)).collect::<Result<Vec<_>>>()?;
    let chain_obj = format!("( {} )", terms.join(" "));
    remove_predicate_object_any_statement(
        text,
        entity_iri,
        "owl:propertyChainAxiom",
        &chain_obj,
        namespaces,
    )
}

fn add_iri_list_axiom(
    text: &mut String,
    subject_iri: &str,
    predicate: &str,
    members: &[String],
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if members.is_empty() {
        return Err(OwlError::PatchInvalid(format!(
            "{predicate} axiom must have at least one member"
        )));
    }
    let terms: Vec<String> =
        members.iter().map(|p| iri_to_turtle_term(p, namespaces)).collect::<Result<Vec<_>>>()?;
    let list_obj = format!("( {} )", terms.join(" "));
    add_object_triple(text, subject_iri, predicate, &list_obj, namespaces)
}

fn remove_iri_list_axiom(
    text: &mut String,
    subject_iri: &str,
    predicate: &str,
    members: &[String],
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let ns = crate::span::namespaces_for_text(text, namespaces);
    let want: std::collections::BTreeSet<_> = members.iter().cloned().collect();
    // Prefer exact serialized order first (common case).
    let terms: Vec<String> =
        members.iter().map(|p| iri_to_turtle_term(p, &ns)).collect::<Result<Vec<_>>>()?;
    let list_obj = format!("( {} )", terms.join(" "));
    if remove_predicate_object_any_statement(text, subject_iri, predicate, &list_obj, &ns).is_ok() {
        return Ok(());
    }
    // Fall back to set equality so Inspector reorder / reverse property lists match (#351).
    let short = short_name_from_iri(subject_iri);
    let ranges = all_entity_statement_ranges(text, subject_iri, &short, &ns);
    if ranges.is_empty() {
        return Err(OwlError::EntityNotFound(subject_iri.to_string()));
    }
    for range in ranges {
        let block = &text[range.start as usize..range.end as usize];
        let mut search_from = 0;
        while let Some(pred_pos) = find_predicate_token(block, search_from, predicate) {
            for (obj_start, obj_end) in objects_in_predicate_value(block, pred_pos, predicate) {
                let obj = block[obj_start..obj_end].trim();
                if let Some(got) = parse_turtle_iri_list(obj, &ns) {
                    let got_set: std::collections::BTreeSet<_> = got.into_iter().collect();
                    if got_set == want {
                        let (remove_start, remove_end) =
                            extend_removal_span(block, pred_pos, predicate, obj_start, obj_end);
                        let mut new_block = String::new();
                        new_block.push_str(&block[..remove_start]);
                        new_block.push_str(&block[remove_end..]);
                        let cleaned = cleanup_block_separators(&new_block);
                        replace_range(text, range, &cleaned);
                        return Ok(());
                    }
                }
            }
            search_from = pred_pos + 1;
        }
    }
    Err(OwlError::ManchesterInvalid(format!("no matching {predicate} axiom")))
}

fn parse_turtle_iri_list(
    object: &str,
    namespaces: &BTreeMap<String, String>,
) -> Option<Vec<String>> {
    let trimmed = object.trim();
    if !trimmed.starts_with('(') || !trimmed.ends_with(')') {
        return None;
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    let mut members = Vec::new();
    for term in tokenize_turtle_terms(inner) {
        members.push(expand_turtle_term_to_iri(&term, namespaces)?);
    }
    Some(members)
}

fn add_pairwise_property_axioms(
    text: &mut String,
    properties: &[String],
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if properties.len() < 2 {
        return Err(OwlError::PatchInvalid(format!(
            "{predicate} requires at least two properties"
        )));
    }
    // Full pairwise closure (not windows(2)) — required for n-ary Disjoint*Properties (#394).
    for i in 0..properties.len() {
        for j in (i + 1)..properties.len() {
            let other = iri_to_turtle_term(&properties[j], namespaces)?;
            add_object_triple(text, &properties[i], predicate, &other, namespaces)?;
        }
    }
    Ok(())
}

fn remove_pairwise_property_axioms(
    text: &mut String,
    properties: &[String],
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if properties.len() < 2 {
        return Err(OwlError::PatchInvalid(format!(
            "{predicate} requires at least two properties"
        )));
    }
    let mut changed = false;
    for i in 0..properties.len() {
        for j in (i + 1)..properties.len() {
            if remove_predicate_iri_object(
                text,
                &properties[i],
                predicate,
                &properties[j],
                namespaces,
            )
            .is_ok()
            {
                changed = true;
            }
            if remove_predicate_iri_object(
                text,
                &properties[j],
                predicate,
                &properties[i],
                namespaces,
            )
            .is_ok()
            {
                changed = true;
            }
        }
    }
    if changed {
        Ok(())
    } else {
        Err(OwlError::ManchesterInvalid(format!("no matching {predicate} axiom")))
    }
}

fn add_pairwise_individual_axioms(
    text: &mut String,
    individuals: &[String],
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if individuals.len() < 2 {
        return Err(OwlError::PatchInvalid(format!(
            "{predicate} requires at least two individuals"
        )));
    }
    for window in individuals.windows(2) {
        let other = iri_to_turtle_term(&window[1], namespaces)?;
        add_object_triple(text, &window[0], predicate, &other, namespaces)?;
    }
    Ok(())
}

fn remove_pairwise_individual_axioms(
    text: &mut String,
    individuals: &[String],
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if individuals.len() < 2 {
        return Err(OwlError::PatchInvalid(format!(
            "{predicate} requires at least two individuals"
        )));
    }
    for window in individuals.windows(2) {
        remove_predicate_iri_object(text, &window[0], predicate, &window[1], namespaces)?;
    }
    Ok(())
}

/// Serialize DifferentIndividuals as OWL 2 RDF `owl:AllDifferent` + `owl:distinctMembers`.
fn add_different_individuals(
    text: &mut String,
    individuals: &[String],
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if individuals.len() < 2 {
        return Err(OwlError::PatchInvalid(
            "owl:AllDifferent requires at least two individuals".into(),
        ));
    }
    let ns = namespaces_for_text(text, namespaces);
    let terms: Vec<String> =
        individuals.iter().map(|i| iri_to_turtle_term(i, &ns)).collect::<Result<_>>()?;
    let list = format!("( {} )", terms.join(" "));
    let block = format!("[] a owl:AllDifferent ;\n    owl:distinctMembers {list} .\n");
    if all_different_block_covers(text, individuals, &ns, /*exact*/ true)? {
        return Ok(());
    }
    // Also skip if a bracket-style AllDifferent with the same member set already exists.
    if normalize_ws(text).contains(&normalize_ws(&block)) {
        return Ok(());
    }
    append_standalone_block(text, &block);
    Ok(())
}

fn remove_different_individuals(
    text: &mut String,
    individuals: &[String],
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if individuals.len() < 2 {
        return Err(OwlError::PatchInvalid(
            "remove_different_individuals requires at least two individuals".into(),
        ));
    }
    let ns = namespaces_for_text(text, namespaces);
    let mut changed = false;
    // Prefer rewriting/removing owl:AllDifferent (+ owl:members / owl:distinctMembers).
    while remove_or_rewrite_one_all_different(text, individuals, &ns)? {
        changed = true;
    }
    // Also clear legacy pairwise owl:differentFrom (full pairwise closure, not windows).
    for i in 0..individuals.len() {
        for j in (i + 1)..individuals.len() {
            if remove_predicate_iri_object(
                text,
                &individuals[i],
                "owl:differentFrom",
                &individuals[j],
                &ns,
            )
            .is_ok()
            {
                changed = true;
            }
            if remove_predicate_iri_object(
                text,
                &individuals[j],
                "owl:differentFrom",
                &individuals[i],
                &ns,
            )
            .is_ok()
            {
                changed = true;
            }
        }
    }
    if changed {
        return Ok(());
    }
    Err(OwlError::ManchesterInvalid(
        "no matching owl:AllDifferent / owl:differentFrom axiom".into(),
    ))
}

fn all_different_block_covers(
    text: &str,
    individuals: &[String],
    namespaces: &BTreeMap<String, String>,
    exact: bool,
) -> Result<bool> {
    for (start, end) in all_different_statement_ranges(text) {
        let block = &text[start..end];
        let Some(members) = parse_all_different_members(block, namespaces)? else {
            continue;
        };
        let member_set: std::collections::BTreeSet<_> = members.iter().cloned().collect();
        let want: std::collections::BTreeSet<_> = individuals.iter().cloned().collect();
        if exact {
            if member_set == want {
                return Ok(true);
            }
        } else if want.iter().all(|i| member_set.contains(i)) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn remove_or_rewrite_one_all_different(
    text: &mut String,
    individuals: &[String],
    namespaces: &BTreeMap<String, String>,
) -> Result<bool> {
    let want: std::collections::BTreeSet<_> = individuals.iter().cloned().collect();
    let ranges = all_different_statement_ranges(text);
    for (start, end) in ranges {
        let block = text[start..end].to_string();
        let Some(members) = parse_all_different_members(&block, namespaces)? else {
            continue;
        };
        let member_set: std::collections::BTreeSet<_> = members.iter().cloned().collect();
        if !want.iter().all(|i| member_set.contains(i)) {
            continue;
        }
        let remaining: Vec<String> = members.into_iter().filter(|m| !want.contains(m)).collect();
        if remaining.len() < 2 {
            // Drop the whole axiom (also covers exact-match remove).
            let mut remove_start = start;
            let mut remove_end = end;
            if remove_start > 0 && text.as_bytes()[remove_start - 1] == b'\n' {
                remove_start -= 1;
            }
            if remove_end < text.len() && text.as_bytes()[remove_end] == b'\n' {
                remove_end += 1;
            }
            text.replace_range(remove_start..remove_end, "");
            return Ok(true);
        }
        // Rewrite distinctMembers list in place.
        let terms: Vec<String> =
            remaining.iter().map(|i| iri_to_turtle_term(i, namespaces)).collect::<Result<_>>()?;
        let new_list = format!("( {} )", terms.join(" "));
        let rewritten = rewrite_all_different_members_list(&block, &new_list).ok_or_else(|| {
            OwlError::PatchInvalid("failed to rewrite AllDifferent members".into())
        })?;
        text.replace_range(start..end, &rewritten);
        return Ok(true);
    }
    Ok(false)
}

/// Ranges covering `[] a owl:AllDifferent … .` statements and `[ … AllDifferent … ] .` blocks.
fn all_different_statement_ranges(text: &str) -> Vec<(usize, usize)> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    let mut state = TurtleScanState::default();
    while i < bytes.len() {
        if state.in_comment || state.in_string() || state.in_iri {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }
        // Subject-style blank node (`[] a owl:AllDifferent … .`) — check before empty `[]`
        // is swallowed as a bracket pair.
        if bytes.get(i..i + 2) == Some(b"[]") {
            let before_ok = i == 0
                || bytes[i - 1].is_ascii_whitespace()
                || matches!(bytes[i - 1], b';' | b',' | b'.');
            if before_ok {
                if let Some(end) = statement_end_byte(text, i) {
                    let stmt = &text[i..end];
                    if stmt.contains("owl:AllDifferent") {
                        out.push((i, end));
                    }
                    i = end;
                    continue;
                }
            }
        }
        // Bracket-style blank node: `[ rdf:type owl:AllDifferent ; … ]`
        if bytes[i] == b'[' {
            if let Some(end) = bracket_end_index(text, i) {
                let block = &text[i..end];
                if block.contains("owl:AllDifferent") {
                    let mut remove_end = end;
                    let after = text[end..].trim_start();
                    let trim_len = text[end..].len() - after.len();
                    remove_end += trim_len;
                    if after.starts_with('.') {
                        remove_end += 1;
                    }
                    out.push((i, remove_end));
                }
                i = end;
                continue;
            }
        }
        i = advance_turtle_scan(bytes, i, &mut state);
    }
    out
}

fn parse_all_different_members(
    block: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<Option<Vec<String>>> {
    let lower_pred = if block.contains("owl:distinctMembers") {
        "owl:distinctMembers"
    } else if block.contains("owl:members") {
        "owl:members"
    } else {
        return Ok(None);
    };
    let Some(pred_at) = block.find(lower_pred) else {
        return Ok(None);
    };
    let after = &block[pred_at + lower_pred.len()..];
    let Some(paren) = after.find('(') else {
        return Ok(None);
    };
    let list_start = pred_at + lower_pred.len() + paren;
    let Some(list_end) = matching_paren_end(block, list_start) else {
        return Ok(None);
    };
    let inner = &block[list_start + 1..list_end];
    let mut members = Vec::new();
    for term in tokenize_turtle_terms(inner) {
        let Some(iri) = expand_turtle_term_to_iri(&term, namespaces) else {
            return Err(OwlError::PatchInvalid(format!(
                "AllDifferent member cannot be expanded (undeclared prefix?): {term}"
            )));
        };
        members.push(iri);
    }
    Ok(Some(members))
}

fn rewrite_all_different_members_list(block: &str, new_list: &str) -> Option<String> {
    let pred = if block.contains("owl:distinctMembers") {
        "owl:distinctMembers"
    } else if block.contains("owl:members") {
        "owl:members"
    } else {
        return None;
    };
    let pred_at = block.find(pred)?;
    let after = &block[pred_at + pred.len()..];
    let paren = after.find('(')?;
    let list_start = pred_at + pred.len() + paren;
    let list_end = matching_paren_end(block, list_start)?;
    let mut out = String::with_capacity(block.len());
    out.push_str(&block[..list_start]);
    out.push_str(new_list);
    out.push_str(&block[list_end + 1..]);
    Some(out)
}

fn matching_paren_end(text: &str, open: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    if bytes.get(open) != Some(&b'(') {
        return None;
    }
    let mut depth = 0i32;
    let mut state = TurtleScanState::default();
    let mut i = open;
    while i < bytes.len() {
        if state.in_comment || state.in_string() || state.in_iri {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }
        match bytes[i] {
            b'(' => {
                depth += 1;
                i += 1;
            }
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
                i += 1;
            }
            b'<' | b'"' | b'\'' | b'#' => {
                i = advance_turtle_scan(bytes, i, &mut state);
            }
            _ => i += 1,
        }
    }
    None
}

fn tokenize_turtle_terms(inner: &str) -> Vec<String> {
    let bytes = inner.as_bytes();
    let mut terms = Vec::new();
    let mut i = 0;
    let mut state = TurtleScanState::default();
    while i < bytes.len() {
        if bytes[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if state.in_comment || state.in_string() || state.in_iri {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }
        if bytes[i] == b'<' {
            let start = i;
            state.in_iri = true;
            i += 1;
            while i < bytes.len() && state.in_iri {
                i = advance_turtle_scan(bytes, i, &mut state);
            }
            terms.push(inner[start..i].to_string());
            continue;
        }
        // Prefixed name / blank / other token until whitespace.
        let start = i;
        while i < bytes.len() && !bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        terms.push(inner[start..i].to_string());
    }
    terms
}

fn expand_turtle_term_to_iri(term: &str, namespaces: &BTreeMap<String, String>) -> Option<String> {
    let t = term.trim();
    if t.starts_with('<') && t.ends_with('>') && t.len() >= 2 {
        return Some(t[1..t.len() - 1].to_string());
    }
    if let Some((prefix, local)) = t.split_once(':') {
        if let Some(base) = namespaces.get(prefix) {
            return Some(format!("{base}{local}"));
        }
        // Default prefix `:`
        if prefix.is_empty() {
            if let Some(base) = namespaces.get("") {
                return Some(format!("{base}{local}"));
            }
        }
    }
    None
}

fn append_standalone_block(text: &mut String, block: &str) {
    if !text.is_empty() && !text.ends_with('\n') {
        text.push('\n');
    }
    if !text.is_empty() && !text.ends_with("\n\n") {
        text.push('\n');
    }
    text.push_str(block);
    if !block.ends_with('\n') {
        text.push('\n');
    }
}

fn add_negative_object_property_assertion(
    text: &mut String,
    entity_iri: &str,
    property_iri: &str,
    target_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let source = iri_to_turtle_term(entity_iri, namespaces)?;
    let prop = iri_to_turtle_term(property_iri, namespaces)?;
    let target = iri_to_turtle_term(target_iri, namespaces)?;
    let block = format!(
        "[ rdf:type owl:NegativePropertyAssertion ;\n  owl:sourceIndividual {source} ;\n  owl:assertionProperty {prop} ;\n  owl:targetIndividual {target}\n] .\n"
    );
    if normalize_ws(text).contains(&normalize_ws(&block)) {
        return Ok(());
    }
    append_standalone_block(text, &block);
    Ok(())
}

fn remove_negative_object_property_assertion(
    text: &mut String,
    entity_iri: &str,
    property_iri: &str,
    target_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let source = iri_to_turtle_term(entity_iri, namespaces)?;
    let prop = iri_to_turtle_term(property_iri, namespaces)?;
    let target = iri_to_turtle_term(target_iri, namespaces)?;
    remove_matching_npa_block(text, &source, &prop, Some(&target), None)
}

fn add_negative_data_property_assertion(
    text: &mut String,
    entity_iri: &str,
    property_iri: &str,
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let source = iri_to_turtle_term(entity_iri, namespaces)?;
    let prop = iri_to_turtle_term(property_iri, namespaces)?;
    let escaped = escape_turtle_string(value);
    let block = format!(
        "[ rdf:type owl:NegativePropertyAssertion ;\n  owl:sourceIndividual {source} ;\n  owl:assertionProperty {prop} ;\n  owl:targetValue \"{escaped}\"\n] .\n"
    );
    if normalize_ws(text).contains(&normalize_ws(&block)) {
        return Ok(());
    }
    append_standalone_block(text, &block);
    Ok(())
}

fn remove_negative_data_property_assertion(
    text: &mut String,
    entity_iri: &str,
    property_iri: &str,
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let source = iri_to_turtle_term(entity_iri, namespaces)?;
    let prop = iri_to_turtle_term(property_iri, namespaces)?;
    remove_matching_npa_block(text, &source, &prop, None, Some(value))
}

fn remove_matching_npa_block(
    text: &mut String,
    source: &str,
    prop: &str,
    target_individual: Option<&str>,
    target_value: Option<&str>,
) -> Result<()> {
    let bytes = text.as_bytes();
    let mut i = 0;
    let mut state = TurtleScanState::default();
    while i < bytes.len() {
        if state.in_comment || state.in_string() || state.in_iri {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }
        if bytes[i] == b'[' {
            if let Some(end) = bracket_end_index(text, i) {
                let block = &text[i..end];
                if block.contains("owl:NegativePropertyAssertion")
                    && block.contains(source)
                    && block.contains(prop)
                    && target_individual.map(|t| block.contains(t)).unwrap_or(true)
                    && target_value
                        .map(|v| {
                            let lexical = v.trim().trim_matches('"').trim_matches('\'');
                            block.contains(lexical)
                        })
                        .unwrap_or(true)
                {
                    let mut remove_end = end;
                    let after = text[end..].trim_start();
                    let trim_len = text[end..].len() - after.len();
                    remove_end += trim_len;
                    if after.starts_with('.') {
                        remove_end += 1;
                        let rest = &text[remove_end..];
                        if rest.starts_with('\n') {
                            remove_end += 1;
                        }
                    }
                    // Also drop a leading blank line if present.
                    let mut remove_start = i;
                    if remove_start > 0 && text.as_bytes()[remove_start - 1] == b'\n' {
                        remove_start -= 1;
                    }
                    text.replace_range(remove_start..remove_end, "");
                    return Ok(());
                }
                i = end;
                continue;
            }
        }
        i = advance_turtle_scan(bytes, i, &mut state);
    }
    Err(OwlError::ManchesterInvalid("no matching NegativePropertyAssertion axiom".to_string()))
}

fn add_datatype_definition(
    text: &mut String,
    datatype_iri: &str,
    manchester: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    if !text_contains_entity(text, datatype_iri, namespaces) {
        create_entity(text, datatype_iri, PatchEntityKind::Datatype, namespaces)?;
    } else if !entity_declared_as(text, datatype_iri, "rdfs:Datatype", namespaces) {
        add_type_triple(text, datatype_iri, "rdfs:Datatype", namespaces)?;
    }
    let ns = crate::span::namespaces_for_text(text, namespaces);
    let trimmed = manchester.trim();
    let object = crate::manchester::parse_data_range(trimmed, &ns)
        .and_then(|dr| crate::manchester::data_range_to_turtle_term(&dr, &ns))
        .or_else(|_| iri_to_turtle_term(trimmed, namespaces))?;
    add_object_triple(text, datatype_iri, "owl:equivalentClass", &object, namespaces)
}

fn remove_datatype_definition(
    text: &mut String,
    datatype_iri: &str,
    manchester: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let ns = crate::span::namespaces_for_text(text, namespaces);
    let trimmed = manchester.trim();
    let object = crate::manchester::parse_data_range(trimmed, &ns)
        .and_then(|dr| crate::manchester::data_range_to_turtle_term(&dr, &ns))
        .or_else(|_| iri_to_turtle_term(trimmed, namespaces))?;
    remove_predicate_object_any_statement(
        text,
        datatype_iri,
        "owl:equivalentClass",
        &object,
        namespaces,
    )
}

fn axiom_op_predicate(axiom_op: &str) -> Result<&'static str> {
    Ok(match axiom_op {
        "sub_class_of" => "rdfs:subClassOf",
        "disjoint_with" | "disjoint_class" => "owl:disjointWith",
        "equivalent_class" => "owl:equivalentClass",
        "domain" => "rdfs:domain",
        "range" => "rdfs:range",
        "sub_object_property_of" | "sub_data_property_of" | "sub_property_of" => {
            "rdfs:subPropertyOf"
        }
        "inverse_of" | "inverse_object_properties" => "owl:inverseOf",
        "equivalent_property" | "equivalent_object_properties" | "equivalent_data_properties" => {
            "owl:equivalentProperty"
        }
        "property_disjoint_with" | "disjoint_object_properties" | "disjoint_data_properties" => {
            "owl:propertyDisjointWith"
        }
        "same_as" | "same_individual" => "owl:sameAs",
        "different_from" | "different_individuals" => "owl:differentFrom",
        other => {
            return Err(OwlError::PatchInvalid(format!(
                "unsupported axiom_op for axiom annotation: {other}"
            )));
        }
    })
}

fn add_axiom_annotation(
    text: &mut String,
    axiom_op: &str,
    subject_iri: &str,
    related_iri: Option<&str>,
    predicate: &str,
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let annotated_property = axiom_op_predicate(axiom_op)?;
    let source = iri_to_turtle_term(subject_iri, namespaces)?;
    let target = match related_iri {
        Some(iri) => iri_to_turtle_term(iri, namespaces)?,
        None => {
            return Err(OwlError::PatchInvalid(
                "axiom annotation requires related_iri for the annotated target".into(),
            ));
        }
    };
    let ann_pred = predicate_to_term(predicate, namespaces)?;
    let ann_value = if let Some(obj) = explicit_iri_annotation_term(value, namespaces)? {
        obj
    } else {
        format!("\"{}\"", escape_turtle_string(value))
    };
    let block = format!(
        "[ rdf:type owl:Axiom ;\n  owl:annotatedSource {source} ;\n  owl:annotatedProperty {annotated_property} ;\n  owl:annotatedTarget {target} ;\n  {ann_pred} {ann_value}\n] .\n"
    );
    if normalize_ws(text).contains(&normalize_ws(&block)) {
        return Ok(());
    }
    append_standalone_block(text, &block);
    Ok(())
}

fn remove_axiom_annotation(
    text: &mut String,
    axiom_op: &str,
    subject_iri: &str,
    related_iri: Option<&str>,
    predicate: &str,
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let annotated_property = axiom_op_predicate(axiom_op)?;
    let source = iri_to_turtle_term(subject_iri, namespaces)?;
    let target = match related_iri {
        Some(iri) => iri_to_turtle_term(iri, namespaces)?,
        None => {
            return Err(OwlError::PatchInvalid(
                "axiom annotation requires related_iri for the annotated target".into(),
            ));
        }
    };
    let ann_pred = predicate_to_term(predicate, namespaces)?;
    let lexical = value.trim().trim_matches('"').trim_matches('\'');
    let bytes = text.as_bytes();
    let mut i = 0;
    let mut state = TurtleScanState::default();
    while i < bytes.len() {
        if state.in_comment || state.in_string() || state.in_iri {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }
        if bytes[i] == b'[' {
            if let Some(end) = bracket_end_index(text, i) {
                let block = &text[i..end];
                if block.contains("owl:Axiom")
                    && block.contains(&source)
                    && block.contains(annotated_property)
                    && block.contains(&target)
                    && block.contains(&ann_pred)
                    && axiom_block_has_exact_annotation_value(block, &ann_pred, lexical)
                {
                    let mut remove_end = end;
                    let after = text[end..].trim_start();
                    let trim_len = text[end..].len() - after.len();
                    remove_end += trim_len;
                    if after.starts_with('.') {
                        remove_end += 1;
                        if text[remove_end..].starts_with('\n') {
                            remove_end += 1;
                        }
                    }
                    let mut remove_start = i;
                    if remove_start > 0 && text.as_bytes()[remove_start - 1] == b'\n' {
                        remove_start -= 1;
                    }
                    text.replace_range(remove_start..remove_end, "");
                    return Ok(());
                }
                i = end;
                continue;
            }
        }
        i = advance_turtle_scan(bytes, i, &mut state);
    }
    Err(OwlError::ManchesterInvalid("no matching axiom annotation".to_string()))
}

fn axiom_block_has_exact_annotation_value(block: &str, ann_pred: &str, lexical: &str) -> bool {
    // `find_predicate_token` skips nested `[...]` content; strip the outer blank-node brackets.
    let inner =
        block.strip_prefix('[').and_then(|s| s.trim_end().strip_suffix(']')).unwrap_or(block);
    let norm_value = normalize_ws(lexical);
    let mut search_from = 0;
    while let Some(pred_pos) = find_predicate_token(inner, search_from, ann_pred) {
        for (obj_start, obj_end) in objects_in_predicate_value(inner, pred_pos, ann_pred) {
            let obj_text = inner[obj_start..obj_end].trim();
            if let Some(got) = turtle_literal_lexical_value(obj_text) {
                if normalize_ws(&got) == norm_value {
                    return true;
                }
            } else {
                let bare = obj_text.trim().trim_matches(|c| c == '<' || c == '>');
                if normalize_ws(bare) == norm_value {
                    return true;
                }
            }
        }
        search_from = pred_pos + 1;
    }
    false
}

fn remove_predicate_triples(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    remove_all_predicate_any_statement(text, entity_iri, predicate, namespaces)
}

fn remove_predicate_object(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    object_value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    remove_predicate_object_any_statement(text, entity_iri, predicate, object_value, namespaces)
}

fn remove_matching_predicate_any(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let value = value.trim().trim_matches('"').trim_matches('\'');
    let ns = crate::span::namespaces_for_text(text, namespaces);
    let short = short_name_from_iri(entity_iri);
    let ranges = all_entity_statement_ranges(text, entity_iri, &short, &ns);
    if ranges.is_empty() {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    for range in ranges {
        let block = &text[range.start as usize..range.end as usize];
        if let Some(new_block) = remove_matching_predicate_by_lexical_value(block, predicate, value)
        {
            replace_range(text, range, &new_block);
            return Ok(());
        }
    }
    Err(OwlError::ManchesterInvalid(format!("no matching {predicate} axiom")))
}

fn remove_all_predicate_any_statement(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    loop {
        let ns = crate::span::namespaces_for_text(text, namespaces);
        let short = short_name_from_iri(entity_iri);
        let ranges = all_entity_statement_ranges(text, entity_iri, &short, &ns);
        if ranges.is_empty() {
            return Ok(());
        }
        let mut removed = false;
        for range in ranges {
            let block = &text[range.start as usize..range.end as usize];
            let new_block = remove_all_predicate_objects(block, predicate);
            if new_block != block {
                replace_range(text, range, &new_block);
                removed = true;
                break;
            }
        }
        if !removed {
            return Ok(());
        }
    }
}

fn insert_into_entity_block(
    text: &mut String,
    entity_iri: &str,
    insertion: &str,
    namespaces: &BTreeMap<String, String>,
    duplicate_is_error: bool,
) -> Result<()> {
    if let Some((predicate, object)) = parse_simple_insertion(insertion) {
        if entity_has_predicate_object(text, entity_iri, &predicate, &object, namespaces) {
            if duplicate_is_error {
                return Err(OwlError::PatchInvalid(format!(
                    "duplicate {predicate} axiom already present: {object}"
                )));
            }
            return Ok(());
        }
    }
    // Same single-path insertion as multiline axioms (no double-insert).
    insert_multiline_into_entity_block(text, entity_iri, insertion, namespaces)
}

fn parse_simple_insertion(insertion: &str) -> Option<(String, String)> {
    let line = insertion.lines().next()?.trim().trim_end_matches(';').trim();
    let mut parts = line.splitn(2, char::is_whitespace);
    let predicate = parts.next()?.to_string();
    let object = parts.next()?.trim().to_string();
    if predicate.is_empty() || object.is_empty() {
        return None;
    }
    Some((predicate, object))
}

fn entity_has_predicate_object(
    text: &str,
    entity_iri: &str,
    predicate: &str,
    object_value: &str,
    namespaces: &BTreeMap<String, String>,
) -> bool {
    let ns = crate::span::namespaces_for_text(text, namespaces);
    let short = short_name_from_iri(entity_iri);
    all_entity_statement_ranges(text, entity_iri, &short, &ns).into_iter().any(|range| {
        let block = &text[range.start as usize..range.end as usize];
        block_has_matching_predicate_object(block, predicate, object_value)
    })
}

fn block_has_matching_predicate_object(block: &str, predicate: &str, object_value: &str) -> bool {
    remove_matching_predicate_object(block, predicate, object_value).is_some()
}

fn replace_range(text: &mut String, range: ByteRange, replacement: &str) {
    let start = range.start as usize;
    let end = range.end.min(text.len() as u64) as usize;
    text.replace_range(start..end, replacement);
}

fn escape_turtle_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn entity_primary_range(
    text: &str,
    entity_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<ByteRange> {
    let ns = crate::span::namespaces_for_text(text, namespaces);
    entity_primary_block_range(text, entity_iri, &ns)
        .ok_or_else(|| OwlError::EntityNotFound(entity_iri.to_string()))
}

fn remove_predicate_object_any_statement(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    object_value: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let ns = crate::span::namespaces_for_text(text, namespaces);
    let short = short_name_from_iri(entity_iri);
    let ranges = all_entity_statement_ranges(text, entity_iri, &short, &ns);
    if ranges.is_empty() {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    for range in ranges {
        let block = &text[range.start as usize..range.end as usize];
        if let Some(new_block) = remove_matching_predicate_object(block, predicate, object_value) {
            replace_range(text, range, &new_block);
            return Ok(());
        }
    }
    Err(OwlError::ManchesterInvalid(format!("no matching {predicate} axiom")))
}

/// Remove a predicate/object where the object is an IRI that may appear as CURIE or `<IRI>`.
fn remove_predicate_iri_object(
    text: &mut String,
    entity_iri: &str,
    predicate: &str,
    object_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<()> {
    let ns = crate::span::namespaces_for_text(text, namespaces);
    let forms = iri_turtle_term_forms(object_iri, &ns)?;
    let short = short_name_from_iri(entity_iri);
    let ranges = all_entity_statement_ranges(text, entity_iri, &short, &ns);
    if ranges.is_empty() {
        return Err(OwlError::EntityNotFound(entity_iri.to_string()));
    }
    for range in ranges {
        let block = &text[range.start as usize..range.end as usize];
        for form in &forms {
            if let Some(new_block) = remove_matching_predicate_object(block, predicate, form) {
                replace_range(text, range, &new_block);
                return Ok(());
            }
        }
    }
    Err(OwlError::ManchesterInvalid(format!("no matching {predicate} axiom")))
}

/// Surface forms an IRI may take in Turtle (absolute, CURIE, default-prefix).
fn iri_turtle_term_forms(iri: &str, namespaces: &BTreeMap<String, String>) -> Result<Vec<String>> {
    if !is_safe_iri(iri) {
        return Err(OwlError::PatchInvalid(format!(
            "IRI contains characters that cannot be safely written to Turtle: {iri:?}"
        )));
    }
    let mut forms = vec![format!("<{iri}>")];
    if let Some((prefix, ns)) = best_namespace_match(iri, namespaces) {
        let local = &iri[ns.len()..];
        if is_valid_pn_local(local) {
            let curie = format!("{prefix}:{local}");
            if !forms.iter().any(|f| f == &curie) {
                forms.push(curie);
            }
        }
    }
    if let Some(default_ns) = namespaces.get("") {
        if iri.starts_with(default_ns.as_str()) {
            let local = &iri[default_ns.len()..];
            if is_valid_pn_local(local) {
                let term = format!(":{local}");
                if !forms.iter().any(|f| f == &term) {
                    forms.push(term);
                }
            }
        }
    }
    if iri == "http://www.w3.org/2002/07/owl#Thing" {
        let thing = "owl:Thing".to_string();
        if !forms.iter().any(|f| f == &thing) {
            forms.push(thing);
        }
    }
    Ok(forms)
}

fn normalize_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn bracket_end_index(text: &str, bracket_start: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    if bytes.get(bracket_start) != Some(&b'[') {
        return None;
    }
    let mut depth = 0i32;
    let mut state = TurtleScanState::default();
    let mut i = bracket_start;
    while i < bytes.len() {
        if state.in_comment || state.in_string() || state.in_iri {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }
        if is_turtle_lex_start(bytes, i) {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }
        match bytes[i] {
            b'[' => {
                depth += 1;
                i += 1;
            }
            b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i + 1);
                }
                i += 1;
            }
            _ => i += 1,
        }
    }
    None
}

fn is_turtle_lex_start(bytes: &[u8], i: usize) -> bool {
    matches!(bytes.get(i), Some(b'#' | b'"' | b'\'' | b'<'))
        || bytes.get(i..i + 3) == Some(br#"""""#)
        || bytes.get(i..i + 3) == Some(br"'''")
}

/// Extend removal to cover the object, and the predicate when it would be left empty.
fn extend_removal_span(
    block: &str,
    pred_pos: usize,
    predicate: &str,
    obj_start: usize,
    obj_end: usize,
) -> (usize, usize) {
    let objects = objects_in_predicate_value(block, pred_pos, predicate);
    let is_only_object = objects.len() == 1;

    let mut start = if is_only_object { pred_pos } else { obj_start };
    while start > 0 && block.as_bytes()[start - 1].is_ascii_whitespace() {
        start -= 1;
    }
    if !is_only_object && start > 0 && block.as_bytes()[start - 1] == b',' {
        start -= 1;
        while start > 0 && block.as_bytes()[start - 1].is_ascii_whitespace() {
            start -= 1;
        }
    }

    let mut end = obj_end;
    while end < block.len() && block.as_bytes()[end].is_ascii_whitespace() {
        end += 1;
    }
    if end < block.len() && (block.as_bytes()[end] == b',' || block.as_bytes()[end] == b';') {
        end += 1;
    }
    (start, end)
}

fn cleanup_block_separators(block: &str) -> String {
    let mut lines: Vec<&str> = block.lines().collect();
    while lines.last().is_some_and(|l| l.trim().is_empty()) {
        lines.pop();
    }
    lines.join("\n").replace(";\n    ;", ";\n").replace(",\n        ,", ",\n")
}

fn remove_all_predicate_objects(block: &str, predicate: &str) -> String {
    let mut result = block.to_string();
    while let Some(next) = remove_first_predicate_object(&result, predicate) {
        result = next;
    }
    cleanup_block_separators(&result)
}

fn remove_first_predicate_object(block: &str, predicate: &str) -> Option<String> {
    let pred_pos = find_predicate_token(block, 0, predicate)?;
    let (obj_start, obj_end) =
        objects_in_predicate_value(block, pred_pos, predicate).first().copied()?;
    let (remove_start, remove_end) =
        extend_removal_span(block, pred_pos, predicate, obj_start, obj_end);
    let mut out = String::new();
    out.push_str(&block[..remove_start]);
    out.push_str(&block[remove_end..]);
    Some(cleanup_block_separators(&out))
}

/// Find `predicate` as a Turtle token outside strings, IRIs, comments, and brackets.
fn find_predicate_token(block: &str, search_from: usize, predicate: &str) -> Option<usize> {
    let bytes = block.as_bytes();
    let pred_bytes = predicate.as_bytes();
    if pred_bytes.is_empty() || search_from >= bytes.len() {
        return None;
    }
    let mut i = search_from;
    let mut state = TurtleScanState::default();
    let mut bracket_depth = 0i32;
    while i + pred_bytes.len() <= bytes.len() {
        if state.in_comment || state.in_string() || state.in_iri {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }
        if is_turtle_lex_start(bytes, i) {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }
        match bytes[i] {
            b'[' => {
                bracket_depth += 1;
                i += 1;
                continue;
            }
            b']' => {
                bracket_depth = bracket_depth.saturating_sub(1);
                i += 1;
                continue;
            }
            _ => {}
        }
        if bracket_depth == 0 && bytes[i..].starts_with(pred_bytes) {
            let after = i + pred_bytes.len();
            let before_ok = i == 0
                || !bytes[i - 1].is_ascii_alphanumeric()
                    && bytes[i - 1] != b':'
                    && bytes[i - 1] != b'_';
            let after_ok = after >= bytes.len()
                || bytes[after].is_ascii_whitespace()
                || bytes[after] == b';'
                || bytes[after] == b'.'
                || bytes[after] == b',';
            if before_ok && after_ok {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

fn find_named_object_end(block: &str, obj_start: usize) -> Option<usize> {
    use crate::span::is_turtle_terminating_dot;
    let bytes = block.as_bytes();
    let mut i = obj_start;
    let mut state = TurtleScanState::default();
    while i < bytes.len() {
        if state.in_comment || state.in_string() || state.in_iri {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }
        if is_turtle_lex_start(bytes, i) {
            i = advance_turtle_scan(bytes, i, &mut state);
            continue;
        }
        match bytes[i] {
            b',' | b';' => return Some(i),
            b'.' if is_turtle_terminating_dot(bytes, i) => return Some(i),
            _ => i += 1,
        }
    }
    Some(block.len())
}

fn objects_in_predicate_value(
    block: &str,
    pred_pos: usize,
    predicate: &str,
) -> Vec<(usize, usize)> {
    let list_start = pred_pos + predicate.len();
    let mut objects = Vec::new();
    let mut i = list_start;
    loop {
        let rest = block.get(i..).unwrap_or("").trim_start();
        if rest.is_empty() || rest.starts_with(';') || rest.starts_with('.') {
            break;
        }
        i += block[i..].len() - rest.len();
        if block.as_bytes().get(i) == Some(&b'[') {
            if let Some(end) = bracket_end_index(block, i) {
                objects.push((i, end));
                i = end;
            } else {
                break;
            }
        } else {
            let end = find_named_object_end(block, i).unwrap_or(block.len());
            objects.push((i, end));
            i = end;
        }
        let rest = block.get(i..).unwrap_or("").trim_start();
        i += block[i..].len() - rest.len();
        if rest.starts_with(',') {
            i += 1;
        } else {
            break;
        }
    }
    objects
}

fn remove_matching_predicate_object(
    block: &str,
    predicate: &str,
    object_value: &str,
) -> Option<String> {
    let obj_trim = object_value.trim();
    let norm_obj = normalize_ws(obj_trim);
    let mut search_from = 0;
    while let Some(pred_pos) = find_predicate_token(block, search_from, predicate) {
        for (obj_start, obj_end) in objects_in_predicate_value(block, pred_pos, predicate) {
            let candidate = normalize_ws(block[obj_start..obj_end].trim());
            if candidate == norm_obj {
                let (remove_start, remove_end) =
                    extend_removal_span(block, pred_pos, predicate, obj_start, obj_end);
                let mut out = String::new();
                out.push_str(&block[..remove_start]);
                out.push_str(&block[remove_end..]);
                return Some(cleanup_block_separators(&out));
            }
        }
        search_from = pred_pos + 1;
    }
    None
}

fn remove_matching_predicate_by_lexical_value(
    block: &str,
    predicate: &str,
    value: &str,
) -> Option<String> {
    let norm_value = normalize_ws(value);
    let mut search_from = 0;
    while let Some(pred_pos) = find_predicate_token(block, search_from, predicate) {
        for (obj_start, obj_end) in objects_in_predicate_value(block, pred_pos, predicate) {
            let obj_text = block[obj_start..obj_end].trim();
            if let Some(lexical) = turtle_literal_lexical_value(obj_text) {
                if normalize_ws(&lexical) == norm_value {
                    let (remove_start, remove_end) =
                        extend_removal_span(block, pred_pos, predicate, obj_start, obj_end);
                    let mut out = String::new();
                    out.push_str(&block[..remove_start]);
                    out.push_str(&block[remove_end..]);
                    return Some(cleanup_block_separators(&out));
                }
            }
        }
        search_from = pred_pos + 1;
    }
    None
}

fn text_contains_entity(
    text: &str,
    entity_iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> bool {
    let namespaces = crate::span::namespaces_for_text(text, namespaces);
    let mut needles = vec![entity_iri.to_string(), format!("<{entity_iri}>")];
    if let Some(default_ns) = namespaces.get("") {
        if entity_iri.starts_with(default_ns.as_str()) {
            let local = &entity_iri[default_ns.len()..];
            if is_valid_pn_local(local) {
                needles.push(format!(":{local}"));
            }
        }
    }
    if let Some((prefix, ns)) = best_namespace_match(entity_iri, &namespaces) {
        let local = &entity_iri[ns.len()..];
        if is_valid_pn_local(local) {
            needles.push(format!("{prefix}:{local}"));
        }
    }
    text.lines().any(|line| {
        let trimmed = line.trim_start();
        needles.iter().any(|needle| line_starts_with_subject(trimmed, needle))
    })
}

fn line_starts_with_subject(trimmed: &str, subject: &str) -> bool {
    trimmed == subject
        || trimmed.starts_with(&format!("{subject} "))
        || trimmed.starts_with(&format!("{subject}\t"))
        || trimmed.starts_with(&format!("{subject};"))
        || trimmed.starts_with(&format!("{subject}."))
}

/// Reject IRIs that would break Turtle `<...>` terms or inject syntax.
pub fn is_safe_iri(iri: &str) -> bool {
    if iri.is_empty() {
        return false;
    }
    !iri.chars().any(|c| {
        c.is_control()
            || c.is_whitespace()
            || matches!(c, '<' | '>' | '"' | '{' | '}' | '|' | '^' | '`' | '\\')
    })
}

/// True when `local` is a valid Turtle PN_LOCAL (simplified).
pub(crate) fn is_valid_pn_local(local: &str) -> bool {
    if local.is_empty() {
        return false;
    }
    local.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '~'))
        && !local.starts_with('.')
        && !local.ends_with('.')
}

fn iri_to_turtle_term(iri: &str, namespaces: &BTreeMap<String, String>) -> Result<String> {
    iri_to_turtle_term_impl(iri, namespaces)
}

pub(crate) fn iri_to_turtle_term_impl(
    iri: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<String> {
    if !is_safe_iri(iri) {
        return Err(OwlError::PatchInvalid(format!(
            "IRI contains characters that cannot be safely written to Turtle: {iri:?}"
        )));
    }
    if iri == "http://www.w3.org/2002/07/owl#Thing" {
        return Ok("owl:Thing".to_string());
    }

    if let Some((prefix, ns)) = best_namespace_match(iri, namespaces) {
        let local = &iri[ns.len()..];
        if is_valid_pn_local(local) {
            return Ok(format!("{prefix}:{local}"));
        }
    }
    Ok(format!("<{iri}>"))
}

pub(crate) fn best_namespace_match<'a>(
    iri: &str,
    namespaces: &'a BTreeMap<String, String>,
) -> Option<(&'a str, &'a str)> {
    let mut best: Option<(&str, &str, usize)> = None;
    for (prefix, ns) in namespaces {
        if prefix.is_empty() || !iri.starts_with(ns.as_str()) {
            continue;
        }
        let len = ns.len();
        if best.as_ref().is_none_or(|(_, _, best_len)| len > *best_len) {
            best = Some((prefix.as_str(), ns.as_str(), len));
        }
    }
    best.map(|(prefix, ns, _)| (prefix, ns))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex_ns() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("ex".to_string(), "http://example.org/people#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
            ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ])
    }

    // Catalog/Horned reparse oracles for success-path patches live in
    // tests/owl_patch_oracles.rs (apply_and_reindex).

    #[test]
    fn add_label_to_existing_class() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![PatchOp::AddLabel {
            entity_iri: "http://example.org/people#Person".to_string(),
            value: "Human".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert_eq!(
            preview.matches("rdfs:label \"Human\"").count(),
            1,
            "must insert label exactly once"
        );
    }

    #[test]
    fn add_label_not_blocked_by_label_text_in_long_comment() {
        let ttl = r#"@prefix ex: <http://example.org/ex#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ex:Foo a owl:Class ;
    rdfs:comment """Documentation mentions rdfs:label \"Bar\" syntax.""" .
"#;
        let ns = BTreeMap::from([
            ("ex".to_string(), "http://example.org/ex#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
            ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ]);
        let patches = vec![PatchOp::AddLabel {
            entity_iri: "http://example.org/ex#Foo".to_string(),
            value: "Bar".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ns).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("rdfs:label \"Bar\""));
    }

    #[test]
    fn add_label_duplicate_returns_error() {
        let ttl = r#"@prefix ex: <http://example.org/ex#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ex:Foo a owl:Class ;
    rdfs:label "Bar" .
"#;
        let ns = BTreeMap::from([
            ("ex".to_string(), "http://example.org/ex#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
            ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ]);
        let patches = vec![PatchOp::AddLabel {
            entity_iri: "http://example.org/ex#Foo".to_string(),
            value: "Bar".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ns).expect("patch");
        assert!(!result.applied);
        assert!(result.diagnostics.iter().any(|d| d.message.contains("duplicate")));
    }

    #[test]
    fn remove_subclass_does_not_leave_orphaned_predicate() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![PatchOp::RemoveSubClassOf {
            entity_iri: "http://example.org/people#Person".to_string(),
            parent_iri: "http://example.org/people#Thing".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("rdfs:subClassOf."));
        assert!(!preview.contains("rdfs:subClassOf ;"));
        assert!(!preview.contains("ex:Person rdfs:subClassOf"));
    }

    #[test]
    fn add_subclass_of_no_duplicate_when_trailing_triple_exists() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![PatchOp::AddSubClassOf {
            entity_iri: "http://example.org/people#Person".to_string(),
            parent_iri: "http://example.org/people#Thing".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        assert!(
            result.preview_text.is_none(),
            "must not duplicate subclass axiom already present as trailing triple"
        );
    }

    #[test]
    fn remove_import_from_trailing_triple() {
        let ttl = r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .

<http://example.org/people> a owl:Ontology .
<http://example.org/people> owl:imports <http://example.org/other> .
"#;
        let ns = ex_ns();
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::RemoveImport {
                ontology_iri: "http://example.org/people".to_string(),
                import_iri: "http://example.org/other".to_string(),
            }],
            true,
            &ns,
        )
        .expect("remove trailing import");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("owl:imports"));
    }

    #[test]
    fn remove_label_from_single_quoted_literal() {
        let ttl = r#"@prefix ex: <http://example.org/people#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class ;
    rdfs:label 'Human' .
"#;
        let patches = vec![PatchOp::RemoveLabel {
            entity_iri: "http://example.org/people#Person".to_string(),
            value: "Human".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("rdfs:label 'Human'"));
        assert!(!preview.contains("'Human'"));
        assert!(preview.contains("ex:Person a owl:Class"));
    }

    #[test]
    fn remove_comment_from_long_single_quoted_literal() {
        let ttl = r#"@prefix ex: <http://example.org/people#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class ;
    rdfs:comment '''A human being.''' .
"#;
        let patches = vec![PatchOp::RemoveComment {
            entity_iri: "http://example.org/people#Person".to_string(),
            value: "A human being.".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("rdfs:comment '''A human being.'''"));
        assert!(preview.contains("ex:Person a owl:Class"));
    }

    #[test]
    fn remove_label_ignores_predicate_inside_long_single_quoted_comment() {
        let ttl = r#"@prefix ex: <http://example.org/people#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class ;
    rdfs:comment '''see rdfs:label usage''' ;
    rdfs:label "Name" .
"#;
        let patches = vec![PatchOp::RemoveLabel {
            entity_iri: "http://example.org/people#Person".to_string(),
            value: "Name".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("see rdfs:label usage"));
        assert!(!preview.contains("rdfs:label \"Name\""));
    }

    #[test]
    fn remove_comment_with_period_in_literal() {
        let ttl = r#"@prefix ex: <http://example.org/people#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class ;
    rdfs:comment "A human being." .
"#;
        let patches = vec![PatchOp::RemoveComment {
            entity_iri: "http://example.org/people#Person".to_string(),
            value: "A human being.".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("A human being."));
        assert!(!preview.contains("rdfs:comment"));
        assert!(preview.contains("ex:Person a owl:Class"));
    }

    #[test]
    fn remove_label_ignores_predicate_name_inside_comment() {
        let ttl = r#"@prefix ex: <http://example.org/people#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Person a owl:Class ;
    rdfs:comment "see rdfs:label usage" ;
    rdfs:label "Name" .
"#;
        let patches = vec![PatchOp::RemoveLabel {
            entity_iri: "http://example.org/people#Person".to_string(),
            value: "Name".to_string(),
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("see rdfs:label usage"));
        assert!(!preview.contains("rdfs:label \"Name\""));
    }

    #[test]
    fn create_new_class() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![PatchOp::CreateEntity {
            entity_iri: "http://example.org/people#Employee".to_string(),
            kind: PatchEntityKind::Class,
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        assert!(result.preview_text.unwrap().contains("ex:Employee"));
    }

    #[test]
    fn batch_failure_leaves_source_unchanged() {
        let ttl = include_str!("../../../fixtures/example.ttl");
        let patches = vec![
            PatchOp::AddLabel {
                entity_iri: "http://example.org/people#Person".to_string(),
                value: "Human".to_string(),
            },
            PatchOp::AddLabel {
                entity_iri: "http://example.org/people#NoSuch".to_string(),
                value: "X".to_string(),
            },
        ];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        assert!(!result.diagnostics.is_empty());
        assert_eq!(result.preview_text.as_deref(), Some(ttl));
        assert!(!result.applied);
    }

    #[test]
    fn crlf_line_offsets_match_byte_positions() {
        let ttl = "ex:Foo a owl:Class ;\r\n    rdfs:label \"Bar\" .\r\n";
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        let range = entity_primary_block_range(ttl, "http://example.org/Foo", &ns).expect("range");
        let block = &ttl[range.start as usize..range.end as usize];
        assert!(block.contains("rdfs:label"));
        assert!(block.trim_end().ends_with('.'));
    }

    #[test]
    fn cleanup_preserves_literal_double_spaces() {
        let block = "ex:Foo rdfs:label \"a  b\" .";
        let cleaned = cleanup_block_separators(block);
        assert!(cleaned.contains("\"a  b\""));
    }

    #[test]
    fn iri_with_angle_bracket_is_rejected_not_injected() {
        let ttl = "@prefix ex: <http://example.org/people#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\nex:Person a owl:Class .\n";
        let evil =
            "http://example.org/people#X> . ex:Pwned a owl:Class . <http://example.org/people#Y";
        let patches = vec![PatchOp::CreateEntity {
            entity_iri: evil.to_string(),
            kind: PatchEntityKind::Class,
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        assert!(!result.applied);
        assert!(!result.diagnostics.is_empty());
        assert_eq!(result.preview_text.as_deref(), Some(ttl));
        // Injection must not appear in any produced text (preview equals original).
        let produced = result.preview_text.as_deref().unwrap_or("");
        assert!(
            !produced.contains("ex:Pwned") && !produced.contains("Pwned"),
            "malicious local name must not be written: {produced}"
        );
        assert!(
            result.diagnostics.iter().any(|d| d.severity == "error"),
            "expected error diagnostic for angle-bracket IRI injection"
        );
    }

    #[test]
    fn iri_with_newline_is_rejected() {
        let ttl = "@prefix ex: <http://example.org/people#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\nex:Person a owl:Class .\n";
        let evil = "http://example.org/people#X\n. ex:Injected a owl:Class .\n#";
        let patches = vec![PatchOp::CreateEntity {
            entity_iri: evil.to_string(),
            kind: PatchEntityKind::Class,
        }];
        let result = apply_patches_to_text(ttl, &patches, true, &ex_ns()).expect("patch");
        assert!(!result.applied);
        assert_eq!(result.preview_text.as_deref(), Some(ttl));
    }

    #[test]
    fn longest_namespace_prefix_wins() {
        let ns = BTreeMap::from([
            ("ex".to_string(), "http://example.org/".to_string()),
            ("exfoo".to_string(), "http://example.org/foo/".to_string()),
        ]);
        let term = iri_to_turtle_term("http://example.org/foo/Bar", &ns).expect("term");
        assert_eq!(term, "exfoo:Bar");
    }

    #[test]
    fn slash_in_local_name_uses_angle_brackets() {
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org/".to_string())]);
        let term = iri_to_turtle_term("http://example.org/foo/Bar", &ns).expect("term");
        assert_eq!(term, "<http://example.org/foo/Bar>");
    }

    #[test]
    fn add_disjoint_class_is_idempotent_when_axiom_exists() {
        let ttl = include_str!("../../../fixtures/disjoint-classes.ttl");
        let ns = BTreeMap::from([
            ("ex".to_string(), "http://example.org/org#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
            ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ]);
        let patches = vec![PatchOp::AddDisjointClass {
            entity_iri: "http://example.org/org#Cat".to_string(),
            other_iri: "http://example.org/org#Dog".to_string(),
        }];
        let before = ttl.matches("owl:disjointWith").count();
        let result = apply_patches_to_text(ttl, &patches, true, &ns).expect("patch");
        let preview = result.preview_text.as_deref().unwrap_or(ttl);
        assert_eq!(before, preview.matches("owl:disjointWith").count());
    }

    fn org_ns() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("ex".to_string(), "http://example.org/org#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
            ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ])
    }

    #[test]
    fn add_property_chain_rejects_class_iris() {
        let ttl = include_str!("../../../fixtures/disjoint-classes.ttl");
        let ns = org_ns();
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddPropertyChain {
                entity_iri: "http://example.org/org#chases".to_string(),
                properties: vec![
                    "http://example.org/org#Cat".to_string(),
                    "http://example.org/org#Dog".to_string(),
                ],
            }],
            true,
            &ns,
        )
        .expect("patch result");
        assert!(!result.diagnostics.is_empty());
        assert!(result.diagnostics[0].message.contains("owl:Class"));
    }

    #[test]
    fn url_shaped_annotation_value_is_literal_not_iri() {
        let ttl = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix skos: <http://www.w3.org/2004/02/skos/core#> .

ex:Cat a owl:Class .
"#;
        let mut ns = org_ns();
        ns.insert("skos".to_string(), "http://www.w3.org/2004/02/skos/core#".to_string());
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddAnnotation {
                entity_iri: "http://example.org/org#Cat".to_string(),
                predicate: "skos:note".to_string(),
                value: "https://example.org/docs/guide".to_string(),
            }],
            true,
            &ns,
        )
        .expect("add url-shaped annotation");
        let preview = result.preview_text.expect("preview");
        assert!(
            preview.contains("skos:note \"https://example.org/docs/guide\""),
            "URL-shaped strings must be quoted literals: {preview}"
        );
        assert!(
            !preview.contains("skos:note <https://example.org/docs/guide>"),
            "must not write URL-shaped strings as IRI objects: {preview}"
        );
    }

    #[test]
    fn bracketed_iri_annotation_value_is_object() {
        let ttl = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix skos: <http://www.w3.org/2004/02/skos/core#> .

ex:Cat a owl:Class .
"#;
        let mut ns = org_ns();
        ns.insert("skos".to_string(), "http://www.w3.org/2004/02/skos/core#".to_string());
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddAnnotation {
                entity_iri: "http://example.org/org#Cat".to_string(),
                predicate: "skos:exactMatch".to_string(),
                value: "<https://example.org/other#Cat>".to_string(),
            }],
            true,
            &ns,
        )
        .expect("add bracketed IRI annotation");
        let preview = result.preview_text.expect("preview");
        assert!(
            preview.contains("skos:exactMatch <https://example.org/other#Cat>")
                || preview.contains("skos:exactMatch ex:"),
            "bracketed values should write as IRI objects: {preview}"
        );
        assert!(!preview.contains("skos:exactMatch \"<https://"));
    }

    #[test]
    fn remove_url_shaped_literal_annotation() {
        let ttl = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix skos: <http://www.w3.org/2004/02/skos/core#> .

ex:Cat a owl:Class ;
    skos:note "https://example.org/docs/guide" .
"#;
        let mut ns = org_ns();
        ns.insert("skos".to_string(), "http://www.w3.org/2004/02/skos/core#".to_string());
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::RemoveAnnotation {
                entity_iri: "http://example.org/org#Cat".to_string(),
                predicate: "skos:note".to_string(),
                value: "https://example.org/docs/guide".to_string(),
            }],
            true,
            &ns,
        )
        .expect("remove url-shaped literal");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("https://example.org/docs/guide"));
    }

    #[test]
    fn adversarial_curie_annotation_predicate_is_rejected() {
        let ttl = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Cat a owl:Class .
"#;
        let evil = "x:y> a owl:Class . <http://ex.org/z";
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddAnnotation {
                entity_iri: "http://example.org/org#Cat".to_string(),
                predicate: evil.to_string(),
                value: "safe".to_string(),
            }],
            true,
            &org_ns(),
        )
        .expect("patch call succeeds with diagnostics");
        assert!(!result.diagnostics.is_empty());
        assert_eq!(result.preview_text.as_deref(), Some(ttl));
        assert!(!ttl.contains("owl:Class . <http://ex.org/z"));
        let preview = result.preview_text.as_deref().unwrap_or("");
        assert!(
            !preview.contains("a owl:Class . <http://ex.org/z"),
            "predicate breakout must not be written: {preview}"
        );
    }

    #[test]
    fn adversarial_curie_ontology_annotation_predicate_is_rejected() {
        let ttl = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

<http://example.org/org> a owl:Ontology .
"#;
        let evil = "evil:x> ; owl:imports <http://evil>";
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddOntologyAnnotation {
                ontology_iri: "http://example.org/org".to_string(),
                predicate: evil.to_string(),
                value: "note".to_string(),
            }],
            true,
            &org_ns(),
        )
        .expect("patch call succeeds with diagnostics");
        assert!(!result.diagnostics.is_empty());
        assert_eq!(result.preview_text.as_deref(), Some(ttl));
        let preview = result.preview_text.as_deref().unwrap_or("");
        assert!(
            !preview.contains("owl:imports <http://evil>"),
            "ontology annotation breakout must not be written: {preview}"
        );
    }

    #[test]
    fn full_iri_annotation_predicate_still_works() {
        let ttl = r#"@prefix ex: <http://example.org/org#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix skos: <http://www.w3.org/2004/02/skos/core#> .

ex:Cat a owl:Class .
"#;
        let mut ns = org_ns();
        ns.insert("skos".to_string(), "http://www.w3.org/2004/02/skos/core#".to_string());
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddAnnotation {
                entity_iri: "http://example.org/org#Cat".to_string(),
                predicate: "http://www.w3.org/2004/02/skos/core#definition".to_string(),
                value: "A feline animal".to_string(),
            }],
            true,
            &ns,
        )
        .expect("add full-IRI annotation predicate");
        let preview = result.preview_text.expect("preview");
        assert!(
            preview.contains("skos:definition \"A feline animal\"")
                || preview.contains(
                    "<http://www.w3.org/2004/02/skos/core#definition> \"A feline animal\""
                ),
            "full IRI predicates must still write safely: {preview}"
        );
    }

    #[test]
    fn add_prefix_after_existing_prefixes() {
        let ttl = "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\n<http://example.org/test> a owl:Ontology .\n";
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddPrefix {
                prefix: "ex".to_string(),
                namespace_iri: "http://example.org/test#".to_string(),
            }],
            true,
            &BTreeMap::new(),
        )
        .expect("add prefix");
        let preview = result.preview_text.expect("preview");
        assert!(preview.starts_with(
            "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n@prefix ex: <http://example.org/test#> .\n"
        ));
    }

    #[test]
    fn set_ontology_iri_rewrites_angle_bracket_subject() {
        let ttl = r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .

<http://example.org/old> a owl:Ontology .
"#;
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::SetOntologyIri { ontology_iri: "http://example.org/new".to_string() }],
            true,
            &BTreeMap::new(),
        )
        .expect("set ontology iri");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("<http://example.org/new> a owl:Ontology"));
        assert!(!preview.contains("<http://example.org/old>"));
        assert_eq!(preview.matches("a owl:Ontology").count(), 1);
    }

    #[test]
    fn set_ontology_iri_rewrites_curie_subject_in_place() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:ont a owl:Ontology .
"#;
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::SetOntologyIri { ontology_iri: "http://example.org/new".to_string() }],
            true,
            &BTreeMap::from([("ex".to_string(), "http://example.org/".to_string())]),
        )
        .expect("set ontology iri");
        let preview = result.preview_text.expect("preview");
        assert!(
            preview.contains("<http://example.org/new> a owl:Ontology"),
            "expected rewritten subject: {preview}"
        );
        assert!(
            !preview.contains("ex:ont a owl:Ontology"),
            "original CURIE declaration must be replaced: {preview}"
        );
        assert_eq!(
            preview.matches("a owl:Ontology").count(),
            1,
            "must not append a second ontology declaration: {preview}"
        );
    }

    #[test]
    fn set_ontology_iri_rewrites_curie_in_multiline_ontology_block() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:ont a owl:Ontology ;
    owl:versionIRI <http://example.org/ont/1.0> .
"#;
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::SetOntologyIri { ontology_iri: "http://example.org/new".to_string() }],
            true,
            &BTreeMap::from([("ex".to_string(), "http://example.org/".to_string())]),
        )
        .expect("set ontology iri");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("<http://example.org/new> a owl:Ontology ;"));
        assert!(preview.contains("owl:versionIRI <http://example.org/ont/1.0>"));
        assert!(!preview.contains("ex:ont a owl:Ontology"));
        assert_eq!(preview.matches("a owl:Ontology").count(), 1);
    }

    #[test]
    fn set_ontology_iri_appends_only_when_no_declaration() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Foo a owl:Class .
"#;
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::SetOntologyIri { ontology_iri: "http://example.org/new".to_string() }],
            true,
            &BTreeMap::new(),
        )
        .expect("set ontology iri");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("<http://example.org/new> a owl:Ontology ."));
        assert!(preview.contains("ex:Foo a owl:Class"));
        assert_eq!(preview.matches("a owl:Ontology").count(), 1);
    }

    #[test]
    fn set_version_iri_replaces_existing() {
        let ttl = r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .

<http://example.org/ont> a owl:Ontology ;
    owl:versionIRI <http://example.org/ont/1> .
"#;
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::SetVersionIri {
                ontology_iri: "http://example.org/ont".to_string(),
                version_iri: "http://example.org/ont/2".to_string(),
            }],
            true,
            &BTreeMap::new(),
        )
        .expect("set version iri");
        let preview = result.preview_text.expect("preview");
        assert!(
            preview.contains("owl:versionIRI <http://example.org/ont/2>"),
            "expected new version IRI: {preview}"
        );
        assert!(
            !preview.contains("http://example.org/ont/1"),
            "old version IRI must be removed: {preview}"
        );
        assert_eq!(
            preview.matches("owl:versionIRI").count(),
            1,
            "must keep exactly one versionIRI: {preview}"
        );
    }

    #[test]
    fn set_version_iri_repeated_does_not_accumulate() {
        let ttl = r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .

<http://example.org/ont> a owl:Ontology .
"#;
        let first = apply_patches_to_text(
            ttl,
            &[PatchOp::SetVersionIri {
                ontology_iri: "http://example.org/ont".to_string(),
                version_iri: "http://example.org/ont/1".to_string(),
            }],
            true,
            &BTreeMap::new(),
        )
        .expect("first set");
        let after_first = first.preview_text.expect("preview");
        let second = apply_patches_to_text(
            &after_first,
            &[PatchOp::SetVersionIri {
                ontology_iri: "http://example.org/ont".to_string(),
                version_iri: "http://example.org/ont/2".to_string(),
            }],
            true,
            &BTreeMap::new(),
        )
        .expect("second set");
        let preview = second.preview_text.expect("preview");
        assert!(preview.contains("owl:versionIRI <http://example.org/ont/2>"));
        assert!(!preview.contains("http://example.org/ont/1"));
        assert_eq!(preview.matches("owl:versionIRI").count(), 1);
    }

    #[test]
    fn remove_prefix_leaves_other_prefixes() {
        let ttl = "@prefix ex: <http://example.org/test#> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\nex:Thing a owl:Class .\n";
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::RemovePrefix { prefix: "ex".to_string() }],
            true,
            &BTreeMap::new(),
        )
        .expect("remove prefix");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("@prefix ex:"));
        assert!(preview.contains("@prefix owl:"));
        assert!(preview.contains("ex:Thing a owl:Class"));
    }

    #[test]
    fn set_prefix_updates_uppercase_at_prefix_in_place() {
        let ttl = "@PREFIX ex: <http://old.example/> .\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\nex:Thing a owl:Class .\n";
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::SetPrefix {
                prefix: "ex".to_string(),
                namespace_iri: "http://new.example/".to_string(),
            }],
            true,
            &BTreeMap::new(),
        )
        .expect("set prefix");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("@PREFIX ex: <http://new.example/> ."));
        assert!(!preview.contains("http://old.example/"));
        assert_eq!(preview.matches("ex:").count(), 2); // declaration + CURIE use
        assert!(!preview.contains("@prefix ex:"));
    }

    #[test]
    fn set_prefix_updates_sparql_style_prefix_in_place() {
        let ttl = "PREFIX ex: <http://old.example/>\n@prefix owl: <http://www.w3.org/2002/07/owl#> .\n\nex:Thing a owl:Class .\n";
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::SetPrefix {
                prefix: "ex".to_string(),
                namespace_iri: "http://new.example/".to_string(),
            }],
            true,
            &BTreeMap::new(),
        )
        .expect("set prefix");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("PREFIX ex: <http://new.example/>"));
        assert!(!preview.contains("http://old.example/"));
        assert!(!preview.contains("@prefix ex:"));
    }

    #[test]
    fn remove_prefix_recognizes_uppercase_and_sparql_forms() {
        let ttl = "@PREFIX ex: <http://example.org/test#> .\nPREFIX owl: <http://www.w3.org/2002/07/owl#>\n\nex:Thing a owl:Class .\n";
        let result = apply_patches_to_text(
            ttl,
            &[
                PatchOp::RemovePrefix { prefix: "ex".to_string() },
                PatchOp::RemovePrefix { prefix: "owl".to_string() },
            ],
            true,
            &BTreeMap::new(),
        )
        .expect("remove prefixes");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("@PREFIX ex:"));
        assert!(!preview.contains("PREFIX owl:"));
        assert!(preview.contains("ex:Thing a owl:Class"));
    }

    #[test]
    fn add_prefix_detects_duplicate_uppercase_declaration() {
        let ttl = "@PREFIX ex: <http://example.org/test#> .\n\nex:Thing a owl:Class .\n";
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddPrefix {
                prefix: "ex".to_string(),
                namespace_iri: "http://other.example/".to_string(),
            }],
            true,
            &BTreeMap::new(),
        )
        .expect("apply returns diagnostics on validation failure");
        assert!(!result.applied);
        assert!(result
            .diagnostics
            .iter()
            .any(|d| d.message.contains("duplicate prefix already present: ex")));
    }

    #[test]
    fn remove_label_matches_language_tagged_literal() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Foo a owl:Class ;
    rdfs:label "Person"@en .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
            ("rdfs".into(), "http://www.w3.org/2000/01/rdf-schema#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::RemoveLabel {
                entity_iri: "http://example.org/Foo".into(),
                value: "Person".into(),
            }],
            true,
            &ns,
        )
        .expect("remove lang-tagged label");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("rdfs:label"));
        assert!(!preview.contains("\"Person\"@en"));
    }

    #[test]
    fn add_label_ignores_a_substring_inside_comment() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .

ex:Foo rdfs:comment "was a prototype" .
ex:Foo a owl:Class .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
            ("rdfs".into(), "http://www.w3.org/2000/01/rdf-schema#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddLabel { entity_iri: "http://example.org/Foo".into(), value: "L".into() }],
            true,
            &ns,
        )
        .expect("add label");
        let preview = result.preview_text.expect("preview");
        let class_stmt = preview
            .lines()
            .find(|l| l.contains("a owl:Class") || l.contains("rdfs:label \"L\""))
            .map(str::to_string)
            .unwrap_or_default();
        assert!(
            preview.contains("ex:Foo a owl:Class") && preview.contains("rdfs:label \"L\""),
            "label must attach to type statement: {preview}"
        );
        assert!(
            !preview.contains("was a prototype\" ;\n    rdfs:label")
                && !preview.contains("\"was a prototype\" ;\n    rdfs:label"),
            "label must not insert into comment statement: {preview}\nclass-ish: {class_stmt}"
        );
        // Stronger: comment line must not gain a label predicate.
        for line in preview.lines() {
            if line.contains("was a prototype") {
                assert!(!line.contains("rdfs:label"), "comment line must stay label-free: {line}");
            }
        }
    }

    #[test]
    fn remove_import_matches_angle_bracket_when_prefix_exists() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
<http://example.org/ont> a owl:Ontology ;
    owl:imports <http://example.org/other> .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::RemoveImport {
                ontology_iri: "http://example.org/ont".into(),
                import_iri: "http://example.org/other".into(),
            }],
            true,
            &ns,
        )
        .expect("remove import");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("owl:imports"));
    }

    #[test]
    fn set_ontology_iri_rewrites_rdf_type_declaration() {
        let ttl = r#"@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
<http://example.org/old> rdf:type owl:Ontology .
"#;
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::SetOntologyIri { ontology_iri: "http://example.org/new".into() }],
            true,
            &BTreeMap::new(),
        )
        .expect("set ontology iri");
        let preview = result.preview_text.expect("preview");
        assert!(
            preview.contains("<http://example.org/new> rdf:type owl:Ontology"),
            "expected in-place rewrite: {preview}"
        );
        assert!(!preview.contains("<http://example.org/old>"));
        assert_eq!(
            preview.matches("owl:Ontology").count(),
            1,
            "must not append a second ontology declaration: {preview}"
        );
    }

    #[test]
    fn remove_data_property_assertion_matches_typed_literal() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
ex:ind a owl:NamedIndividual ;
    ex:age "42"^^xsd:integer .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
            ("xsd".into(), "http://www.w3.org/2001/XMLSchema#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::RemoveDataPropertyAssertion {
                entity_iri: "http://example.org/ind".into(),
                property_iri: "http://example.org/age".into(),
                value: "42".into(),
            }],
            true,
            &ns,
        )
        .expect("remove typed data assertion");
        let preview = result.preview_text.expect("preview");
        assert!(!preview.contains("ex:age"));
        assert!(!preview.contains("\"42\""));
    }

    #[test]
    fn set_functional_ignores_characteristic_text_inside_comment() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

ex:p a owl:ObjectProperty ;
    rdfs:comment "Not a owl:FunctionalProperty despite the wording" .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
            ("rdfs".into(), "http://www.w3.org/2000/01/rdf-schema#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::SetFunctional { entity_iri: "http://example.org/p".into(), value: true }],
            true,
            &ns,
        )
        .expect("set functional");
        let preview = result.preview_text.expect("preview");
        assert!(
            preview.contains("a owl:FunctionalProperty")
                || preview.contains("owl:FunctionalProperty ;")
                || preview.contains("owl:FunctionalProperty\n"),
            "must insert a real FunctionalProperty type: {preview}"
        );
        // Ensure the characteristic is not only inside the comment string.
        let without_comment = preview
            .lines()
            .filter(|l| !l.contains("despite the wording"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            without_comment.contains("owl:FunctionalProperty"),
            "FunctionalProperty must appear outside the comment: {preview}"
        );
    }

    #[test]
    fn property_chain_does_not_false_positive_on_short_prefix_curie() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
ex:Bar a owl:Class .
<http://example.org/foo/Bar> a owl:ObjectProperty .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddPropertyChain {
                entity_iri: "http://example.org/foo/Bar".into(),
                properties: vec!["http://example.org/foo/Bar".into()],
            }],
            true,
            &ns,
        )
        .expect("property chain on absolute IRI ObjectProperty");
        assert!(
            result.diagnostics.is_empty(),
            "must not treat colliding short-name Class as the chain member: {:?}",
            result.diagnostics
        );
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("owl:propertyChainAxiom"));
    }

    #[test]
    fn complex_subclass_uses_document_prefixes_when_caller_map_empty() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
ex:A a owl:Class .
ex:B a owl:Class .
ex:p a owl:ObjectProperty .
"#;
        let empty = BTreeMap::new();
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddComplexSubClassOf {
                entity_iri: "http://example.org/A".into(),
                manchester: "ex:p some ex:B".into(),
            }],
            true,
            &empty,
        )
        .expect("patch");
        assert!(
            result.diagnostics.is_empty(),
            "document @prefix must resolve Manchester CURIEs: {:?}",
            result.diagnostics
        );
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("owl:someValuesFrom") || preview.contains("ex:B"));
    }

    #[test]
    fn add_has_key_writes_rdf_list() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
ex:Person a owl:Class .
ex:hasSSN a owl:ObjectProperty .
ex:hasName a owl:DatatypeProperty .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddHasKey {
                class_iri: "http://example.org/Person".into(),
                properties: vec![
                    "http://example.org/hasSSN".into(),
                    "http://example.org/hasName".into(),
                ],
            }],
            true,
            &ns,
        )
        .expect("has key");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("owl:hasKey"));
        assert!(preview.contains("( ex:hasSSN ex:hasName )") || preview.contains("ex:hasSSN"));
    }

    #[test]
    fn add_and_remove_disjoint_union() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
ex:Color a owl:Class .
ex:Red a owl:Class .
ex:Blue a owl:Class .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        let add = apply_patches_to_text(
            ttl,
            &[PatchOp::AddDisjointUnion {
                class_iri: "http://example.org/Color".into(),
                members: vec!["http://example.org/Red".into(), "http://example.org/Blue".into()],
            }],
            true,
            &ns,
        )
        .expect("add disjoint union");
        let preview = add.preview_text.expect("preview");
        assert!(preview.contains("owl:disjointUnionOf"));
        let removed = apply_patches_to_text(
            &preview,
            &[PatchOp::RemoveDisjointUnion {
                class_iri: "http://example.org/Color".into(),
                members: vec!["http://example.org/Red".into(), "http://example.org/Blue".into()],
            }],
            true,
            &ns,
        )
        .expect("remove disjoint union");
        let out = removed.preview_text.expect("preview");
        assert!(!out.contains("owl:disjointUnionOf"));
    }

    #[test]
    fn add_inverse_object_properties() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
ex:hasChild a owl:ObjectProperty .
ex:hasParent a owl:ObjectProperty .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddInverseObjectProperties {
                property_iri: "http://example.org/hasChild".into(),
                inverse_iri: "http://example.org/hasParent".into(),
            }],
            true,
            &ns,
        )
        .expect("inverse");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("owl:inverseOf") && preview.contains("ex:hasParent"));
    }

    #[test]
    fn add_negative_object_property_assertion() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
ex:alice a owl:NamedIndividual .
ex:bob a owl:NamedIndividual .
ex:knows a owl:ObjectProperty .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
            ("rdf".into(), "http://www.w3.org/1999/02/22-rdf-syntax-ns#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddNegativeObjectPropertyAssertion {
                entity_iri: "http://example.org/alice".into(),
                property_iri: "http://example.org/knows".into(),
                target_iri: "http://example.org/bob".into(),
            }],
            true,
            &ns,
        )
        .expect("nopa");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("owl:NegativePropertyAssertion"));
        assert!(preview.contains("owl:sourceIndividual") && preview.contains("ex:alice"));
        assert!(preview.contains("owl:targetIndividual") && preview.contains("ex:bob"));
        let removed = apply_patches_to_text(
            &preview,
            &[PatchOp::RemoveNegativeObjectPropertyAssertion {
                entity_iri: "http://example.org/alice".into(),
                property_iri: "http://example.org/knows".into(),
                target_iri: "http://example.org/bob".into(),
            }],
            true,
            &ns,
        )
        .expect("remove nopa");
        let out = removed.preview_text.expect("preview");
        assert!(!out.contains("owl:NegativePropertyAssertion"));
    }

    #[test]
    fn add_same_individual_pairwise() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
ex:a a owl:NamedIndividual .
ex:b a owl:NamedIndividual .
ex:c a owl:NamedIndividual .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddSameIndividual {
                individuals: vec![
                    "http://example.org/a".into(),
                    "http://example.org/b".into(),
                    "http://example.org/c".into(),
                ],
            }],
            true,
            &ns,
        )
        .expect("same individual");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("owl:sameAs"));
        assert!(preview.contains("ex:b") && preview.contains("ex:c"));
    }

    #[test]
    fn add_different_individuals_writes_all_different() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
ex:a a owl:NamedIndividual .
ex:b a owl:NamedIndividual .
ex:c a owl:NamedIndividual .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddDifferentIndividuals {
                individuals: vec![
                    "http://example.org/a".into(),
                    "http://example.org/b".into(),
                    "http://example.org/c".into(),
                ],
            }],
            true,
            &ns,
        )
        .expect("add different");
        let preview = result.preview_text.expect("preview");
        assert!(
            preview.contains("owl:AllDifferent") && preview.contains("owl:distinctMembers"),
            "must emit AllDifferent, got: {preview}"
        );
        assert!(
            preview.contains("ex:a") && preview.contains("ex:b") && preview.contains("ex:c"),
            "must list all members: {preview}"
        );
        // Non-adjacent pair A≠C must appear in the same list (not windows-only triples).
        assert!(
            !preview.contains("owl:differentFrom"),
            "should not emit chain-only differentFrom: {preview}"
        );
    }

    #[test]
    fn remove_different_individuals_rewrites_all_different_fixture_style() {
        let ttl = r#"@prefix ex: <http://example.org/abox#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
ex:alice a owl:NamedIndividual .
ex:bob a owl:NamedIndividual .
ex:carol a owl:NamedIndividual .
[] a owl:AllDifferent ;
    owl:distinctMembers ( ex:alice ex:bob ex:carol ) .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/abox#".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        // Inspector-style remove of one projected pair strips those members; carol alone → drop axiom.
        let removed = apply_patches_to_text(
            ttl,
            &[PatchOp::RemoveDifferentIndividuals {
                individuals: vec![
                    "http://example.org/abox#alice".into(),
                    "http://example.org/abox#bob".into(),
                ],
            }],
            true,
            &ns,
        )
        .expect("remove pair from AllDifferent");
        let out = removed.preview_text.expect("preview");
        assert!(
            !out.contains("owl:AllDifferent"),
            "AllDifferent must be gone after removing two of three members: {out}"
        );

        // Rewrite path: remove only carol from a fresh copy → alice+bob remain.
        let rewritten = apply_patches_to_text(
            ttl,
            &[PatchOp::RemoveDifferentIndividuals {
                individuals: vec![
                    "http://example.org/abox#alice".into(),
                    "http://example.org/abox#carol".into(),
                ],
            }],
            true,
            &ns,
        )
        .expect("rewrite AllDifferent");
        // alice+carol remove leaves bob alone → axiom dropped ( <2 members ).
        let out2 = rewritten.preview_text.expect("preview");
        assert!(!out2.contains("owl:AllDifferent"), "expected drop: {out2}");

        let four = r#"@prefix ex: <http://example.org/abox#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
ex:alice a owl:NamedIndividual .
ex:bob a owl:NamedIndividual .
ex:carol a owl:NamedIndividual .
ex:dave a owl:NamedIndividual .
[] a owl:AllDifferent ;
    owl:distinctMembers ( ex:alice ex:bob ex:carol ex:dave ) .
"#;
        let keep = apply_patches_to_text(
            four,
            &[PatchOp::RemoveDifferentIndividuals {
                individuals: vec![
                    "http://example.org/abox#alice".into(),
                    "http://example.org/abox#bob".into(),
                ],
            }],
            true,
            &ns,
        )
        .expect("rewrite leave two");
        let out3 = keep.preview_text.expect("preview");
        assert!(
            out3.contains("owl:AllDifferent")
                && out3.contains("owl:distinctMembers ( ex:carol ex:dave )"),
            "must rewrite membership: {out3}"
        );
        assert!(
            !out3.contains("distinctMembers ( ex:alice") && !out3.contains("ex:alice ex:bob"),
            "removed members must leave the list: {out3}"
        );
    }

    #[test]
    fn remove_different_individuals_fails_closed_on_unexpanded_curie() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
ex:a a owl:NamedIndividual .
ex:b a owl:NamedIndividual .
ex:d a owl:NamedIndividual .
[] a owl:AllDifferent ;
    owl:distinctMembers ( ex:a ex:b other:c ex:d ) .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::RemoveDifferentIndividuals {
                individuals: vec!["http://example.org/a".into(), "http://example.org/b".into()],
            }],
            true,
            &ns,
        )
        .expect("preview path returns ApplyPatchResult");
        assert!(!result.applied, "undeclared other:c must not apply: {:?}", result.diagnostics);
        assert!(
            result.diagnostics.iter().any(|d| {
                d.message.contains("cannot be expanded") || d.message.contains("other:c")
            }),
            "expected expansion diagnostic: {:?}",
            result.diagnostics
        );
        let preview = result.preview_text.unwrap_or_default();
        assert!(
            preview.contains("other:c") && preview.contains("owl:AllDifferent"),
            "must not drop unexpanded members: {preview}"
        );
    }

    #[test]
    fn create_datatype_and_definition() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
            ("rdfs".into(), "http://www.w3.org/2000/01/rdf-schema#".into()),
            ("xsd".into(), "http://www.w3.org/2001/XMLSchema#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddDatatypeDefinition {
                datatype_iri: "http://example.org/SSN".into(),
                manchester: "xsd:string".into(),
            }],
            true,
            &ns,
        )
        .expect("datatype def");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("rdfs:Datatype") || preview.contains("ex:SSN"));
        assert!(preview.contains("owl:equivalentClass") && preview.contains("xsd:string"));
    }

    #[test]
    fn add_axiom_annotation_owl_axiom_block() {
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
ex:A a owl:Class ;
    rdfs:subClassOf ex:B .
ex:B a owl:Class .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
            ("rdfs".into(), "http://www.w3.org/2000/01/rdf-schema#".into()),
            ("rdf".into(), "http://www.w3.org/1999/02/22-rdf-syntax-ns#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddAxiomAnnotation {
                axiom_op: "sub_class_of".into(),
                subject_iri: "http://example.org/A".into(),
                related_iri: Some("http://example.org/B".into()),
                predicate: "rdfs:comment".into(),
                value: "explained".into(),
            }],
            true,
            &ns,
        )
        .expect("axiom ann");
        let preview = result.preview_text.expect("preview");
        assert!(preview.contains("owl:Axiom"));
        assert!(preview.contains("owl:annotatedSource") && preview.contains("owl:annotatedTarget"));
        assert!(preview.contains("explained"));
    }

    #[test]
    fn remove_has_key_matches_reversed_property_list() {
        // #351 — HasKey property order is a set for authoring UX.
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
ex:Person a owl:Class ;
    owl:hasKey ( ex:p2 ex:p1 ) .
ex:p1 a owl:ObjectProperty .
ex:p2 a owl:ObjectProperty .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        let removed = apply_patches_to_text(
            ttl,
            &[PatchOp::RemoveHasKey {
                class_iri: "http://example.org/Person".into(),
                properties: vec!["http://example.org/p1".into(), "http://example.org/p2".into()],
            }],
            true,
            &ns,
        )
        .expect("remove reversed hasKey");
        let out = removed.preview_text.expect("preview");
        assert!(!out.contains("owl:hasKey"), "expected hasKey removed: {out}");
    }

    #[test]
    fn add_disjoint_object_properties_emits_full_pairwise_closure() {
        // #394 — n-ary DisjointObjectProperties needs all pairs, not windows(2).
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
ex:p1 a owl:ObjectProperty .
ex:p2 a owl:ObjectProperty .
ex:p3 a owl:ObjectProperty .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::AddDisjointObjectProperties {
                properties: vec![
                    "http://example.org/p1".into(),
                    "http://example.org/p2".into(),
                    "http://example.org/p3".into(),
                ],
            }],
            true,
            &ns,
        )
        .expect("add disjoint props");
        let preview = result.preview_text.expect("preview");
        // Full pairwise for 3 properties = 3 triples (p1-p2, p1-p3, p2-p3).
        let count = preview.matches("owl:propertyDisjointWith").count();
        assert_eq!(count, 3, "expected 3 pairwise disjoint triples, got {count}: {preview}");
    }

    #[test]
    fn remove_axiom_annotation_exact_value_not_substring() {
        // #349 — "x" must not delete the "xy" axiom annotation block.
        let ttl = r#"@prefix ex: <http://example.org/> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
ex:A a owl:Class ;
    rdfs:subClassOf ex:B .
ex:B a owl:Class .
[
    a owl:Axiom ;
    owl:annotatedSource ex:A ;
    owl:annotatedProperty rdfs:subClassOf ;
    owl:annotatedTarget ex:B ;
    rdfs:comment "xy"
] .
[
    a owl:Axiom ;
    owl:annotatedSource ex:A ;
    owl:annotatedProperty rdfs:subClassOf ;
    owl:annotatedTarget ex:B ;
    rdfs:comment "x"
] .
"#;
        let ns = BTreeMap::from([
            ("ex".into(), "http://example.org/".into()),
            ("owl".into(), "http://www.w3.org/2002/07/owl#".into()),
            ("rdfs".into(), "http://www.w3.org/2000/01/rdf-schema#".into()),
            ("rdf".into(), "http://www.w3.org/1999/02/22-rdf-syntax-ns#".into()),
        ]);
        let result = apply_patches_to_text(
            ttl,
            &[PatchOp::RemoveAxiomAnnotation {
                axiom_op: "sub_class_of".into(),
                subject_iri: "http://example.org/A".into(),
                related_iri: Some("http://example.org/B".into()),
                predicate: "rdfs:comment".into(),
                value: "x".into(),
            }],
            true,
            &ns,
        )
        .expect("remove exact x");
        assert!(result.diagnostics.is_empty(), "unexpected diagnostics: {:?}", result.diagnostics);
        let preview = result.preview_text.expect("preview should change");
        assert!(preview.contains("\"xy\""), "must keep xy block: {preview}");
        assert!(!preview.contains("rdfs:comment \"x\""), "must remove exact x: {preview}");
    }
}
