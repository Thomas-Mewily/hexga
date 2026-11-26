#![allow(dead_code)]
#![allow(unused)]
use hexga::prelude::*;

use gpu::experimental::prelude::*;
use gpu::prelude::*;
use hexga::range::Range;

pub use hexga_wgpu as gpu;
pub use hexga_wgpu::wgpu;

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

pub mod scissor;
pub(crate) use scissor::*;

pub mod viewport;
pub(crate) use viewport::*;

pub mod prelude {
    pub use super::typedef::*;
    pub use hexga_wgpu::prelude::*;

    pub use super::traits::*;
    pub use super::{camera::prelude::*, mesh::prelude::*, render::prelude::*, scissor::prelude::*, vertex::prelude::*, viewport::prelude::*};
}

pub mod traits {
    pub use super::{camera::traits::*, mesh::traits::*, render::traits::*, scissor::traits::*, vertex::traits::*, viewport::traits::*};
}
