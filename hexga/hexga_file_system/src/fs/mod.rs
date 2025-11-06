use super::*;

mod file_system;
pub use file_system::*;

mod disk;
pub use disk::*;

mod auto_correct;
pub use auto_correct::*;


mod node;
pub use node::*;

mod result;
pub use result::*;

#[cfg(feature = "serde")]
mod serde_impl;
#[cfg(feature = "serde")]
pub use serde_impl::*;


pub mod prelude
{
    pub use super::
    {
        file_system::{FsWrite,FsRead},
        result::*,
    };

    #[cfg(feature = "serde")]
    pub use super::serde_impl::prelude::*;
}