use crate::*;

/*
pub trait ContextMultiMedia : ContextWindow + RenderBackend {}
impl<T> ContextMultiMedia for T where T: ContextWindow + RenderBackend {}
*/

pub trait MainLoop
{
    fn handle_event(&mut self, event : Event) -> bool;
    fn update(&mut self);
    fn draw(&mut self);
}
impl MainLoop for ()
{
    fn handle_event(&mut self, event : Event) -> bool { true }
    fn update(&mut self) {}
    fn draw(&mut self) { }
}

use modules::*;

pub mod prelude
{
    use crate::*;
    //pub use super::{ContextMultiMedia,MainLoop};
}

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    //pub use super::{ContextMultiMedia,MainLoop};
}
