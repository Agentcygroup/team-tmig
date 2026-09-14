use thiserror::Error;

#[derive(Debug, Error)]
pub enum hitl_Error {
    #[error("invalid id")]
    InvalidId,
    #[error("forbidden transition")]
    ForbiddenTransition,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type hitl_Result<T> = Result<T, hitl_Error>;
