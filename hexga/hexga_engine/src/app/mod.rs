use crate::*;

pub mod prelude
{
    pub use super::{App,AppRun,IInput,IUserEvent};
}

pub trait IInput
{

}

pub trait IUserEvent : 'static + Debug + Send {}
impl IUserEvent for () {}

pub trait App<UserEvent=()>
{
    fn event(&mut self) {}
    fn update(&mut self) {}
    fn draw(&mut self) {}
}

impl<T, UserEvent> AppRun<UserEvent> for T where T: App<UserEvent>, UserEvent: IUserEvent {}
pub trait AppRun<UserEvent> : App<UserEvent> where UserEvent: IUserEvent, Self: Sized
{
    fn run(self) { self.run_with_param(___()); }

    fn run_with_param(self, mut param: AppParam)
    {
        init_ctx_if_needed();
        let ctx = ctx_mut();

        ctx.windows.init_main_window(std::mem::take(&mut param.window));

        let event_loop = winit::event_loop::EventLoop::<AppInternalEvent<UserEvent>>::with_user_event().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let proxy = event_loop.create_proxy();
        //ctx.windows
        //ctx.window.init_main_window(std::mem::take(&mut param.window));

    }
}

pub(crate) struct AppRunner<A,UserEvent=()> where A: App<UserEvent>, UserEvent:IUserEvent
{
    app : A,
    proxy : WinitEventLoopProxy<AppInternalEvent<UserEvent>>,
}

impl <A,UserEvent> AppRunner<A,UserEvent> where A:App<UserEvent>, UserEvent:IUserEvent
{
    fn new(app : A, proxy : WinitEventLoopProxy<AppInternalEvent<UserEvent>>) -> Self { Self { app, proxy }}

    fn run(self, event_loop : WinitEventLoop<AppInternalEvent<UserEvent>>) { self.run_with_param(event_loop, ___()) }
    fn run_with_param(mut self, event_loop : WinitEventLoop<AppInternalEvent<UserEvent>>, _param : AppParam)
    {
        init_logger_if_needed();

        #[cfg(target_arch = "wasm32")]
        {
            // Runs the app async via the browsers event loop
            use winit::platform::web::EventLoopExtWebSys;
            wasm_bindgen_futures::spawn_local(async move {
                event_loop.spawn_app(app);
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Runs the app on the current thread.
            let _ = event_loop.run_app(&mut self);
        }
    }

    fn send_event(&mut self, event: impl Into<AppInternalEvent<UserEvent>>)
    {
        let r = self.proxy.send_event(event.into());
        assert!(r.is_ok(), "Failed to send event");
    }
    fn send_window_event(&mut self, id : WindowID, event: impl Into<WindowEventKind>)
    {
        self.send_event(WindowEvent::new(id, event.into()));
    }
    fn send_internal_window_event(&mut self, id : WindowID, event: impl Into<WindowInternalEventKind>)
    {
        self.send_event(WindowInternalEvent::new(id, event.into()));
    }
}



#[derive(Debug)]
pub(crate) enum AppInternalEvent<UserEvent=()> where UserEvent: IUserEvent
{
    UserEvent(UserEvent),
    Window(WindowEvent),
    WindowInternal(WindowInternalEvent),
}

impl<U> From<WindowInternalEvent> for AppInternalEvent<U> where U: IUserEvent { fn from(value: WindowInternalEvent) -> Self { Self::WindowInternal(value) } }
impl<U> From<WindowEvent> for AppInternalEvent<U> where U: IUserEvent { fn from(value: WindowEvent) -> Self { Self::Window(value) } }

#[non_exhaustive]
pub enum AppEvent<UserEvent=()> where UserEvent: IUserEvent
{
    UserEvent(UserEvent),
    Window(WindowEvent),
}
impl<UserEvent> Debug for AppEvent<UserEvent> where UserEvent: IUserEvent
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self
        {
            Self::UserEvent(v) => write!(f, "{:?}", v),
            Self::Window(v) => write!(f, "{:?}", v),
        }
    }
}
impl<U> From<WindowEvent> for AppEvent<U> where U: IUserEvent { fn from(value: WindowEvent) -> Self { Self::Window(value) } }


pub struct AppParam
{
    pub window : Option<WindowParam>,
}
impl Default for AppParam
{
    fn default() -> Self {
        Self { window: Some(WindowParam::new()) }
    }
}

impl AppParam
{
    pub fn new() -> Self { ___() }

    fn with_window_mut_or_create(mut self, f: impl for<'a> FnOnce(WindowParam) -> WindowParam) -> Self
    {
        self.window.get_or_insert_with(WindowParam::new);
        self.window = Some(f(self.window.unwrap()));
        self
    }

    pub fn with_window(mut self, window : impl Into<Option<WindowParam>>) -> Self  { self.window = window.into(); self }
}

impl IWindowParam for AppParam
{
    fn with_title(self, title: impl Into<String>) -> Self {
        self.with_window_mut_or_create(|w| w.with_title(title))
    }

    fn with_size(self, size: impl Into<Option<Point2>>) -> Self {
        self.with_window_mut_or_create(|w| w.with_size(size))
    }

    fn with_position(self, position: impl Into<Option<Point2>>) -> Self {
        self.with_window_mut_or_create(|w| w.with_position(position))
    }
}