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
        /* 
        let ctx = ctx_mut();
        let Some(wgpu_ctx) = ctx.gfx.as_mut() else { return; };
        
        assert!(ctx.pen.encoder.is_none());
        assert!(ctx.pen.rpass.is_none());

        let surface_texture = wgpu_ctx
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = wgpu_ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

        rpass.set_pipeline(&wgpu_ctx.render_pipeline);

        rpass.set_vertex_buffer(0, wgpu_ctx.vertex_buffer.slice(..));
            // 消费存放的 vertex_index_buffer
            rpass.set_index_buffer(
                wgpu_ctx.vertex_index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            ); // 1.

        rpass.draw_indexed(0..VERTEX_INDEX_LIST.len() as u32, 0, 0..1);
        rpass.draw(0..VERTEX_LIST.len() as u32, 0..1);
        
        ctx.pen.encoder = Some(encoder);
        ctx.pen.rpass = Some(rpass);

        self.app.draw();

        let ctx = ctx_mut();

        wgpu_ctx.queue.submit(Some(ctx.pen.encoder.take().unwrap().finish()));
        ctx.pen.rpass = None;
        surface_texture.present();
        */
    }
}