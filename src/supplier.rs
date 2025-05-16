use std::sync::Arc;
use crate::errors::SupplierError;
use crate::models::{SupplierRequest, SupplierResponse};

// Supplier trait untuk semua jenis provider (tanpa Send/Sync)
pub trait Supplier {
    fn name(&self) -> &str;

    /// Query data, eksekusi dilakukan oleh implementor
    fn query(
        &self,
        request: SupplierRequest,
    ) -> Result<SupplierResponse, SupplierError>;
}

// SupplierRegistry untuk mendaftarkan dan mengambil supplier
pub struct SupplierRegistry {
    suppliers: std::collections::HashMap<String, Arc<dyn Supplier>>,
}

impl SupplierRegistry {
    pub fn new() -> Self {
        Self {
            suppliers: std::collections::HashMap::new(),
        }
    }

    pub fn register<S>(&mut self, name: &str, supplier: S)
    where
        S: Supplier + 'static,
    {
        self.suppliers.insert(name.to_string(), Arc::new(supplier));
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Supplier>> {
        self.suppliers.get(name).cloned()
    }

    pub fn all_names(&self) -> Vec<String> {
        self.suppliers.keys().cloned().collect()
    }
}
