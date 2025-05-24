use crate::*;

pub trait ContextMultiMedia : ContextWindow + ContextRender {}
impl<T> ContextMultiMedia for T where T : ContextWindow + ContextRender {}

pub mod prelude
{
    use crate::*;
    pub use super::ContextMultiMedia;
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use super::ContextMultiMedia;
}
