//! implementation of the Hexga Engine Base using [MiniQuad](https://github.com/not-fl3/miniquad)
#![allow(dead_code)]

use hexga_engine_base::*;
use prelude::*;

pub mod quad;


pub mod prelude
{
    pub use hexga_engine_base::prelude::*;
    //pub use crate::quad::QuadRunner;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use super::quad;
    pub use hexga_engine_base::modules::*;
}