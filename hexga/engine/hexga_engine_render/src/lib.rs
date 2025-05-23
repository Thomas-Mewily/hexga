//! Rendering interface for the Hexga Engine based on [miniquad](https://github.com/not-fl3/miniquad)
use std::{fmt::Debug, marker::PhantomData, ops::{Deref, DerefMut}};

pub type RenderColorRGBA = [f32;4];
pub type RenderColorRGBAMask = [bool;4];

pub mod buffer;
use buffer::*;

pub mod render_pass;
use render_pass::*;

pub mod vertex;
use vertex::*;

pub mod blend;
use blend::*;

pub mod stencil;
use stencil::*;

pub mod shader;
use shader::*;

pub mod pipeline;
use pipeline::*;

pub mod texture;
use texture::*;

pub mod bindings;
use bindings::*;

mod render;
pub use render::*;

mod untyped_slice;
pub use untyped_slice::*;

pub mod prelude
{
    //pub use super::buffer::Buffer;
    pub use super::texture::RawTextureID;
    pub use super::render::ContextRender;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules 
{
    pub use super::{buffer,render_pass,vertex,shader,pipeline,texture,bindings,blend,stencil};
    pub use super::{ContextRender, UntypedSlice};
}