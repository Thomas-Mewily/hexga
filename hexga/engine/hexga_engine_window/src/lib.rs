//! Window / Events interface for the Hexga Engine based on [Winit](https://crates.io/crates/winit)
#![allow(unused_imports)]

pub mod window;
pub mod message;
pub mod event;
pub mod app;

use window::*;
use message::*;
use event::*;
use app::*;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, Save, Load};

pub(crate) type EventLoop = winit::event_loop::EventLoop<()>;
pub(crate) type ActiveEventLoop = winit::event_loop::ActiveEventLoop;
pub(crate) use hexga_core::prelude::*;
pub(crate) use hexga_generational::prelude::*;
pub(crate) use hexga_math::prelude::*;

use prelude::*;

pub mod prelude
{
    pub use crate::window::prelude::*;
    pub use crate::message::prelude::*;
    pub use crate::event::prelude::*;
    pub use crate::app::prelude::*;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use super::window;
    pub use super::message;
    pub use super::event;
    pub use super::app;
}