use crate::*;

pub use modules::*;

pub mod prelude
{
    use crate::*;
    pub use hexga_engine_render::prelude::*;
    pub use RenderBackend;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules 
{
    pub use hexga_engine_render::modules::*;
    pub use RenderBackend;
}

/* 
pub struct Render;
impl Deref for Render
{
    type Target=ContextRender;
    fn deref(&self) -> &Self::Target {
        ctx_ref().render
    }
}

impl DerefMut for Render
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        ctx().render
    }
}

pub struct ContextRender
{
    texture
}
    */