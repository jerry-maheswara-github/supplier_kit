//! # supplier_kit
//!
//! **A modular toolkit for managing and grouping dynamic data suppliers.**
//!
//! `supplier_kit` helps you build robust and extensible architectures that rely
//! on multiple external or internal data providers — known as *suppliers*. It is ideal
//! for use cases such as federated API calls, service orchestration, or data aggregation
//! (e.g., e-commerce platforms).
//!
//! ---
//! 
//! ## ✨ Features
//!
//! - `Supplier` trait: defines a common interface for all data providers
//! - `SupplierRegistry`: a container for registering and accessing suppliers by name
//! - `SupplierGroup`: abstraction to query multiple suppliers in a batch
//! - `SupplierGroupResult`: returns per-supplier successes and failures
//! - `register_suppliers!` macro: for easy supplier registration
//! - Utility helpers like `add_supplier_from_registry` and `add_suppliers_from_registry` as an option
//!
//! ---
//! 
//! ## 🔧  Use Cases
//!
//! - Aggregating product listings from multiple platforms
//! - Wrapping multiple internal microservices behind unified access
//! - Resilient systems that gracefully handle partial failure
//!
//! ---
//! 
//! ## 🚀  Quick Start
//! 
//! ### Example: Registering and Querying Multiple Suppliers
//!
//! This example demonstrates how to use `SupplierRegistry` and `BasicSupplierGroup`
//! to register multiple suppliers, group them, and execute a `Search` operation.
//!
//! It includes:
//! - Using the `register_suppliers!` macro
//! - Handling partial failures using `add_suppliers_from_registry`
//! - Group querying with `SupplierGroup` trait
//!
//! ```rust
//! use supplier_kit::models::{SupplierRequest, SupplierResponse, SupplierOperation};
//! use supplier_kit::supplier::{Supplier, SupplierRegistry};
//! use supplier_kit::supplier_group::{SupplierGroup, BasicSupplierGroup, SupplierGroupResult};
//! use supplier_kit::errors::SupplierError;
//! use supplier_kit::register_suppliers;
//! use serde_json::json;
//! use supplier_kit::utils::add_suppliers_from_registry;
//!
//! struct MockSupplier {
//!     name: String,
//!     should_fail: bool,
//! }
//!
//! impl MockSupplier {
//!     fn new(name: &str, should_fail: bool) -> Self {
//!         Self {
//!             name: name.to_string(),
//!             should_fail,
//!         }
//!     }
//! }
//!
//! impl Supplier for MockSupplier {
//!     fn name(&self) -> &str {
//!         &self.name
//!     }
//!
//!     fn query(&self, request: SupplierRequest) -> Result<SupplierResponse, SupplierError> {
//!         if self.should_fail {
//!             Err(SupplierError::Internal(format!("{} failed", self.name)))
//!         } else {
//!             Ok(SupplierResponse {
//!                 data: json!({
//!                     "supplier": self.name,
//!                     "params": request.params
//!                 }),
//!             })
//!         }
//!     }
//! }
//!
//! fn main() -> Result<(), SupplierError> {
//!     let mut registry = SupplierRegistry::new();
//!     register_suppliers!(
//!         registry,
//!         "s1" => MockSupplier::new("s1", false),
//!         "s2" => MockSupplier::new("s2", true),
//!         "s3" => MockSupplier::new("s3", false),
//!     );
//!
//!     let mut group = BasicSupplierGroup::new("example_group");
//!
//!     let failures = add_suppliers_from_registry(&mut group, &registry, &["s1", "s2", "s4"]);
//!
//!     if !failures.is_empty() {
//!         for (name, err) in failures {
//!             println!("Failed to add '{}': {}", name, err);
//!         }
//!     }
//!
//!     let request = SupplierRequest {
//!         operation: SupplierOperation::Search,
//!         params: json!({ "keyword": "laptop" }),
//!     };
//!
//!     let result: SupplierGroupResult = group.query(request);
//!
//!     println!("Successes:");
//!     for (name, response) in result.successes {
//!         println!("- [{}] Response: {}", name, response.data);
//!     }
//!
//!     println!("Failures:");
//!     for (name, error) in result.failures {
//!         println!("- [{}] Error: {}", name, error);
//!     }
//!
//!     Ok(())
//! }
//! ```
//! 
//! ---
//!
//! ## 📄 License
//!
//! Licensed under the [Apache-2.0 license](http://www.apache.org/licenses/LICENSE-2.0.txt)
//!
//! ---
//!
//! ## 👨 Author
//!
//! Jerry Maheswara <jerrymaheswara@gmail.com>
//!
//! ---
//!
//! ## ❤️ Built with Love in Rust
//!
//! This project is built with ❤️ using **Rust** — a systems programming language that is safe, fast, and concurrent.  
//! Rust is the perfect choice for building reliable and efficient applications.
//!
//! ---
//!
//! ## 🤝 Contributing
//!
//! Pull requests, issues, and feedback are welcome!  
//! If you find this crate useful, give it a ⭐ and share it with others in the Rustacean community.
//!
//! ---

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
