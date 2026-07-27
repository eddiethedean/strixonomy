use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityChangeKind {
    Added,
    Removed,
    Renamed,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityChange {
    pub kind: EntityChangeKind,
    pub iri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_iri: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxiomChange {
    pub change: String,
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub axiom_kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationChange {
    pub change: String,
    pub subject: String,
    pub predicate: String,
    pub object: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportChange {
    pub change: String,
    pub ontology_id: String,
    pub import_iri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceChange {
    pub class_iri: String,
    pub change: String,
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakingReason {
    RemovedEntity,
    RenamedIri,
    RemovedSuperclass,
    RemovedImport,
    UnsatisfiableClass,
    DomainRangeChange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakingChange {
    pub reason: BreakingReason,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_iri: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffResult {
    pub entity_changes: Vec<EntityChange>,
    pub axiom_changes: Vec<AxiomChange>,
    pub annotation_changes: Vec<AnnotationChange>,
    pub import_changes: Vec<ImportChange>,
    pub inference_changes: Vec<InferenceChange>,
    pub breaking_changes: Vec<BreakingChange>,
}

impl DiffResult {
    pub fn is_empty(&self) -> bool {
        self.entity_changes.is_empty()
            && self.axiom_changes.is_empty()
            && self.annotation_changes.is_empty()
            && self.import_changes.is_empty()
            && self.inference_changes.is_empty()
            && self.breaking_changes.is_empty()
    }

    pub fn summary_counts(&self) -> DiffSummaryCounts {
        DiffSummaryCounts {
            entities: self.entity_changes.len(),
            axioms: self.axiom_changes.len(),
            annotations: self.annotation_changes.len(),
            imports: self.import_changes.len(),
            inferences: self.inference_changes.len(),
            breaking: self.breaking_changes.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffSummaryCounts {
    pub entities: usize,
    pub axioms: usize,
    pub annotations: usize,
    pub imports: usize,
    pub inferences: usize,
    pub breaking: usize,
}
