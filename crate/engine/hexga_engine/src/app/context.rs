use super::*;



pub type Tick = u64;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct TimeManager
{
    /// Current accumulated time
    pub(crate) dt: DeltaTime,
    /// Current time
    pub(crate) current: Time,
    /// Previous time (before current dt)
    pub(crate) last: Time,
    /// Frame/step counter
    pub(crate) tick: Tick,

    pub strategy : TimeStrategy,
}

impl TimeManager
{
    pub fn dt(&self) -> DeltaTime { self.dt }
    pub fn current(&self) -> Time { self.current }
    pub fn last(&self) -> Time { self.last }
    pub fn tick(&self) -> Tick { self.tick }

    pub(crate) fn new(strategy : TimeStrategy) -> Self 
    {
        let time = Time::since_launch();
        Self { dt: zero(), current: time, last: time, tick: 0, strategy }
    }
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
        Self::Fixed((1.0).secs() / 60. as float)
    }
}



#[derive(Clone, Debug)]
pub struct AppWithCtx<A,User,Ctx>
    where 
    A : App<User,Ctx>,
    Ctx: App<User,A>,
    User: AppUserEvent,
{
    pub app : A,
    pub ctx : Ctx,
    phantom: PhantomData<User>,
}


impl<A,User,Ctx> AppWithCtx<A,User,Ctx>
    where 
    A : App<User,Ctx>,
    Ctx: App<User,A>,
    User: AppUserEvent
{
    pub fn new(app: A, ctx: Ctx) -> Self 
    {
        Self { app, ctx, phantom: PhantomData }
    }
    pub fn into_app_and_ctx(self) -> (A,Ctx) { (self.app, self.ctx) }
}

impl<A,User,Ctx> App<User,()> for AppWithCtx<A,User,Ctx>
    where 
    A : App<User,Ctx>,
    Ctx: App<User,A>,
    User: AppUserEvent
{
    fn event(&mut self, ev: AppEvent<User>, ctx: &mut AppCtx<User,()>) -> Option<AppEvent<User>> {
        self.ctx.event(ev, &mut ctx.with_ctx(&mut self.app))
    }
    fn update(&mut self, dt: DeltaTime, ctx: &mut AppCtx<User, ()>) {
        self.ctx.update(dt, &mut ctx.with_ctx(&mut self.app))
    }
    fn draw(&mut self, ctx: &mut AppCtx<User, ()>) {
        self.ctx.draw(&mut ctx.with_ctx(&mut self.app))
    }
    fn paused(&mut self, ctx: &mut AppCtx<User, ()>) {
        self.ctx.paused(&mut ctx.with_ctx(&mut self.app))
    }
    fn resumed(&mut self, ctx: &mut AppCtx<User, ()>) {
        self.ctx.resumed(&mut ctx.with_ctx(&mut self.app))
    }

    fn exit(&mut self, ctx: &mut AppCtx<User, ()>) {
        self.ctx.exit(&mut ctx.with_ctx(&mut self.app))
    }
}


/*
pub trait AppContext<A> : 
    AppWithEventLoop<AppEvent,A> + HasMutTimeManager //+ HasMutGraphics
    where A: AppWithEventLoop<AppEvent,Self>
{
    #[doc(hidden)]
    fn set_graphics(&mut self, gfx: Option<Graphics>, app: &mut A);
    #[doc(hidden)]
    fn end_draw(&mut self, app: &mut A) {}
    #[doc(hidden)]
    fn end_update(&mut self, dt: DeltaTime, app: &mut A) {}
}
    */
/*
pub trait AppContext : HasMut<Keyboard>
{
    fn window(&mut self) -> &mut Window;
    fn graphics(&mut self) -> &mut AppGraphics { self.try_graphics().as_mut().expect("graphics are not init") }
    fn try_graphics(&mut self) -> &mut Option<AppGraphics>;
    fn time(&mut self) -> &mut TimeManager;
    fn clipboard(&mut self) -> &mut Clipboard;
    fn keyboard(&mut self) -> &mut Keyboard { self.retrive_mut() }
}
*/



impl_has_mut_trait!(HasMutWindow, Window, window);
impl_has_mut_trait!(HasMutKeyboard, Keyboard, keyboard);
impl_has_mut_trait!(HasMutClipboard, Clipboard, clipboard);
pub trait HasMutGraphics: HasMut<Graphics>
{
    fn graphics(&mut self) ->  &mut Graphics 
    {
        self.retrive_mut()
    }
}
impl <T> HasMutGraphics for T where T: HasMut<Graphics> {}
