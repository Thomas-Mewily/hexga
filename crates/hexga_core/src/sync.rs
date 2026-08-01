re_export_items_from_std_or_alloc!(sync);

pub type ArcWeak<T> = crate::sync::Weak<T>;

pub mod prelude
{
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::ArcWeak;
}