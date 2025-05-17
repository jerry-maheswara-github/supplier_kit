//! # supplier_kit
//!
//! Kerangka kerja modular untuk mengelola dan mengelompokkan supplier dinamis.

//! Contoh penggunaan dasar supplier_kit:
//!
//! ```
//! use supplier_kit::models::{SupplierRequest, SupplierResponse, SupplierOperation};
//! use supplier_kit::supplier::{Supplier, SupplierRegistry};
//! use supplier_kit::errors::SupplierError;
//! use serde_json::json;
//!
//! struct EchoSupplier;
//!
//! impl Supplier for EchoSupplier {
//!     fn name(&self) -> &str {
//!         "echo"
//!     }
//!
//!     fn query(&self, request: SupplierRequest) -> Result<SupplierResponse, SupplierError> {
//!         Ok(SupplierResponse {
//!             data: json!({
//!                 "operation": format!("{:?}", request.operation),
//!                 "params": request.params
//!             }),
//!         })
//!     }
//! }
//!
//! fn main() -> Result<(), SupplierError> {
//!     let mut registry = SupplierRegistry::new();
//!     registry.register("echo", EchoSupplier);
//!
//!     let supplier = registry.get("echo").unwrap();
//!     let request = SupplierRequest {
//!         operation: SupplierOperation::Search,
//!         params: json!({"keyword": "laptop"}),
//!     };
//!
//!     let response = supplier.query(request)?;
//!     assert_eq!(response.data["params"]["keyword"], "laptop");
//!     Ok(())
//! }
//! ```

/// Module for handling errors throughout the supplier kit.
/// 
/// It defines custom error types used within the crate, such as timeout,
/// unauthorized access, and internal errors.
pub mod errors;

/// Module containing common data models used by the supplier kit.
/// 
/// It defines structures such as `SupplierRequest`, `SupplierResponse`, and `SupplierOperation`,
/// which are used for communication between the system and suppliers.
pub mod models;

/// Module for defining suppliers and their interactions.
/// 
/// This includes the `Supplier` trait that must be implemented by any provider (supplier),
/// as well as the `SupplierRegistry` for registering and retrieving suppliers.
pub mod supplier;

/// Module for grouping suppliers together into logical collections.
/// 
/// This module defines the `SupplierGroup` trait and provides a default implementation
/// via `BasicSupplierGroup` to allow querying multiple suppliers at once.
pub mod supplier_group;

/// Utility module for helper functions and internal logic.
/// 
/// This module contains utilities that are shared across other modules,
/// such as serialization or validation functions that aren't specific to one part of the crate.
pub mod utils;

/// Macros used throughout the supplier kit to improve ergonomics and reduce boilerplate code.
/// 
/// For example, macros for registering multiple suppliers in a concise manner.
pub mod macros;
