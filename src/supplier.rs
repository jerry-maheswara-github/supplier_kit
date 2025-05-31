use std::sync::Arc;
use crate::errors::SupplierError;
use crate::models::{SupplierRequest, SupplierResponse};

/// A trait that represents a supplier, which is a provider of data or services.
/// A supplier can be queried with a `SupplierRequest` and will return a `SupplierResponse`.
/// It is implemented by different types that provide the actual supplier logic.
pub trait Supplier {
    /// Returns the name of the supplier.
    ///
    /// # Example
    /// ```
    /// use serde_json::json;
    /// use supplier_kit::errors::SupplierError;
    /// use supplier_kit::models::{SupplierRequest, SupplierResponse};
    /// use supplier_kit::supplier::Supplier;
    ///
    /// struct MySupplier {
    ///     name: String,
    ///     should_fail: bool,
    /// }
    /// impl MySupplier {
    ///     fn new(name: &str, should_fail: bool) -> Self {
    ///         Self {
    ///             name: name.to_string(),
    ///             should_fail,
    ///         }
    ///     }
    /// }
    /// impl Supplier for MySupplier {
    ///     fn name(&self) -> &str {
    ///         &self.name
    ///     }
    ///
    ///     fn query(&self, request: SupplierRequest) -> Result<SupplierResponse, SupplierError> {
    ///         if self.should_fail {
    ///             Err(SupplierError::Internal(format!("{} failed", self.name)))
    ///         } else {
    ///             Ok(SupplierResponse {
    ///                 data: json!({
    ///                     "supplier": self.name,
    ///                     "params": request.params
    ///                 }),
    ///             })
    ///         }
    ///     }
    /// }    
    /// 
    /// let supplier = MySupplier {name: "my_supplier".to_string(), should_fail: false};
    /// assert_eq!(supplier.name(), "my_supplier");
    /// ```
    fn name(&self) -> &str;

    /// Queries the supplier for data based on the given request.
    ///
    /// # Parameters
    /// - `request`: A `SupplierRequest` containing the operation and parameters.
    ///
    /// # Returns
    /// - `Ok(SupplierResponse)`: The response containing the requested data.
    /// - `Err(SupplierError)`: An error indicating what went wrong during the query.
    ///
    /// # Example
    /// ```
    /// use serde_json::json;
    /// use supplier_kit::errors::SupplierError;
    /// use supplier_kit::models::{SupplierOperation, SupplierRequest, SupplierResponse};
    /// use supplier_kit::supplier::Supplier;
    /// struct MySupplier {
    ///     name: String,
    ///     should_fail: bool,
    /// }
    /// impl MySupplier {
    ///     fn new(name: &str, should_fail: bool) -> Self {
    ///         Self {
    ///             name: name.to_string(),
    ///             should_fail,
    ///         }
    ///     }
    /// }
    /// impl Supplier for MySupplier {
    ///     fn name(&self) -> &str {
    ///         &self.name
    ///     }
    ///
    ///     fn query(&self, request: SupplierRequest) -> Result<SupplierResponse, SupplierError> {
    ///         if self.should_fail {
    ///             Err(SupplierError::Internal(format!("{} failed", self.name)))
    ///         } else {
    ///             Ok(SupplierResponse {
    ///                 data: json!({
    ///                     "supplier": self.name,
    ///                     "params": request.params
    ///                 }),
    ///             })
    ///         }
    ///     }
    /// }
    ///
    /// let supplier = MySupplier {name: "my_supplier".to_string(),should_fail: false};
    /// let request = SupplierRequest {
    ///     operation: SupplierOperation::Search,
    ///     params: json!({"query": "item"}),
    /// };
    /// let response = supplier.query(request);
    /// ```
    fn query(
        &self,
        request: SupplierRequest,
    ) -> Result<SupplierResponse, SupplierError>;
}

/// A registry for managing suppliers by name. It allows suppliers to be registered, retrieved by name, 
/// and provides a list of all registered suppliers.
#[derive(Default)]
pub struct SupplierRegistry {
    suppliers: std::collections::HashMap<String, Arc<dyn Supplier>>,
}

impl SupplierRegistry {
    /// Creates a new, empty supplier registry.
    ///
    /// # Returns
    /// A new instance of `SupplierRegistry`.
    ///
    /// # Example
    /// ```
    /// use supplier_kit::supplier::SupplierRegistry;
    /// let registry = SupplierRegistry::new();
    /// ```
    pub fn new() -> Self {
        Self {
            suppliers: std::collections::HashMap::new(),
        }
    }

