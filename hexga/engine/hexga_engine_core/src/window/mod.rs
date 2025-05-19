use crate::*;

pub mod prelude;

pub mod log;

pub trait LoopWindow
{
    fn window_draw  (&mut self);
    fn window_update(&mut self);
    fn window_handle_event(&mut self, event : Event) -> bool;
}
impl LoopWindow for ()
{
    fn window_draw  (&mut self) {}
    fn window_update(&mut self) {}
    fn window_handle_event(&mut self, _ : Event) -> bool { true }
}