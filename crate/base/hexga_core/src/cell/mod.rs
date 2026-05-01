use super::*;

re_export_items_from_std_or_core!(cell);

#[cfg(feature = "std")]
mod single_thread_cell;
#[cfg(feature = "std")]
pub use single_thread_cell::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DifferentThreadError;

impl Debug for DifferentThreadError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "different thread") }
}
