use serde::{Deserialize, Serialize};
use strixonomy_obo::OboPatchOp;
use strixonomy_owl::PatchOp;

/// A single semantic edit, represented as a format-specific patch op until
/// RDF/XML and OWL/XML adapters land (v0.21).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "format", rename_all = "snake_case")]
pub enum SemanticChange {
    Turtle { change: PatchOp },
    Obo { change: OboPatchOp },
}

impl SemanticChange {
    pub fn turtle(change: PatchOp) -> Self {
        Self::Turtle { change }
    }

    pub fn obo(change: OboPatchOp) -> Self {
        Self::Obo { change }
    }

    pub fn is_turtle(&self) -> bool {
        matches!(self, Self::Turtle { .. })
    }

    pub fn is_obo(&self) -> bool {
        matches!(self, Self::Obo { .. })
    }
}
