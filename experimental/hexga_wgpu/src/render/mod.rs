use super::*;

mod surface;
pub use surface::*;

mod sampler;
pub use sampler::*;

// Todo change it / Make a proper type and auto pack / atlas texture
pub type Texture = experimental::GpuTexture;

pub mod prelude
{
    pub use super::traits::*;
    pub use super::{GpuSurface, GpuSurfaceConfigured, Texture};
}

pub mod traits
{}
