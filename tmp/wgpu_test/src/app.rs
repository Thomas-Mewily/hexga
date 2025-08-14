use winit::event::WindowEvent;

use crate::*;



pub trait App
{
    fn draw(&self);


    fn run(self) where Self: Sized
    {
        AppRunner.run(self);
    }   
}

pub struct AppRunner;

pub trait IAppRunner
{
    fn run<A:App>(&mut self, app:A);
}
impl IAppRunner for AppRunner
{
    fn run<A:App>(&mut self, app:A) 
    {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let mut runner = CtxRunner::new(app);
        event_loop.run_app(&mut runner).unwrap();
    }
}


pub struct CtxRunner<A> where A:App
{
    app : A,
}
impl<A> CtxRunner<A> where A:App
{
    pub fn new(app : A) -> Self 
    {
        init_ctx_if_needed();
        Self { app }
    }
}
impl<A> Deref for CtxRunner<A> where A:App
{
    type Target=Context;
    fn deref(&self) -> &Self::Target { ctx() }
}
impl<A> DerefMut for CtxRunner<A> where A:App
{
    fn deref_mut(&mut self) -> &mut Self::Target { ctx_mut() }
}


impl<A> winit::application::ApplicationHandler for CtxRunner<A> where A:App
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let win_attr = Window::default_attributes().with_title("wgpu winit example");
            // use Arc.
            let window = Arc::new(
                event_loop
                    .create_window(win_attr)
                    .expect("create window err."),
            );
            self.window = Some(window.clone());
            let gfx = WgpuCtx::new(window.clone());
            self.gfx = Some(gfx);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                // macOS err: https://github.com/rust-windowing/winit/issues/3668
                // This will be fixed as winit 0.30.1.
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let (Some(wgpu_ctx), Some(window)) =
                    (ctx_mut().gfx.as_mut(), ctx().window.as_ref())
                {
                    wgpu_ctx.resize((new_size.width, new_size.height));
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                self.draw();
            }
            _ => (),
        }
    }
}

impl<A> CtxRunner<A> where A:App
{
    fn draw(&mut self)
    {
        let ctx = ctx_mut();
        let Some(wgpu_ctx) = ctx.gfx.as_mut() else { return; };
        assert!(ctx.encoder.is_none());

        let surface_texture = wgpu_ctx
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let _texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = wgpu_ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        ctx.encoder = Some(encoder);

        self.app.draw();

        let ctx = ctx_mut();

        wgpu_ctx.queue.submit(Some(ctx.encoder.take().unwrap().finish()));
        surface_texture.present();
    }
}