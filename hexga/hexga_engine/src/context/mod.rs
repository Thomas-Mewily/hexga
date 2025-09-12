use super::*;

mod ctx;
pub use ctx::*;

pub(crate) type ContextWinit = Arc<WinitWindow>;

#[derive(Default)]
pub struct Context
{
    pub(crate) winit: Option<ContextWinit>,
    pub(crate) perf : ContextPerformance,
    pub(crate) input : ContextInput,
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