//! Stuff inside this crate need to be moved somewhere else/other subcrate...
#![allow(unused_imports)]
use hexga_core::prelude::*;
use std::ops::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeSeq};

pub mod non_empty_stack;
// pub mod asset;
pub mod button;
pub mod dirty;
pub mod evolution;
pub mod lazy;
pub mod message;
pub mod scope;
//pub mod factorized;

use prelude::*;
pub mod prelude
{
    pub use super::non_empty_stack::prelude::*;
    // pub use super::asset::*;
    pub use super::button::prelude::*;
    pub use super::dirty::prelude::*;
    pub use super::evolution::prelude::*;
    pub use super::lazy::prelude::*;
    pub use super::message::prelude::*;
    pub use super::scope::prelude;
}
