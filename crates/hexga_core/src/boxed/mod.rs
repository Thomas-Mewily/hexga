use super::*;

mod pre_allocate_box;
pub use pre_allocate_box::*;

/*
Todo: make some mod pre_allocate_vec ?
*/

re_export_items_from_std_or_alloc!(boxed);
re_export_item_from_std_or_alloc!(boxed::Box);

pub mod prelude
{
    re_export_item_from_std_or_alloc!(boxed::Box);
}

pub mod traits
{}
