use super::*;

mod keycode;
pub use keycode::*;
mod event;
pub use event::*;
mod input;
pub use input::*;
mod keyboard;
pub use keyboard::*;
mod binding;
pub use binding::*;
mod shortcuts;
pub use shortcuts::*;

pub fn input() -> impl DerefMut<Target=AppInput> { APP.get_mut().guard_map_mut(|a| a.input()) }

pub mod prelude
{
    //pub use super::{KeyBinding, input};

    //pub(crate) use super::AppInput;
    pub(crate) use super::*;

    #[allow(hidden_glob_reexports)]
    pub(crate) mod prelude {}
}