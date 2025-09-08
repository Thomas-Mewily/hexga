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