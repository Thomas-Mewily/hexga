use crate::*;

pub trait ContextMultiMedia : ContextWindow + RenderBackend {}
impl<T> ContextMultiMedia for T where T: ContextWindow + RenderBackend {}

use modules::*;

pub mod prelude
{
    use crate::*;
    pub use super::{ContextMultiMedia};
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use super::{ContextMultiMedia};
}
