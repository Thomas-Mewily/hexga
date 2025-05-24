#![allow(unused_imports)]
use std::{fmt::Debug, ops::*};
use hexga::prelude::*;
use hexga::ansi_color::AnsiColor;

pub use hexga;
pub mod log;

pub mod window;
pub mod render;
pub mod events;

pub mod pen;
pub mod multi_media;

mod context;
pub use context::*;

use prelude::*;

pub mod prelude
{
    pub use crate::log::*;
    pub use crate::hexga ::prelude::*;

    pub use crate::render::prelude::*;
    pub use crate::events::prelude::*;
    pub use crate::window::prelude::*;

    pub use crate::multi_media::prelude::*;

    pub use crate::pen::prelude::*;
    pub use crate::context::{MainLoop,set_context};
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

    pub use super::pen;

    pub use super::multi_media;
    
    pub use super::{MainLoop,set_context};
}