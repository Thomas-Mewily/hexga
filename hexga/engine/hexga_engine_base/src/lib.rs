#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::{fmt::Debug, ops::*};
use hexga::prelude::*;
use hexga::ansi_color::AnsiColor;
use std::marker::PhantomData;
use hexga_graphics::prelude::*;

pub use hexga_engine_core as engine_core;
use hexga;

pub use hexga::graphics;

pub mod window;
pub mod events;

//pub mod pen;
pub mod multi_media;

pub mod context;

use prelude::*;

pub mod prelude
{
    pub use crate::hexga ::prelude::*;

    pub use crate::events::prelude::*;
    pub use crate::window::prelude::*;

    pub use crate::multi_media::prelude::*;

    pub use crate::context::prelude::*;
    //pub use crate::pen::prelude::*;

    pub use crate::graphics::prelude::*;

    pub(crate) use crate::engine_core::prelude::*;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use super::window;
    //pub use super::render;
    pub use super::events;


    pub use super::multi_media;

    pub use super::context;
    //pub use super::pen;

    pub use super::graphics;
}