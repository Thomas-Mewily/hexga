#![allow(unused_imports)]

pub(crate) use std::io::{Read, BufReader, Write, BufWriter};

pub(crate) use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

pub mod fs;
use fs::*;


// Todo: add the ostcard format: https://docs.rs/postcard/latest/postcard/

#[cfg(feature = "derive")]
pub use hexga_io_derive::*;

pub mod prelude;

mod core;
pub use core::*;

mod default_impl;
pub use default_impl::*;



mod error;
pub use error::*;

mod io;
pub use io::*;

mod load;
pub use load::*;

mod save;
pub use save::*;

mod path_extension;
pub use path_extension::*;

mod markup_extension;
pub use markup_extension::*;
