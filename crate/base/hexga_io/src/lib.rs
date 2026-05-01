//! Io file abstraction based on [Hexga Encoding](https://crates.io/crates/hexga_encoding)
//! `Load` and `Save` to allow loading/saving a value to a file.
//!
//! It support custom user define extension and convertion,
//! and it's also support common markup extension (json, ron, xml...).
use hexga_encoding::{Base64Error, EncodeError, prelude::*};
use std::fmt::Display;
use std::{str::Utf8Error, string::FromUtf8Error};

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::Visitor,
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
};

//mod fs_path;
//pub use fs_path::*;

mod path_extension;
pub use path_extension::*;

pub use std::path::{Path, PathBuf};

mod io;
pub use io::*;

mod result;
pub use result::*;

mod fs;
//pub(crate) use fs::*;

pub mod prelude
{
    pub use super::{
        FileError,
        IoError,
        IoLoad,
        IoResult,
        IoSave, //fs_path::*,
        //result::*,
        PathExtension,
        io::*,
    };
}

pub trait IoLoad: Load + Sized
{
    fn load<P>(path: P) -> IoResult<Self>
    where
        P: AsRef<Path>,
    {
        Io.load(path)
    }
}
impl<T> IoLoad for T where T: Load {}
pub trait IoSave: Save
{
    fn save<P>(&self, path: P) -> IoResult
    where
        P: AsRef<Path>,
    {
        Io.save(path, self)
    }
}
impl<T> IoSave for T where T: Save + ?Sized {}
