use std::sync::Arc;
use crate::errors::SupplierError;
use crate::models::{SupplierRequest, SupplierResponse};
use crate::supplier::Supplier;

// SupplierGroup trait untuk mengelompokkan supplier (tanpa Send/Sync)
pub trait SupplierGroup {
    fn group_name(&self) -> &str;

    /// Query data untuk semua supplier dalam grup (implementasi async di luar crate)
    fn query(
        &self,
        request: SupplierRequest,
    ) -> Result<Vec<SupplierResponse>, SupplierError>;
}

// Implementasi grup supplier dasar (tanpa paralelisme langsung)
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
}

impl SupplierGroup for BasicSupplierGroup {
    fn group_name(&self) -> &str {
        &self.name
    }

    fn query(
        &self,
        request: SupplierRequest,
    ) -> Result<Vec<SupplierResponse>, SupplierError> {
        // Implementor akan menangani paralelisme di luar crate ini
        let mut responses = Vec::new();
        for supplier in &self.suppliers {
            match supplier.query(request.clone()) {
                Ok(response) => responses.push(response),
                Err(e) => eprintln!("Supplier failed: {:?}", e),
            }
        }

        if responses.is_empty() {
            Err(SupplierError::Internal("No supplier succeeded".into()))
        } else {
            Ok(responses)
        }
    }
}
