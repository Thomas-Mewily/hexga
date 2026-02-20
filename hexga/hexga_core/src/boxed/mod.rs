use super::*;

mod drop_only_box;
pub use drop_only_box::*;

re_export_items_from_std_or_alloc!(boxed);

pub mod prelude
{
    pub use super::Box;
}
