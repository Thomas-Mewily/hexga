#![allow(unused_imports)]
use std::{fmt::Debug, ops::*};
use hexga::prelude::*;
use hexga::ansi_color::AnsiColor;

pub use hexga;
pub mod log;

pub mod window;
pub mod render;
pub mod events;

pub mod config;
pub mod multi_media;


use prelude::*;

pub mod prelude
{
    pub use crate::log::*;
    pub use crate::hexga ::prelude::*;

    pub use crate::render::prelude::*;
    pub use crate::events::prelude::*;
    pub use crate::window::prelude::*;

    pub use crate::config::prelude::*;
    pub use crate::multi_media::prelude::*;
}
