#![allow(unused_imports)]

#![feature(error_generic_member_access)] // std::error::Error for IoError

use std::default;
use std::io::{Read, BufReader, Write, BufWriter};
use std::borrow::Cow;
use std::{fmt::Display, str::Utf8Error, string::FromUtf8Error};
use std::{ops::*};
use std::any::Any;
use std::fmt::Formatter;
use std::{borrow::{Borrow, BorrowMut}, ffi::OsStr, iter::FusedIterator};


#[cfg(feature = "serde")]
use serde::
{
    Serialize, Serializer, Deserialize, Deserializer, de::Visitor,
    ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant}
};
#[cfg(feature = "serde")]
use hexga_serde::{prelude::*,de::*,ser::*};

use hexga_encoding::{prelude::*,Base64Error};

pub mod fs;
use fs::*;

pub mod io;
use io::*;

pub mod prelude
{
    pub use super::
    {
        fs::prelude::*,
        io::prelude::*,
    };
}