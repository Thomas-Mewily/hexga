use super::*;
use hexga_event_loop::window::experimental::{WinitWindow, WinitWindowShared};
pub use hexga_graphics::*;

pub mod prelude
{
    //pub(crate) use super::{Graphics, GpuEvent, wgpu};
    pub use hexga_graphics::prelude::*;
}

#[derive(Clone, Copy)]
pub struct Pen;

