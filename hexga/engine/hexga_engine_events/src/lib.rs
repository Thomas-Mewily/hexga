//! Window / Events interface for the Hexga Engine based on [MiniQuad](https://github.com/not-fl3/miniquad)

pub use modules::*;

pub mod prelude
{
    pub use crate::modules::*;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules;