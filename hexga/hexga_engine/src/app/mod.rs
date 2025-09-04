use super::*;

pub mod prelude
{
    pub use super::{SpawnFutur,IUserEvent,App,AppRun};
}

pub trait IUserEvent : 'static + Debug + Send {}
impl IUserEvent for () {}

pub trait App : 'static
{
    type UserEvent : IUserEvent;

    fn update(&mut self) {}
    fn draw(&mut self) {}
}

pub trait AppRun
{
    fn run(self) -> Result<(), ()>;
}
impl<A> AppRun for A where A:App
{
    fn run(self) -> Result<(), ()> 
    {
        Ctx::init();
        let ctx = Ctx::try_as_mut().ok_or_void()?;

        let event_loop = EventLoop::with_user_event().build().ok_or_void()?;
        let proxy = event_loop.create_proxy();

        #[allow(unused_mut)]
        let mut runner = AppRunner::new(self, ctx, proxy);

        #[cfg(not(target_arch = "wasm32"))]
        {
            return event_loop.run_app(&mut runner).ok_or_void();
        }
        #[cfg(target_arch = "wasm32")]
        {
            async move { let _ = event_loop.run_app(&mut runner); }.spawn();
            Ok(())
        }
    }
}

pub(crate) enum AppInternalMessage<U> where U: IUserEvent
{
    Message(AppMessage<U>),
    ContextGpu(Result<ContextGpu,String>),
}

pub enum AppMessage<U> where U: IUserEvent
{
    UserEvent(U)
}

pub(crate) type EvLoopProxy<U> = EventLoopProxy<AppInternalMessage<U>>;
pub(crate) type EvLoop<U> = EventLoop<AppMessage<U>> ;

pub(crate) struct AppRunner<A> where A:App
{
    app : A,
    ctx : &'static mut Context,
    proxy : EvLoopProxy<A::UserEvent>,
}
impl<A> AppRunner<A> where A:App
{
    pub fn new(app : A, ctx : &'static mut Context, proxy : EvLoopProxy<A::UserEvent>) -> Self { Self { app, ctx, proxy }}
}

impl<A> ApplicationHandler<AppInternalMessage<A::UserEvent>> for AppRunner<A> where A:App
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) 
    {
        if self.ctx.winit.is_none() 
        {
            #[allow(unused_mut)]
            let mut win_attr = Window::default_attributes().with_title("wgpu winit example");
            
            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowAttributesExtWebSys;
                win_attr = win_attr.with_append(true);
            }


            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("create window err."),
            );
            self.ctx.winit = Some(window.clone());
            ContextGpu::request(window, self.proxy.clone()).unwrap();
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: AppInternalMessage<A::UserEvent>) {
        match event
        {
            AppInternalMessage::Message(app_message) => {},
            AppInternalMessage::ContextGpu(context_wgpu) => 
            {
                Gpu::replace(Some(context_wgpu.unwrap()));
                self.ctx.winit.as_ref().map(|w| w.request_redraw());
            },
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) 
    {
        if !Gpu::is_init() { return; }

        match event 
        {
            WindowEvent::CloseRequested =>  { event_loop.exit(); }
            WindowEvent::Resized(new_size) => {
                if let Some(window) = self.ctx.winit.as_ref()
                {
                    Gpu.resize([new_size.width as _, new_size.height as _].into());
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => { Gpu.draw(); }
            _ => (),
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        Ctx::destroy();
    }
}


// TODO: make an internal private trait, to be sure SpawnFutur can't be impl by external crate

#[cfg(not(target_arch = "wasm32"))]
/// Note : the trait bound vary if you are on wasm32 or not
pub trait SpawnFutur where
    Self: Future<Output = ()> + Send + 'static,
{
    fn spawn(self);
}

#[cfg(not(target_arch = "wasm32"))]
impl<F> SpawnFutur for F where
    F: Future<Output = ()> + Send + 'static,
{
    fn spawn(self)
    {
        async_std::task::spawn(self);
    }
}


#[cfg(target_arch = "wasm32")]
/// Note : the trait bound vary if you are on wasm32 or not
pub trait SpawnFutur where
    Self: Future<Output = ()> + 'static,
{
    fn spawn(self);
}
#[cfg(target_arch = "wasm32")]
impl<F> SpawnFutur for F where
    F: Future<Output = ()> + 'static,
{
    fn spawn(self)
    {
        wasm_bindgen_futures::spawn_local(self);
    }
}