use super::*;

mod input;
pub use input::*;
mod keyboard;
pub use keyboard::*;
mod keycode;
pub use keycode::*;
mod button_state;
pub use button_state::*;

pub mod prelude
{
    pub use super::input::prelude::*;
    pub use super::keyboard::prelude::*;
    pub use super::keycode::prelude::*;
    pub use super::button_state::prelude::*;
}
