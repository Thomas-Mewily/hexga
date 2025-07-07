#![allow(unused_imports, dead_code)]
use hexga_engine_graphics::{prelude::*, WindowGraphicsData};
use hexga_engine_window::{event::IDeviceMessage, prelude::*, window::{EventLoopProxy, WindowContext, WindowRunParam}};
use hexga_core::prelude::*;//
use std::fmt::Debug;

/*
mod asset;
use asset::*;
*/

mod context;
use context::*;

pub mod prelude
{
    pub use hexga_engine_window::window::{
        IWindowRunParam, // `game()` `software()` shortcut
        WindowParam,
    };

    pub use super::window::prelude::*;
    pub use super::context::prelude::*;
}

pub mod window
{
    pub mod prelude
    {
        pub use hexga_engine_window::window::prelude::*;
    }

    pub use modules::*;
    /// Modules/Items without the prelude
    #[doc(hidden)]
    pub mod modules
    {
        pub use hexga_engine_window::window::*;
    }
}

pub mod event
{
    pub mod prelude
    {
        pub use hexga_engine_window::event::prelude::*;
    }

    pub use modules::*;
    /// Modules/Items without the prelude
    #[doc(hidden)]
    pub mod modules
    {
        pub use hexga_engine_window::event::*;
    }
}

pub use modules::*;
/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use super::context::*;
    pub use super::window;
}