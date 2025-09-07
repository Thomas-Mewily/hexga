use super::*;

mod ctx;
pub use ctx::*;

mod performance;
pub use performance::*;

mod singleton;
pub use singleton::*;

pub type ContextWinit = Arc<Window>;

#[derive(Default)]
pub struct Context
{
    pub(crate) winit: Option<ContextWinit>,
    pub(crate) perf : ContextPerformance,
}

impl Scoped<Update> for Context
{
    fn begin(&mut self) 
    { 
        Perf.begin_update();
    }
    fn end(&mut self) 
    { 
        Perf.end_update();
    }
}

impl Scoped<Draw> for Context
{
    fn begin(&mut self) 
    { 
        Gpu.begin_draw();
        Perf.begin_draw();
    }
    fn end(&mut self) 
    { 
        Perf.end_draw();
        Gpu.end_draw();
    }
}

pub mod prelude
{
    pub use super::Ctx;
    pub use super::singleton::prelude::*;
    pub use super::performance::prelude::*;
}