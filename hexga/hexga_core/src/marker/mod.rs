use super::*;

mod collection;
pub use collection::*;

re_export_items_from_std!(marker);

pub mod prelude
{
    pub use super::
    {
        Copy,Sized,Send,Sync,
        collection::*,
    };
}