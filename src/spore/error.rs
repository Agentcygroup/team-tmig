use thiserror::Error;

#[derive(Debug, Error)]
pub enum spore_Error {
    #[error("invalid id")]
    InvalidId,
    #[error("forbidden transition")]
    ForbiddenTransition,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type spore_Result<T> = Result<T, spore_Error>;
