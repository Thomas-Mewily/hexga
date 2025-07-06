#![allow(dead_code)]
#![allow(unused_imports)]

// Thank to https://github.com/w4ngzhen/wgpu_winit_example


use core::error;
use std::marker::PhantomData;

use hexga_core::prelude::*;
use hexga_generational::prelude::{GenVec, GenVecID};
use wgpu::{ShaderSource, Trace, MemoryHints};
use hexga_engine_window::prelude::*;

pub mod prelude
{
    pub use super::{Graphics, GraphicsParam, IGraphicsParam};
}

#[derive(Debug)]
pub struct Graphics
{
    instance: wgpu::Instance,
    surfaces: GenVec<Surface>,
}

#[derive(Debug)]
pub struct Surface
{
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

pub type SurfaceID = GenVecID<Surface>;

#[derive(Default, Debug, Clone, PartialEq, Hash)]
pub struct WindowGraphicsData<T=()>
{
    pub surface: SurfaceID,
    pub value  : T,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GraphicsParam
{

}
impl GraphicsParam
{
    pub fn new() -> Self { ___() }
}
pub trait IGraphicsParam : Default
{

}
impl IGraphicsParam for GraphicsParam
{

}

pub struct WindowSurface<T=()>
{
    data : T,

}

impl Graphics
{
    pub fn new() -> Self { Self { instance: ___(), surfaces: GenVec::new() } }

    pub async fn new_surface_async<W>(&mut self, window: &Window<WindowGraphicsData<W>>) -> SurfaceID { self.try_new_surface_with_param_async(window, ___()).await.unwrap() }
    pub async fn new_surface_with_param_async<W>(&mut self, window: &Window<WindowGraphicsData<W>>, _param : GraphicsParam) -> SurfaceID { self.try_new_surface_with_param_async(window, _param).await.unwrap() }
    pub async fn try_new_surface_with_param_async<W>(&mut self, window: &Window<WindowGraphicsData<W>>, _param : GraphicsParam) -> Result<SurfaceID,String>
    {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.winit_window().clone()).unwrap();
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
        // Create the logical device and command queue

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


        let size = window.physical_size();
        let surface_config = surface.get_default_config(&adapter, size.x as _, size.y as _).unwrap();
        surface.configure(&device, &surface_config);

        let s = Surface
        {
            adapter,
            surface,
            surface_config,
            device,
            queue,
        };

        Ok(self.surfaces.insert(s))
    }


    pub fn new_surface<W>(&mut self, window: &Window<WindowGraphicsData<W>>) -> SurfaceID { self.new_surface_with_param(window, ___()) }
    pub fn new_surface_with_param<W>(&mut self, window: &Window<WindowGraphicsData<W>>, param : GraphicsParam) -> SurfaceID { self.try_new_surface_with_param(window, param).unwrap() }
    pub fn try_new_surface_with_param<W>(&mut self, window: &Window<WindowGraphicsData<W>>, param : GraphicsParam) -> Result<SurfaceID,String>
    {
        // Todo: Handle WASM and probably need some rework for supporting Android

        #[cfg(not(target_arch = "wasm32"))]
        return pollster::block_on(self.try_new_surface_with_param_async(window, param));

        //#[cfg(target_arch = "wasm32")]
        //return wasm_bindgen_futures::spawn_local(create_graphics(window, proxy));
    }
}