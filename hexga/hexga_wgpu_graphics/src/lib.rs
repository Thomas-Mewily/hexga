#![allow(dead_code)]
#![allow(unused)]
use hexga::prelude::*;
pub(crate) type GpuVertexBufferLayout<'a> = wgpu::VertexBufferLayout<'a>;

pub use hexga_wgpu::*;

mod format;
pub use format::*;

pub mod prelude
{
    pub use hexga_wgpu::prelude::*;
    pub use hexga_graphics::prelude::*;
}