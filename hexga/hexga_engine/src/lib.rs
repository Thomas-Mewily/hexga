#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{iter, sync::Arc};
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};
use wgpu::
{
    Surface, Instance,
    util::{BufferInitDescriptor, DeviceExt}
};


use hexga::prelude::*;
use std::ops::*;

pub mod context;
use context::*;

mod wgpu_context;
use wgpu_context::*;

mod vertex;
use vertex::*;

pub mod texture;
use texture::*;

pub mod app;
use app::*;

mod debug;
pub use debug::*;

pub mod prelude
{
    pub use super::app::prelude::*;
    pub use super::texture::prelude::*;
    pub use super::debug::prelude::*;
}
