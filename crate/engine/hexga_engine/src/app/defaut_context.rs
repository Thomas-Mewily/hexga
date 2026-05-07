use super::*;

pub struct UserEvent
{
    inner : AppDefaultUserEventInner,
}
impl UserEvent
{
    pub(crate) fn new(inner : AppDefaultUserEventInner) -> Self { Self { inner }}
}
impl From<GpuEvent> for UserEvent
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
pub struct AppCtx
{
    pub(crate) window : Window,
    pub(crate) graphics : Option<Graphics>,
    pub(crate) time : TimeManager,
    pub(crate) clipboard : Clipboard,
    pub(crate) keyboard : Keyboard,
    pub(crate) unhandled_event : Vec<AppEvent>,
    pub(crate) fully_init : bool,
}

impl AppCtx
{
    fn init_app_if_needed<A>(&mut self, l: &mut AppLoop<UserEvent>, app: &mut A)
        where A: App<UserEvent, Self>
    {
        if self.fully_init || self.try_graphics().is_none() { return; }
        self.fully_init = true;

        let mut unhandled_event = mem::take(&mut self.unhandled_event);
        app.resumed(l, self);

        for ev in unhandled_event.drain(..)
        {
            app.event(ev, l, self);
        }

        
        
        /*
        let time = Time::since_launch();
        self.ctx.time().current = time;
        self.ctx.time().last = time;
        self.ctx.time().dt = zero();
        //self.ctx.time().tick = 0;

        let app = self.app.as_mut();
        app.resumed(&mut self.ctx);
        for ev in self.unhandled_event.drain(..)
        {
            app.event(ev, &mut self.ctx);
        }
        */
    }
}

impl<A> App<UserEvent, A> for AppCtx
    where A: App<UserEvent, Self>
{
    fn event(&mut self, ev: AppEvent<UserEvent>, l: &mut AppLoop<UserEvent>, app: &mut A) -> Option<AppEvent<UserEvent>> {
        if self.fully_init
        {
            app.event(ev, l, self)
        }else
        {
            self.unhandled_event.push(ev);
            None
        }
    }
    fn paused(&mut self, l: &mut AppLoop<UserEvent>, app: &mut A) 
    {
        if self.fully_init
        {
            app.paused(l, self);
        }
    }

    fn resumed(&mut self, l: &mut AppLoop<UserEvent>, app: &mut A) 
    {
        if self.window().init_window_if_needed(l)
        {
            if self.try_graphics().is_none()
            {
                let shared_window = self.window().window.as_ref().unwrap().clone();

                let proxy = l.proxy.clone();
                Graphics::init(
                    shared_window,
                    l.param.gpu.clone(),
                    None,
                    proxy,
                )
                .expect("failed to init the gpu");
                //self.event(AppEvent::Window(WindowEvent::Open));
            }
        }
    }
    fn tick(&mut self, dt: DeltaTime, l: &mut AppLoop<UserEvent>, app: &mut A) {
        if self.fully_init
        {
            app.tick(dt, l, self);
        }
    }

    fn draw(&mut self, l: &mut AppLoop<UserEvent>, app: &mut A) {
        if self.fully_init
        {
            app.draw(l, self);
        }
    }
    
    fn exit(&mut self, l: &mut AppLoop<UserEvent>, app: &mut A) {
        if self.fully_init
        {
            app.exit(l, self);
        }
    }

    fn user_event(&mut self, ev: UserEvent, l: &mut AppLoop<UserEvent>, app: &mut A) -> Option<UserEvent> {
        match ev.inner
        {
            AppDefaultUserEventInner::Gpu(graphics) => 
            {
                *self.try_graphics() = Some(graphics.expect("failed to init the gpu"));
                self.window().init_surface_if_needed();
                self.init_app_if_needed(l, app);
                None
            },
        }
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



impl HasMut<Graphics> for AppCtx
{
    fn retrive_mut(&mut self) -> &mut Graphics {
        self.graphics.as_mut().expect("graphics not init")
    }
}
impl HasMut<Keyboard> for AppCtx
{
    fn retrive_mut(&mut self) -> &mut Keyboard {
        &mut self.keyboard
    }
}
impl HasMut<Window> for AppCtx
{
    fn retrive_mut(&mut self) -> &mut Window {
        &mut self.window
    }
}
impl HasMut<TimeManager> for AppCtx
{
    fn retrive_mut(&mut self) -> &mut TimeManager {
        &mut self.time
    }
}

impl AppCtx
{
    fn try_graphics(&mut self) -> &mut Option<Graphics> 
    {
        &mut self.graphics
    }
}