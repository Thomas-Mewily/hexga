//! Stuff inside this crate need to be moved somewhere else/other subcrate...
#![allow(unused_imports)]
use std::ops::*;
use hexga_core::prelude::*;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, ser::SerializeSeq};


pub mod non_empty_stack;
// pub mod asset;
pub mod scope;
pub mod dirty_flag;
pub mod button;
pub mod evolution;
pub mod lazy;

use prelude::*;
pub mod prelude
{
    pub use super::non_empty_stack::prelude::*;
    // pub use super::asset::*;
    pub use super::scope::prelude;
    pub use super::dirty_flag::prelude::*;
    pub use super::button::prelude::*;
    pub use super::evolution::prelude::*;
    pub use super::lazy::prelude::*;
}


