use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageKind {
    EntityDeclaration,
    AxiomSubject,
    AxiomObject,
    AxiomPredicate,
    AnnotationSubject,
    AnnotationObject,
    AnnotationPredicate,
    Import,
    TextReference,
    SwrlReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub iri: String,
    pub referenced_iri: String,
    pub file: PathBuf,
    pub line: Option<u64>,
    pub column: Option<u64>,
    pub start_byte: Option<u64>,
    pub end_byte: Option<u64>,
    pub kind: UsageKind,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hunk {
    pub start_byte: u64,
    pub end_byte: u64,
    pub old_text: String,
    pub new_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: PathBuf,
    pub preview_text: String,
    pub original_text: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hunks: Vec<Hunk>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RefactorPlan {
    pub changes: Vec<FileChange>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    /// Distinct entity IRIs touched by this plan (best-effort).
    #[serde(default)]
    pub affected_entity_count: usize,
    /// Approximate axiom/hunk impact (sum of hunks, or 1 per changed file when hunks empty).
    #[serde(default)]
    pub affected_axiom_count: usize,
}

impl RefactorPlan {
    /// Populate impact metrics from file changes and explicitly named entity IRIs.
    pub fn with_metrics(mut self, entity_iris: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        let mut entities: std::collections::BTreeSet<String> =
            entity_iris.into_iter().map(|s| s.as_ref().to_string()).collect();
        entities.retain(|iri| !iri.is_empty());
        self.affected_entity_count = entities.len();
        self.affected_axiom_count =
            self.changes.iter().map(|c| if c.hunks.is_empty() { 1 } else { c.hunks.len() }).sum();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RefactorRequest {
    RenameIri {
        from_iri: String,
        to_iri: String,
    },
    MergeEntities {
        keep_iri: String,
        merge_iri: String,
    },
    ReplaceEntity {
        from_iri: String,
        to_iri: String,
    },
    MigrateNamespace {
        from_base: String,
        to_base: String,
    },
    MoveEntity {
        entity_iri: String,
        target_file: PathBuf,
    },
    ExtractModule {
        entity_iris: Vec<String>,
        output_file: PathBuf,
        #[serde(default)]
        leave_stub: bool,
        /// When true, expand `entity_iris` under the bottom-locality heuristic before extract.
        #[serde(default)]
        locality: bool,
    },
    /// Move selected Turtle subject statements for an entity to another file.
    /// `statement_indexes` are 0-based indexes into `all_entity_statement_ranges`
    /// (excluding the primary type declaration when `exclude_primary` is true).
    MoveAxioms {
        entity_iri: String,
        target_file: PathBuf,
        #[serde(default)]
        statement_indexes: Vec<usize>,
        #[serde(default = "default_true")]
        exclude_primary: bool,
    },
    /// Merge one or more source ontology Turtle files into a target file.
    MergeOntologies {
        source_paths: Vec<PathBuf>,
        target_file: PathBuf,
    },
    /// Inline imported Turtle axioms into a root ontology and remove `owl:imports`.
    FlattenImports {
        ontology_file: PathBuf,
    },
    /// Remove unused `owl:imports` lines (heuristic: no imported entities referenced).
    CleanupImports {
        ontology_file: PathBuf,
    },
}

fn default_true() -> bool {
    true
}
