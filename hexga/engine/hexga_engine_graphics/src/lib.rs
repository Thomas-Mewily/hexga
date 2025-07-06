#![allow(dead_code)]
#![allow(unused_imports)]

// Thank to https://github.com/w4ngzhen/wgpu_winit_example


use core::error;
use std::marker::PhantomData;

use hexga_core::prelude::*;
use hexga_generational::prelude::{GenVec, GenVecID};
use hexga_math::prelude::Point2;
use wgpu::{ShaderSource, Trace, MemoryHints};
use hexga_engine_window::{prelude::*, window::{EventLoopProxy, WinitConvert, WinitWindowPtr}};

pub mod prelude
{
    pub use super::{Graphics, GraphicsParam, IGraphicsParam, SurfaceCreated, SurfaceID, GraphicsEvent};
}

#[derive(Debug, Default)]
pub struct Graphics
{
    param: GraphicsParam,
    surfaces: GenVec<SurfaceResult>,
}
impl Graphics
{
    pub fn new_with_param(param : GraphicsParam) -> Self { Self { param, surfaces: GenVec::new() } }
    pub fn new() -> Self { Self::new_with_param(___()) }


    pub fn handle_event(&mut self, event: GraphicsEvent)
    {
        match event
        {
            GraphicsEvent::SurfaceCreated(mut surface_created) =>
            {
                let id = surface_created.surface.as_ref().unwrap().id;
                match id.is_null()
                {
                    true =>
                    {
                        let id = self.surfaces.insert(surface_created.surface);
                        self.surfaces[id].as_mut().unwrap().id = id;
                    },
                    false =>
                    {
                        let surface = self.surfaces.get_mut(id).unwrap();
                        std::mem::swap(surface, &mut surface_created.surface);
                    },
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct GraphicsParam
{
    pub(crate) instance: wgpu::Instance,
}
impl GraphicsParam
{
    pub fn new() -> Self { ___() }
    pub fn wgpu_instance(&self) -> &wgpu::Instance { &self.instance }
    pub fn wgpu_instance_mut(&mut self) -> &mut wgpu::Instance { &mut self.instance }
}

pub trait IGraphicsParam : Default
{

}
impl IGraphicsParam for GraphicsParam
{

}

pub type WindowGraphicsData = SurfaceID;


#[derive(Debug)]
pub struct Surface
{
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) id : SurfaceID,
}

#[derive(Debug)]
pub struct SurfaceCreated
{
    pub window  : WinitWindowPtr,
    pub surface : SurfaceResult,
}

pub enum GraphicsEvent
{
    SurfaceCreated(SurfaceCreated),
}

pub type SurfaceResult = Result<Surface, String>;
impl SurfaceCreated
{
    pub(crate) fn new(window: WinitWindowPtr, surface : SurfaceResult) -> Self
    {
        Self
        {
            window,
            surface,
        }
    }
}

impl SurfaceCreated
{
    pub fn pause(&mut self)
    {
        self.surface = Err(String::new());
    }
}

// Use an Asset instead ?
pub type SurfaceID = GenVecID<SurfaceResult>;

pub struct WindowSurface<T=()>
{
    data : T,
}

impl Graphics
{
    pub async fn new_surface_async_and_send_it(param : GraphicsParam, window : WinitWindowPtr, proxy: EventLoopProxy<GraphicsEvent>)
    {
        let _ = proxy.send_event(GraphicsEvent::SurfaceCreated(SurfaceCreated::new(window.clone(), Self::new_surface_async(param, window).await)));
    }
    pub async fn new_surface_async(param : GraphicsParam, window : WinitWindowPtr) -> SurfaceResult
    {
        let instance = param.instance;
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions
                {
                    power_preference: wgpu::PowerPreference::default(),
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                }
            )
            .await
            .map_err(|e| format!("Failed to find an appropriate adapter: {e}"))?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor
            {
                label: None,
                required_features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: MemoryHints::Performance,
                trace: Trace::Off,
            })
            .await
            .map_err(|e|  format!("Failed to create device: {e}"))?;


        let mut size : Point2 = window.inner_size().convert();
        size.iter_mut().for_each(|v| { *v = (*v).max(1); });
        let surface_config = surface.get_default_config(&adapter, size.x as _, size.y as _).unwrap();

        #[cfg(not(target_arch = "wasm32"))]
        surface.configure(&device, &surface_config);

        Ok
        (
            Surface
            {
                adapter,
                surface,
                surface_config,
                device,
                queue,
                id: ___(),
            }
        )
    }


    pub fn pause(&mut self)
    {
        for (_, s) in self.surfaces.iter_mut()
        {
            *s = Err(String::new());
        }
    }

    pub fn resume(&mut self, ctx: &mut WindowCtx<WindowGraphicsData>, proxy: EventLoopProxy<GraphicsEvent>)
    {
        for id in ctx.iter_windows_id().collect::<Vec<_>>()
        {
            let data = ctx.window_data_mut(id).unwrap();
            *data = SurfaceID::NULL;

            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(Self::new_surface_async_and_send_it(self.param.clone(), ctx.window(id).unwrap().winit_window().unwrap().clone(), proxy.clone()));

            #[cfg(not(target_arch = "wasm32"))]
            pollster::block_on(Self::new_surface_async_and_send_it(self.param.clone(), ctx.window(id).unwrap().winit_window().unwrap().clone(), proxy.clone()));
        }
    }
}