use serde::{Deserialize, Serialize};

/// OBO patch operation (ADR-0019).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum OboPatchOp {
    SetName {
        term_id: String,
        value: String,
    },
    AddSynonym {
        term_id: String,
        value: String,
        scope: String,
    },
    RemoveSynonym {
        term_id: String,
        value: String,
        /// When set, only a synonym with this scope is removed.
        /// When omitted and multiple scopes share the same text, apply fails.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scope: Option<String>,
    },
    AddDef {
        term_id: String,
        value: String,
    },
    RemoveDef {
        term_id: String,
    },
    AddXref {
        term_id: String,
        xref: String,
    },
    RemoveXref {
        term_id: String,
        xref: String,
    },
    SetNamespace {
        term_id: String,
        namespace: String,
    },
    SetDeprecated {
        term_id: String,
        value: bool,
    },
    AddIsA {
        term_id: String,
        parent_id: String,
    },
    RemoveIsA {
        term_id: String,
        parent_id: String,
    },
}
