use super::*;

pub trait App<User=UserEvent,Ctx=AppCtx> : Sized
    where User: AppUserEvent
{
    fn tick(&mut self, dt: DeltaTime, l: &mut AppLoop<User>, ctx: &mut Ctx) 
    {
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
    }

    fn update(&mut self, dt: DeltaTime, l: &mut AppLoop<User>, ctx: &mut Ctx) { let _ = (dt, ctx); }
    fn draw(&mut self, l: &mut AppLoop<User>, ctx: &mut Ctx) { let _ = ctx; }

    fn resumed(&mut self, l: &mut AppLoop<User>, ctx: &mut Ctx) { let _ = ctx; }
    fn paused(&mut self, l: &mut AppLoop<User>, ctx: &mut Ctx) { let _ = ctx; }

    fn exit(&mut self, l: &mut AppLoop<User>, ctx: &mut Ctx) { let _ = ctx; }

    fn event(&mut self, ev: AppEvent<User>, l: &mut AppLoop<User>, ctx: &mut Ctx) -> Option<AppEvent<User>> 
    {
        match ev
        {
            AppEvent::Input(input) => self.input_event(input, l, ctx).map(AppEvent::Input),
            AppEvent::Window(window) => self.window_event(window, l, ctx).map(AppEvent::Window),
            AppEvent::User(user) => self.user_event(user, l, ctx).map(AppEvent::User),
        }
    }

    fn user_event(&mut self, ev: User, l: &mut AppLoop<User>, ctx: &mut Ctx) -> Option<User> { let _ = ctx; Some(ev) }
    fn window_event(&mut self, ev: WindowEvent, l: &mut AppLoop<User>, ctx: &mut Ctx) -> Option<WindowEvent> { let _ = ctx; Some(ev) }
    fn input_event(&mut self, ev: InputEvent, l: &mut AppLoop<User>, ctx: &mut Ctx) -> Option<InputEvent> { let _ = ctx; Some(ev) }
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

pub type ProxyResult<T=()> = Result<T,()>;

impl<User,E> AppSendEvent<E> for AppProxy<User> where
    User: From<E> + AppUserEvent,
{
    fn send_event(&mut self, ev: E) -> ProxyResult {
        match self.winit.send_event(ev.into())
        {
            Ok(_) => Ok(()),
            Err(e) => Err(()),
        }
    }
}
pub trait AppSendEvent<E>
{
    fn send_event(&mut self, ev: E) -> ProxyResult;
}

impl<User> AppProxy<User>
    where User: AppUserEvent
{
    pub(crate) fn new(winit: WinitEventLoopProxy<User>) -> Self { Self { winit }}
    pub fn winit(&self) -> &WinitEventLoopProxy<User> { &self.winit }
}

