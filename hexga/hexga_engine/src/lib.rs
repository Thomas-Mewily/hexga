#![allow(unused_imports)]
#![allow(dead_code)]
use hexga::prelude::*;
use std::sync::Arc;

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[allow(unused_imports)]
#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, Save, Load};

pub use hexga::*;
pub mod app;
pub mod input;
pub mod log;
pub mod gpu;
pub mod model;
pub mod window;

use prelude::*;
pub mod prelude
{
    pub use ::hexga::prelude::*;
    pub use super::app::*;
    pub use super::input::*;
    pub use super::log::*;
    pub use super::gpu::*;
    pub use super::model::*;
    pub use super::window::*;
}