use crate::*;

pub use engine_core::render::{buffer,render_pass,vertex,shader,pipeline,texture,bindings};
pub use engine_core::render::{LoopDraw, Render, UntypedSlice};

pub mod prelude
{
    use crate::*;
    pub use engine_core::render::prelude::*;
}