use std::io::{Read,Write,BufReader};
use std::{fmt::{Display,Formatter}, str::Utf8Error, string::FromUtf8Error};
use std::borrow::Cow;
use hexga_core::cfg::*;
use hexga_core::utils::*;
use hexga_core::map_on::prelude::*;

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

#[cfg(feature = "derive")]
pub use hexga_encoding_derive::*;

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

    #[cfg(feature = "derive")]
    pub use hexga_encoding_derive::{Load,Save,io};
}

pub mod cfg
{
    pub use hexga_core::cfg::CfgSerialize;
    pub use hexga_core::cfg::CfgDeserialize;
}