use crate::*;

pub use modules::*;

pub mod prelude
{
    use crate::*;
    pub use super::{ContextMultiMedia,MainLoopWithContextMultiMedia};
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    use crate::*;
    pub trait ContextMultiMedia : ContextWindow + ContextRender {}
    impl<T> ContextMultiMedia for T where T : ContextWindow + ContextRender {}

    pub trait MainLoopWithContextMultiMedia
    {
        fn handle_event(&mut self, event : Event, ctx : &mut dyn ContextMultiMedia) -> bool;
        fn update(&mut self, ctx : &mut dyn ContextMultiMedia);
        fn draw(&mut self, ctx : &mut dyn ContextMultiMedia);
    }
}
