#![allow(unused_imports)]
use std::{fmt::Debug, ops::*};
use hexga::prelude::*;
use hexga::ansi_color::AnsiColor;


pub use hexga_engine_render as render;
pub use hexga_engine_events as events;
pub use hexga;

pub mod window;

use events::prelude::*;
use render::prelude::*;

pub mod prelude
{
    pub use crate::hexga ::prelude::*;
    pub use crate::render::prelude::*;
    pub use crate::events::prelude::*;
    pub use crate::window::prelude::*;
}
