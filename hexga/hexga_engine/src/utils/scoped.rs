use super::*;


// Wrapper arround Scoped, but non public


pub(crate) trait ScopedSuspended
{
    fn suspended(&mut self);
    fn resumed(&mut self);
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct ScopedDrawParam
{
    pub window_size : Point2,
}

pub(crate) trait ScopedDraw
{
    fn begin_draw(&mut self, param: ScopedDrawParam);
    fn end_draw(&mut self);
    fn scoped_draw<F>(&mut self, param: ScopedDrawParam, f: F) where F: FnOnce() { self.begin_draw(param); f(); self.end_draw(); }
}
pub (crate) trait ScopedUpdate
{
    fn begin_update(&mut self);
    fn end_update(&mut self);
    fn scoped_update<F>(&mut self, f: F) where F: FnOnce() { self.begin_update(); f(); self.end_update(); }
}