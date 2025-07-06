//! Window / Events interface for the Hexga Engine based on [Winit](https://crates.io/crates/winit)
#![allow(unused_imports)]

pub mod window;
pub mod event;

use window::*;
use event::*;

use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

#[cfg(feature = "hexga_io")]
use hexga_io::{IoSave, IoLoad, Save, Load};

pub(crate) type EventLoop<T> = winit::event_loop::EventLoop<T>;
pub(crate) type ActiveEventLoop = winit::event_loop::ActiveEventLoop;
pub(crate) use hexga_core::prelude::*;
pub(crate) use hexga_generational::prelude::*;
pub(crate) use hexga_math::prelude::*;
pub(crate) use std::ops::{Deref,DerefMut};
pub(crate) use std::sync::Arc;


use prelude::*;

pub mod prelude
{
    pub use crate::window::prelude::*;
    pub use crate::event::prelude::*;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use super::window;
    pub use super::event;
}