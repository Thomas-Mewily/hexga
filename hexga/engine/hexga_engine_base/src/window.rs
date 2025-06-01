use crate::*;

pub use modules::*;

pub mod prelude
{
    use crate::*;
    pub use hexga_engine_window::prelude::*;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use hexga_engine_window::modules::*;
    pub use super::window::*;
}