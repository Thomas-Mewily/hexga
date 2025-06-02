
#[allow(unused_imports)]
use prelude::*;

pub mod prelude
{
    pub use hexga_engine_base::prelude::*;
}

pub use modules::*;

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    pub use hexga_engine_base::modules::*;
}

pub trait ContextRun
{
    fn run<T>(self, state : impl 'static + FnOnce() -> T) where T: MainLoop + 'static;
}
impl ContextRun for MultiMediaParam
{
    fn run<T>(self, state : impl 'static + FnOnce() -> T) where T: MainLoop + 'static {
        todo!()
    }
}