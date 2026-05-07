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

    /// The update strategy
    pub strategy : TimeStrategy,
}


pub struct TimeManagerUpdater<'a>
{
    manager: &'a mut TimeManager,
    target: DeltaTime,
    //finish: bool,
}

impl<'a> Drop for TimeManagerUpdater<'a>
{
    fn drop(&mut self) {
        self.manager.last = self.manager.current;
    }
}

impl<'a> Iterator for TimeManagerUpdater<'a>
{
    type Item=DeltaTime;
    fn next(&mut self) -> Option<Self::Item> 
    {
        let time = &mut self.manager;
        let target = self.target;

        if time.current >= target { return None; };

        let mut dt = target - time.current;

        if dt <= DeltaTime::ZERO { return None; }
        
        let (step_dt, consume_dt_rest) = match time.strategy 
        {
            TimeStrategy::Variable => 
            {
                time.last = time.current;
                time.current = target;
                time.dt = dt;
                return Some(dt);
            }
            TimeStrategy::Fixed(step_dt) => (step_dt, false),
            TimeStrategy::Capped(max_dt) => (dt.min_partial(max_dt), true)
        };

        if step_dt.is_negative_or_zero() { return None; }
        
        if dt >= step_dt
        {
            time.last = time.current;
            time.current += step_dt;
            time.dt = step_dt;
            Some(step_dt)
        }
        else if consume_dt_rest
        {
            time.last = time.current;
            time.current = target;
            time.dt = dt;
            Some(dt)
        } else
        {
            None
        }
    }
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

    pub fn update(&mut self, dt: DeltaTime) -> TimeManagerUpdater<'_>
    {
        //self.last = self.current;
        //self += dt;
        //self.dt = dt;
        //self.tick += 1;
        let target = self.current + dt;
        TimeManagerUpdater { manager: self, target }
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
    fn event(&mut self, ev: AppEvent<User>, l: &mut AppLoop<User>, ctx: &mut ()) -> Option<AppEvent<User>> {
        self.ctx.event(ev, l, &mut self.app)
    }
    fn tick(&mut self, dt: DeltaTime, l: &mut AppLoop<User>, ctx: &mut ()) {
        self.ctx.tick(dt, l, &mut self.app)
    }
    fn draw(&mut self, l: &mut AppLoop<User>, ctx: &mut ()) {
        self.ctx.draw(l, &mut self.app)
    }
    fn paused(&mut self, l: &mut AppLoop<User>, ctx: &mut ()) {
        self.ctx.paused(l, &mut self.app)
    }
    fn resumed(&mut self, l: &mut AppLoop<User>, ctx: &mut ()) {
        self.ctx.resumed(l, &mut self.app)
    }

    fn exit(&mut self, l: &mut AppLoop<User>, ctx: &mut ()) {
        self.ctx.exit(l, &mut self.app)
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
    fn gfx(&mut self) ->  &mut Graphics 
    {
        self.retrive_mut()
    }
}
impl <T> HasMutGraphics for T where T: HasMut<Graphics> {}
