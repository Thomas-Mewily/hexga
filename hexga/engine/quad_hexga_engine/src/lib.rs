//! implementation of the Hexga Engine using MiniQuad
#![allow(dead_code)]

pub use hexga_engine::*;
use prelude::*;


pub mod quad;


pub mod prelude
{
    pub use hexga_engine::prelude::*;
    pub use crate::quad::QuadRunner;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use super::quad;
    pub use hexga_engine::modules::*;
}