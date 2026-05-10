#![allow(unused)]
#![allow(dead_code)]

use hexga::prelude::*;
use hexga_graphics::gpu::{Gpu, GpuConfiguredSurface, GpuInit, GpuParam, GpuSurface};
use hexga_event_loop::prelude::*;
use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::*;
use std::rc::Rc;
use std::sync::{Arc, LazyLock};

pub use hexga::*;
pub use hexga_event_loop as event_loop;
pub mod graphics;
pub mod app;

use prelude::*;
pub mod prelude
{
    pub use super::{
        graphics::prelude::*,
        app::prelude::*,
    };
    pub use hexga::prelude::*;
    pub use super::traits::*;
}

pub mod traits
{
    pub use hexga::traits::*;
    pub use super::
    {
        app::traits::*,
    };
}
