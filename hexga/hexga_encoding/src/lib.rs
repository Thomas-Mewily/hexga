#[allow(unused_imports)]
use std::io::{Read,Write,BufReader};
use std::{fmt::{Display,Formatter}, str::Utf8Error, string::FromUtf8Error};
use std::borrow::Cow;
use hexga_core::cfg::*;
use hexga_core::utils::*;

#[cfg(feature = "serde")]
use serde::
{
    Serialize, Serializer, Deserialize, Deserializer,
    de::Visitor,
    ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant}
};

mod fs_extension;
pub use fs_extension::*;

mod default_impl;

mod result;
pub use result::*;

mod save;
pub use save::*;

mod load;
pub use load::*;

mod base64;
pub use base64::*;

#[cfg(feature = "serde")]
mod serde_impl;
#[cfg(feature = "serde")]
pub use serde_impl::*;

mod url_data;
pub use url_data::*;

pub mod prelude
{
    pub use super::
    {
        fs_extension::prelude::*,
        result::*,
        save::prelude::*,
        load::prelude::*,
        url_data::prelude::*,
    };
}