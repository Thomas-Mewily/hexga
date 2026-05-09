use super::*;

#[derive(Debug)]
pub struct EventLoop<'a, Ev=()>
    where Ev: PlatformCustomEvent
{
    winit : &'a WinitEventLoopActive,
    state: &'a mut EventLoopState,
    //pub time : &'a mut TimeManager,
    proxy: &'a EventLoopProxy<Ev>,
}

impl<'a, Ev> EventLoop<'a, Ev>
    where Ev: PlatformCustomEvent
{
    pub fn dt(&self) -> Duration { self.state.dt }
    pub fn time(&self) -> Time { self.state.time }
    pub fn proxy(&self) -> &'a EventLoopProxy<Ev> { self.proxy }

    pub(crate) fn new(winit : &'a WinitEventLoopActive, state: &'a mut EventLoopState, proxy: &'a EventLoopProxy<Ev>) -> Self 
    {
        Self { winit, state, proxy }
    }

    pub fn winit_event_loop(&self) -> &'a WinitEventLoopActive { self.winit }
}
impl<'a, Ev> EventLoopSendEvent<PlatformEvent<Ev>> for EventLoop<'a, Ev>
    where Ev: PlatformCustomEvent
{
    fn send_event(&self, ev: PlatformEvent<Ev>) -> ProxyResult {
        self.proxy().send_event(ev)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct EventLoopState
{
    pub dt: Duration,
    pub time : Time,
}

pub trait PlatformEventHandler<CustomEvent=()> : Sized
    where CustomEvent: PlatformCustomEvent
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

    fn update(&mut self, dt: Duration, event_loop: &mut EventLoop<CustomEvent>) { let _ = dt; }
    fn draw(&mut self, event_loop: &mut EventLoop<CustomEvent>) { let _ = event_loop; }

    fn resumed(&mut self, event_loop: &mut EventLoop<CustomEvent>) { let _ = event_loop; }
    fn paused(&mut self, event_loop: &mut EventLoop<CustomEvent>) { let _ = event_loop; }

    fn exit(&mut self, event_loop: &mut EventLoop<CustomEvent>) { let _ = event_loop; }
    

    fn event(&mut self, ev: PlatformEvent<CustomEvent>, event_loop: &mut EventLoop<CustomEvent>) -> Option<PlatformEvent<CustomEvent>> 
    {
        self.dispatch_event(ev, event_loop)
    }
    fn dispatch_event(&mut self, ev: PlatformEvent<CustomEvent>, event_loop: &mut EventLoop<CustomEvent>) -> Option<PlatformEvent<CustomEvent>> 
    {
        match ev
        {
            PlatformEvent::Input(input) => self.input_event(input, event_loop).map(PlatformEvent::Input),
            PlatformEvent::Window(window) => self.window_event(window, event_loop).map(PlatformEvent::Window),
            PlatformEvent::Custom(custom) => self.custom_event(custom, event_loop).map(PlatformEvent::Custom),
        }
    }

    fn custom_event(&mut self, ev: CustomEvent, event_loop: &mut EventLoop<CustomEvent>) -> Option<CustomEvent> { Some(ev) }
    fn window_event(&mut self, ev: WindowEvent, event_loop: &mut EventLoop<CustomEvent>) -> Option<WindowEvent> { Some(ev) }
    fn input_event(&mut self, ev: InputEvent, event_loop: &mut EventLoop<CustomEvent>) -> Option<InputEvent> { Some(ev) }


    fn run_event_loop_with_param(self, param: EventLoopParam) -> EventLoopResult 
    {
        crate::event_loop::run(self, param)
    }
}

pub trait PlatformEventHandlerExtension<CustomEvent> : PlatformEventHandler<CustomEvent>
    where CustomEvent: PlatformCustomEvent
{
    fn run_event_loop(self) -> EventLoopResult { self.run_event_loop_with_param(___()) }
}
impl<CustomEvent,EventHandler> PlatformEventHandlerExtension<CustomEvent> for EventHandler
    where 
    CustomEvent: PlatformCustomEvent,
    EventHandler: PlatformEventHandler<CustomEvent>
{}

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
    where CustomEvent: PlatformCustomEvent
{
    winit : WinitEventLoopProxy<CustomEvent>,
}
impl<CustomEvent> Debug for EventLoopProxy<CustomEvent> where CustomEvent: PlatformCustomEvent
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventLoopProxy")
    }
}
impl<CustomEvent> Clone for EventLoopProxy<CustomEvent> where CustomEvent: PlatformCustomEvent
{
    fn clone(&self) -> Self {
        Self { winit: self.winit.clone() }
    }
}

pub type ProxyResult<T=()> = Result<T,()>;

impl<CustomEvent> EventLoopSendEvent<PlatformEvent<CustomEvent>> for EventLoopProxy<CustomEvent> 
    where CustomEvent: PlatformCustomEvent
{
    fn send_event(&self, ev: PlatformEvent<CustomEvent>) -> ProxyResult {
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
    where CustomEvent: PlatformCustomEvent
{
    pub(crate) fn new(winit: WinitEventLoopProxy<CustomEvent>) -> Self { Self { winit }}
    pub fn winit(&self) -> &WinitEventLoopProxy<CustomEvent> { &self.winit }
}

