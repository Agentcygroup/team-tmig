use thiserror::Error;

#[derive(Debug, Error)]
pub enum controlplane_Error {
    #[error("invalid id")]
    InvalidId,
    #[error("forbidden transition")]
    ForbiddenTransition,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type controlplane_Result<T> = Result<T, controlplane_Error>;
