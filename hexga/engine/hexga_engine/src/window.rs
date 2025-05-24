use crate::*;

pub use modules::*;

pub mod prelude
{
    use crate::*;
    pub use engine_core::window::prelude::*;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules 
{
    pub use super::engine_core::window::modules::*;
}

