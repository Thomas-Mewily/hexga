use super::*;

pub trait App<Ctx=AppDefaultCtx> : Sized
{
    fn event(&mut self, ev: AppEvent, ctx: &mut AppCtx<Ctx>) -> Option<AppEvent> { let _ = ctx; Some(ev) }

    fn update(&mut self, dt: DeltaTime, ctx: &mut AppCtx<Ctx>) { let _ = (dt, ctx); }
    fn draw(&mut self, ctx: &mut AppCtx<Ctx>) { let _ = ctx; }

    fn resumed(&mut self, ctx: &mut AppCtx<Ctx>) { let _ = ctx; }
    fn paused(&mut self, ctx: &mut AppCtx<Ctx>) { let _ = ctx; }

    fn exit(&mut self, ctx: &mut AppCtx<Ctx>) { let _ = ctx; }
}

pub struct AppCtx<'a,'b,Ctx=AppDefaultCtx>
{
    ctx: &'a mut Ctx,
    event_loop: &'a mut AppEventLoop<'b>,
    app_param: &'a AppParam,
    proxy: &'a AppProxy,
}
impl<'a,'b,Ctx> AppCtx<'a,'b,Ctx>
{
    pub(crate) fn new(ctx: &'a mut Ctx, event_loop: &'a mut AppEventLoop<'b>, app_param: &'a AppParam, proxy : &'a AppProxy) -> Self
    {
        Self { ctx, event_loop, app_param, proxy }
    }
    pub fn event_loop(&mut self) -> &mut AppEventLoop<'b> { self.event_loop }
    pub fn context(&mut self) -> &mut Ctx { self.ctx }
    pub fn app_param(&mut self) -> &AppParam { self.app_param }
    // Todo : wrap the proxy type ? 
    pub fn proxy(&mut self) -> &AppProxy { self.proxy }

    pub fn with_ctx<'c,C>(&'c mut self, new_ctx: &'c mut C) -> AppCtx<'c,'b,C> where 'a: 'c
    {
        AppCtx { ctx: new_ctx, event_loop: self.event_loop, app_param: self.app_param, proxy: self.proxy }
    }
}

pub struct AppProxy
{
    winit : WinitEventLoopProxy,   
}
impl AppProxy
{
    pub(crate) fn new(winit: WinitEventLoopProxy) -> Self { Self { winit }}
    pub fn winit(&self) -> &WinitEventLoopProxy { &self.winit }
}

