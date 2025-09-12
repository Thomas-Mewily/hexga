#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{iter, sync::Arc};
use std::marker::PhantomData;
use std::ops::*;
use std::collections::HashMap;

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[allow(unused_imports)]
#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, Save, Load};

pub use hexga::*;
pub mod app;
pub mod gpu;
pub mod context;
pub mod performance;
pub mod input;
pub mod utils;
pub mod log;

use prelude::*;
pub mod prelude
{
    pub use super::app::*;
    pub use super::gpu::*;
    pub use super::context::*;
    pub use super::performance::*;
    pub use super::input::*;
    pub use super::utils::*;
    pub use super::log::*;
    pub use ::hexga::prelude::*;
}

/*
Todo: put in the run config
Is the camera 2D or 3D by default ?
*/