use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum SupplierError {
    #[error("timeout")]
    Timeout,

    #[error("unauthorized")]
    Unauthorized,

    #[error("not found")]
    NotFound,

    #[error("internal error: {0}")]
    Internal(String),

    #[error("upstream error: {0}")]
    Upstream(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("unsupported operation: {0}")]
    UnsupportedOperation(String),
}
