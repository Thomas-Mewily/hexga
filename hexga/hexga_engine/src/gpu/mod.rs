use super::*;

// Todo: move this module his in own crate

use wgpu::util::DeviceExt;
pub type GpuDevice = wgpu::Device;
pub type GpuVecUsages = wgpu::BufferUsages;

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

mod pen;
pub use pen::*;

mod drawer;
pub use drawer::*;

mod vertex;
pub use vertex::*;

mod format;
pub use format::*;

pub mod prelude
{
    pub use super::typedef::prelude::*;
    pub use super::format::prelude::*;
    pub use super::context_gpu::prelude::*;

    pub use super::mesh::prelude::*;
    pub use super::vertex::prelude::*;

    pub use super::vec::prelude::*;
    
    pub use super::Gpu;
    pub use super::Pen;
}