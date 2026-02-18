use super::*;

mod collect_to;
pub use collect_to::*;

mod try_from_iter;
pub use try_from_iter::*;


// Warning : std::iter define the `Empty` iterator, but we also define `Empty` as the Empty type ()
re_export_items_from_std!(iter);

pub mod prelude
{
    pub use super::{collect_to::*,try_from_iter::*};
}