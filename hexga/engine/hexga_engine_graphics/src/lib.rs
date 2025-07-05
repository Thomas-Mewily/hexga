#![allow(dead_code)]
#![allow(unused_imports)]

// Thank to https://github.com/w4ngzhen/wgpu_winit_example

pub mod prelude
{
    pub use super::Graphics;
}

use std::sync::Arc;

use wgpu::{ShaderSource, Trace, MemoryHints};
use hexga_engine_window::prelude::*;


pub struct Graphics
{
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,

    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,

    /*
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    */
}


impl Graphics
{
    pub async fn new_async(window: &Window) -> Self
    {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.winit_window().clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");
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
            .expect("Failed to create device");


        let size = window.physical_size();
        let surface_config = surface.get_default_config(&adapter, size.x as _, size.y as _).unwrap();
        surface.configure(&device, &surface_config);

        Graphics
        {
            surface,
            surface_config,
            adapter,
            device,
            queue,
        }
    }

    pub fn new(window: &Window) -> Self {
        pollster::block_on(Self::new_async(window))
    }
}