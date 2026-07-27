//! SWRL rule model, validation, and formatting for Strixonomy (v0.23).

mod builtins;
mod model;
mod turtle;
mod validate;

pub use builtins::{is_supported_builtin, SUPPORTED_BUILTINS, SWRLB_NS};
pub use model::{
    SwrlAtom, SwrlDArg, SwrlDiagnostic, SwrlIArg, SwrlRule, SwrlRuleSummary, SwrlSeverity,
};
pub use turtle::{
    parse_swrl_rule_json, rewrite_swrl_iris_in_turtle, rule_to_turtle_fragment,
    rules_from_turtle_document,
};
pub use validate::validate_rule;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SwrlError {
    #[error("{0}")]
    Message(String),
    #[error("invalid rule JSON: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, SwrlError>;
