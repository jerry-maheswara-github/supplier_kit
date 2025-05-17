use std::sync::Arc;
use crate::errors::SupplierError;
use crate::models::{SupplierRequest, SupplierResponse};
use crate::supplier::Supplier;

/// Represents the result of querying a group of suppliers.
/// Contains both successful and failed responses for each supplier in the group.
pub struct SupplierGroupResult {
    /// A list of successful supplier queries, with each success containing the supplier's name and its response.
    pub successes: Vec<(String, SupplierResponse)>,

    /// A list of failed supplier queries, with each failure containing the supplier's name and the error encountered.
    pub failures: Vec<(String, SupplierError)>,
}

/// A trait representing a group of suppliers. 
/// A `SupplierGroup` can query all its suppliers and return their responses.
pub trait SupplierGroup {
    /// Returns the name of the supplier group.
    ///
    /// # Example
    /// ```
    /// use supplier_kit::supplier_group::{BasicSupplierGroup, SupplierGroup};
    /// let group = BasicSupplierGroup::new("group1");
    /// assert_eq!(group.group_name(), "group1");
    /// ```
    fn group_name(&self) -> &str;

    /// Queries all suppliers in the group with the provided request and returns the result of the query.
    ///
    /// # Parameters
    /// - `request`: A `SupplierRequest` containing the operation and parameters for the query.
    ///
    /// # Returns
    /// A `SupplierGroupResult` containing the successes and failures from querying the suppliers in the group.
    ///
    /// # Example
    /// ```
    /// use supplier_kit::models::{SupplierOperation, SupplierRequest};
    /// use supplier_kit::supplier_group::{BasicSupplierGroup, SupplierGroup};
    /// let group = BasicSupplierGroup::new("group1");
    /// let request = SupplierRequest {
    ///     operation: SupplierOperation::Search,
    ///     params: serde_json::json!({"query": "item"}),
    /// };
    /// let result = group.query(request);
    /// ```
    fn query(&self, request: SupplierRequest) -> SupplierGroupResult;
}

/// A basic implementation of a `SupplierGroup`, which can hold a list of suppliers 
/// and perform queries against all of them.
pub struct BasicSupplierGroup {
    name: String,
    suppliers: Vec<Arc<dyn Supplier>>,
}

impl BasicSupplierGroup {
    /// Creates a new supplier group with the specified name.
    ///
    /// # Parameters
    /// - `name`: The name of the group.
    ///
    /// # Returns
    /// A new `BasicSupplierGroup` instance.
    ///
    /// # Example
    /// ```
    /// use supplier_kit::supplier_group::{BasicSupplierGroup, SupplierGroup};
    /// let group = BasicSupplierGroup::new("group1");
    /// assert_eq!(group.group_name(), "group1");
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            suppliers: vec![],
        }
    }

    /// Adds a supplier to the group.
    /// This function takes ownership of the supplier and wraps it in an `Arc` for shared ownership.
    ///
    /// # Parameters
    /// - `supplier`: A supplier instance to add to the group.
    ///
    /// # Example
    /// ```
    /// use serde_json::json;
    /// use supplier_kit::errors::SupplierError;
    /// use supplier_kit::models::{SupplierRequest, SupplierResponse};
    /// use supplier_kit::supplier::Supplier;
    /// use supplier_kit::supplier_group::BasicSupplierGroup;
    ///
    /// struct MockSupplier {
    ///     name: String,
    ///     should_fail: bool,
    /// }
    ///
    /// impl MockSupplier {
    ///     fn new(name: &str, should_fail: bool) -> Self {
    ///         Self {
    ///             name: name.to_string(),
    ///             should_fail,
    ///         }
    ///     }
    /// }
    ///
    /// impl Supplier for MockSupplier {
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
    /// let mut group = BasicSupplierGroup::new("group1");
    /// group.add_supplier(MockSupplier::new("mock1", false));
    /// ```
    pub fn add_supplier<S>(&mut self, supplier: S)
    where
        S: Supplier + 'static,
    {
        self.suppliers.push(Arc::new(supplier));
    }

    /// Adds a supplier to the group using an already wrapped `Arc<dyn Supplier>`.
    ///
    /// # Parameters
    /// - `supplier`: An `Arc` containing a `dyn Supplier` to add to the group.
    ///
    /// # Example
    /// ```
    /// use std::sync::Arc;
    /// use serde_json::json;
    /// use supplier_kit::errors::SupplierError;
    /// use supplier_kit::models::{SupplierRequest, SupplierResponse};
    /// use supplier_kit::supplier::Supplier;
    /// use supplier_kit::supplier_group::BasicSupplierGroup;
    ///
    /// struct MockSupplier {
    ///     name: String,
    ///     should_fail: bool,
    /// }
    ///
    /// impl MockSupplier {
    ///     fn new(name: &str, should_fail: bool) -> Self {
    ///         Self {
    ///             name: name.to_string(),
    ///             should_fail,
    ///         }
    ///     }
    /// }
    ///
    /// impl Supplier for MockSupplier {
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
    /// let mut group = BasicSupplierGroup::new("group1");
    /// let supplier = Arc::new(MockSupplier::new("mock1", false));
    /// group.add_supplier_arc(supplier);
    /// ```
    pub fn add_supplier_arc(&mut self, supplier: Arc<dyn Supplier>) {
        self.suppliers.push(supplier);
    }
}

impl SupplierGroup for BasicSupplierGroup {
    fn group_name(&self) -> &str {
        &self.name
    }

    fn query(&self, request: SupplierRequest) -> SupplierGroupResult {
        let mut successes = Vec::new();
        let mut failures = Vec::new();

        for supplier in &self.suppliers {
            match supplier.query(request.clone()) {
                Ok(response) => successes.push((supplier.name().to_string(), response)),
                Err(e) => failures.push((supplier.name().to_string(), e)),
            }
        }

        SupplierGroupResult { successes, failures }
    }
}
