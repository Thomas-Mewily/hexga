#![allow(unused_imports)]
#![allow(dead_code)]
use hexga_core::prelude::*;
use hexga_math::prelude::*;
use std::ops::*;

mod context;
use context::*;

pub mod window;
use window::*;

pub mod app;
use app::*;

pub mod prelude
{
    pub use super::app::prelude::*;
    pub use super::context::prelude::*;
}