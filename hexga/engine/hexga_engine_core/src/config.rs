//! mainly based on [MiniQuad](https://github.com/not-fl3/miniquad)
use crate::*;

pub mod prelude
{
    pub use super::MultiMediaConfig;
}

#[derive(Default, PartialEq, Eq, Clone, Hash)]
pub struct MultiMediaConfig 
{
    pub window_config : WindowConfig,
}

impl MultiMediaConfig
{
    pub fn new() -> Self { ___() }
    pub fn with_window_config(mut self, window : WindowConfig) -> Self { self.window_config = window; self }
    //pub fn with_pen_config(mut self, pen : PenConfig) -> Self { self.pen_config = pen; self }
}

impl MultiMediaConfig
{
    // Impl this function from a trait in your engine implementation
    /*
    pub fn run_with_context<T>(self, state : impl 'static + FnOnce() -> T) where T : MainLoopWithContext + 'static
    */
}