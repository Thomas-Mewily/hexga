use super::*;

mod save;
pub use save::*;

mod load;
pub use load::*;

mod result;
pub use result::*;

mod default_impl;
pub use default_impl::*;

#[cfg(feature = "derive")]
pub use hexga_io_derive::*;

pub mod prelude
{
    pub use super::
    {
        save::*,
        load::*
    };
    #[cfg(feature = "derive")]
    pub use super::{Load,Save,io};
}