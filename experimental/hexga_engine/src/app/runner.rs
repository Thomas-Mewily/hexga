//use crate::window::WindowInitGpu;
use hexga_event_loop::event_loop::EventLoopResult;

use super::*;

pub type AppResult = EventLoopResult;
pub type AppError = EventLoopError;

pub trait AppInit<A>: Fn() -> A + Async {}
impl<S, A> AppInit<A> for S where S: Fn() -> A + Async {}

pub(crate) struct AppRunner<F, A>
where
    F: AppInit<A>,
    A: App,
{
    init: F,
    app: Option<A>,
    param: AppParam,
    proxy: AppInternalProxy,
}

impl<F, A> AppRunner<F, A>
where
    F: AppInit<A>,
    A: App,
{
    fn exit(&mut self)
    {
        WINDOW.reset();
        self.app = None;
    }

    fn init_stuff_if_needed(&mut self, event_loop: &mut AppInternalEventLoop)
    {
        self.init_graphics_if_needed();
        self.init_app_if_needed(event_loop);
    }

    fn init_graphics_if_needed(&mut self)
    {
        if CurrentWindow::is_not_init() || Gpu::is_not_init()
        {
            return;
        }

        if GRAPHICS.try_get_mut().is_err()
        {
            let mut w = WINDOW.get_mut();
            if let Some(surface) = w.surface()
            {
                GRAPHICS
                    .init_from_fn(|| Graphics::new(surface.surface(), w.size()))
                    .ok_or_void()
                    .expect("Can't init the graphics");
            }
        }
    }

    fn init_app_if_needed(&mut self, event_loop: &mut AppInternalEventLoop)
    {
        if self.app.is_some() || Gpu::is_not_init()
        {
            return;
        }

        match WINDOW.try_get_mut()
        {
            Ok(w) =>
            {
                if w.surface().is_some()
                {
                    let mut app = (self.init)();
                    //app.resume()
                    self.app = Some(app);
                }
            }
            Err(e) =>
            {}
        }
    }
}

impl<F, A> Drop for AppRunner<F, A>
where
    F: AppInit<A>,
    A: App,
{
    fn drop(&mut self) { self.exit(); }
}

impl<F, A> PlatformEventHandler<AppCustomEvent> for AppRunner<F, A>
where
    F: AppInit<A>,
    A: App,
{
    fn update(&mut self, dt: Duration, event_loop: &mut AppInternalEventLoop)
    {
        match &mut self.app
        {
            Some(app) => app.update(dt, &mut ()),
            None =>
            {}
        }
    }

    fn draw(&mut self, event_loop: &mut AppInternalEventLoop)
    {
        match &mut self.app
        {
            Some(app) =>
            {
                /*
                let Ok(mut graphics) = GRAPHICS.try_get_mut()
                else
                {
                    return;
                };
                let Ok(mut window) = WINDOW.try_get_mut()
                else
                {
                    return;
                };
                let Some(surface) = window.surface()
                else
                {
                    return;
                };


                let surface = surface.surface();
                let output = match surface.wgpu.get_current_texture()
                {
                    Ok(s) => s,
                    Err(_) => return,
                };

                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder =
                    Gpu.device()
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.9,
                                    g: 0.9,
                                    b: 0.2,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        occlusion_query_set: None,
                        timestamp_writes: None,
                    });

                    render_pass.set_pipeline(&graphics.pipeline);
                    render_pass.draw(0..3, 0..1);
                }

                Gpu.queue().submit(iter::once(encoder.finish()));
                output.present();

                app.draw(1., &mut ());
                */

                {
                    let Ok(mut window) = WINDOW.try_get_mut()
                    else
                    {
                        return;
                    };

                    let Some(surface) = window.surface()
                    else
                    {
                        return;
                    };
                }

                {
                    let Ok(mut graphics) = GRAPHICS.try_get_mut()
                    else
                    {
                        return;
                    };
                    graphics.begin_draw();
                }
                app.draw(1., &mut ());

                {
                    let Ok(mut graphics) = GRAPHICS.try_get_mut()
                    else
                    {
                        return;
                    };
                    graphics.end_draw();
                }
            }
            None =>
            {}
        }
    }

    fn event(&mut self, ev: AppInternalEvent, event_loop: &mut AppInternalEventLoop) -> Option<AppInternalEvent>
    {
        let (ev, app_internal) = ev.replace_custom_event(|| ());

        match app_internal
        {
            Some(e) => match e
            {
                AppCustomEvent::SurfaceReady(surface) =>
                {
                    let mut window = match WINDOW.try_get_mut()
                    {
                        Ok(w) => w,
                        Err(_) =>
                        {
                            return None;
                        }
                    };
                    let size = window.size();
                    window.replace_surface(Some(GpuSurfaceConfigured::from_surface(surface, size)));
                    window.configure_surface();
                    window.request_draw();
                    drop(window);

                    self.init_stuff_if_needed(event_loop);

                    return None;
                }
                AppCustomEvent::GpuReady(gpu) =>
                {
                    hexga_graphics::gpu::experimental::GPU.init(gpu).ok_or_void().expect("Can't init the gpu");
                    assert!(Gpu::is_init());

                    self.init_stuff_if_needed(event_loop);

                    return None;
                }
                AppCustomEvent::GpuError(gpu_error) => panic!("Can't init the gpu"),
            },
            None =>
            {}
        }

        match &ev
        {
            PlatformEvent::Resize(size) =>
            {
                WINDOW.try_get_mut().map(|mut w| w.configure_surface());
            }
            _ =>
            {}
        }

        match &mut self.app
        {
            Some(app) => app.event(ev, &mut ()).map(|ev| ev.replace_custom_event(|| app_internal.unwrap()).0),
            None => None,
        }
    }

    fn resumed(&mut self, event_loop: &mut AppInternalEventLoop)
    {
        let mut created = false;
        let mut window = WINDOW
            .init_from_fn(|| {
                created = true;
                event_loop.create_window(self.param.window.clone()).expect("failed to create main window")
            })
            .ok_or_void()
            .expect("can't init the main window");

        if created || window.surface().is_none()
        {
            window.initialize_surface(&self.param.gpu, event_loop).expect("failed to init the surface");
        }
    }

    fn paused(&mut self, event_loop: &mut EventLoop<AppCustomEvent>)
    {
        WINDOW.try_get_mut().map(|mut w| {
            w.destroy_surface();
        });
    }

    fn exit(&mut self, event_loop: &mut AppInternalEventLoop) { self.exit(); }
}

pub trait AppRun: Sized
{
    fn run(self) -> AppResult { self.run_with_param(___()) }
    fn run_with_param(self, param: AppParam) -> AppResult;
}

impl<F, A> AppRun for F
where
    F: AppInit<A> + Fn() -> A,
    A: App,
{
    fn run_with_param(self, param: AppParam) -> AppResult
    {
        let event_loop_param = param.event_loop.clone();

        event_loop::event_loop::run_with_param(
            |proxy| AppRunner {
                app: None,
                init: self,
                param,
                proxy,
            },
            event_loop_param,
        )
    }
}
