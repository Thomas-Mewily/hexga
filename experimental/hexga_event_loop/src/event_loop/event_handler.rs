use super::*;

#[derive(Debug)]
pub struct EventLoop<'a, Ev = ()>
where
    Ev: IPlatformEvent,
{
    winit: &'a WinitEventLoopActive,
    state: &'a mut EventLoopState,
    //pub time : &'a mut TimeManager,
    proxy: &'a EventLoopProxy<Ev>,
}

impl<'a, Ev, Surface> WindowManager<Surface> for EventLoop<'a, Ev>
where
    Ev: IPlatformEvent,
{
    fn create_window(&mut self, mut param: WindowParam) -> WindowResult<Window<Surface>>
    {
        match self.winit.create_window(param.clone().into())
        {
            Ok(window) =>
            {
                let size = window.inner_size().convert();
                let pos = window.inner_position().map(|w| w.convert()).unwrap_or_default();
                Ok(Window {
                    param,
                    window: Arc::new(window),
                    surface: None,
                    size,
                    pos,
                })
            }
            Err(_) => Err(()),
        }
    }
}

impl<'a, Ev> EventLoop<'a, Ev>
where
    Ev: IPlatformEvent,
{
    pub fn dt(&self) -> Duration { self.state.dt }
    pub fn time(&self) -> Time { self.state.time }
    pub fn proxy(&self) -> &'a EventLoopProxy<Ev> { self.proxy }

    pub(crate) fn new(winit: &'a WinitEventLoopActive, state: &'a mut EventLoopState, proxy: &'a EventLoopProxy<Ev>) -> Self { Self { winit, state, proxy } }

    pub fn winit_event_loop(&self) -> &'a WinitEventLoopActive { self.winit }
}
impl<'a, Ev> Clipboardable for EventLoop<'a, Ev>
where
    Ev: IPlatformEvent,
{
    fn get_clipboard(&mut self) -> Option<String> { self.state.get_clipboard() }

    fn set_clipboard(&mut self, paste: String) -> Result<(), ()> { self.state.set_clipboard(paste) }
}
impl<'a, Ev> EventLoopSendEvent<PlatformMessage<Ev>> for EventLoop<'a, Ev>
where
    Ev: IPlatformEvent,
{
    fn send_event(&self, ev: PlatformMessage<Ev>) -> ProxyResult { self.proxy().send_event(ev) }
}

#[derive(Debug)]
pub(crate) struct EventLoopState
{
    pub(crate) dt: Duration,
    pub(crate) time: Time,
    pub(crate) clipboard: Clipboard,
    pub(crate) key_modifiers: KeyModifiersFlags,
}

impl Clipboardable for EventLoopState
{
    fn get_clipboard(&mut self) -> Option<String> { self.clipboard.get_clipboard() }

    fn set_clipboard(&mut self, paste: String) -> Result<(), ()> { self.clipboard.set_clipboard(paste) }
}

pub trait PlatformEventHandler<CustomEvent = ()>: Sized + 'static
where
    CustomEvent: IPlatformEvent,
{
    /*
    fn tick(&mut self, dt: DeltaTime, l: &mut AppLoop<CustomEvent>)
    {
        /*
        l.time.dt
        while let Some(dt) = l.time.next()
        {

        }*/
        // Todo: mettre la target dans l.time
        /*
        while let Some(dt) = l.time.next()
        {

        }
        */
        /*
        for dt in l.time.update(dt)
        {
            self.update(dt, l, ctx);
        }
        */
    }*/

    //fn tick// + make a struct ApplyUpdateStrategyEventHandler to echantilloner les update. fn event_end + fn event_begin ?

    fn message(&mut self, ev: PlatformMessage<CustomEvent>, event_loop: &mut EventLoop<CustomEvent>) -> Option<PlatformMessage<CustomEvent>> 
    {
        match ev
        {
            PlatformMessage::Event(ev) => self.event(ev, event_loop).map(PlatformMessage::Event),
            PlatformMessage::Flow(flow) => { self.flow(flow, event_loop); None },
            PlatformMessage::Custom(ev) => self.custom(ev, event_loop).map(PlatformMessage::Custom),
        }
    }

    fn flow(&mut self, flow: PlatformFlow, event_loop: &mut EventLoop<CustomEvent>)
    {
        match flow
        {
            PlatformFlow::Update(dt) => self.update(dt, event_loop),
            PlatformFlow::Draw => self.draw(event_loop),
            PlatformFlow::Resume => self.resume(event_loop),
            PlatformFlow::Pause => self.pause(event_loop),
            PlatformFlow::Exit => self.exit(event_loop),
        }
    }

    fn event(&mut self, ev: PlatformEvent, event_loop: &mut EventLoop<CustomEvent>) -> Option<PlatformEvent> { Some(ev) }
    fn custom(&mut self, ev: CustomEvent, event_loop: &mut EventLoop<CustomEvent>) -> Option<CustomEvent> { Some(ev) }

    fn update(&mut self, dt: Duration, event_loop: &mut EventLoop<CustomEvent>) { let _ = dt; }
    fn draw(&mut self, event_loop: &mut EventLoop<CustomEvent>) { let _ = event_loop; }

    fn resume(&mut self, event_loop: &mut EventLoop<CustomEvent>) { let _ = event_loop; }
    fn pause(&mut self, event_loop: &mut EventLoop<CustomEvent>) { let _ = event_loop; }

    fn exit(&mut self, event_loop: &mut EventLoop<CustomEvent>) { let _ = event_loop; }
}

