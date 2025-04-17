#![allow(unused_imports)]

pub(crate) use std::{fs::{self, File}, io::{Read, Write}};
pub(crate) use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};
pub(crate) use hexga_base::*;

pub type Reason = String;

pub type Path = String;
#[allow(non_camel_case_types)]
pub type path = str;

pub type Extension = String;
#[allow(non_camel_case_types)]
pub type extension = str;

mod mediator;
pub use mediator::*;

mod io;
pub use io::*;

mod path_extension;
pub use path_extension::*;

mod markup_extension;
pub use markup_extension::*;
