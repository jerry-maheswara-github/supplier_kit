use thiserror::Error;

#[derive(Debug, Error)]
pub enum SupplierError {
    #[error("timeout")]
    Timeout,
    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,
    #[error("internal error: {0}")]
    Internal(String),
}
