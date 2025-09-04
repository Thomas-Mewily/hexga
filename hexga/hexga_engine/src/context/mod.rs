use super::*;

mod singleton;
pub use singleton::*;

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