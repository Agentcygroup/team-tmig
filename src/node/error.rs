use thiserror::Error;

#[derive(Debug, Error)]
pub enum node_Error {
    #[error("invalid id")]
    InvalidId,
    #[error("forbidden transition")]
    ForbiddenTransition,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type node_Result<T> = Result<T, node_Error>;
