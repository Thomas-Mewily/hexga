use super::*;

mod ctx;
pub use ctx::*;

pub type ContextWinit = Arc<Window>;

#[derive(Default)]
pub struct Context
{
    pub(crate) winit : Option<ContextWinit>,
}

pub mod prelude
{
    pub use super::Ctx;
}