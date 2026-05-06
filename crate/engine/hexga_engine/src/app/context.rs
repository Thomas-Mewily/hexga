use hexga::ptr::NonNull;

use super::*;


/// A single window context
#[derive(Default)]
pub struct AppDefaultCtx // <UserData>
{
    pub(crate) window : Window,
    pub(crate) graphics : Option<Graphics>,
    pub(crate) time : TimeManager,
    pub(crate) clipboard : Clipboard,
    pub(crate) keyboard : Keyboard,
    pub(crate) unhandled_event : Vec<AppEvent>,
}

impl<A> App<A> for AppDefaultCtx
{
    fn event(&mut self, ev: AppEvent, ctx: &mut AppCtx<A>) -> Option<AppEvent> 
    {
        Some(ev)
    }
    fn paused(&mut self, ctx: &mut AppCtx<A>) {
        
    }
    fn resumed(&mut self, ctx: &mut AppCtx<A>) {
        
    }
    fn update(&mut self, dt: DeltaTime, ctx: &mut AppCtx<A>) {
        
    }
    fn draw(&mut self, ctx: &mut AppCtx<A>) {
        
    }
}

/*
impl AppWithEventLoop<AppEvent,()> for AppCtx
{
    fn event(&mut self, ev: AppEvent, ctx: &mut ()) -> Option<AppEvent> {
        match &ev
        {
            AppEvent::Input(input) => match input
            {
                InputEvent::Key(k) => { self.keyboard().key_event(*k); None },
            },
            AppEvent::Window(window) => match window
            {
                WindowEvent::Resize(size) => { self.window().configure_surface(); None },
                WindowEvent::Move(_pos) => Some(ev),
                WindowEvent::Open => Some(ev),
                WindowEvent::Close => Some(ev),
                WindowEvent::Destroy => { self.window().destroy(); None },
            },
        }
    }

    fn resumed(&mut self, ctx: &mut ()) {
        
    }

    fn paused(&mut self, ctx: &mut ()) {
        
    }

    fn update(&mut self, dt: DeltaTime, ctx: &mut ()) {
        
    }

    fn draw(&mut self, ctx: &mut ()) {
        
    }
}
impl<A> AppContext<A> for AppCtx where A: AppWithEventLoop<AppEvent,Self>
{
    fn set_graphics(&mut self, gfx: Option<Graphics>, app: &mut A) 
    {
        self.graphics = gfx;
    }
}
*/



impl HasMut<Graphics> for AppDefaultCtx
{
    fn retrive_mut(&mut self) -> &mut Graphics {
        self.graphics.as_mut().expect("graphics not init")
    }
}
impl HasMut<Keyboard> for AppDefaultCtx
{
    fn retrive_mut(&mut self) -> &mut Keyboard {
        &mut self.keyboard
    }
}
impl HasMut<Window> for AppDefaultCtx
{
    fn retrive_mut(&mut self) -> &mut Window {
        &mut self.window
    }
}
impl HasMut<TimeManager> for AppDefaultCtx
{
    fn retrive_mut(&mut self) -> &mut TimeManager {
        &mut self.time
    }
}

impl AppDefaultCtx
{
    fn try_graphics(&mut self) -> &mut Option<Graphics> 
    {
        &mut self.graphics
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
        Self::Fixed((1.0).secs() / 60. as float)
    }
}



#[derive(Clone, Debug)]
pub struct AppWithCtx<A,Ctx>
    where 
    A : App<Ctx>,
    Ctx: App<A>
{
    pub app : A,
    pub ctx : Ctx,
    no_unpack: PhantomData<()>,
}


impl<A,Ctx> AppWithCtx<A,Ctx>
    where 
    A : App<Ctx>,
    Ctx: App<A>
{
    pub fn new(app: A, ctx: Ctx) -> Self 
    {
        Self { app, ctx, no_unpack: PhantomData }
    }
    pub fn into_app_and_ctx(self) -> (A,Ctx) { (self.app, self.ctx) }
}

impl<A,Ctx> App<()> for AppWithCtx<A,Ctx>
    where 
    A : App<Ctx>,
    Ctx: App<A>
{
    fn event(&mut self, ev: AppEvent, ctx: &mut AppCtx<()>) -> Option<AppEvent> {
        self.ctx.event(ev, &mut AppCtx::new(&mut self.app, ctx.event_loop))
    }
    fn draw(&mut self, ctx: &mut AppCtx<()>) {
        self.ctx.draw(&mut AppCtx::new(&mut self.app, ctx.event_loop))
    }
    fn paused(&mut self, ctx: &mut AppCtx<()>) {
        self.ctx.paused(&mut AppCtx::new(&mut self.app, ctx.event_loop))
    }
    fn resumed(&mut self, ctx: &mut AppCtx<()>) {
        self.ctx.resumed(&mut AppCtx::new(&mut self.app, ctx.event_loop))
    }
    fn update(&mut self, dt: DeltaTime, ctx: &mut AppCtx<()>) {
        self.ctx.update(dt, &mut AppCtx::new(&mut self.app, ctx.event_loop))
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
impl_has_mut_trait!(HasMutTimeManager, TimeManager, time);
pub trait HasMutGraphics: HasMut<Graphics>
{
    fn graphics(&mut self) ->  &mut Graphics 
    {
        self.retrive_mut()
    }
}
impl <T> HasMutGraphics for T where T: HasMut<Graphics> {}
