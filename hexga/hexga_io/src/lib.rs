#![allow(unused_imports)]

use std::io::{Read, BufReader, Write, BufWriter};
use std::borrow::Cow;
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};


pub mod fs;
use fs::*;

pub mod asset;
use asset::*;

pub mod markup;
use markup::*;

mod extensions;
pub use extensions::*;

mod path_extension;
pub use path_extension::*;

mod result;
pub use result::*;


pub mod prelude
{
    pub use super::path_extension::*;
    pub use super::fs::prelude::*;
    pub use super::result::*;
    pub use super::markup::*;
    pub use super::asset::prelude::*;
}