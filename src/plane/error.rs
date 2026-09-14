use thiserror::Error;

#[derive(Debug, Error)]
pub enum plane_Error {
    #[error("invalid id")]
    InvalidId,
    #[error("forbidden transition")]
    ForbiddenTransition,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type plane_Result<T> = Result<T, plane_Error>;
