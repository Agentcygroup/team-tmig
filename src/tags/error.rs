use thiserror::Error;

#[derive(Debug, Error)]
pub enum tags_Error {
    #[error("invalid id")]
    InvalidId,
    #[error("forbidden transition")]
    ForbiddenTransition,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type tags_Result<T> = Result<T, tags_Error>;
