#![allow(dead_code)]
#![allow(unused_imports)]

// Thank to https://github.com/w4ngzhen/wgpu_winit_example


use core::error;
use std::{borrow::Cow, marker::PhantomData};

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

    pub fn surfaces_result(&self) -> impl Iterator<Item=&SurfaceResult> { self.surfaces.values() }
    pub fn surfaces(&self) -> impl Iterator<Item=&Surface> { self.surfaces_result().filter_map(|v| v.as_ref().ok()) }

    pub fn handle_event(&mut self, event: GraphicsEvent, ctx : &mut WindowCtx<WindowGraphicsData>)
    {
        match event
        {
            GraphicsEvent::SurfaceCreated(mut surface_created) =>
            {
                let old_id = surface_created.surface.as_ref().unwrap().id;
                let window_id = surface_created.window;

                let id = match old_id.is_null()
                {
                    true =>
                    {
                        let id = self.surfaces.insert(surface_created.surface);
                        self.surfaces[id].as_mut().unwrap().id = id;
                        id
                    },
                    false =>
                    {
                        let surface = self.surfaces.get_mut(old_id).unwrap();
                        std::mem::swap(surface, &mut surface_created.surface);
                        old_id
                    },
                };

                ctx.window_data_mut(window_id).map(|v| *v = id);
            }
        }
    }


    pub fn draw_surface(&mut self, id : SurfaceID)
    {
        let Some(Ok(s)) = self.surfaces.get_mut(id) else { return; };
        s.draw();
    }


    pub fn draw_all_window(&mut self)
    {
        for s in self.surfaces.values_mut().filter_map(|v| v.as_mut().ok())
        {
            s.draw();
        }
    }



    pub fn draw_window(&mut self, window: &Window<WindowGraphicsData>)
    {
        let surface_id = *window.data();
        if let Some(surface) = self.surfaces.get_mut(surface_id)
        {
            if let Ok(surface) = surface.as_mut()
            {
                surface.draw();
            }
        }
    }

    pub fn resize_window(&mut self, window: &Window<WindowGraphicsData>, size: Point2)
    {
        let surface_id = *window.data();
        if let Some(surface) = self.surfaces.get_mut(surface_id)
        {
            if let Ok(surface) = surface.as_mut()
            {
                let (width, height) = size.into();
                surface.config.width = width.max(1) as _;
                surface.config.height = height.max(1) as _;
                surface.surface.configure(&surface.device, &surface.config);
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
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) id : SurfaceID,
}

impl Surface
{
    pub fn draw(&mut self)
    {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
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
            rpass.set_pipeline(&self.pipeline);
            rpass.draw(0..3, 0..1);
        }
        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }

    pub fn resize(&mut self, size: Point2)
    {
        self.config.width = size.x.max(1) as _;
        self.config.height = size.y.max(1) as _;
        self.surface.configure(&self.device, &self.config);
    }
}

pub type WindowGraphicsID = hexga_engine_window::window::WindowID<WindowGraphicsData>;
pub type LocalizedGrahicsEvent = hexga_engine_window::event::LocalizedEvent<WindowGraphicsData>;

#[derive(Debug)]
pub struct SurfaceCreated
{
    pub window  : WindowGraphicsID,
    pub surface : SurfaceResult,
}

pub enum GraphicsEvent
{
    SurfaceCreated(SurfaceCreated),
}

pub type SurfaceResult = Result<Surface, String>;
impl SurfaceCreated
{
    pub(crate) fn new(window: WindowGraphicsID, surface : SurfaceResult) -> Self
    {
        Self
        {
            window,
            surface,
        }
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
    pub async fn new_surface_async_and_send_it(param : GraphicsParam, window_id : WindowGraphicsID, window : WinitWindowPtr, proxy: EventLoopProxy<GraphicsEvent>)
    {
        let _ = proxy.send_event(GraphicsEvent::SurfaceCreated(SurfaceCreated::new(window_id, Self::new_surface_async(param, window).await)));
    }

    // Thank to https://github.com/w4ngzhen/wgpu_winit_example and https://github.com/Foxicution/wgpu-template ,
    // the following code is mainly based on them
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

        surface.configure(&device, &surface_config);

        let pipeline =
        {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
            });
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: None,
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(surface_config.format.into())],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    // strip_index_format: None,
                    // front_face: wgpu::FrontFace::Ccw,
                    // cull_mode: Some(wgpu::Face::Back),
                    // // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                    // // or Features::POLYGON_MODE_POINT
                    // polygon_mode: wgpu::PolygonMode::Fill,
                    // // Requires Features::DEPTH_CLIP_CONTROL
                    // unclipped_depth: false,
                    // // Requires Features::CONSERVATIVE_RASTERIZATION
                    // conservative: false,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            })
        };


        Ok
        (
            Surface
            {
                adapter,
                surface,
                config: surface_config,
                device,
                queue,
                id: ___(),
                pipeline,
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
        for window_id in ctx.iter_windows_id().collect::<Vec<_>>()
        {
            let data = ctx.window_data_mut(window_id).unwrap();
            *data = SurfaceID::NULL;

            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(Self::new_surface_async_and_send_it(self.param.clone(), window_id, ctx.window(window_id).unwrap().winit_window().unwrap().clone(), proxy.clone()));

            #[cfg(not(target_arch = "wasm32"))]
            pollster::block_on(Self::new_surface_async_and_send_it(self.param.clone(), window_id, ctx.window(window_id).unwrap().winit_window().unwrap().clone(), proxy.clone()));
        }
    }
}