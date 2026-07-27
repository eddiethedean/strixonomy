//! Structured quick-fix payloads for diagnostics and LSP code actions.

use serde::{Deserialize, Serialize};

/// Machine-readable quick fix attached to a [`crate::Diagnostic`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum QuickFix {
    /// Insert text at a 1-based line/column in the document.
    InsertText { label: String, line: usize, column: usize, text: String },
    /// Apply Strixonomy Turtle patch operations (same JSON as `strixonomy patch`).
    ApplyPatch { label: String, document_path: String, patches: Vec<serde_json::Value> },
    /// Remove a line (1-based) from the document.
    RemoveLine { label: String, line: usize },
}

impl QuickFix {
    pub fn encode(&self) -> Option<String> {
        serde_json::to_string(self).ok()
    }

    pub fn decode(raw: &str) -> Option<Self> {
        serde_json::from_str(raw).ok()
    }
}
