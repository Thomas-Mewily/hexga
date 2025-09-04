#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{iter, sync::Arc};
use std::marker::PhantomData;
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};
use hexga::prelude::*;
use std::ops::*;



pub mod gpu;
use gpu::*;

pub mod app;
use app::*;

pub mod context;
use context::*;



pub mod log;
pub use log::*;

mod debug;
pub use debug::*;

pub mod prelude
{
    pub use super::app::prelude::*;
    pub use super::context::prelude::*;
    pub use super::log::prelude::*;
    pub use super::gpu::prelude::*;
}
use prelude::*;