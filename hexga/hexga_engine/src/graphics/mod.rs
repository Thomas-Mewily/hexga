use super::*;

use wgpu::util::DeviceExt;
pub(crate) type GpuVertexBufferLayout<'a> = wgpu::VertexBufferLayout<'a>;


mod pen;
pub use pen::*;

mod as_u8_slice;
pub use as_u8_slice::*;

mod model;
pub use model::*;

mod typedef;
pub use typedef::*;

mod format;
pub use format::*;

mod render;
pub use render::*;

mod camera;
pub use camera::*;

mod mesh;
pub use mesh::*;

mod vec;
pub use vec::*;

mod texture;
pub use texture::*;


pub mod prelude
{
    pub use super::{
        texture::{Texture,TextureAsset},
        typedef::*,
        model::prelude::*,
        camera::*,
        Pen,
        mesh::BuilderMesh,
    };
    pub(crate) use super::*;
}