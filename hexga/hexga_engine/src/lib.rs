#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{iter, sync::Arc};
use std::marker::PhantomData;
use hexga::prelude::*;
use std::ops::*;
use std::collections::HashMap;

#[allow(unused_imports)]
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[allow(unused_imports)]
#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, Save, Load};

pub mod gpu;
pub mod app;
pub mod context;
pub mod performance;
pub mod input;
pub mod utils;

use prelude::*;

pub mod prelude
{
    pub use super::app::prelude::*;
    pub use super::context::prelude::*;
    pub use super::gpu::prelude::*;
    pub use super::performance::prelude::*;
    pub use super::input::prelude::*;
    pub use super::utils::prelude::*;
}

/*
Todo: put in the run config
Is the camera 2D or 3D by default ?
*/