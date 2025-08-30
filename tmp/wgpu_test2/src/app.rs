use super::*;


pub trait IUserEvent : 'static
{

}
impl IUserEvent for () {}

pub trait App 
{
    type UserEvent : IUserEvent;
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

        let event_loop = EventLoop::with_user_event().build().ok_or_void()?;

        let proxy = event_loop.create_proxy();

        let ctx = unsafe { Ctx::try_as_mut().ok_or_void() }?;
        event_loop.run_app(&mut AppRunner::new(self, ctx, proxy)).ok_or_void()
    }
}

pub enum AppMessage<U> where U: IUserEvent
{
    UserEvent(U)
}

pub type EvLoopProxy<U> = EventLoopProxy<AppMessage<U>>;
pub type EvLoop<U> = EventLoop<AppMessage<U>> ;

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

impl<A> ApplicationHandler<AppMessage<A::UserEvent>> for AppRunner<A> where A:App
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) 
    {
        if self.ctx.winit.is_none() {
            let win_attr = Window::default_attributes().with_title("wgpu winit example");
            // use Arc.
            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("create window err."),
            );
            self.ctx.winit = Some(window.clone());
            // Todo: remove pollster
            self.ctx.wgpu = Some(pollster::block_on(ContextWgpu::new(window)));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) 
    {
        match event 
        {
            WindowEvent::CloseRequested =>  { event_loop.exit(); }
            WindowEvent::Resized(new_size) => {
                if let (Some(wgpu_ctx), Some(window)) =
                    (self.ctx.wgpu.as_mut(), self.ctx.winit.as_ref())
                {
                    wgpu_ctx.resize([new_size.width as _, new_size.height as _].into());
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(wgpu_ctx) = self.ctx.wgpu.as_mut() 
                {
                    wgpu_ctx.draw();
                }
            }
            _ => (),
        }
    }
}
