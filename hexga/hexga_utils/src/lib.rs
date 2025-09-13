//! Stuff inside this crate need to move somewhere else...
#![allow(unused_imports)]
use std::ops::*;
use hexga_core::prelude::*;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, ser::SerializeSeq};

#[cfg(feature = "hexga_io")]
use hexga_io::{Save, Load};

pub mod non_empty_stack;
pub mod asset;
pub mod scope;

pub mod prelude
{
    pub use super::non_empty_stack::*;
    pub use super::asset::*;
    pub use super::scope::*;
}


