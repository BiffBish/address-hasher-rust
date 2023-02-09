#[macro_export]
macro_rules! setup_profile_method {
    ($name:ident) => {
        #[allow(unused_variables)]
        static $name: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);
    };
}
