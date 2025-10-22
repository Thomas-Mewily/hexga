use super::*;

mod file_system;
pub use file_system::*;

mod disk;
pub use disk::*;

mod bytes;
pub use bytes::*;

mod node;
pub use node::*;

mod result;
pub use result::*;

pub mod prelude
{
    pub use super::
    {
        file_system::{FsWrite,FsRead},
        disk::{SaveToDisk,LoadFromDisk},
        result::*
    };
}