use super::*;

#[derive(Default)]
pub struct AppCtx
{
    pub(crate) window : Window,
    pub(crate) graphics : Option<AppGraphics>,
    pub(crate) time : TimeManager,
}

impl AppContext for AppCtx
{
    fn window(&mut self) -> &mut Window {
        &mut self.window
    }

    fn try_graphics(&mut self) -> &mut Option<AppGraphics> 
    {
        &mut self.graphics
    }
    
    fn time(&mut self) -> &mut TimeManager {
        &mut self.time
    }
}

pub type Tick = u64;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct TimeManager
{
    /// Current accumulated time
    pub dt: DeltaTime,
    /// Current time
    pub current: Time,
    /// Previous time (before current dt)
    pub last: Time,
    /// Frame/step counter
    pub tick: Tick,

    pub strategy : TimeStrategy,
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TimeStrategy
{
    /// Updates run with the actual time delta between frames.
    /// The delta time will vary based on frame rate performance.
    Variable,

    /// Updates run at a constant, fixed interval regardless of frame rate.
    /// The same delta time value is used for every update step.
    Fixed(DeltaTime),

    /// Updates run with variable delta times, but the value is clamped to a maximum.
    /// Prevents large delta jumps during frame rate spikes or debugging pauses.
    /// Each delta time will be less than or equal to the specified cap.
    Capped(DeltaTime)
}

impl Default for TimeStrategy
{
    fn default() -> Self {
        Self::Capped((1.0).secs() / 30. as float)
    }
}




pub mod experimental
{
    pub(crate) use super::*;

    pub trait AppContext
    {
        fn window(&mut self) -> &mut Window;
        fn graphics(&mut self) -> &mut AppGraphics { self.try_graphics().as_mut().expect("graphics are not init") }
        fn try_graphics(&mut self) -> &mut Option<AppGraphics>;
        fn time(&mut self) -> &mut TimeManager;
    }
}
use experimental::*;


pub trait App<Event=AppEvent,Ctx=AppCtx>
{
    fn event(&mut self, ev: Event, ctx: &mut Ctx) -> Option<Event> { let _ = ev; Some(ev) }

    fn update(&mut self, dt: DeltaTime, ctx: &mut Ctx) { let _ = (dt, ctx); }
    fn draw(&mut self, ctx: &mut Ctx) { let _ = ctx; }

    fn resumed(&mut self, ctx: &mut Ctx) {}
    fn suspended(&mut self, ctx: &mut Ctx) {}
}
