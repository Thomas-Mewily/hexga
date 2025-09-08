use super::*;


pub mod prelude
{
    //pub use super::{ContextGpu,GpuVec,GpuVecDesc};
    //pub use super::*;
    pub(crate) use super::ContextGpu;
}


pub struct GpuSurface
{
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration, 
}
impl GpuSurface
{
    pub(crate) fn new(surface: wgpu::Surface<'static>, surface_config: wgpu::SurfaceConfiguration) -> Self 
    { Self { surface, surface_config }}

    pub fn resize(&mut self, size: Point2) 
    {
        self.surface_config.width = size.x.max(1) as _;
        self.surface_config.height = size.y.max(1) as _;
        self.surface.configure(&Gpu.device, &self.surface_config);
    }
}

pub struct GpuBase
{
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) render_pipeline: wgpu::RenderPipeline,
}

pub struct ContextGpu
{
    pub(crate) base: GpuBase,
    pub(crate) surface: GpuSurface,
    pub(crate) draw: Drawer,
}
impl Deref for ContextGpu
{
    type Target=GpuBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl DerefMut for ContextGpu
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}
impl ContextGpu
{
    pub(crate) fn request<UserEvent>(window: Arc<Window>, proxy : EventLoopProxy<AppInternalMessage<UserEvent>>) -> Result<(), String> where UserEvent: IUserEvent
    {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: 
            if cfg!(debug_assertions) 
            {
                wgpu::InstanceFlags::VALIDATION
            } else 
            {
                wgpu::InstanceFlags::empty()
            }
            ,
            ..Default::default()
        });
        let surface = instance.create_surface(Arc::clone(&window)).ok_or_debug()?;

        Self::request_async(instance, window, surface, proxy).spawn();

        Ok(())
    }
    pub(crate) async fn request_async<UserEvent>(instance : wgpu::Instance, window: Arc<Window>, surface : wgpu::Surface<'static>, proxy : EventLoopProxy<AppInternalMessage<UserEvent>>) where UserEvent: IUserEvent
    {
        let _ = proxy.send_event(AppInternalMessage::ContextGpu(Self::new(instance, window, surface).await));
    }

    
    pub(crate) async fn new(instance : wgpu::Instance, window: Arc<Window>, surface : wgpu::Surface<'static>) -> Result<Self, String> 
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

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::Performance,
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
            contents: Mat4::IDENTITY.as_u8_slice(),
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
                        format: GpuColor::GPU_VERTEX_FORMAT,
                    },
                ],
            }
        };


        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
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
                cull_mode: None, //Some(wgpu::Face::Back),
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
                surface: GpuSurface{ surface, surface_config },
                draw: Drawer { camera: CameraManager::new(camera_buffer, camera_bind_group), immediate: ___(), draw_call: ___() },
            }
        )
    }
}
impl ContextGpu
{
    pub fn resize(&mut self, size: Point2)
    {
        self.surface.resize(size);
    }
}


impl Scoped<Draw> for ContextGpu
{
    fn begin(&mut self) {
        self.draw.begin_draw();
    }

    fn end(&mut self) 
    {
        self.draw.end_draw();
        self.send_data_to_gpu();
    }
}

impl ContextGpu
{
    pub(crate) fn send_data_to_gpu(&mut self)
    {
        let surface_texture = self
            .surface.surface
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
            rpass.set_pipeline(&self.render_pipeline);

            rpass.set_bind_group(0, &self.draw.camera.camera_bind_group, &[]);

            for draw_calls in self.draw.draw_call.iter()
            {
                for draw_call in draw_calls.calls.iter()
                {
                    let Mesh { vertices, indices } = &draw_call.mesh;
                    rpass.set_vertex_buffer(0, vertices.buffer.slice(..));
                    rpass.set_index_buffer(indices.buffer.slice(..), VertexIndex::GPU_INDEX_FORMAT);
                    rpass.draw_indexed(0..(indices.len as _), 0, 0..1);
                }
            }

            //todo!()
            //rpass.set_vertex_buffer(0, self.draw.vertices.buffer.slice(..));
            //rpass.set_index_buffer(self.draw.indices.buffer.slice(..), Vertex::WGPU_INDEX_FORMAT);
            //rpass.draw_indexed(0..(self.draw.indices.len as _), 0, 0..1);
            //rpass.draw(0..VERTEX_LIST.len() as u32, 0..1);
        }

        
        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}