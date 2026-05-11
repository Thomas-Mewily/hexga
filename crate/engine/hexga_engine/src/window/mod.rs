use crate::app::AppInternalProxy;

use super::*;

mod main_window;
use hexga_event_loop::event_loop::EventLoopProxy;
use hexga_graphics::gpu::{GpuInstance, GpuInstanceDescriptor};
pub use main_window::*;

pub(crate) type WindowType = hexga_event_loop::window::Window<GpuSurface<'static>>;

pub(crate) trait WindowInitGpu
{
    fn init_gpu_if_needed(&self, param: &GpuParam, event_loop: &AppInternalEventLoop) -> GpuResult<Option<GpuSurface>>;
}

impl WindowInitGpu for WindowType
{
    fn init_gpu_if_needed(&self, param: &GpuParam, event_loop: &AppInternalEventLoop) -> GpuResult<Option<GpuSurface>>
    {
        if Gpu::is_init() { return Ok(None); }

        let size = self.size().max(one());

        let instance = GpuInstance::new(&param.instance);

        let surface = instance.wgpu.create_surface(self.winit_window())?.map(|v| v.into());

        async_init_gpu_in_proxy(instance, param.clone(), surface, event_loop.proxy().clone()).spawn();

        self.

        Ok(surface)
    }
}


pub(crate) async fn async_init_gpu_in_proxy(instance: GpuInstance, param: GpuParam, surface: Option<GpuSurface<'static>>, proxy: AppInternalProxy)
{
    let _ = match async_init_gpu(instance, param, surface).await
    {
        Ok(o) => proxy.send_event(PlatformEvent::Custom(AppCustomEvent::GpuReady)),
        Err(e) => proxy.send_event(PlatformEvent::Custom(AppCustomEvent::GpuError(e))),
    };
}

pub(crate) async fn async_init_gpu(instance: GpuInstance, param: GpuParam, surface: Option<GpuSurface<'static>>) -> GpuResult
{
    let adapter = instance
        .wgpu
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: param.power_preference.into(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: surface.as_ref().map(|s| &s.wgpu),
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

    Ok(())
}

pub mod prelude
{
    pub(crate) use super::WindowType;
}