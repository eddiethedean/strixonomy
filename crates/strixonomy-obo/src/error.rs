use thiserror::Error;

pub type Result<T> = std::result::Result<T, OboError>;

#[derive(Debug, Error)]
pub enum OboError {
    #[error("OBO parse error: {0}")]
    Parse(String),
    #[error("term not found: {0}")]
    TermNotFound(String),
    #[error("invalid patch: {0}")]
    PatchInvalid(String),
    #[error(transparent)]
    Core(#[from] strixonomy_core::StrixonomyError),
    #[error("IO error: {0}")]
    Io(String),
}
