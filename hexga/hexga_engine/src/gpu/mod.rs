use super::*;

// Todo: move this module his in own crate

use wgpu::util::DeviceExt;
pub type GpuDevice = wgpu::Device;
pub type GpuVecUsages = wgpu::BufferUsages;
pub(crate) type GpuVertexBufferLayout<'a> = wgpu::VertexBufferLayout<'a>;

pub mod typedef;
use typedef::*;

mod context_gpu;
pub use context_gpu::*;

mod gpu;
pub use gpu::*;

mod mesh;
pub use mesh::*;

mod vec;
pub use vec::*;



mod vertex;
pub use vertex::*;

mod format;
pub use format::*;

mod camera;
pub use camera::*;

mod pen;
pub use pen::*;

pub mod prelude
{
    pub use super::typedef::prelude::*;
    pub use super::format::prelude::*;
    pub use super::context_gpu::prelude::*;

    pub use super::camera::prelude::*;

    pub use super::mesh::prelude::*;
    pub use super::vertex::prelude::*;

    pub use super::vec::prelude::*;
    pub use super::pen::prelude::*;
    
    pub use super::Gpu;
}