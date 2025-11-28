use hexga::singleton::singleton_single_thread_access;

use super::*;

pub type GpuDevice = wgpu::Device;
pub(crate) type GpuEvent = Result<AppPen,String>;

singleton_single_thread_access!(
    pub Pen,
    AppPen,
    { App::try_read().map(|v|v.inner_reference.pen.as_ref()).flatten().map(|v| v.into()) },
    { App::try_write().map(|v|v.inner_reference.pen.as_mut()).flatten().map(|v| v.into()) }
);


#[derive(Debug)]
pub struct GpuBase
{
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) render_pipeline: wgpu::RenderPipeline,
}

#[derive(Debug)]
pub struct AppPen
{
    pub(crate) base: GpuBase,
    pub(crate) surface: GpuSurface,

    pub(crate) binding: GpuBinding,
    pub(crate) render: GpuRender,

    pub(crate) immediate_mesh: Option<Mesh>,
    pub(crate) background_color : Option<Color>,
    pub(crate) white_pixel: Option<Texture>,
}

impl Deref for AppPen
{
    type Target = GpuRender;
    fn deref(&self) -> &Self::Target { &self.render }
}
impl DerefMut for AppPen
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.render }
}

impl ScopedFlow for AppPen
{
    fn begin_flow(&mut self, flow: FlowMessage) {
        self.render.begin_flow(flow);
        self.dispatch_begin_flow(flow);
    }

    fn end_flow(&mut self, flow: FlowMessage) {
        self.render.end_flow(flow);
        self.dispatch_end_flow(flow);
    }

    fn end_flow_draw(&mut self)
    {
        self.send_data_to_gpu();
    }
}

impl AppPen
{
    pub fn background_color(&self) -> Option<Color>
    {
        self.background_color
    }

    pub fn set_background_color(&mut self, color: Option<Color>)
    {
        self.background_color = color;
    }


    pub(crate) fn send_data_to_gpu(&mut self)
    {
        if self.white_pixel.is_none()
        {
            self.white_pixel = Some(Texture::from(Image::one_by_one(ColorU8::WHITE)));
        }

        let surface_texture = self
            .surface.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .base.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.background_color().unwrap_or_default().convert()),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&self.base.render_pipeline);

            rpass.set_bind_group(0, &self.binding.camera_bind_group, &[]);

            if !self.render.big_mesh.is_empty()
            {
                // Todo: avoid create a new buffer at each frame.

                match self.immediate_mesh.as_mut()
                {
                    Some(m) => self.render.big_mesh.build_in(m),
                    None =>
                    {
                        self.immediate_mesh = Some(self.render.big_mesh.build());
                    },
                };
                let Mesh { vertices, indices } = self.immediate_mesh.as_ref().expect("missing immediate mesh");

                for dc in self.render.draw_calls.iter()
                {
                    //dbg!(&dc);

                    let mut viewport = dc.viewport;
                    let (viewport_min_depth, viewport_max_depth) = (dc.viewport_min_depth, dc.viewport_max_depth);
                    let mut scissor = dc.scissor;

                    viewport = self.render.max_viewport().intersect_or_empty(viewport);
                    scissor = self.render.max_scissor().intersect_or_empty(scissor);

                    if viewport.size.is_empty() || scissor.size.is_empty()
                    {
                        continue;
                    }

                    self.base.queue.write_buffer(&self.binding.camera_buffer, 0, dc.param.camera.matrix().as_u8_slice());

                    /*
                    let viewport : Rect2i = viewport.cast_into();
                    let scissor : Rect2i = scissor.cast_into();
                    dbg!(viewport);

                    if viewport.width().is_zero() { continue; }
                    if scissor.width().is_zero() { continue; }
                    */

                    rpass.set_viewport(viewport.pos.x as _, viewport.pos.y as _, viewport.size.x as _, viewport.size.y as _, viewport_min_depth, viewport_max_depth);
                    rpass.set_scissor_rect(scissor.pos.x as _, scissor.pos.y as _, scissor.size.x as _, scissor.size.y as _);

                    let texture_index = 1;
                    let offset = &[];
                    match &dc.texture
                    {
                        DrawTexture::None =>
                        {
                            rpass.set_bind_group(texture_index, &self.white_pixel.as_ref().unwrap().shared.bind_group, offset)
                        },
                        DrawTexture::Texture(texture) =>
                        {
                            rpass.set_bind_group(texture_index, &texture.shared.bind_group, offset)
                        },
                        DrawTexture::Asset(asset) => match asset.get_or_placeholder()
                        {
                            Some(texture) => rpass.set_bind_group(1, &texture.shared.bind_group, offset),
                            None => rpass.set_bind_group(texture_index, &self.white_pixel.as_ref().unwrap().shared.bind_group, offset),
                        },
                    };
                    //let texture = dc.texture.as_ref().unwrap_or(self.white_pixel.as_ref().unwrap());
                    //rpass.set_bind_group(1, bindgroup, &[]);

                    match &dc.geometry
                    {
                        DrawGeometry::Immediate(im) =>
                        {
                            if im.is_empty() { continue; }
                            let (vertices_begin, vertices_len) = (im.vertices_begin, im.vertices_len);
                            let vertices_end = im.vertices_begin+im.vertices_len;

                            let (indices_begin, indices_len) = (im.indices_begin, im.indices_len);
                            let indices_end = im.indices_begin+im.indices_len;

                            rpass.set_vertex_buffer(0, vertices.wgpu_slice(vertices_begin..vertices_end));
                            rpass.set_index_buffer(indices.wgpu_slice(indices_begin..indices_end), VertexIndex::GPU_INDEX_FORMAT);
                            //rpass.draw_indexed(0 ..(indices_len as _), 0, 0..1);
                            // Indice are relative to global big mesh, not relative to the current vertices slice passed to wgpu, hence the -(vertices_begin as i32)
                            rpass.draw_indexed(0 ..(indices_len as _), -(vertices_begin as i32), 0..1);
                        },
                    }
                }
            }
        }


        self.base.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}


