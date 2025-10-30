#![allow(unused_imports)]

#![feature(error_generic_member_access)] // std::error::Error for IoError

use std::default;
use std::io::{Read, BufReader, Write, BufWriter};
use std::borrow::Cow;
use std::{fmt::Display, str::Utf8Error, string::FromUtf8Error};
use std::{ops::{Deref, DerefMut}};
use std::any::Any;
use std::fmt::Formatter;


#[cfg(feature = "serde")]
use serde::
{
    Serialize, Serializer, Deserialize, Deserializer, de::Visitor,
    ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant}
};

pub mod fs;
use fs::*;

#[cfg(feature = "serde")]
pub mod markup;
#[cfg(feature = "serde")]
use markup::*;

pub mod io;
use io::*;

pub mod encoding;
use encoding::*;

#[cfg(feature = "serde")]
pub(crate) mod serde_impl;
#[cfg(feature = "serde")]
pub(crate) use serde_impl::*;


pub mod prelude
{
    pub use super::
    {
        fs::prelude::*,
        io::prelude::*,
        encoding::prelude::*,
    };

    #[cfg(feature = "serde")]
    pub use super::markup::*;
}