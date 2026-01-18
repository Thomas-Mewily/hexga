#[macro_export]
macro_rules! trait_marker {
    (
        $(#[$meta:meta])*
        $name:ident : $($bounds:tt)+
    ) => {
        $(#[$meta])*
        pub trait $name: $($bounds)+ {}
        impl<T> $name for T where T: $($bounds)+ {}
    };
}

pub mod prelude
{
    pub use crate::trait_marker;
}