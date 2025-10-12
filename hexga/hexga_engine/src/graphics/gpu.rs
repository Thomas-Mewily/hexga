use super::*;


pub(crate) type GpuEvent = Result<AppGpu,String>;

singleton_access!(
    pub Gpu,
    AppGpu,
    { App::try_as_ref().map(|ctx| ctx.gpu.as_ref()).flatten() },
    { App::try_as_mut().map(|ctx| ctx.gpu.as_mut()).flatten() }
);


#[derive(Debug)]
pub struct GpuBase
{
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

#[derive(Debug)]
pub struct AppGpu
{
    pub(crate) base: GpuBase,
    pub(crate) surface: Surface,

    pub(crate) render_pipeline: wgpu::RenderPipeline,

    pub(crate) camera_buffer: wgpu::Buffer,
    pub(crate) camera_bind_group: wgpu::BindGroup,
}

impl ScopedFlow for Option<AppGpu>
{
    fn begin_flow_resumed(&mut self) {
        if self.is_none()
        {
            if let Some(w) = App.window.active.as_ref()
            {
                AppGpu::request(w.clone(), App.proxy.clone()).unwrap();
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Surface
{
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
}

impl Surface
{
    fn resize(&mut self, size: Point2)
    {
        let size = size.max(one());
        self.surface_config.width = size.x as _;
        self.surface_config.height = size.y as _;
        self.surface.configure(&Gpu.base.device, &self.surface_config);
    }

    fn size(&self) -> Point2
    {
        point2(self.surface_config.width as _, self.surface_config.height as _)
    }
}

impl AppGpu
{
    pub(crate) fn resize(&mut self, size: Point2)
    {
        self.surface.resize(size);
    }
}


impl AppGpu
{
    pub(crate) fn request(window: Arc<WinitWindow>, proxy : EventLoopProxy) -> Result<(), String>
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

        let s = Self
        {
            base: GpuBase { adapter, device, queue },
            surface: Surface { surface, surface_config },
            render_pipeline,
            camera_buffer,
            camera_bind_group,
        };
        Ok(s)
    }

}