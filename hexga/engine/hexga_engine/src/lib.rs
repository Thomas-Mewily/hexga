
//#[allow(unused_imports)]
//use prelude::*;

/*
pub(crate) use hexga_engine_base as engine_base;

mod quad;
pub(crate) use quad::*;

pub mod prelude
{
    pub use hexga_engine_base::prelude::*;
    //pub use super::ContextRunnerExtension;
}

pub use modules::*;

/// Modules/Items without the prelude
#[doc(hidden)]
pub mod modules
{
    use crate::*;

    /*
    use hexga_engine_base::context::{ContextRunner, Ctx};
    pub use hexga_engine_base::modules::*;

    pub trait ContextRunnerExtension
    {
        fn run<T>(self, state : impl 'static + FnOnce() -> T) where T: MainLoop + 'static;
    }
    impl ContextRunnerExtension for MultiMediaParam
    {
        fn run<T>(self, state : impl 'static + FnOnce() -> T) where T: MainLoop + 'static {
            miniquad::start(self.window_param.clone().convert(),
                move ||
                {
                    let ctx = Box::new(QuadContext{ render: miniquad::window::new_rendering_backend(), textures: ___(), tmp_vertex: ___(), tmp_textures: ___() });
                    unsafe { context::set_context(Some(Ctx::new(ctx, self))); }
                    Box::new(super::QuadState { state : ContextRunner::new(state()) })
                }
            );
        }
    }
    */
}

*/