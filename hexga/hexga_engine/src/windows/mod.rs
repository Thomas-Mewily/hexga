use crate::*;

mod manager;
pub use manager::*;

mod window;
pub use window::*;

pub(crate) type WindowLookupID = HashMap<WinitWindowID, WindowID>;
pub type WindowID = GenVecID<WindowData>;

declare_context_singleton!(Windows, ContextWindows, windows);

pub mod prelude
{

}