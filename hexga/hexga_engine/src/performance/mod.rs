use super::*;

pub mod prelude
{
    pub use super::Perf;
    pub use super::ContextPerformance;
}


ctx_singleton!(
    Perf,
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
    pub fn fps(&self) -> TimeCounter { self.fps }
    pub fn nb_fps(&self) -> int { self.fps.nb() }

    pub fn ups(&self) -> TimeCounter { self.ups }
    pub fn nb_ups(&self) -> int { self.ups.nb() }
}

impl Scoped<Update> for ContextPerformance
{
    fn begin(&mut self) { self.ups.increase(); }
    fn end(&mut self) { }
}

impl Scoped<Draw> for ContextPerformance
{
    fn begin(&mut self) { self.fps.increase(); }
    fn end(&mut self) { }
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeCounter
{
    timer: Time,
    counter: int,
    last_counter: int,
}
impl Default for TimeCounter
{
    fn default() -> Self {
        Self { timer: Time::now(), counter: 0, last_counter: 0 }
    }
}
impl TimeCounter
{
    pub fn nb(&self) -> int { self.last_counter }
    pub fn increase(&mut self) 
    {
        let now = Time::now();
        if now - self.timer >= 1.s()
        {
            self.timer = now;
            self.last_counter = self.counter;
            self.counter = 0;
        }
        self.counter += 1;
    } 
}

