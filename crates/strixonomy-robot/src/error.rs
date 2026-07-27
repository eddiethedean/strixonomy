use thiserror::Error;

#[derive(Debug, Error)]
pub enum RobotError {
    #[error("ROBOT not found on PATH. Install from https://github.com/ontodev/robot/releases")]
    NotFound,

    #[error("failed to run ROBOT: {0}")]
    Io(#[from] std::io::Error),

    #[error("ROBOT exited with status {code}: {stderr}")]
    Failed { code: i32, stderr: String },

    #[error("ROBOT timed out after {0}s")]
    TimedOut(u64),

    #[error("{0}")]
    Run(String),
}

pub type Result<T> = std::result::Result<T, RobotError>;
