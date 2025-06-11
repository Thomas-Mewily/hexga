use crate::*;

pub trait LoopEvent
{
    fn handle_event(&mut self, event : &Event) -> bool;
}
impl LoopEvent for () { fn handle_event(&mut self, _ : &Event) -> bool { true } }


mod event;
pub use event::*;

mod keyboard;
pub use keyboard::*;

mod mouse;
pub use mouse::*;

mod touch;
pub use touch::*;

mod window;
pub use window::*;

mod device;
pub use device::*;