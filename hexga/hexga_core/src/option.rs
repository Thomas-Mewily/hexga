pub type Option<T> = core::option::Option<T>;

re_export_item_from_std!(option::Iter);
re_export_item_from_std!(option::IterMut);
re_export_item_from_std!(option::IntoIter);

pub mod prelude
{
    pub use super::Option;
}