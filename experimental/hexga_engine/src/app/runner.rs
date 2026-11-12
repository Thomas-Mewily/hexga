//use crate::window::WindowInitGpu;
use super::*;

//pub type AppResult = AppResult;
//pub type AppError = AppError;

// Todo wrap app run for the engine

pub trait AppInit<A>: Fn() -> A + Async {}
impl<S, A> AppInit<A> for S where S: Fn() -> A + Async {}

pub trait AppRun: for<'a> App<(), AppCtx<'a>> + Sized + 'static //where CustomEvent: IAppEvent,
{
    fn run<F>(init: F) -> AppResult
    where
        F: AppInit<Self>,
    {
        Self::run_with_param(init, ___())
    }

    fn run_with_param<F>(init: F, param: AppParam) -> AppResult
    where
        F: AppInit<Self>,
    {
        let p = param.event_loop.clone();
        AppRunner::<F, Self>::run_raw_with_param(|proxy| AppRunner { init, app: None, param, proxy }, p)
        /*
        AppRunner
        {
            init,
            app: None,
            param,
            proxy: todo!(),
        }
        */
    }
}
impl<S> AppRun for S where S: for<'a> App<(), AppCtx<'a>> + Sized + 'static {} //, CustomEvent: IAppEvent {}

pub type AppCtx<'a> = ();

pub(crate) struct AppRunner<F, A>
where
    F: AppInit<A>,
    A: for<'a> App<(), AppCtx<'a>>,
{
    init: F,
    app: Option<A>,
    param: AppParam,
    proxy: AppInternalProxy,
}

impl<F, A> AppRunner<F, A>
where
    F: AppInit<A>,
    A: for<'a> App<(), AppCtx<'a>>,
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
            Ok(mut w) =>
            {
                if w.surface().is_some()
                {
                    let mut app = (self.init)();
                    app.message(AppMessage::Event(AppEvent::WindowOpen), &mut ());
                    app.message(AppMessage::Flow(AppFlow::Resume), &mut ());
                    //app.resume()
                    self.app = Some(app);
                    w.request_draw();
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
    A: for<'a> App<(), AppCtx<'a>>,
{
    fn drop(&mut self) { self.exit(); }
}

impl<'a, F, A> App<AppCustomEvent, AppInternalEventLoop<'a>> for AppRunner<F, A>
where
    F: AppInit<A>,
    A: App<(), AppCtx<'a>>,
{
    fn update(&mut self, dt: Duration, el: &mut AppInternalEventLoop)
    {
        if let Some(app) = &mut self.app {
            app.message(AppMessage::Flow(AppFlow::Update(dt)), &mut ());
        }
    }

    fn draw(&mut self, event_loop: &mut AppInternalEventLoop)
    {
        if let Some(app) = &mut self.app {
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
            */
            /*
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

            app.message(AppMessage::Flow(AppFlow::Draw), &mut ());
            */

            //app.draw(1., &mut ());

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

            app.message(AppMessage::Flow(AppFlow::Draw), &mut ());

            {
                let Ok(mut graphics) = GRAPHICS.try_get_mut()
                else
                {
                    return;
                };
                graphics.end_draw();
            }
        }
    }

    fn message(&mut self, ev: AppInternalMessage, ctx: &mut AppInternalEventLoop) -> Option<AppInternalMessage>
    {
        match ev
        {
            AppMessage::Custom(e) => match e
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

                    self.init_stuff_if_needed(ctx);

                    None
                }
                AppCustomEvent::GpuReady(gpu) =>
                {
                    hexga_graphics::gpu::experimental::GPU.init(gpu).ok_or_void().expect("Can't init the gpu");
                    assert!(Gpu::is_init());

                    self.init_stuff_if_needed(ctx);

                    None
                }
                AppCustomEvent::GpuError(gpu_error) => panic!("Can't init the gpu"),
            },
            s => self.dispatch_message(s, ctx),
        }

        /*
        let e: AppMessage = match ev
        {
            AppMessage::Event(ev) =>
            {
                match &ev
                {
                    AppEvent::Resize(size) =>
                    {
                        WINDOW.try_get_mut().map(|mut w| w.configure_surface());
                    }
                    _ =>
                    {}
                }
                ev.into()
            }
            AppMessage::Custom(_) => return None,
            AppMessage::Flow(flow) =>
            {
                flow.into()
            },
        };

        match &mut self.app
        {
            Some(app) => app.message(e, &mut ()).map(|e| e.remove_custom()).flatten(),
            None => None,
        }
        */
    }

    fn event(&mut self, ev: AppEvent, ctx: &mut AppInternalEventLoop<'a>) -> Option<AppEvent>
    {
        if let AppEvent::WindowResize(size) = &ev {
            if let Ok(mut w) = WINDOW.try_get_mut() { w.configure_surface() }
        };

        let Some(app) = &mut self.app
        else
        {
            return None;
        };
        let Some(ev) = app.message(ev.into(), &mut ())
        else
        {
            return None;
        };
        match ev
        {
            AppMessage::Event(ev) => Some(ev),
            _ =>
            {
                #[cfg(debug_assertions)]
                {
                    // probably a logic bug, an AppEvent should not return anythings else than another AppEvent (if the passed event was consumed)
                    panic!("Invalid event {:?}", ev);
                }
                None
            }
        }
    }

    fn resume(&mut self, event_loop: &mut AppInternalEventLoop)
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

        if let Some(app) = &mut self.app
        {
            app.message(AppMessage::Flow(AppFlow::Resume), &mut ());
        }
    }

    fn pause(&mut self, event_loop: &mut EventLoop<AppCustomEvent>)
    {
        if let Some(app) = &mut self.app
        {
            app.message(AppMessage::Flow(AppFlow::Pause), &mut ());
        }

        if let Ok(mut w) = WINDOW.try_get_mut() { w.destroy_surface(); }
    }

    fn exit(&mut self, event_loop: &mut AppInternalEventLoop)
    {
        if let Some(app) = &mut self.app
        {
            app.message(AppMessage::Flow(AppFlow::Exit), &mut ());
        }

        self.exit();
    }
}
