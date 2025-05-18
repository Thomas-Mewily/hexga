//! Rendering interface for the Hexga Engine based on [miniquad](https://github.com/not-fl3/miniquad)
use std::{fmt::Debug, marker::PhantomData, ops::{Deref, DerefMut}};

/* 
mod gpu;
use gpu::*;
*/

use hexga::prelude::*;
use hexga_graphics::*;

pub mod buffer;
use buffer::*;

pub mod render_pass;
use render_pass::*;

pub mod vertex;
use vertex::*;

pub mod shader;
use shader::*;

pub mod pipeline;
use pipeline::*;

pub mod texture;
use texture::*;

pub mod bindings;
use bindings::*;

mod render;
use render::*;

mod untyped_slice;
use untyped_slice::*;

pub mod prelude
{
    //pub use super::buffer::Buffer;
    pub use super::texture::Texture;
    pub use super::render::Render;
}
