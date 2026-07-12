#![allow(unused_imports)]
use std::borrow::Cow;
pub use std::path::{Path, PathBuf};
pub use std::{fs, io};

use hexga_core::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

mod file_kind;
pub use file_kind::*;

mod file_system;
pub use file_system::*;

mod common_io;
pub use common_io::*;

/*
mod path_wrapper;
pub use path_wrapper::*;
*/

mod path_extension;
pub use path_extension::*;

mod result;
pub use result::*;

pub mod prelude
{
    pub use super::traits::*;
    pub use super::{
        IoGlobal, IoData,
        IoResult, IoError, IoErrorKind, Path, PathBuf
    };
}

pub mod traits
{
    pub use super::
    {
        FileSystem,

        FileSystemDynRead,
        FileSystemDynWrite,

        FileSystemRead,
        FileSystemWrite,

        FileSystemProvider,
        FileSystemIsolated,

        GetPath,
        SetPath,

        //FsProvider,
        IoErrorExt,
    };
}