    /// Registers a new supplier with the given name.
    ///
    /// # Parameters
    /// - `name`: The name of the supplier to register.
    /// - `supplier`: The supplier instance to register.
    ///
    /// # Example
    /// ```
    /// use serde_json::json;
    /// use supplier_kit::errors::SupplierError;
    /// use supplier_kit::models::{SupplierRequest, SupplierResponse};
    /// use supplier_kit::supplier::{Supplier, SupplierRegistry};
    /// struct MySupplier {
    ///     name: String,
    ///     should_fail: bool,
    /// }
    /// impl MySupplier {
    ///     fn new(name: &str, should_fail: bool) -> Self {
    ///         Self {
    ///             name: name.to_string(),
    ///             should_fail,
    ///         }
    ///     }
    /// }
    /// impl Supplier for MySupplier {
    ///     fn name(&self) -> &str {
    ///         &self.name
    ///     }
    ///
    ///     fn query(&self, request: SupplierRequest) -> Result<SupplierResponse, SupplierError> {
    ///         if self.should_fail {
    ///             Err(SupplierError::Internal(format!("{} failed", self.name)))
    ///         } else {
    ///             Ok(SupplierResponse {
    ///                 data: json!({
    ///                     "supplier": self.name,
    ///                     "params": request.params
    ///                 }),
    ///             })
    ///         }
    ///     }
    /// }
    /// let mut registry = SupplierRegistry::new();
    /// registry.register("my_supplier", MySupplier {name: "my_supplier".to_string(), should_fail: false});
    ///
    /// ```
    pub fn register<S>(&mut self, name: &str, supplier: S)
    where
        S: Supplier + 'static,
    {
        self.suppliers.insert(name.to_string(), Arc::new(supplier));
    }

    /// Retrieves a supplier by its name.
    ///
    /// # Parameters
    /// - `name`: The name of the supplier to retrieve.
    ///
    /// # Returns
    /// - `Some(Arc<dyn Supplier>)`: A wrapped supplier instance if found.
    /// - `None`: If the supplier with the specified name does not exist.
    ///
    /// # Example
    /// ```
    /// use supplier_kit::supplier::SupplierRegistry;
    /// let registry = SupplierRegistry::new();
    /// let supplier = registry.get("my_supplier");
    /// ```
    pub fn get(&self, name: &str) -> Option<Arc<dyn Supplier>> {
        self.suppliers.get(name).cloned()
    }

    /// Retrieves all the names of the registered suppliers.
    ///
    /// # Returns
    /// A vector of all the supplier names in the registry.
    ///
    /// # Example
    /// ```
    /// use serde_json::json;
    /// use supplier_kit::errors::SupplierError;
    /// use supplier_kit::models::{SupplierRequest, SupplierResponse};
    /// use supplier_kit::supplier::{Supplier, SupplierRegistry};
    /// struct MySupplier {
    ///     name: String,
    ///     should_fail: bool,
    /// }
    /// impl MySupplier {
    ///     fn new(name: &str, should_fail: bool) -> Self {
    ///         Self {
    ///             name: name.to_string(),
    ///             should_fail,
    ///         }
    ///     }
    /// }
    /// impl Supplier for MySupplier {
    ///     fn name(&self) -> &str {
    ///         &self.name
    ///     }
    ///
    ///     fn query(&self, request: SupplierRequest) -> Result<SupplierResponse, SupplierError> {
    ///         if self.should_fail {
    ///             Err(SupplierError::Internal(format!("{} failed", self.name)))
    ///         } else {
    ///             Ok(SupplierResponse {
    ///                 data: json!({
    ///                     "supplier": self.name,
    ///                     "params": request.params
    ///                 }),
    ///             })
    ///         }
    ///     }
    /// }
    ///
    /// let mut registry = SupplierRegistry::new();
    /// registry.register("my_supplier", MySupplier {name: "my_supplier".to_string(), should_fail: false});
    /// let names = registry.all_names();
    /// assert!(names.contains(&"my_supplier".to_string()));
    /// ```
    pub fn all_names(&self) -> Vec<String> {
        self.suppliers.keys().cloned().collect()
    }
}
