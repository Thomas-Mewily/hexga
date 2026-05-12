#![allow(unused)]
#![allow(dead_code)]

use hexga::prelude::*;
use hexga_event_loop::{
    event_loop::prelude::*,
    input::*,
    window::{WindowParam, traits::*},
};
use hexga_graphics::gpu::{Gpu, GpuConfiguredSurface, GpuParam, GpuSurface, wgpu};
pub(crate) type Singleton<T> = hexga::singleton::SingletonOptionMutex<T>;
use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::*;
use std::rc::Rc;
use std::sync::{Arc, LazyLock};

pub use hexga::*;
pub use hexga_event_loop as event_loop;
pub mod app;
pub mod experimental;
pub mod graphics;
pub mod window;

use prelude::*;
pub mod prelude
{
    pub(crate) use super::experimental::*;
    pub use super::traits::*;
    pub use super::{app::prelude::*, graphics::prelude::*, window::prelude::*};
    pub use hexga::prelude::*;
}

pub mod traits
{
    pub use super::app::traits::*;
    pub use hexga::traits::*;
}
