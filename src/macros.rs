/// A helper function to register multiple suppliers at once in the `SupplierRegistry`.
#[macro_export]
macro_rules! register_suppliers {
    ($registry:expr, $( $name:expr => $supplier:expr ),+ $(,)?) => {
        $(
            $registry.register($name, $supplier);
        )+
    };
}
