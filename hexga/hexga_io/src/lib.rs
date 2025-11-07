//! Io file abstraction based on [Hexga Encoding](https://crates.io/crates/hexga_encoding)
//! `Load` and `Save` to allow loading/saving a value to a file.
//!
//! It support custom user define extension and convertion,
//! and it's also support common markup extension (json, ron, xml...).
#![allow(unused_imports)]

use std::fmt::Display;
use std::{str::Utf8Error, string::FromUtf8Error};
use std::{borrow::{Cow, Borrow, BorrowMut}, ffi::OsStr, iter::FusedIterator, ops::*};
use hexga_encoding::{Base64Error, EncodeError, BinUrlData, prelude::*};
type StdPath = std::path::Path;

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::
{
    Serialize, Serializer, Deserialize, Deserializer, de::Visitor,
    ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant}
};

#[cfg(feature = "serde")]
mod serde_impl;
#[cfg(feature = "serde")]
pub use serde_impl::*;

mod fs_path;
pub use fs_path::*;

mod io;
pub use io::*;

mod result;
pub use result::*;

mod fs;
pub(crate) use fs::*;

pub mod prelude
{
    pub use super::{
        io::*,
        result::*,
        fs_path::*,
    };

    #[cfg(feature = "serde")]
    pub use super::{
        serde_impl::*
    };
}