use super::*;

pub struct AppDefaultUserEvent
{
    inner : AppDefaultUserEventInner,
}
impl AppDefaultUserEvent
{
    pub(crate) fn new(inner : AppDefaultUserEventInner) -> Self { Self { inner }}
}
impl From<GpuEvent> for AppDefaultUserEvent
{
    fn from(value: GpuEvent) -> Self 
    {
        Self { inner: AppDefaultUserEventInner::Gpu(value) }
    }
}

pub(crate) enum AppDefaultUserEventInner
{
    Gpu(GpuEvent),
    //Custom(CustomEvent),
}

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

impl<A> App<AppDefaultUserEvent, A> for AppDefaultCtx
{
    fn event(&mut self, ev: AppEvent, ctx: &mut AppCtx<AppDefaultUserEvent,A>) -> Option<AppEvent> 
    {
        Some(ev)
    }
    fn paused(&mut self, ctx: &mut AppCtx<AppDefaultUserEvent,A>) {
        
    }
    fn resumed(&mut self, ctx: &mut AppCtx<AppDefaultUserEvent,A>) 
    {
        if self.window().init_window_if_needed(ctx.event_loop())
        {
            if self.try_graphics().is_none()
            {
                let shared_window = self.window().window.as_ref().unwrap().clone();

                let proxy = ctx.proxy().clone();
                Graphics::init(
                    shared_window,
                    ctx.app_param().gpu.clone(),
                    None,
                    proxy,
                )
                .expect("failed to init the gpu");
                //self.event(AppEvent::Window(WindowEvent::Open));
            }
        }
    }
    fn update(&mut self, dt: DeltaTime, ctx: &mut AppCtx<AppDefaultUserEvent,A>) {
        
    }
    fn draw(&mut self, ctx: &mut AppCtx<AppDefaultUserEvent,A>) {
        
    }
    
    fn exit(&mut self, ctx: &mut AppCtx<AppDefaultUserEvent,A>) {
        todo!()
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