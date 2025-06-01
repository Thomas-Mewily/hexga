#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::{fmt::Debug, ops::*};
use hexga::prelude::*;
use hexga::ansi_color::AnsiColor;
use std::marker::PhantomData;

pub use hexga_graphics::prelude::*;

pub use hexga;
pub mod log;

pub mod window;
pub mod render;
pub mod events;

pub mod multi_media;

use prelude::*;

pub mod prelude
{
    pub use crate::log::*;
    pub use crate::hexga ::prelude::*;

    pub use crate::render::prelude::*;
    pub use crate::events::prelude::*;
    pub use crate::window::prelude::*;

    pub use crate::multi_media::prelude::*;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use super::hexga;
    pub use super::log;

    pub use super::window;
    pub use super::render;
    pub use super::events;

    pub use super::multi_media;
}