use crate::app::AppInternalProxy;

use super::*;

mod main_window;
use hexga_event_loop::event_loop::EventLoopProxy;
use hexga_graphics::gpu::{GpuContext, GpuInstance, GpuInstanceDescriptor, WgpuContext};
pub use main_window::*;

pub(crate) type WindowType = hexga_event_loop::window::Window<GpuConfiguredSurface<'static>>;

pub(crate) trait WindowInitGpu
{
    fn initialize_gpu_andsurface(&mut self, param: &GpuParam, event_loop: &AppInternalEventLoop) -> GpuResult;
    fn configure_surface(&mut self);
}

impl WindowInitGpu for WindowType
{
    fn initialize_gpu_andsurface(&mut self, param: &GpuParam, event_loop: &AppInternalEventLoop) -> GpuResult
    {
        if Gpu::is_init() { return Ok(()); }

        let size = self.size().max(one());

        let instance = GpuInstance::new(&param.instance);

        let surface = instance.wgpu.create_surface(self.winit_window())?.into();

        async_init_gpu_in_proxy(instance, param.clone(), surface, event_loop.proxy().clone()).spawn();

        Ok(())
    }

    fn configure_surface(&mut self)
    {
        if let Some(mut surface) = self.replace_surface(None)
        {
            let size = self.size().max(one());
            if surface.size() != size 
            {
                surface.resize(size);
            }
            self.replace_surface(Some(surface));
        }
    }
}


pub(crate) async fn async_init_gpu_in_proxy(instance: GpuInstance, param: GpuParam, surface: GpuSurface<'static>, proxy: AppInternalProxy)
{
    let _ = match async_init_gpu(instance, param, surface).await
    {
        Ok((surface, gpu)) => proxy.send_event(PlatformEvent::Custom(AppCustomEvent::GpuReady{ surface, gpu })),
        Err(e) => proxy.send_event(PlatformEvent::Custom(AppCustomEvent::GpuError(e))),
    };
}

pub(crate) async fn async_init_gpu(instance: GpuInstance, param: GpuParam, surface: GpuSurface<'static>) -> GpuResult<(GpuSurface<'static>, GpuContext)>
{
    let compatible_surface = Some(&surface.wgpu);

    let adapter = instance
        .wgpu
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: param.power_preference.into(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface
        })
        .await?;

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            // WebGL doesn't support all of wgpu's features, so if
            // we're building for the web we'll have to disable some.
            required_limits: if cfg!(target_arch = "wasm32")
            {
                wgpu::Limits::downlevel_webgl2_defaults()
            }
            else
            {
                wgpu::Limits::default()
            },
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        })
        .await?;

    let gpu = GpuContext
    {
        wgpu: WgpuContext
        {
            instance,
            adapter,
            device,
            queue,
        },
    };

    Ok((surface, gpu))
}

pub mod prelude
{
    pub(crate) use super::WindowType;
    pub use super::MainWindow;
}