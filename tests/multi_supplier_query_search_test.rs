#[cfg(test)]
mod tests {
    use supplier_kit::models::*;
    use supplier_kit::supplier::*;
    use supplier_kit::supplier_group::*;
    use supplier_kit::errors::*;
    use serde_json::json;

    struct MockSupplier {
        name: String,
        should_fail: bool,
    }

    impl MockSupplier {
        fn new(name: &str, should_fail: bool) -> Self {
            Self {
                name: name.to_string(),
                should_fail,
            }
        }
    }

    impl Supplier for MockSupplier {
        fn name(&self) -> &str {
            &self.name
        }

        fn query(&self, request: SupplierRequest) -> Result<SupplierResponse, SupplierError> {
            if self.should_fail {
                Err(SupplierError::Internal(format!("{} failed", self.name)))
            } else {
                Ok(SupplierResponse {
                    data: json!({"echo": request.params}),
                })
            }
        }
    }

    #[test]
    fn test_basic_supplier_group_mixed_success_and_failure() {
        let mut group = BasicSupplierGroup::new("group_partial");
        group.add_supplier(MockSupplier::new("mock1", false)); // success
        group.add_supplier(MockSupplier::new("mock2", true));  // fail
        group.add_supplier(MockSupplier::new("mock3", false)); // success

        let request = SupplierRequest {
            operation: SupplierOperation::Search,
            params: json!({"query": "partial"}),
        };

        let result = group.query(request);
        assert_eq!(result.successes.len(), 2);
        assert_eq!(result.failures.len(), 1);

        let success_names: Vec<_> = result.successes.iter().map(|(n, _)| n).collect();
        assert!(success_names.contains(&&"mock1".to_string()));
        assert!(success_names.contains(&&"mock3".to_string()));

        let fail_names: Vec<_> = result.failures.iter().map(|(n, _)| n).collect();
        assert!(fail_names.contains(&&"mock2".to_string()));
    }

    #[test]
    fn test_basic_supplier_group_failure() {
        let mut group = BasicSupplierGroup::new("group_fail");
        group.add_supplier(MockSupplier::new("mock1", true));
        group.add_supplier(MockSupplier::new("mock2", true));

        let request = SupplierRequest {
            operation: SupplierOperation::Search,
            params: json!({"query": "fail-all"}),
        };

        let result = group.query(request);
        assert_eq!(result.successes.len(), 0);
        assert_eq!(result.failures.len(), 2);

        for (name, err) in result.failures {
            assert!(["mock1", "mock2"].contains(&name.as_str()));
            match err {
                SupplierError::Internal(msg) => assert!(msg.contains("failed")),
                _ => panic!("Unexpected error variant"),
            }
        }
    }

    #[test]
    fn test_basic_supplier_group_all_success() {
        let mut group = BasicSupplierGroup::new("group_success");
        group.add_supplier(MockSupplier::new("mock1", false));
        group.add_supplier(MockSupplier::new("mock2", false));

        let request = SupplierRequest {
            operation: SupplierOperation::Search,
            params: json!({"query": "test"}),
        };

        let result = group.query(request);
        assert_eq!(result.successes.len(), 2);
        assert_eq!(result.failures.len(), 0);

        for (name, resp) in result.successes {
            assert!(["mock1", "mock2"].contains(&name.as_str()));
            assert_eq!(resp.data["echo"]["query"], "test");
        }
    }

    #[test]
    fn test_supplier_group_names() {
        let mut group = BasicSupplierGroup::new("group4");
        group.add_supplier(MockSupplier::new("mock1", false));
        group.add_supplier(MockSupplier::new("mock2", false));

        assert_eq!(group.group_name(), "group4");
    }
}