#[derive(Debug)]

pub struct GpuBinding
{
    pub(crate) camera_buffer: wgpu::Buffer,
    pub(crate) camera_bind_group: wgpu::BindGroup,
    pub(crate) texture_bind_group: wgpu::BindGroupLayout,
}

impl ScopedFlow for Option<AppPen>
{
    fn begin_flow(&mut self, flow: FlowMessage) {
        self.as_mut().map(|gpu| gpu.begin_flow(flow));
        self.dispatch_begin_flow(flow);
    }

    fn end_flow(&mut self, flow: FlowMessage) {
        self.as_mut().map(|gpu| gpu.end_flow(flow));
        self.dispatch_end_flow(flow);
    }

    fn begin_flow_resumed(&mut self) {
        if self.is_none()
        {
            if let Some(w) = App.window.active.as_ref()
            {
                AppPen::request(w.clone(), App.proxy.as_ref().unwrap().clone()).unwrap();
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct GpuSurface
{
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
}

impl GpuSurface
{
    fn resize(&mut self, size: Point2)
    {
        let size = size.max(one());
        self.surface_config.width = size.x as _;
        self.surface_config.height = size.y as _;
        self.surface.configure(&Pen.base.device, &self.surface_config);
    }

    fn size(&self) -> Point2
    {
        point2(self.surface_config.width as _, self.surface_config.height as _)
    }
}

impl AppPen
{
    pub(crate) fn resize(&mut self, size: Point2)
    {
        self.surface.resize(size);
    }
}


impl AppPen
{
    pub(crate) fn request(window: Arc<WinitWindow>, proxy : EventLoopProxy) -> Result<(), String>
    {
        let mut flags = wgpu::InstanceFlags::empty();
        if cfg!(debug_assertions)
        {
            flags |= wgpu::InstanceFlags::VALIDATION;
        }

        let mut backends = wgpu::Backends::empty();
        backends |= wgpu::Backends::GL;
        backends |= wgpu::Backends::METAL;
        //backends |= wgpu::Backends::DX12; // Seem to Allocate at least 250 MB of RAM
        backends |= wgpu::Backends::BROWSER_WEBGPU;
        // Todo: make a flag for it
        // backends |= wgpu::Backends::VULKAN; // Why it is slow as hell to start


        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends,
            flags,
            ..Default::default()
        });
        let surface = instance.create_surface(Arc::clone(&window)).ok_or_debug()?;

        Self::request_async(instance, window, surface, proxy).spawn();

        Ok(())
    }
    pub(crate) async fn request_async(instance : wgpu::Instance, window: Arc<WinitWindow>, surface : wgpu::Surface<'static>, proxy: EventLoopProxy)
    {
        let _ = proxy.send_event(AppInternalEvent::Gpu(Self::new(instance, window, surface).await));
    }




    pub(crate) async fn new(instance : wgpu::Instance, window: Arc<WinitWindow>, surface : wgpu::Surface<'static>) -> GpuEvent
    {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .map_err(|_| "Failed to find an appropriate adapter".to_owned())?;

        //rintln!();
        //println!("{:?}", adapter.get_info());
        //println!();

        let required_limits = wgpu::Limits::default();

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits,
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(|_| "Failed to create device".to_owned())?;

        let size = window.inner_size();

        let width = size.width.max(1);
        let height = size.height.max(1);

        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        surface.configure(&device, &surface_config);

        let swap_chain_format = surface_config.format;

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: GpuMat4::IDENTITY.as_u8_slice(),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });


        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            label: Some("camera_bind_group_layout"),
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });


        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
        });



        let vertex_layout =
        {
            GpuVertexBufferLayout {
                array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: GpuVec3::GPU_VERTEX_FORMAT,
                    },
                    wgpu::VertexAttribute {
                        offset: size_of::<GpuVec3>() as wgpu::BufferAddress,
                        shader_location: 1,
                        format: UV::GPU_VERTEX_FORMAT,
                    },
                    wgpu::VertexAttribute {
                        offset: (size_of::<GpuVec3>() + size_of::<UV>()) as wgpu::BufferAddress,
                        shader_location: 2,
                        format: GpuColor::GPU_VERTEX_FORMAT,
                    },
                ],
            }
        };

        let texture_bind_group =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[vertex_layout ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(swap_chain_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, //Some(wgpu::Face::{Back,Back})
                polygon_mode: wgpu::PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        //let render_pipeline = Self::create_pipeline(&device, surface_config.format);

        Ok
        (
            Self
            {
                base: GpuBase { adapter, device, queue, render_pipeline },
                surface: GpuSurface { surface, surface_config },
                binding: GpuBinding { camera_buffer, camera_bind_group, texture_bind_group },
                render: GpuRender::new(DrawParam { camera: Camera::CAMERA_3D, ..___() }),
                background_color: Some(Color::BLACK),
                immediate_mesh: None,
                white_pixel: None,
            }
        )
    }

}