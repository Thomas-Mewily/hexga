use super::*;
pub use hexga_graphics::*;

pub mod prelude
{
    pub(crate) use super::{AppGraphics, GpuMessage, wgpu};
    pub use hexga_graphics::prelude::*;
}


#[derive(Debug)]
pub struct AppGraphics 
{

}

impl AppGraphics
{
    pub(crate) fn new() -> Self { Self {} }

    pub(crate) async fn init_gpu(
        instance: gpu::GpuInstance,
        surface: Option<graphics::GpuSurface<'static>>,
        compatible_surface: Option<wgpu::SurfaceTarget<'static>>,
        window: WinitWindowShared,
        mut param: GpuParam,
    ) -> GpuResult<Self>
    {
        let gpu_init = GpuInit::from_instance_and_compatible_surface(instance, surface, compatible_surface, param).await?;
        let output = Gpu::from_init(gpu_init).await?;

        Ok(Self::new())
    }

    pub(crate) async fn async_init_gpu(
        instance: gpu::GpuInstance,
        surface: Option<graphics::GpuSurface<'static>>,
        compatible_surface: Option<wgpu::SurfaceTarget<'static>>,
        window: WinitWindowShared,
        param: GpuParam,
        proxy: WinitEventLoopProxy,
    )
    {
        let _ = proxy.send_event(AppInternalEvent::Gpu(
            Self::init_gpu(instance, surface, compatible_surface, window, param).await,
        ));
    }

    pub(crate) fn init(
        window: WinitWindowShared,
        mut param: GpuParam,
        mut compatible_surface: Option<wgpu::SurfaceTarget<'static>>,
        proxy: WinitEventLoopProxy,
    ) -> GpuResult
    {
        let surface_size: Point2 = window.inner_size().convert();
        let surface_size = surface_size.max(one());

        if compatible_surface.is_none()
        {
            compatible_surface = Some(window.clone().into());
        }

        let instance = gpu::GpuInstance::new(&param.instance);
        let surface = Some(
            instance
                .wgpu
                .create_surface(compatible_surface.take().expect("missing surface"))?
                .into(),
        );

        Self::async_init_gpu(instance, surface, compatible_surface, window, param, proxy).spawn();
        Ok(())
    }
}


pub(crate) type GpuMessage = GpuResult<AppGraphics>;
