pub type Result<T = (), E = ()> = core::result::Result<T, E>;

re_export_item_from_std!(result::IntoIter);
re_export_item_from_std!(result::IterMut);
re_export_item_from_std!(result::Iter);

pub mod prelude
{
    pub use super::Result;
}