#![allow(dead_code)]
#![allow(unused)]
use hexga::prelude::*;

#[cfg(feature = "gpu")]
pub use hexga_wgpu as gpu;

//#[cfg(feature = "gpu")]
//pub use hexga_wgpu::wgpu;
//#[cfg(feature = "gpu")]
//pub(crate) use hexga_wgpu::{GpuVec, prelude::*};
//#[cfg(feature = "gpu")]
//pub(crate) type GpuVertexBufferLayout<'a> = wgpu::VertexBufferLayout<'a>;

pub mod render;
pub(crate) use render::*;

pub mod camera;
pub(crate) use camera::*;

pub mod mesh;
pub(crate) use mesh::*;

pub mod vertex;
pub(crate) use vertex::*;

pub mod typedef;
pub(crate) use typedef::*;


pub mod prelude
{
    pub use super::typedef::*;
    #[cfg(feature = "gpu")]
    pub use hexga_wgpu::prelude::*;

    pub use super::traits::*;
    pub use super::
    {
        camera::prelude::*,
        mesh::prelude::*,
        render::prelude::*,
        vertex::prelude::*,
    };
}

pub mod traits
{
    pub use super::
    {
        camera::traits::*,
        mesh::traits::*,
        render::traits::*,
        vertex::traits::*,
    };
}