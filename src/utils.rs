use crate::supplier::SupplierRegistry;
use crate::supplier_group::BasicSupplierGroup;
use crate::errors::SupplierError;

/// Adds a single supplier from the registry into a group by name.
///
/// This function looks up a supplier by its name in the provided `SupplierRegistry`,
/// and if found, adds it into the given `BasicSupplierGroup`.
///
/// Returns:
/// - `Ok(())` if the supplier was found and added successfully
/// - `Err(SupplierError::NotFound)` if the supplier name does not exist in the registry
///
/// # Example
///
/// ```rust
/// use supplier_kit::supplier::{Supplier, SupplierRegistry};
/// use supplier_kit::supplier_group::{BasicSupplierGroup};
/// use supplier_kit::utils::add_supplier_from_registry;
/// use supplier_kit::errors::SupplierError;
///
/// struct DummySupplier;
///
/// impl Supplier for DummySupplier {
///     fn name(&self) -> &str { "dummy" }
///     fn query(&self, _request: supplier_kit::models::SupplierRequest)
///         -> Result<supplier_kit::models::SupplierResponse, SupplierError> {
///         Err(SupplierError::Timeout)
///     }
/// }
///
/// let mut registry = SupplierRegistry::new();
/// registry.register("dummy", DummySupplier);
///
/// let mut group = BasicSupplierGroup::new("group1");
/// let result = add_supplier_from_registry(&mut group, &registry, "dummy");
/// assert!(result.is_ok());
/// ```
pub fn add_supplier_from_registry(
    group: &mut BasicSupplierGroup,
    registry: &SupplierRegistry,
    name: &str,
) -> Result<(), SupplierError> {
    match registry.get(name) {
        Some(supplier) => {
            group.add_supplier_arc(supplier.clone());
            Ok(())
        }
        None => Err(SupplierError::NotFound),
    }
}

/// Adds multiple suppliers from the registry into a group by their names.
///
/// Iterates through a list of supplier names, tries to fetch each from the registry,
/// and adds all valid ones into the given `BasicSupplierGroup`.
///
/// Returns a list of failures: for each name not found in the registry, a tuple of
/// `(name, SupplierError::NotFound)` is returned.
///
/// # Example
///
/// ```rust
/// use supplier_kit::supplier::{Supplier, SupplierRegistry};
/// use supplier_kit::supplier_group::{BasicSupplierGroup};
/// use supplier_kit::utils::add_suppliers_from_registry;
/// use supplier_kit::errors::SupplierError;
///
/// struct DummySupplier;
///
/// impl Supplier for DummySupplier {
///     fn name(&self) -> &str { "dummy" }
///     fn query(&self, _request: supplier_kit::models::SupplierRequest)
///         -> Result<supplier_kit::models::SupplierResponse, SupplierError> {
///         Ok(supplier_kit::models::SupplierResponse {
///             data: serde_json::json!({ "ok": true }),
///         })
///     }
/// }
///
/// let mut registry = SupplierRegistry::new();
/// registry.register("dummy", DummySupplier);
///
/// let mut group = BasicSupplierGroup::new("group2");
/// let failures = add_suppliers_from_registry(&mut group, &registry, &["dummy", "not_found"]);
///
/// assert_eq!(failures.len(), 1);
/// assert_eq!(failures[0].0, "not_found");
/// ```
pub fn add_suppliers_from_registry(
    group: &mut BasicSupplierGroup,
    registry: &SupplierRegistry,
    names: &[&str],
) -> Vec<(String, SupplierError)> {
    let mut failures = Vec::new();

    for &name in names {
        match registry.get(name) {
            Some(supplier) => group.add_supplier_arc(supplier.clone()),
            None => failures.push((name.to_string(), SupplierError::NotFound)),
        }
    }

    failures
}
