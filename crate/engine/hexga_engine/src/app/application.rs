use super::*;

/*
pub trait App<Ctx=AppCtx> : Sized
{
    fn event(&mut self, ev: AppEvent, ctx: &mut Ctx) -> Option<AppEvent> { let _ = ctx; Some(ev) }

    fn update(&mut self, dt: DeltaTime, ctx: &mut Ctx) { let _ = (dt, ctx); }
    fn draw(&mut self, ctx: &mut Ctx) { let _ = ctx; }

    fn resumed(&mut self, ctx: &mut Ctx) { let _ = ctx; }
    fn paused(&mut self, ctx: &mut Ctx) { let _ = ctx; }
}

impl<Ctx,S> AppWithEventLoop<Ctx> for S where S: App<Ctx>
{
    fn event(&mut self, ev: AppEvent, ctx: &mut Ctx, el: &AppEventLoop) -> Option<AppEvent> { App::event(self, ev, ctx) }

    fn update(&mut self, dt: DeltaTime, ctx: &mut Ctx, el: &AppEventLoop) { App::update(self, dt, ctx); }

    fn draw(&mut self, ctx: &mut Ctx, el: &AppEventLoop) { App::draw(self, ctx); }

    fn resumed(&mut self, ctx: &mut Ctx, el: &AppEventLoop) { App::resumed(self, ctx); }

    fn paused(&mut self, ctx: &mut Ctx, el: &AppEventLoop) { App::paused(self, ctx); }
}

pub trait App<Ctx=AppCtx> : Sized
{
    fn event(&mut self, ev: AppEvent, ctx: &mut Ctx, el: &AppEventLoop) -> Option<AppEvent> { let _ = (ctx, el); Some(ev) }

    fn update(&mut self, dt: DeltaTime, ctx: &mut Ctx, el: &AppEventLoop) { let _ = (dt, ctx, el); }
    fn draw(&mut self, ctx: &mut Ctx, el: &AppEventLoop) { let _ = (ctx, el); }

    fn resumed(&mut self, ctx: &mut Ctx, el: &AppEventLoop) { let _ = (ctx, el); }
    fn paused(&mut self, ctx: &mut Ctx, el: &AppEventLoop) { let _ = (ctx, el); }
}
*/

pub trait App<Ctx=AppDefaultCtx> : Sized
{
    fn event(&mut self, ev: AppEvent, ctx: &mut AppCtx<Ctx>) -> Option<AppEvent> { let _ = ctx; Some(ev) }

    fn update(&mut self, dt: DeltaTime, ctx: &mut AppCtx<Ctx>) { let _ = (dt, ctx); }
    fn draw(&mut self, ctx: &mut AppCtx<Ctx>) { let _ = ctx; }

    fn resumed(&mut self, ctx: &mut AppCtx<Ctx>) { let _ = ctx; }
    fn paused(&mut self, ctx: &mut AppCtx<Ctx>) { let _ = ctx; }
}

/*
pub trait App<Ctx=AppDefaultCtx> : Sized
{
    fn event(&mut self, ev: AppEvent, ctx: &mut AppCtx<Ctx>) -> Option<AppEvent> { let _ = ctx; Some(ev) }

    fn update(&mut self, dt: DeltaTime, ctx: &mut AppCtx<Ctx>) { let _ = (dt, ctx); }
    fn draw(&mut self, ctx: &mut AppCtx<Ctx>) { let _ = ctx; }

    fn resumed(&mut self, ctx: &mut AppCtx<Ctx>) { let _ = ctx; }
    fn paused(&mut self, ctx: &mut AppCtx<Ctx>) { let _ = ctx; }
}
*/

pub struct AppCtx<'a,'b,Ctx=AppDefaultCtx>
{
    ctx: &'a mut Ctx,
    event_loop: &'a mut AppEventLoop<'b>,
    app_param: &'a AppParam,
    proxy: &'a WinitEventLoopProxy,
}
impl<'a,'b,Ctx> AppCtx<'a,'b,Ctx>
{
    pub(crate) fn new(ctx: &'a mut Ctx, event_loop: &'a mut AppEventLoop<'b>, app_param: &'a AppParam, proxy : &'a WinitEventLoopProxy) -> Self
    {
        Self { ctx, event_loop, app_param, proxy }
    }
    pub fn event_loop(&mut self) -> &mut AppEventLoop<'b> { self.event_loop }
    pub fn context(&mut self) -> &mut Ctx { self.ctx }
    pub fn app_param(&mut self) -> &AppParam { self.app_param }
    // Todo : wrap the proxy type ? 
    pub fn proxy(&mut self) -> &WinitEventLoopProxy { self.proxy }

    pub fn with_ctx<'c,C>(&'c mut self, new_ctx: &'c mut C) -> AppCtx<'c,'b,C> where 'a: 'c
    {
        AppCtx { ctx: new_ctx, event_loop: self.event_loop, app_param: self.app_param, proxy: self.proxy }
    }
}

