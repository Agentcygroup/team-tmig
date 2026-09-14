use thiserror::Error;

#[derive(Debug, Error)]
pub enum swarm_Error {
    #[error("invalid id")]
    InvalidId,
    #[error("forbidden transition")]
    ForbiddenTransition,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type swarm_Result<T> = Result<T, swarm_Error>;
