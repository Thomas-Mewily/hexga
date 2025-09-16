use super::*;

singleton_access!(
    pub Perf,
    ContextPerformance,
    { Ctx::try_as_ref().map(|ctx| &ctx.perf) },
    { Ctx::try_as_mut().map(|ctx| &mut ctx.perf) }
);


#[derive(Default, Clone, PartialEq, Debug)]
pub struct ContextPerformance
{
    fps : TimeCounter,
    ups: TimeCounter,
}

impl ContextPerformance
{
    /// Return the previous number of frame per second
    pub fn fps(&self) -> int { self.fps.nb() }

    /// Return the previous update per second
    pub fn ups(&self) -> int { self.ups.nb() }
}

impl ScopedUpdate for ContextPerformance
{
    fn begin_update(&mut self) { self.ups.count(); }
    fn end_update(&mut self) { }
}

impl ScopedDraw for ContextPerformance
{
    fn begin_draw(&mut self, param: ScopedDrawParam) { self.fps.count(); }
    fn end_draw(&mut self) { }
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TimeCounter
{
    timer: Time,
    counter: int,
    last_counter: int,
}
impl Default for TimeCounter
{
    fn default() -> Self {
        Self { timer: Time::since_launch(), counter: 0, last_counter: 0 }
    }
}
impl TimeCounter
{
    pub fn nb(&self) -> int { self.last_counter }
    pub fn count(&mut self) 
    {
        let now = Time::since_launch();
        if now - self.timer >= 1.s()
        {
            self.timer = now;
            self.last_counter = self.counter;
            self.counter = 0;
        }
        self.counter += 1;
    } 
}

