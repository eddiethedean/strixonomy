//! Full-document XML write-back: load → mutate → serialize (v0.21).

use crate::error::{OwlError, Result};
use crate::mutate::apply_patches_to_ontology_with_ns;
use crate::patch::{atomic_write, ApplyPatchResult, PatchOp};
use crate::serialize::{
    load_owl_xml_ontology, load_rdf_xml_ontology, serialize_owl_xml, serialize_rdf_xml,
};
use std::collections::BTreeMap;
use std::path::Path;
use strixonomy_core::OntologyFormat;

/// Apply patches to RDF/XML or OWL/XML source text via Horned re-serialization.
pub fn apply_xml_patches_to_text(
    source: &str,
    format: OntologyFormat,
    patches: &[PatchOp],
    preview_only: bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<ApplyPatchResult> {
    match format {
        OntologyFormat::Owl | OntologyFormat::RdfXml => {
            let (mut ont, incomplete) = load_rdf_xml_ontology(source)?;
            if incomplete {
                return Err(OwlError::LoadFailed(
                    "RDF/XML load incomplete; refusing write-back to avoid data loss".into(),
                ));
            }
            let before = serialize_rdf_xml(&ont)?;
            let diagnostics = apply_patches_to_ontology_with_ns(&mut ont, patches, namespaces)?;
            let after = serialize_rdf_xml(&ont)?;
            let changed = before != after;
            Ok(ApplyPatchResult {
                applied: changed && !preview_only,
                preview_text: Some(if changed { after } else { source.to_string() }),
                diagnostics,
                document_path: None,
            })
        }
        OntologyFormat::OwlXml => {
            let (mut ont, mut ns, incomplete) = load_owl_xml_ontology(source)?;
            if incomplete {
                return Err(OwlError::LoadFailed(
                    "OWL/XML load incomplete; refusing write-back to avoid data loss".into(),
                ));
            }
            for (k, v) in namespaces {
                ns.entry(k.clone()).or_insert_with(|| v.clone());
            }
            let before = serialize_owl_xml(&ont, &ns)?;
            let diagnostics = apply_patches_to_ontology_with_ns(&mut ont, patches, namespaces)?;
            let after = serialize_owl_xml(&ont, &ns)?;
            let changed = before != after;
            Ok(ApplyPatchResult {
                applied: changed && !preview_only,
                preview_text: Some(if changed { after } else { source.to_string() }),
                diagnostics,
                document_path: None,
            })
        }
        other => Err(OwlError::UnsupportedFormat(format!(
            "apply_xml_patches_to_text expects RDF/XML or OWL/XML, got {}",
            other.as_str()
        ))),
    }
}

/// Apply patches to an RDF/XML or OWL/XML document on disk.
pub fn apply_xml_patches(
    document_path: &Path,
    patches: &[PatchOp],
    preview_only: bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<ApplyPatchResult> {
    let format = OntologyFormat::from_extension(
        document_path.extension().and_then(|e| e.to_str()).unwrap_or(""),
    );
    if !matches!(format, OntologyFormat::Owl | OntologyFormat::RdfXml | OntologyFormat::OwlXml) {
        return Err(OwlError::UnsupportedFormat(format!(
            "XML write-back supports .owl/.rdf/.owx, got {}",
            format.as_str()
        )));
    }
    let source =
        strixonomy_core::read_to_string_capped(document_path, strixonomy_core::MAX_FILE_BYTES)
            .map_err(OwlError::Core)?;
    let mut result = apply_xml_patches_to_text(&source, format, patches, preview_only, namespaces)?;
    result.document_path = Some(document_path.display().to_string());
    if result.applied && !preview_only {
        if let Some(text) = &result.preview_text {
            atomic_write(document_path, text)?;
        }
    }
    Ok(result)
}
