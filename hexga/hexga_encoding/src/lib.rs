#![feature(error_generic_member_access)] // std::error::Error for IoError

use std::io::{Read,Write};
use std::{fmt::Display, str::Utf8Error, string::FromUtf8Error};


#[cfg(feature = "serde")]
use serde::
{
    Serializer, Deserializer,
    de::{Error, Visitor}
};

pub type Extension = String;
#[allow(non_camel_case_types)]
pub type extension = str;



mod result;
pub use result::*;

mod save;
pub use save::*;

mod load;
pub use load::*;

mod base64;
pub use base64::*;

mod url_data;
pub use url_data::*;

pub mod prelude
{
    pub use super::
    {
        Extension,extension,
        result::*,
        save::*,
        load::*,
        url_data::prelude::*,
    };
}