#[cfg(test)]
mod tests {
    use supplier_kit::errors::SupplierError;
    use supplier_kit::models::{SupplierOperation, SupplierRequest, SupplierResponse};
    use supplier_kit::supplier::{Supplier, SupplierRegistry};

    #[derive(Debug)]
    struct FailingSupplier {
        name: String,
        error: SupplierError,
    }

    impl Supplier for FailingSupplier {
        fn name(&self) -> &str {
            &self.name
        }

        fn query(&self, _request: SupplierRequest) -> Result<SupplierResponse, SupplierError> {
            Err(self.error.clone())
        }
    }

    #[test]
    fn test_supplier_not_found() {
        let registry = SupplierRegistry::new();
        let result = registry.get("unknown_supplier");
        assert!(result.is_none(), "Expected None for unknown supplier");
    }

    #[test]
    fn test_supplier_query_returns_error() {
        let mut registry = SupplierRegistry::new();
        let failing_supplier = FailingSupplier {
            name: "bad_supplier".to_string(),
            error: SupplierError::Timeout,
        };

        registry.register("bad", failing_supplier);
        let supplier = registry.get("bad").expect("Supplier should be registered");

        let request = SupplierRequest {
            operation: SupplierOperation::Other( "search".to_string()).into(),
            params: serde_json::json!({}),
        };

        let result = supplier.query(request);
        
        eprintln!("{:?}", supplier.name());
        assert!(matches!(result, Err(SupplierError::Timeout)));
    }
}
