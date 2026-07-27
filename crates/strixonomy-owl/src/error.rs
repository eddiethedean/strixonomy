use thiserror::Error;

pub type Result<T> = std::result::Result<T, OwlError>;

#[derive(Debug, Error)]
pub enum OwlError {
    #[error("core error: {0}")]
    Core(#[from] strixonomy_core::StrixonomyError),

    #[error("unsupported format for write-back: {0}")]
    UnsupportedFormat(String),

    #[error("patch validation failed: {0}")]
    PatchInvalid(String),

    #[error("entity not found: {0}")]
    EntityNotFound(String),

    #[error("entity already exists: {0}")]
    EntityExists(String),

    #[error("Horned-OWL load failed: {0}")]
    LoadFailed(String),

    #[error("Horned-OWL serialize failed: {0}")]
    SerializeFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("store error: {0}")]
    Store(String),

    #[error("invalid Manchester expression: {0}")]
    ManchesterInvalid(String),
}
