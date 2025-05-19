//! implementation of the Hexga Engine using MiniQuad

use hexga_engine_core as engine_core;

pub mod render;
pub mod window;
pub mod events;

use prelude::*;

pub mod prelude
{
    use crate::*;
    pub use engine_core::prelude::*;
}