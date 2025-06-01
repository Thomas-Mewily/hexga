//! Window / Events interface for the Hexga Engine based on [MiniQuad](https://github.com/not-fl3/miniquad)
#![allow(unused_imports)]
pub use modules::*;


#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, Save, Load};

pub mod prelude
{
    pub use crate::modules::*;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules;