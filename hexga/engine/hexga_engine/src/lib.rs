//! implementation of the Hexga Engine using MiniQuad

use hexga_engine_core as engine_core;

pub use modules::*;
use prelude::*;

pub mod prelude
{
    //pub use engine_core::prelude::*;
    pub use crate::engine_core::hexga::prelude::*;

    pub use crate::config::prelude::*;
    pub use crate::window::prelude::*;
    pub use crate::events::prelude::*;

    pub use crate::multi_media::prelude::*;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules;