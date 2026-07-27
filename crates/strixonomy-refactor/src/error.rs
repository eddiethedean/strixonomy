use thiserror::Error;

#[derive(Debug, Error)]
pub enum RefactorError {
    #[error("entity not found: {0}")]
    EntityNotFound(String),
    #[error("unsupported format for write-back: {0}")]
    UnsupportedFormat(String),
    #[error("invalid refactor: {0}")]
    Invalid(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Core(#[from] strixonomy_core::StrixonomyError),
}

pub type Result<T> = std::result::Result<T, RefactorError>;
