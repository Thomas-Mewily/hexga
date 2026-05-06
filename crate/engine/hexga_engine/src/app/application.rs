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
    pub(crate) ctx: &'a mut Ctx,
    pub(crate) event_loop: &'a mut AppEventLoop<'b>,
}
impl<'a,'b,Ctx> AppCtx<'a,'b,Ctx>
{
    pub(crate) fn new(ctx: &'a mut Ctx, event_loop: &'a mut AppEventLoop<'b>) -> Self
    {
        Self { ctx, event_loop }
    }
}