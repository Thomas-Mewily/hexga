use super::*;
use hexga_event_loop::window::experimental::{WinitWindow, WinitWindowShared};
pub use hexga_graphics::*;

use hexga_graphics::gpu::experimental::prelude::*;
use hexga_graphics::render::Viewport;
use hexga_graphics::{vertex::WgpuVertexDesc, wgpu::RenderPipeline};

mod pen;
pub use pen::*;

mod graphics;
pub use graphics::*;

pub mod prelude
{
    //pub(crate) use super::{Graphics, GpuEvent, wgpu};
    pub(crate) use super::Graphics;
    pub use super::Pen;
    pub use super::traits::*;
    pub use hexga_graphics::prelude::*;
}

pub mod traits
{
    pub(crate) use super::ExternLibConvert;
    pub use hexga_graphics::traits::*;
}
