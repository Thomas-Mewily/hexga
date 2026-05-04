#![allow(unused)]
#![allow(dead_code)]

use hexga::prelude::*;
use hexga_graphics::gpu::{GpuConfiguredSurface, GpuInit, GpuParam, GpuSurface};
use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::*;
use std::rc::Rc;
use std::sync::{Arc, LazyLock};

pub use hexga::*;
pub mod app;
pub mod clipboard;
mod free_function;
pub mod graphics;
pub mod input;
pub mod log;
pub mod perf;
pub mod window;
pub use free_function::*;

use prelude::*;
pub mod prelude
{
    pub use super::{
        app::prelude::*,
        clipboard::prelude::*,
        graphics::prelude::*,
        //clipboard::*,
        input::prelude::*,
        log::prelude::*,
        perf::prelude::*,
        window::prelude::*,
    };
    pub use hexga::prelude::*;
}
