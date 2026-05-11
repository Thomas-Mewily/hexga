use super::*;
use hexga_event_loop::window::experimental::{WinitWindow, WinitWindowShared};
pub use hexga_graphics::*;

mod pen;
pub use pen::*;

pub mod prelude
{
    //pub(crate) use super::{Graphics, GpuEvent, wgpu};
    pub use hexga_graphics::prelude::*;
}
