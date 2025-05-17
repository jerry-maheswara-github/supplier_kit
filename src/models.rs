use serde::{Serialize, Deserialize};
use serde_json::Value;

/// Represents the type of operation requested from a supplier.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SupplierOperation {
    /// A search operation, such as product search
    Search,
    /// Retrieve detailed information for a specific item
    GetDetail,
    /// A custom, non-standard operation
    Other(String),
}

impl SupplierOperation {
    /// Normalizes the `Other(String)` variant into `snake_case` format.
    ///
    /// This only affects the `Other` variant. `Search` and `GetDetail` are returned unchanged.
    pub fn normalize(self) -> Self {
        match self {
            SupplierOperation::Other(s) => {
                let normalized = s
                    .trim()
                    .to_ascii_lowercase()
                    .replace([' ', '-', '/'], "_");
                SupplierOperation::Other(normalized)
            }
            other => other,
        }
    }

    /// Returns the operation as a &str for convenience (including the `Other` inner value).
    pub fn as_str(&self) -> &str {
        match self {
            SupplierOperation::Search => "search",
            SupplierOperation::GetDetail => "get_detail",
            SupplierOperation::Other(s) => s.as_str(),
        }
    }
}


/// Represents a request to be processed by a supplier.
///
/// This struct contains the desired operation (`operation`)
/// and a JSON value (`params`) holding any required parameters.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SupplierRequest {
    /// The operation to perform (e.g. search, get_detail, etc.)
    pub operation: SupplierOperation,

    /// Free-form parameters required by the operation.
    /// This can be any valid JSON structure (object, array, etc.)
    pub params: Value,
}

/// Represents a response returned by a supplier.
///
/// The response contains a single JSON value (`data`)
/// that holds the result of the requested operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SupplierResponse {
    /// The raw data returned from the supplier.
    /// This can be any valid JSON value.
    pub data: Value,
}