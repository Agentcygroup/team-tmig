use thiserror::Error;

#[derive(Debug, Error)]
pub enum mold_Error {
    #[error("invalid id")]
    InvalidId,
    #[error("forbidden transition")]
    ForbiddenTransition,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type mold_Result<T> = Result<T, mold_Error>;
