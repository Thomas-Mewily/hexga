//! Rendering interface for the Hexga Engine based on [miniquad](https://github.com/not-fl3/miniquad)
#![allow(unused_imports)]
use std::{fmt::Debug, marker::PhantomData, ops::Deref};

use hexga_math::prelude::*;

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

pub mod gpu;
use gpu::*;

pub mod prelude
{
    //pub use super::buffer::Buffer;
    pub use super::texture::RawTextureID;
    pub use super::render::RenderBackend;
    pub use super::gpu::*;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules 
{
    pub use super::{buffer,render_pass,vertex,shader,pipeline,texture,bindings,blend,stencil,gpu};
    pub use super::{RenderBackend, UntypedSlice};
}