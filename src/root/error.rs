use thiserror::Error;

#[derive(Debug, Error)]
pub enum root_Error {
    #[error("invalid id")]
    InvalidId,
    #[error("forbidden transition")]
    ForbiddenTransition,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type root_Result<T> = Result<T, root_Error>;
