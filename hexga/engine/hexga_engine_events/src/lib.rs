//! Window / Events interface for the Hexga Engine based on [MiniQuad](https://github.com/not-fl3/miniquad)
#![allow(unused_imports)]

use std::fmt::Debug;
use hexga_core::prelude::*;
use hexga_generational::prelude::GenVecID;
use hexga_math::prelude::*;
use hexga_bitflags::bitindex;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, Save, Load};

pub use modules::*;

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules;


pub mod prelude
{
    pub use crate::modules::*;
}