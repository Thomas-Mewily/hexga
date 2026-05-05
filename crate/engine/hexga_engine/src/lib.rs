#![allow(unused)]
#![allow(dead_code)]

use hexga::prelude::*;
use hexga_graphics::gpu::{Gpu, GpuConfiguredSurface, GpuInit, GpuParam, GpuSurface};
use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::*;
use std::rc::Rc;
use std::sync::{Arc, LazyLock};

pub use hexga::*;
pub mod app;
pub mod graphics;
pub mod log;
pub mod window;
pub mod input;
mod free_fn;

use prelude::*;
pub mod prelude
{
    pub use super::{
        app::prelude::*,
        graphics::prelude::*,
        log::prelude::*,
        window::prelude::*,
        input::prelude::*,
    };
    pub use hexga::prelude::*;
}
