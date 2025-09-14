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


impl ScopedSuspended for Context
{
    fn suspended(&mut self) {
        Input.suspended();
    }

    fn resumed(&mut self) {
        Input.resumed();
    }
}

impl ScopedUpdate for Context
{
    fn begin_update(&mut self) 
    { 
        Perf.begin_update();
        Input.begin_update();
    }
    fn end_update(&mut self) 
    { 
        Input.end_update();
        Perf.end_update();
    }
}

impl ScopedDraw for Context
{
    fn begin_draw(&mut self) 
    { 
        Gpu.begin_draw();
        Perf.begin_draw();
    }
    fn end_draw(&mut self) 
    { 
        Perf.end_draw();
        Gpu.end_draw();
    }
}