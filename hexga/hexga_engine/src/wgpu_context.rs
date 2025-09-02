use super::*;

pub(crate) struct ContextWgpu {
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    surface: wgpu::Surface<'static>,
}


impl ContextWgpu
{
    pub fn request<UserEvent>(window: Arc<Window>, proxy : EventLoopProxy<AppInternalMessage<UserEvent>>) -> Result<(), String> where UserEvent: IUserEvent
    {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(Arc::clone(&window)).ok_or_debug()?;

        Self::request_async(instance, window, surface, proxy).spawn();

        Ok(())
    }
    pub async fn request_async<UserEvent>(instance : Instance, window: Arc<Window>, surface : Surface<'static>, proxy : EventLoopProxy<AppInternalMessage<UserEvent>>) where UserEvent: IUserEvent
    {
        let _ = proxy.send_event(AppInternalMessage::Wgpu(Self::new(instance, window, surface).await));
    }

    pub async fn new(instance : Instance, window: Arc<Window>, surface : Surface<'static>) -> Result<Self, String> 
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

        // 获取窗口内部物理像素尺寸（没有标题栏）
        let size = window.inner_size();
        // 至少（w = 1, h = 1），否则Wgpu会panic
        let width = size.width.max(1);
        let height = size.height.max(1);
        // 获取一个默认配置
        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        // 完成首次配置
        surface.configure(&device, &surface_config);

        let render_pipeline = Self::create_pipeline(&device, surface_config.format);

        let bytes: &[u8] = bytemuck::cast_slice(&VERTEX_LIST);
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });

        Ok(
            Self 
            {
                surface,
                surface_config,
                adapter,
                device,
                queue,
                render_pipeline,
                vertex_buffer,
            }
        )
    }



    
    fn create_pipeline(
        device: &wgpu::Device,
        swap_chain_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
        });
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[create_vertex_buffer_layout()],
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
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        })
    }


    pub fn resize(&mut self, size: Point2) 
    {
        self.surface_config.width = size.x.max(1) as _;
        self.surface_config.height = size.y.max(1) as _;
        self.surface.configure(&self.device, &self.surface_config);
    }

    
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
            rpass.set_pipeline(&self.render_pipeline);
            // 消费存放的 vertex_buffer
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            // 顶点有原来的固定3个顶点，调整为根据 VERTEX_LIST 动态来计算
            rpass.draw(0..VERTEX_LIST.len() as u32, 0..1);
        }
        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}