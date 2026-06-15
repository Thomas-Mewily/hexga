#![allow(unused_imports)]
use std::borrow::Cow;
pub use std::{
    fs, io,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use hexga_core::prelude::*;
use hexga_encoding::*;

mod file_system;
pub use file_system::*;

mod global_io;
pub use global_io::*;

//mod path_wrapper;
//pub use path_wrapper::*;

pub use std::path::{Path, PathBuf};

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
    pub use super::{FsDynRead, FsDynWrite, FsRead, FsWrite, Fs, IoErrorExt};
}
