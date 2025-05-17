use thiserror::Error;

/// Represents all possible errors that can occur in the supplier framework.
#[derive(Debug, Error, Clone)]
pub enum SupplierError {
    /// The operation timed out, possibly due to a slow or unresponsive supplier.
    #[error("timeout")]
    Timeout,

    /// Authorization failed when attempting to query the supplier.
    #[error("unauthorized")]
    Unauthorized,

    /// The requested resource or supplier was not found.
    #[error("not found")]
    NotFound,

    /// A generic internal error occurred, often used as a fallback for unknown issues.
    #[error("internal error: {0}")]
    Internal(String),

    /// An error originating from an upstream service or external API.
    #[error("upstream error: {0}")]
    Upstream(String),

    /// Input provided to the supplier was invalid or malformed.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// The requested operation is not supported by the supplier implementation.
    #[error("unsupported operation: {0}")]
    UnsupportedOperation(String),
}
