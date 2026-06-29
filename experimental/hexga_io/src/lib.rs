#![allow(unused_imports)]
use std::borrow::Cow;
pub use std::path::{Path, PathBuf};
pub use std::{
    fs, io,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use hexga_core::prelude::*;

mod read_write;
pub use read_write::*;

mod global_io;
pub use global_io::*;

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
    pub use super::{Io, IoError, IoErrorKind, IoResult, PathBuf, Path};
}

pub mod traits
{
    pub use super::{
        FsDynRead, FsDynWrite, 
        FsRead, FsWrite, Fs, 
        //FsProvider,
        IoErrorExt,

        GetPath, SetPath,
    };
}
