/// This macro will create a `new` function for your type that return
/// `Default::default()`.
/// Your type must implement the Default trait to be used with this macro.
#[macro_export]
macro_rules! empty_new {
    ($type:ty) => {
        impl $type {
            pub fn new() -> Self {
                Default::default()
            }
        }
    };
}

/// This implement the `std::fmt::Display` trait for your type.
/// For the type `Add` it will display `"add"`.
#[macro_export]
macro_rules! empty_display {
    ($type:ty) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", stringify!($type).to_lowercase())
            }
        }
    };
}
