use super::*;

mod file_system;
pub use file_system::*;

mod disk;
pub use disk::*;

#[cfg(feature = "serde")]
mod load;
#[cfg(feature = "serde")]
pub use load::*;

#[cfg(feature = "serde")]
mod load_deserializer;
#[cfg(feature = "serde")]
pub use load_deserializer::*;


#[cfg(feature = "serde")]
mod save;
#[cfg(feature = "serde")]
pub use save::*;

#[cfg(feature = "serde")]
mod save_serializer;
#[cfg(feature = "serde")]
pub use save_serializer::*;


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
    pub use super::{load::*,save::*};
}