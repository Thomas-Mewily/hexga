use super::*;

use wgpu::util::DeviceExt;

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
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
}

pub struct ContextGpu
{
    base: GpuBase,
    surface: GpuSurface,
    draw : Drawer,
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
        let (mut device, queue) = adapter
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

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&VERTEX_LIST),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&VERTEX_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        //let nb_indice = VERTEX_INDICES.len() as _;

        // FIXME: change capacity
        let vertices = GpuVec::<Vertex>::_with_capacity(&mut device, 100, GpuVecDesc::VERTEX);
        let indices = GpuVec::<VertexIndex>::_with_capacity(&mut device, 100, GpuVecDesc::INDEX);

        Ok(
            Self 
            {
                base: GpuBase { adapter, device, queue, render_pipeline },
                surface: GpuSurface{ surface, surface_config },
                draw: Drawer{ immediate: ImmediateMode { vertices, indices }, batch: ___() },
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
                buffers: &[Vertex::create_buffer_layout()],
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
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        })
    }


}
impl ContextGpu
{
    pub fn resize(&mut self, size: Point2)
    {
        self.surface.resize(size);
    }

    pub fn draw(&mut self)
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
            rpass.set_vertex_buffer(0, self.draw.vertices.buffer.slice(..));
            rpass.set_index_buffer(self.draw.indices.buffer.slice(..), Vertex::WGPU_INDEX_FORMAT);
            rpass.draw_indexed(0..(self.draw.indices.len as _), 0, 0..1);
            //rpass.draw(0..VERTEX_LIST.len() as u32, 0..1);
        }
        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}

pub(crate) type WgpuDevice = wgpu::Device;
pub(crate) type BufferUsages = wgpu::BufferUsages;

pub struct GpuVec<T> 
{
    pub(crate) buffer   : wgpu::Buffer,
    pub(crate) capacity : usize,
    pub(crate) len      : usize,
    pub(crate) desc     : GpuVecDesc,
    phantom : PhantomData<T>,
}
impl<T> GpuVec<T>
{
    pub fn name(&self) -> Option<&'static str> { self.desc.name }
    pub fn clear(&mut self)
    {
        self.len = 0;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GpuVecDesc
{
    pub usages: BufferUsages,
    pub name  : Option<&'static str>,
}
impl Default for GpuVecDesc
{
    fn default() -> Self {
        Self { usages: BufferUsages::empty(), name: None }
    }
}

impl GpuVecDesc
{
    pub const fn new() -> Self { Self { usages: BufferUsages::empty(), name: None }}

    pub const fn with_usages(mut self, usages : BufferUsages) -> Self { self.usages = usages; self }
    pub const fn with_label(mut self, label : Option<&'static str>) -> Self { self.name = label; self }

    pub const VERTEX : Self = Self::new().with_usages(BufferUsages::VERTEX);
    pub const INDEX : Self = Self::new().with_usages(BufferUsages::INDEX);
}

impl<T> GpuVec<T>
{
    pub(crate) fn new_full(buffer: wgpu::Buffer, capacity: usize, len: usize, desc: GpuVecDesc) -> Self
    {
        assert!(capacity >= len);
        Self { buffer, capacity, len, desc, phantom: PhantomData }
    }

    pub(crate) fn _with_capacity(device: &mut wgpu::Device, capacity: usize, desc: GpuVecDesc) -> Self
    {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: desc.usages,
            size: (capacity * std::mem::size_of::<T>()) as _,
            mapped_at_creation: false,
        });

        Self::new_full(buffer, capacity, capacity, desc)
    }

    pub fn with_capacity(capacity: usize, desc: GpuVecDesc) -> Self
    {
        Self::_with_capacity(&mut Gpu.device, capacity, desc)
    }

    pub(crate) fn _new(device: &mut wgpu::Device, value: &[T], desc: GpuVecDesc) -> Self
    {
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                value.as_ptr() as *const u8,
                value.len() * std::mem::size_of::<T>(),
            )
        };
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytes,
            usage: desc.usages,
        });
        let capacity = value.len();
        let len = value.len();
        Self::new_full(buffer, capacity, len, desc)
    }

    pub fn new(value: &[T], desc: GpuVecDesc) -> Self
    {
        Self::_new(&mut Gpu.device, value, desc)
    }
}

pub struct ImmediateMode
{
    pub(crate) vertices: GpuVec<Vertex>,
    pub(crate) indices: GpuVec<VertexIndex>,
}
impl ImmediateMode
{
}

pub struct Drawer
{
    immediate : ImmediateMode,
    pub(crate) batch : Vec<GpuRenderBatch>
}
impl Deref for Drawer
{
    type Target=ImmediateMode;
    fn deref(&self) -> &Self::Target { &self.immediate }
}
impl DerefMut for Drawer
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.immediate }
}

pub struct GpuRenderBatch
{
    pass : Vec<GpuRenderPass>
}

pub struct GpuRenderPass
{
    verts: wgpu::Buffer
}

pub mod prelude
{
    //pub use super::{ContextGpu,GpuVec,GpuVecDesc};
    //pub use super::*;
}

pub struct Pen;
