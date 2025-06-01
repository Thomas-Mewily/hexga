use crate::*;

pub use modules::*;

pub mod prelude
{
    use crate::*;
    pub use hexga_engine_render::prelude::*;
    pub use RenderBackend;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use hexga_engine_render::modules::*;
    pub use RenderBackend;
}