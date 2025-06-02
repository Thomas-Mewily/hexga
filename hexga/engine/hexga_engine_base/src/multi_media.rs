use crate::*;

#[derive(Default, PartialEq, Eq, Clone, Hash)]
pub struct MultiMediaParam
{
    pub window_param : WindowParam,
}

impl MultiMediaParam
{
    pub fn new() -> Self { ___() }
    pub fn window(mut self, window : WindowParam) -> Self { self.window_param = window; self }
    //pub fn pen(mut self, pen_param : PenParam) -> Self { self.pen_param = pen_param; self }
}

impl MultiMediaParam
{
    // Impl this function from a trait in your engine implementation
    /*
    pub fn run_with_context<T>(self, state : impl 'static + FnOnce() -> T) where T: MainLoopWithContext + 'static
    */
}


pub mod prelude
{
    pub use super::MultiMediaParam;
    pub use super::{MainLoopWithContext};
    pub use hexga_engine_core::multi_media::MainLoop;
}

pub trait MainLoopWithContext
{
    fn handle_event(&mut self, event : Event, ctx : &mut Ctx) -> bool;
    fn update(&mut self, ctx : &mut Ctx);
    fn draw(&mut self, ctx : &mut Ctx);
}

impl<T> MainLoopWithContext for T where T : MainLoop
{
    fn handle_event(&mut self, event : Event, _ : &mut Ctx) -> bool { self.handle_event(event) }
    fn update(&mut self, ctx : &mut Ctx) { self.update() }
    fn draw(&mut self, ctx : &mut Ctx) { self.draw() }
}




/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use super::MultiMediaParam;
}
