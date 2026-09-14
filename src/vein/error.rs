use thiserror::Error;

#[derive(Debug, Error)]
pub enum vein_Error {
    #[error("invalid id")]
    InvalidId,
    #[error("forbidden transition")]
    ForbiddenTransition,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type vein_Result<T> = Result<T, vein_Error>;
