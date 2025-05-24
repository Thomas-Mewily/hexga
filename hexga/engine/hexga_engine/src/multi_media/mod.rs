use crate::*;

mod context;
pub use context::*;

mod param;
pub use param::*;

pub mod prelude
{
    use crate::*;
    pub use engine_core::multi_media::prelude::*;
    pub use super::context::MainLoop;
    pub use super::param::MultiMediaParam;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules 
{
    pub use super::engine_core::multi_media::modules::*;
    pub use super::context::*;
    pub use super::param::*;
}