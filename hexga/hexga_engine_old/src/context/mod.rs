use super::*;

mod ctx;
pub use ctx::*;

pub(crate) type ContextWinit = Arc<WinitWindow>;

#[derive(Default)]
pub struct Context
{
    pub(crate) window: Option<ContextWinit>,
    pub(crate) perf  : ContextPerformance,
    pub(crate) input : ContextInput,
}

impl Context
{
    /// Return the size of the drawable region in the current window
    /// 
    /// Each axis of the returned vector is >= 1
    pub fn window_size(&self) -> Point2
    {
        self.window.as_ref().map(|w| 
            {
                let size = w.inner_size();
                point2(size.width as _, size.height as _).max(one())
            }).unwrap_or(one())
    }
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
    fn begin_draw(&mut self, param: ScopedDrawParam) 
    { 
        Gpu.begin_draw(param);
        Perf.begin_draw(param);
    }
    fn end_draw(&mut self) 
    { 
        Perf.end_draw();
        Gpu.end_draw();
    }
}