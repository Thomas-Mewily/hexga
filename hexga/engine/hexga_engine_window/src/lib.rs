//! Window / Events interface for the Hexga Engine based on [MiniQuad](https://github.com/not-fl3/miniquad)

mod window;
pub use window::*;

pub mod prelude
{
    pub use crate::window::{LoopWindow,WindowConfig};
}