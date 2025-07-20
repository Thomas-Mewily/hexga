#![allow(unused_imports, dead_code)]
use std::ops::*;
use hexga::prelude::*;
use std::fmt::{Debug,Formatter,Result as DResult};

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, Save, Load};


pub mod input;
use input::*;

pub mod context;
use context::*;

pub mod window;
use context::*;

pub mod clipboard;
use clipboard::*;


mod prelude
{
    pub use super::input::prelude::*;
    pub use super::context::prelude::*;
    pub use super::window::prelude::*;
}