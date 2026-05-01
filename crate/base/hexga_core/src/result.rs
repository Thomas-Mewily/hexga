pub type Result<T = (), E = ()> = core::result::Result<T, E>;

re_export_item_from_std_or_core!(result::IntoIter);
re_export_item_from_std_or_core!(result::IterMut);
re_export_item_from_std_or_core!(result::Iter);

pub mod prelude
{
    pub use super::Result;
}
