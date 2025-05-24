use crate::*;

/* 
pub trait MainLoopWithContextMultiMedia
{
    fn handle_event(&mut self, event : Event, ctx : &mut dyn ContextMultiMedia) -> bool;
    fn update(&mut self, ctx : &mut dyn ContextMultiMedia);
    fn draw(&mut self, ctx : &mut dyn ContextMultiMedia);
}
*/

#[derive(Default, PartialEq, Eq, Clone, Hash)]
pub struct MultiMediaParam 
{
    pub window_param : WindowParam,
    pub pen_param : PenParam,
}

impl MultiMediaParam
{
    pub fn new() -> Self { ___() }
    pub fn window(mut self, window : WindowParam) -> Self { self.window_param = window; self }
    pub fn pen(mut self, pen_param : PenParam) -> Self { self.pen_param = pen_param; self }
}

impl MultiMediaParam
{
    // Impl this function from a trait in your engine implementation
    /*
    pub fn run_with_context<T>(self, state : impl 'static + FnOnce() -> T) where T : MainLoopWithContext + 'static
    */
}