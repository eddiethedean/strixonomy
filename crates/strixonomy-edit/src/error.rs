use thiserror::Error;

#[derive(Debug, Error)]
pub enum EditError {
    #[error("empty transaction")]
    Empty,

    #[error("transaction mixes Turtle and OBO changes")]
    MixedFormats,

    #[error("change is not invertible: {0}")]
    NotInvertible(String),

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("unsupported format for transaction: {0}")]
    UnsupportedFormat(String),

    #[error(transparent)]
    Owl(#[from] strixonomy_owl::OwlError),

    #[error(transparent)]
    Obo(#[from] strixonomy_obo::OboError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, EditError>;
