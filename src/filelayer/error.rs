use thiserror::Error;

#[derive(Debug, Error)]
pub enum filelayer_Error {
    #[error("invalid id")]
    InvalidId,
    #[error("forbidden transition")]
    ForbiddenTransition,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type filelayer_Result<T> = Result<T, filelayer_Error>;
