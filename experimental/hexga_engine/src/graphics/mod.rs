use super::*;
use hexga_event_loop::window::experimental::{WinitWindow, WinitWindowShared};
pub use hexga_graphics::*;

use hexga_graphics::gpu::experimental::prelude::*;
use hexga_graphics::{vertex::WgpuVertexDesc, wgpu::RenderPipeline};

mod pen;
pub use pen::*;

//mod draw;
//pub use draw::*;

mod graphics_ctx;
pub use graphics_ctx::*;

pub mod prelude
{
    //pub(crate) use super::{Graphics, GpuEvent, wgpu};
    pub(crate) use super::graphics_ctx::Graphics;
    pub use super::pen::Pen;
    pub use super::traits::*;
    pub use hexga_graphics::prelude::*;
}

pub mod traits
{
    pub(crate) use super::graphics_ctx::ExternLibConvert;
    pub use hexga_graphics::traits::*;
    //pub use super::draw::{Drawer, Drawable};
}
pub(crate) use prelude::*;