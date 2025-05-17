use supplier_kit::models::{SupplierRequest, SupplierResponse, SupplierOperation};
use supplier_kit::supplier::{Supplier, SupplierRegistry};
use supplier_kit::supplier_group::{SupplierGroup, BasicSupplierGroup, SupplierGroupResult};
use supplier_kit::errors::SupplierError;
use supplier_kit::register_suppliers;
use serde_json::json;
use supplier_kit::utils::add_suppliers_from_registry;

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
                data: json!({
                    "supplier": self.name,
                    "params": request.params
                }),
            })
        }
    }
}

fn main() -> Result<(), SupplierError> {
    let mut registry = SupplierRegistry::new();
    register_suppliers!(
        registry,
        "s1" => MockSupplier::new("s1", false),
        "s2" => MockSupplier::new("s2", true),
        "s3" => MockSupplier::new("s3", false),
    );

    let mut group = BasicSupplierGroup::new("example_group");

    let failures = add_suppliers_from_registry(&mut group, &registry, &["s1", "s2", "s4"]);

    if !failures.is_empty() {
        for (name, err) in failures {
            println!("❌  Failed to add '{}': {}", name, err);
        }
    }

    let request = SupplierRequest {
        operation: SupplierOperation::Search,
        params: json!({ "keyword": "laptop" }),
    };

    let result: SupplierGroupResult = group.query(request);

    println!("Successes:");
    for (name, response) in result.successes {
        println!("- [{}] Response: {}", name, response.data);
    }

    println!("Failures:");
    for (name, error) in result.failures {
        println!("- [{}] Error: {}", name, error);
    }

    Ok(())
}