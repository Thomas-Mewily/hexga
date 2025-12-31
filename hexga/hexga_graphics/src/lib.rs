#![allow(dead_code)]
#![allow(unused)]
use hexga::prelude::*;

#[cfg(feature = "wgpu")]
pub use hexga_wgpu as gpu;

#[cfg(feature = "wgpu")]
pub use hexga_wgpu::wgpu;
#[cfg(feature = "wgpu")]
pub(crate) use hexga_wgpu::{prelude::*,GpuVec};
#[cfg(feature = "wgpu")]
pub(crate) type GpuVertexBufferLayout<'a> = wgpu::VertexBufferLayout<'a>;




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

#[cfg(feature = "wgpu")]
pub mod format;
#[cfg(feature = "wgpu")]
pub(crate) use format::*;

#[cfg(feature = "wgpu")]
pub mod gfx;
#[cfg(feature = "wgpu")]
pub(crate) use gfx::*;

pub mod prelude
{
    pub use super::typedef::*;
    #[cfg(feature = "wgpu")]
    pub use hexga_wgpu::prelude::*;
}