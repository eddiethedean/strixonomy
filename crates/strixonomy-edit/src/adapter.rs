use crate::error::{EditError, Result};
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use strixonomy_core::OntologyFormat;
use strixonomy_obo::{self, OboPatchOp};
use strixonomy_owl::{self, PatchDiagnostic, PatchOp};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditFormat {
    Turtle,
    Obo,
    /// RDF/XML (`.owl` / `.rdf`) — full-document Horned re-serialize.
    RdfXml,
    /// OWL/XML (`.owx`) — full-document Horned re-serialize.
    OwlXml,
}

impl EditFormat {
    pub fn from_ontology_format(format: OntologyFormat) -> Result<Self> {
        match format {
            OntologyFormat::Turtle => Ok(Self::Turtle),
            OntologyFormat::Obo => Ok(Self::Obo),
            OntologyFormat::Owl | OntologyFormat::RdfXml => Ok(Self::RdfXml),
            OntologyFormat::OwlXml => Ok(Self::OwlXml),
            other => Err(EditError::UnsupportedFormat(other.as_str().to_string())),
        }
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        Self::from_ontology_format(OntologyFormat::from_extension(ext))
    }

    pub fn to_ontology_format(self) -> OntologyFormat {
        match self {
            Self::Turtle => OntologyFormat::Turtle,
            Self::Obo => OntologyFormat::Obo,
            Self::RdfXml => OntologyFormat::RdfXml,
            Self::OwlXml => OntologyFormat::OwlXml,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyTextResult {
    pub applied: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<PatchDiagnostic>,
}

impl From<strixonomy_owl::ApplyPatchResult> for ApplyTextResult {
    fn from(value: strixonomy_owl::ApplyPatchResult) -> Self {
        Self {
            applied: value.applied,
            preview_text: value.preview_text,
            diagnostics: value.diagnostics,
        }
    }
}

/// Apply using the transaction's inherent change format (Turtle span or OBO stanza).
pub fn apply_transaction_to_text(
    transaction: &Transaction,
    source: &str,
    preview_only: bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<ApplyTextResult> {
    apply_transaction_to_text_as(
        transaction,
        source,
        preview_only,
        namespaces,
        transaction.format()?,
    )
}

/// Apply using an explicit document format (required for RDF/XML and OWL/XML write-back).
pub fn apply_transaction_to_text_as(
    transaction: &Transaction,
    source: &str,
    preview_only: bool,
    namespaces: &BTreeMap<String, String>,
    format: EditFormat,
) -> Result<ApplyTextResult> {
    match format {
        EditFormat::Turtle => {
            let patches: Vec<PatchOp> = transaction.turtle_patches()?;
            Ok(strixonomy_owl::apply_patches_to_text(source, &patches, preview_only, namespaces)?
                .into())
        }
        EditFormat::Obo => {
            let patches: Vec<OboPatchOp> = transaction.obo_patches()?;
            let result = strixonomy_obo::apply_patches_to_text(source, &patches, preview_only)?;
            Ok(ApplyTextResult {
                applied: result.applied,
                preview_text: result.preview_text,
                diagnostics: result
                    .diagnostics
                    .into_iter()
                    .map(|d| PatchDiagnostic { severity: d.severity, message: d.message })
                    .collect(),
            })
        }
        EditFormat::RdfXml | EditFormat::OwlXml => {
            let patches: Vec<PatchOp> = transaction.turtle_patches()?;
            Ok(strixonomy_owl::apply_xml_patches_to_text(
                source,
                format.to_ontology_format(),
                &patches,
                preview_only,
                namespaces,
            )?
            .into())
        }
    }
}

pub fn apply_transaction_to_path(
    transaction: &Transaction,
    document_path: &Path,
    preview_only: bool,
    namespaces: &BTreeMap<String, String>,
) -> Result<strixonomy_owl::ApplyPatchResult> {
    let format = EditFormat::from_path(document_path)?;
    match format {
        EditFormat::Turtle => {
            let patches = transaction.turtle_patches()?;
            strixonomy_owl::apply_patches(document_path, &patches, preview_only, namespaces)
                .map_err(EditError::from)
        }
        EditFormat::Obo => {
            let patches = transaction.obo_patches()?;
            let result = strixonomy_obo::apply_patches(document_path, &patches, preview_only)?;
            Ok(strixonomy_owl::ApplyPatchResult {
                applied: result.applied,
                preview_text: result.preview_text,
                diagnostics: result
                    .diagnostics
                    .into_iter()
                    .map(|d| PatchDiagnostic { severity: d.severity, message: d.message })
                    .collect(),
                document_path: result.document_path,
            })
        }
        EditFormat::RdfXml | EditFormat::OwlXml => {
            let patches = transaction.turtle_patches()?;
            strixonomy_owl::apply_xml_patches(document_path, &patches, preview_only, namespaces)
                .map_err(EditError::from)
        }
    }
}
