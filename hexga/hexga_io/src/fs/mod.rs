use super::*;

mod file_system;
pub use file_system::*;

mod disk;
pub use disk::*;

// mod single_file;
// pub use single_file::*;

mod node;
pub use node::*;

mod result;
pub use result::*;

pub mod prelude
{
    pub use super::
    {
        file_system::{FsWrite,FsRead},
        result::*,
    };

    #[cfg(feature = "serde")]
    pub use super::disk::{SaveToDisk,LoadFromDisk};
}