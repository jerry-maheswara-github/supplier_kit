#[cfg(test)]
mod tests {
    use supplier_kit::models::SupplierOperation;

    #[test]
    fn test_normalize_other_variant() {
        let raw = SupplierOperation::Other("Submit Transaction".into());
        let norm = raw.normalize();
        assert_eq!(norm, SupplierOperation::Other("submit_transaction".to_string()));

        let raw2 = SupplierOperation::Other("cancel-order".into());
        let norm2 = raw2.normalize();
        assert_eq!(norm2, SupplierOperation::Other("cancel_order".to_string()));

        let raw3 = SupplierOperation::Other(" check/status ".into());
        let norm3 = raw3.normalize();
        assert_eq!(norm3, SupplierOperation::Other("check_status".to_string()));
    }

    #[test]
    fn test_normalize_preserves_known_variants() {
        let op = SupplierOperation::Search;
        assert_eq!(op.clone().normalize(), op);

        let op = SupplierOperation::GetDetail;
        assert_eq!(op.clone().normalize(), op);
    }

    #[test]
    fn test_as_str_for_all_variants() {
        assert_eq!(SupplierOperation::Search.as_str(), "search");
        assert_eq!(SupplierOperation::GetDetail.as_str(), "get_detail");

        let other = SupplierOperation::Other("sync_inventory".into());
        assert_eq!(other.as_str(), "sync_inventory");
    }
}
