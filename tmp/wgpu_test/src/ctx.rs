use wgpu::{util::{BufferInitDescriptor, DeviceExt}, CommandEncoder};
use crate::*;

#[derive(Default)]
pub struct Context
{
    pub(crate) gfx : Option<WgpuCtx>,
    pub(crate) window : Option<Arc<Window>>,
    pub(crate) encoder : Option<CommandEncoder>,
}

pub struct WgpuCtx 
{
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) render_pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) vertex_index_buffer: wgpu::Buffer, // 新增
}

impl WgpuCtx
{
pub async fn new_async(window: Arc<Window>) -> WgpuCtx {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");
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
            .expect("Failed to create device");

        // 获取窗口内部物理像素尺寸（没有标题栏）
        let size = window.inner_size();
        // 至少（w = 1, h = 1），否则Wgpu会panic
        let width = size.width.max(1);
        let height = size.height.max(1);
        // 获取一个默认配置
        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        // 完成首次配置
        surface.configure(&device, &surface_config);

        let render_pipeline = create_pipeline(&device, surface_config.format);

        let bytes: &[u8] = bytemuck::cast_slice(&crate::VERTEX_LIST);
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        // 将顶点索引数据转为字节数据
        let vertex_index_bytes = bytemuck::cast_slice(&VERTEX_INDEX_LIST);
        // 创建顶点索引缓冲数据
        let vertex_index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertex_index_bytes,
            usage: wgpu::BufferUsages::INDEX, // 注意，usage字段使用INDEX枚举，表明是顶点索引
        });

        WgpuCtx {
            surface,
            surface_config,
            adapter,
            device,
            queue,
            render_pipeline,
            vertex_buffer,
            vertex_index_buffer,
        }
    }

    pub fn new(window: Arc<Window>) -> WgpuCtx {
        pollster::block_on(WgpuCtx::new_async(window))
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        let (width, height) = new_size;
        self.surface_config.width = width.max(1);
        self.surface_config.height = height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }
}

pub struct Ctx;
impl Deref for Ctx
{
    type Target=Context;
    fn deref(&self) -> &Self::Target { ctx() }
}
impl DerefMut for Ctx
{
    fn deref_mut(&mut self) -> &mut Self::Target { ctx_mut() }
}

static mut CONTEXT : Option<Context> = None;
#[allow(static_mut_refs)]
pub(crate) fn ctx_mut() -> &'static mut Context { unsafe { CONTEXT.as_mut().expect("Ctx not initialized") } }
#[allow(static_mut_refs)]
pub(crate) fn ctx() -> &'static Context { unsafe { CONTEXT.as_ref().expect("Ctx not initialized") } }

#[allow(static_mut_refs)]
pub(crate) fn ctx_mut_or_init() -> &'static mut Context { init_ctx_if_needed(); ctx_mut() }
#[allow(static_mut_refs)]
pub(crate) fn ctx_or_init() -> &'static Context { init_ctx_if_needed(); ctx() }

#[allow(static_mut_refs)]
pub(crate) fn init_ctx_if_needed()
{
    unsafe
    {
        if CONTEXT.is_none()
        {
            CONTEXT = Some(Context::___());
        }
        std::panic::set_hook(Box::new(|info| {
            CONTEXT = None;
            eprintln!("Panic occurred: {info}");
        }));
    }
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
