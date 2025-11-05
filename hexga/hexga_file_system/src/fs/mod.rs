use super::*;

mod file_system;
pub use file_system::*;

mod disk;
pub use disk::*;


mod node;
pub use node::*;

mod result;
pub use result::*;

#[cfg(feature = "serde")]
pub mod serde_impl;
#[cfg(feature = "serde")]
use serde_impl::*;


pub mod prelude
{
    pub use super::
    {
        file_system::{FsWrite,FsRead},
        result::*,
    };

    #[cfg(feature = "serde")]
    use super::serde_impl::prelude::*;
}