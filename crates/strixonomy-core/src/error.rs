use thiserror::Error;

pub type Result<T> = std::result::Result<T, StrixonomyError>;

#[derive(Debug, Error)]
pub enum StrixonomyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("scanner error: {0}")]
    Scanner(String),

    #[error("{0}")]
    Other(String),
}
