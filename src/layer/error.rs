use thiserror::Error;

#[derive(Debug, Error)]
pub enum layer_Error {
    #[error("invalid id")]
    InvalidId,
    #[error("forbidden transition")]
    ForbiddenTransition,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type layer_Result<T> = Result<T, layer_Error>;
