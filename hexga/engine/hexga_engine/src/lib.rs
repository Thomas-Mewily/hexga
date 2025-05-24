//! implementation of the Hexga Engine using MiniQuad
#![allow(unused_imports)]
use hexga_engine_core as engine_core;

pub mod events;
pub mod window;
pub mod multi_media;
pub mod render;
pub mod pen;

pub use modules::*;
use prelude::*;

pub mod prelude
{
    //pub use engine_core::prelude::*;
    pub use crate::engine_core::hexga::prelude::*;

    pub use crate::multi_media::prelude::*;
    pub use crate::window::prelude::*;
    pub use crate::events::prelude::*;
    //pub use crate::render::prelude::*;
    pub use crate::pen::prelude::*;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use super::{
        events,
        window,
        multi_media,
        //render,
        pen,
    };
}