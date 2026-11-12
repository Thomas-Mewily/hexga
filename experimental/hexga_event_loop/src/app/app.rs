use super::*;

pub struct EventLoop<'a, Ev>
where
    Ev: IAppEvent,
{
    winit: &'a WinitEventLoopActive,
    state: &'a mut AppState,
    //pub time : &'a mut TimeManager,
    proxy: &'a AppProxy<Ev>,
}

impl<'a, Ev> Debug for EventLoop<'a, Ev>
where
    Ev: IAppEvent,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        f.debug_struct("EventLoop")
            .field("winit", &self.winit)
            .field("state", &self.state)
            .field("proxy", &self.proxy)
            .finish()
    }
}

impl<'a, Ev, Surface> WindowManager<Surface> for EventLoop<'a, Ev>
where
    Ev: IAppEvent,
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
    Ev: IAppEvent,
{
    pub fn dt(&self) -> Duration { self.state.dt }
    pub fn time(&self) -> Time { self.state.time }
    pub fn proxy(&self) -> &'a AppProxy<Ev> { self.proxy }

    pub(crate) fn new(winit: &'a WinitEventLoopActive, state: &'a mut AppState, proxy: &'a AppProxy<Ev>) -> Self { Self { winit, state, proxy } }

    pub fn winit_event_loop(&self) -> &'a WinitEventLoopActive { self.winit }
}
impl<'a, Ev> Clipboardable for EventLoop<'a, Ev>
where
    Ev: IAppEvent,
{
    fn get_clipboard(&mut self) -> Option<String> { self.state.get_clipboard() }

    fn set_clipboard(&mut self, paste: String) -> Result<(), ()> { self.state.set_clipboard(paste) }
}
impl<'a, Ev> AppSendEvent<AppMessage<Ev>> for EventLoop<'a, Ev>
where
    Ev: IAppEvent,
{
    fn send_event(&self, ev: AppMessage<Ev>) -> ProxyResult { self.proxy().send_event(ev) }
}

#[derive(Debug)]
pub(crate) struct AppState
{
    pub(crate) dt: Duration,
    pub(crate) time: Time,
    pub(crate) clipboard: Clipboard,
    pub(crate) key_modifiers: KeyModifiersFlags,
}

impl Clipboardable for AppState
{
    fn get_clipboard(&mut self) -> Option<String> { self.clipboard.get_clipboard() }
    fn set_clipboard(&mut self, paste: String) -> Result<(), ()> { self.clipboard.set_clipboard(paste) }
}

pub trait App<CustomEvent, Ctx>
where
    CustomEvent: IAppEvent,
{
    fn message(&mut self, ev: AppMessage<CustomEvent>, ctx: &mut Ctx) -> Option<AppMessage<CustomEvent>> { self.dispatch_message(ev, ctx) }
    fn dispatch_message(&mut self, ev: AppMessage<CustomEvent>, ctx: &mut Ctx) -> Option<AppMessage<CustomEvent>>
    {
        match ev
        {
            AppMessage::Event(ev) => self.event(ev, ctx).map(AppMessage::Event),
            AppMessage::Flow(flow) =>
            {
                self.flow(flow, ctx);
                None
            }
            AppMessage::Custom(ev) => self.custom(ev, ctx).map(AppMessage::Custom),
        }
    }

    fn flow(&mut self, flow: AppFlow, ctx: &mut Ctx) { self.dispatch_flow(flow, ctx) }
    fn dispatch_flow(&mut self, flow: AppFlow, ctx: &mut Ctx)
    {
        match flow
        {
            AppFlow::Update(dt) => self.update(dt, ctx),
            AppFlow::Draw => self.draw(ctx),
            AppFlow::Resume => self.resume(ctx),
            AppFlow::Pause => self.pause(ctx),
            AppFlow::Exit => self.exit(ctx),
        }
    }

    fn event(&mut self, ev: AppEvent, ctx: &mut Ctx) -> Option<AppEvent> { Some(ev) }
    fn custom(&mut self, ev: CustomEvent, ctx: &mut Ctx) -> Option<CustomEvent> { Some(ev) }

    fn update(&mut self, dt: Duration, ctx: &mut Ctx) { let _ = dt; }
    fn draw(&mut self, ctx: &mut Ctx) { let _ = ctx; }

    fn resume(&mut self, ctx: &mut Ctx) { let _ = ctx; }
    fn pause(&mut self, ctx: &mut Ctx) { let _ = ctx; }

    fn exit(&mut self, ctx: &mut Ctx) { let _ = ctx; }
}

/*
pub struct Update
{
    dt: DeltaTime,
}
pub struct Draw
{
    coef: coef,
}

*/

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

/// Proxy to send event to the App.
pub struct AppProxy<CustomEvent>
where
    CustomEvent: IAppEvent,
{
    winit: WinitEventLoopProxy<CustomEvent>,
}
impl<CustomEvent> Debug for AppProxy<CustomEvent>
where
    CustomEvent: IAppEvent,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "EventLoopProxy") }
}
impl<CustomEvent> Clone for AppProxy<CustomEvent>
where
    CustomEvent: IAppEvent,
{
    fn clone(&self) -> Self { Self { winit: self.winit.clone() } }
}

pub type ProxyResult<T = ()> = Result<T, ()>;

impl<CustomEvent> AppSendEvent<AppMessage<CustomEvent>> for AppProxy<CustomEvent>
where
    CustomEvent: IAppEvent,
{
    fn send_event(&self, ev: AppMessage<CustomEvent>) -> ProxyResult
    {
        match self.winit.send_event(ev)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(()),
        }
    }
}
pub trait AppSendEvent<E>
{
    fn send_event(&self, ev: E) -> ProxyResult;
}

impl<CustomEvent> AppProxy<CustomEvent>
where
    CustomEvent: IAppEvent,
{
    pub(crate) fn new(winit: WinitEventLoopProxy<CustomEvent>) -> Self { Self { winit } }
    pub fn winit(&self) -> &WinitEventLoopProxy<CustomEvent> { &self.winit }
}
