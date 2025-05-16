use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierRequest {
    pub operation: String, // Misal: "search", "get_detail", dll.
    pub params: Value,     // Parameter bebas, bisa objek atau array
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierResponse {
    pub data: Value,       // Data yang diterima dari supplier, bisa apa saja
}
