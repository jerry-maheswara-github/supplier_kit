use std::sync::Arc;
use crate::errors::SupplierError;
use crate::models::{SupplierRequest, SupplierResponse};
use crate::supplier::Supplier;

pub struct SupplierGroupResult {
    pub successes: Vec<(String, SupplierResponse)>,
    pub failures: Vec<(String, SupplierError)>,
}

pub trait SupplierGroup {
    fn group_name(&self) -> &str;

    fn query(&self, request: SupplierRequest) -> SupplierGroupResult;
}

pub struct BasicSupplierGroup {
    name: String,
    suppliers: Vec<Arc<dyn Supplier>>,
}

impl BasicSupplierGroup {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            suppliers: vec![],
        }
    }

    pub fn add_supplier<S>(&mut self, supplier: S)
    where
        S: Supplier + 'static,
    {
        self.suppliers.push(Arc::new(supplier));
    }

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
