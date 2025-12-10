#![allow(unused)]
#![allow(dead_code)]

use std::any::Any;
use std::fmt::Debug;
use std::sync::{Arc,LazyLock};
use std::ops::*;
use std::marker::PhantomData;
use hexga::prelude::*;
use hexga_graphics::GpuParam;


pub use hexga::*;
pub mod app;
pub mod log;
pub mod input;
pub mod perf;
pub mod window;
pub mod clipboard;
pub mod graphics;

use prelude::*;
pub mod prelude
{
    pub use hexga::prelude::*;
    pub use super::
    {
        app::prelude::*,
        log::prelude::*,
        input::prelude::*,
        perf::prelude::*,
        window::prelude::*,
        clipboard::prelude::*,
        graphics::prelude::*,
        //clipboard::*,
    };
}