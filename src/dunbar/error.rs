use thiserror::Error;

#[derive(Debug, Error)]
pub enum dunbar_Error {
    #[error("invalid id")]
    InvalidId,
    #[error("forbidden transition")]
    ForbiddenTransition,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type dunbar_Result<T> = Result<T, dunbar_Error>;
