use supplier_kit::models::{SupplierOperation, SupplierRequest};

#[test]
fn test_deserialize_supplier_request() {
    let json = r#"
        {
            "operation": "search",
            "params": { "keyword": "baju" }
        }
    "#;

    let req: SupplierRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.operation, SupplierOperation::Search);
}