/*
pub struct AppCtx<'a,'b,User=AppDefaultUserEvent, Ctx=AppDefaultCtx>
    where User: AppUserEvent
{
    ctx: &'a mut Ctx,
    event_loop: &'a mut AppLoop<'b, User>,
    app_param: &'a AppParam,
}
impl<'a,'b,User,Ctx> Deref for AppCtx<'a,'b,User,Ctx>
    where User: AppUserEvent
{
    type Target=Ctx;

    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}
impl<'a,'b,User,Ctx> DerefMut for AppCtx<'a,'b,User,Ctx>
    where User: AppUserEvent
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ctx
    }
}
*/

/*
impl<'a,'b,User,Ctx,T> HasRef<T> for AppCtx<'a,'b,User,Ctx>
    where User: AppUserEvent, Ctx: HasRef<T>
{
    fn retrive_ref(&self) -> &T {
        self.ctx.retrive_ref()
    }
}
*/
/*
impl<'a,'b,User,Ctx> AppCtx<'a,'b,User,Ctx>
    where User: AppUserEvent
{
    pub(crate) fn new(ctx: &'a mut Ctx, event_loop: &'a mut AppLoop<'b,User>, app_param: &'a AppParam) -> Self
    {
        Self { ctx, event_loop, app_param }
    }
    pub fn event_loop(&mut self) -> &mut AppLoop<'b, User> { self.event_loop }
    pub fn context(&mut self) -> &mut Ctx { self.ctx }
    pub fn app_param(&mut self) -> &AppParam { self.app_param }

    #[doc(hidden)]
    pub fn change_ctx<'c,C>(&'c mut self, new_ctx: &'c mut C) -> (AppCtx<'c,'b,User,C>, &'c mut Ctx) where 'a: 'c
    {
        (AppCtx { ctx: new_ctx, event_loop: self.event_loop, app_param: self.app_param }, self.ctx)
    }
    #[doc(hidden)]
    pub fn with_ctx<'c,C>(&'c mut self, new_ctx: &'c mut C) -> AppCtx<'c,'b,User,C> where 'a: 'c
    {
        self.change_ctx(new_ctx).0
    }
}
*/

pub struct EventLoopProxy<CustomEvent>
where
    CustomEvent: IPlatformEvent,
{
    winit: WinitEventLoopProxy<CustomEvent>,
}
impl<CustomEvent> Debug for EventLoopProxy<CustomEvent>
where
    CustomEvent: IPlatformEvent,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "EventLoopProxy") }
}
impl<CustomEvent> Clone for EventLoopProxy<CustomEvent>
where
    CustomEvent: IPlatformEvent,
{
    fn clone(&self) -> Self { Self { winit: self.winit.clone() } }
}

pub type ProxyResult<T = ()> = Result<T, ()>;

impl<CustomEvent> EventLoopSendEvent<PlatformMessage<CustomEvent>> for EventLoopProxy<CustomEvent>
where
    CustomEvent: IPlatformEvent,
{
    fn send_event(&self, ev: PlatformMessage<CustomEvent>) -> ProxyResult
    {
        match self.winit.send_event(ev)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(()),
        }
    }
}
pub trait EventLoopSendEvent<E>
{
    fn send_event(&self, ev: E) -> ProxyResult;
}

impl<CustomEvent> EventLoopProxy<CustomEvent>
where
    CustomEvent: IPlatformEvent,
{
    pub(crate) fn new(winit: WinitEventLoopProxy<CustomEvent>) -> Self { Self { winit } }
    pub fn winit(&self) -> &WinitEventLoopProxy<CustomEvent> { &self.winit }
}
