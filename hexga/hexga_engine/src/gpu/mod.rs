use super::*;

pub mod typedef;
use typedef::*;

mod context_gpu;
pub use context_gpu::*;

mod singleton;
pub use singleton::*;

mod vertex;
pub use vertex::*;

/* 
mod wgpu_context;
use wgpu_context::*;
*/

pub mod prelude
{
    pub use super::typedef::prelude::*;
    pub use super::context_gpu::prelude::*;
    pub use super::vertex::prelude::*;
    pub use super::Gpu;
}