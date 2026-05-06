use super::*;

pub trait App<User=AppDefaultUserEvent,Ctx=AppDefaultCtx> : Sized
    where User: AppUserEvent
{
    fn event(&mut self, ev: AppEvent<User>, ctx: &mut AppCtx<User,Ctx>) -> Option<AppEvent<User>> { let _ = ctx; Some(ev) }

    fn update(&mut self, dt: DeltaTime, ctx: &mut AppCtx<User,Ctx>) { let _ = (dt, ctx); }
    fn draw(&mut self, ctx: &mut AppCtx<User,Ctx>) { let _ = ctx; }

    fn resumed(&mut self, ctx: &mut AppCtx<User,Ctx>) { let _ = ctx; }
    fn paused(&mut self, ctx: &mut AppCtx<User,Ctx>) { let _ = ctx; }

    fn exit(&mut self, ctx: &mut AppCtx<User,Ctx>) { let _ = ctx; }
}

pub struct AppCtx<'a,'b,User=AppDefaultUserEvent, Ctx=AppDefaultCtx>
    where User: AppUserEvent
{
    ctx: &'a mut Ctx,
    event_loop: &'a mut AppEventLoop<'b>,
    app_param: &'a AppParam,
    proxy: &'a AppProxy<User>,
}
impl<'a,'b,User,Ctx> AppCtx<'a,'b,User,Ctx>
    where User: AppUserEvent
{
    pub(crate) fn new(ctx: &'a mut Ctx, event_loop: &'a mut AppEventLoop<'b>, app_param: &'a AppParam, proxy : &'a AppProxy<User>) -> Self
    {
        Self { ctx, event_loop, app_param, proxy }
    }
    pub fn event_loop(&mut self) -> &mut AppEventLoop<'b> { self.event_loop }
    pub fn context(&mut self) -> &mut Ctx { self.ctx }
    pub fn app_param(&mut self) -> &AppParam { self.app_param }
    // Todo : wrap the proxy type ? 
    pub fn proxy(&mut self) -> &AppProxy<User> { self.proxy }

    pub fn with_ctx<'c,C>(&'c mut self, new_ctx: &'c mut C) -> AppCtx<'c,'b,User,C> where 'a: 'c
    {
        AppCtx { ctx: new_ctx, event_loop: self.event_loop, app_param: self.app_param, proxy: self.proxy }
    }
}

pub struct AppProxy<User>
    where User: AppUserEvent
{
    winit : WinitEventLoopProxy<User>,   
}
impl<User> Debug for AppProxy<User> where User: AppUserEvent
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppProxy")
    }
}
impl<User> Clone for AppProxy<User> where User: AppUserEvent
{
    fn clone(&self) -> Self {
        Self { winit: self.winit.clone() }
    }
}

impl<User,E> AppSendEvent<E> for AppProxy<User> where
    User: From<E> + AppUserEvent,
{
    fn send_event(&mut self, ev: E) -> AppResult {
        self.winit.send_event(ev.into()).map_err(|_|())
    }
}
pub trait AppSendEvent<E>
{
    fn send_event(&mut self, ev: E) -> AppResult;
}

impl<User> AppProxy<User>
    where User: AppUserEvent
{
    pub(crate) fn new(winit: WinitEventLoopProxy<User>) -> Self { Self { winit }}
    pub fn winit(&self) -> &WinitEventLoopProxy<User> { &self.winit }
}

