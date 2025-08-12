//! Stuff inside this crate need to move somewhere else...

use std::ops::*;
use hexga_core::prelude::*;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, ser::SerializeSeq};

#[cfg(feature = "hexga_io")]
use hexga_io::{Save, Load};

pub mod non_empty_stack;
pub mod asset;

pub mod prelude
{
    pub use super::non_empty_stack::prelude::*;
    pub use super::asset::prelude::*;
}