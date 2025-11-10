#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use std::ops::*;
use std::marker::PhantomData;


pub mod app;
pub mod input;
pub mod log;
pub mod window;
pub mod perf;
pub mod utils;
pub mod clipboard;
pub mod graphics;
pub use hexga::*;

use prelude::*;
pub mod prelude
{
    pub use super::
    {
        app::prelude::*,
        input::prelude::*,
        log::prelude::*,
        perf::*,
        window::*,
        clipboard::*,
        utils::*,
        graphics::prelude::*,
    };
    pub use hexga::prelude::*;
}