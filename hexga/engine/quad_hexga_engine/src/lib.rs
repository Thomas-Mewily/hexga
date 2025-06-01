//! implementation of the Hexga Engine using MiniQuad
#![allow(dead_code)]

use hexga_engine_core::*;
use prelude::*;

pub mod quad;


pub mod prelude
{
    pub use hexga_engine_core::prelude::*;
    //pub use crate::quad::QuadRunner;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use super::quad;
    pub use hexga_engine_core::modules::*;
}