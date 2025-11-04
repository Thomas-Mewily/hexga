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
mod deserializer_load;
#[cfg(feature = "serde")]
pub use deserializer_load::*;

#[cfg(feature = "serde")]
mod deserializer_txt;
#[cfg(feature = "serde")]
pub use deserializer_txt::*;

#[cfg(feature = "serde")]
mod save;
#[cfg(feature = "serde")]
pub use save::*;

#[cfg(feature = "serde")]
mod serializer_save;
#[cfg(feature = "serde")]
pub use serializer_save::*;

#[cfg(feature = "serde")]
mod serializer_txt;
#[cfg(feature = "serde")]
pub use serializer_txt::*;

#[cfg(feature = "serde")]
mod keyword;
#[cfg(feature = "serde")]
pub use keyword::*;

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