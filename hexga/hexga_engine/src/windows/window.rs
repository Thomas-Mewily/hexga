use super::*;


#[derive(Debug)]
pub struct Window
{
    pub(crate) id : WindowID,
}
impl Window
{
    pub fn new(param : WindowParam) -> Option<Self>
    {
        Windows.new_window(param)
    }

    pub fn id(&self) -> WindowID { self.id }
    pub(crate) unsafe fn from_id(id : WindowID) -> Self { Self { id }}
}
impl Deref for Window
{
    type Target=WindowData;
    fn deref(&self) -> &Self::Target { ctx_or_init().windows.get(self.id()).expect("Invalid window") }
}
impl DerefMut for Window
{
    fn deref_mut(&mut self) -> &mut Self::Target { ctx_mut_or_init().windows.get_mut(self.id()).expect("Invalid window") }
}

pub(crate) type WindowGraphicsAsset = Asset<WindowGraphics,(),WindowGraphicsLoadingState>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum WindowGraphicsLoadingState
{
    Pending,
    Loading,
}

pub struct WindowData
{
    pub(crate) winit_window: Option<WinitWindowPtr>,
    pub(crate) winit_id    : Option<WinitWindowID>,
    pub(crate) graphics    : WindowGraphicsAsset,

    pub(crate) dirty: bool,
    pub(crate) rectangle : Rect2P,
    pub(crate) param: WindowParam,
    pub(crate) id : WindowID,
}
impl Debug for WindowData
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("id", &self.id())
            .field("param", &self.param).finish()
    }
}
impl Deref for WindowData
{
    type Target=WindowParam;
    fn deref(&self) -> &Self::Target { &self.param }
}
impl DerefMut for WindowData
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.param }
}
impl WindowData
{
    pub fn id(&self) -> WindowID { self.id }
    pub(crate) fn winit_id(&self) -> Option<WinitWindowID> { self.winit_id }
}
impl GetPosition<int,2> for WindowData
{
    fn pos(&self) -> Point2 {
        self.rectangle.pos
    }
}
impl SetPosition<int,2> for WindowData
{
    fn set_pos(&mut self, pos : Point2) -> &mut Self 
    {
        if pos != self.rectangle.pos
        {
            self.rectangle.pos = pos;
            self.set_dirty();
        }
        self
    }
}
impl GetRectangle<int,2> for WindowData
{
    fn size(&self) -> Vector<int,2> {
        self.rectangle.size
    }
}
impl SetRectangle<int,2> for WindowData
{
    fn set_size(&mut self, size : Vector<int, 2>) -> &mut Self 
    {
        if size != self.rectangle.size
        {
            self.rectangle.size = size;
            self.set_dirty();
        }
        self
    }
}

impl WindowData
{
    pub(crate) fn set_dirty(&mut self)
    {
        self.dirty = true;
        Windows.any_dirty = true;
    }

    pub(crate) fn update_dirty<UserEvent>(&mut self, lookup: &mut WindowLookupID, gfx : &Graphics, event_loop: &WinitActiveEventLoop, proxy : &WinitEventLoopProxy<AppInternalEvent<UserEvent>>) where UserEvent: IUserEvent
    {
        if !self.dirty { return; }
        self.dirty = false;

        if self.winit_window.is_none()
        {
            // winit window creation
            debug_assert!(self.winit_id().is_none());

            let mut win_attr = WinitWindow::default_attributes();

            
            #[cfg(not(target_arch = "wasm32"))]
            {
                win_attr = win_attr.with_title(self.param.title());
            }

            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowAttributesExtWebSys;
                win_attr = win_attr.with_append(true);
            }

            todo!();

            if let Some(m)= event_loop.primary_monitor()
            {
                // Todo : Store it somewhere in the context
                let screen_size = m.size().convert_vec2();
                let window_size = self.param.rectangle.to_rectangle(); //screen_size, screen_size);

                win_attr = win_attr.with_position(WinitPhysicalPosition::new(window_size.pos.x as f64, window_size.pos.y as f64));
                win_attr = win_attr.with_inner_size(WinitPhysicalSize::new(window_size.size.x as f64, window_size.size.y as f64));
            }

            let winit_window = event_loop
                    .create_window(win_attr)
                    .expect("create window err.");

            let winit_window_ptr = WinitWindowPtr::new(winit_window);

            let winit_id = winit_window_ptr.winit_window().id();
            self.winit_id = Some(winit_id);
            self.winit_window = Some(winit_window_ptr);
            lookup.insert(winit_id, self.id());


            // wgpu async surface creation
            match &mut self.graphics
            {
                Asset::Loading(l) => match l 
                {
                    WindowGraphicsLoadingState::Pending => 
                    {
                        *l = WindowGraphicsLoadingState::Loading;
                        let winit_window = self.winit_window.as_ref().expect("winit_window should have been init just before").clone();

                        let instance = gfx.instance.clone();
                        let surface: WgpuSurface = instance.create_surface(winit_window.window.clone()).unwrap();
                        let size = self.size();

                        crate::spawn_task(Self::request_surface(gfx.instance.clone(), surface, size, self.id(), proxy.clone()));

                    },
                    WindowGraphicsLoadingState::Loading => todo!(),
                },
                Asset::Loaded(_gfx) => {},
                Asset::Error(_) => { panic!("Can't create the window gfx"); },
            }
            
        }

        /* 
        self.set_pos(self.position());
        self.resize(self.size());
        self.set_cursor_icon(self.cursor_icon());
        self.set_cursor_grab(self.cursor_grab());
        self.set_cursor_visible(self.is_cursor_visible());
        self.set_transparency_support(self.support_transparency());

        let title = self.title().to_owned();
        self.set_title(title); // I don't like this clone

        self.set_level(self.level);
        */
    }
}

impl WindowData
{
    pub(crate) async fn request_surface<UserEvent>(instance : wgpu::Instance, surface : WgpuSurface, size : Point2, id: WindowID, proxy : WinitEventLoopProxy<AppInternalEvent<UserEvent>>) where UserEvent: IUserEvent
    {
        let event = AppInternalEvent::WindowInternal(
            WindowInternalEvent
            {
                id,
                kind: WindowInternalEventKind::SurfaceCreated(Self::request_surface_result::<UserEvent>(instance, surface, size).await),
            }
        );
        let _ = proxy.send_event(event);
    }

    pub(crate) async fn request_surface_result<UserEvent>(instance : wgpu::Instance, surface : WgpuSurface, size : Point2) -> WindowSurfaceResult where UserEvent: IUserEvent
    {
        //let surface: wgpu::Surface<'_> = instance.create_surface(window.window.clone()).unwrap();
        let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Could not get an adapter (GPU).");


        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::Performance,
                    trace: wgpu::Trace::Off,
                }
            )
            .await
            .expect("Failed to get device");

        // Make the dimensions at least size 1, otherwise wgpu would panic
        let size = size.max_with(one());
        let config = surface.get_default_config(&adapter, size.x as _, size.y as _).unwrap();

        surface.configure(&device, &config);
        let pipeline = Self::create_pipeline(&device, config.format);

        Ok(WindowGraphics{ adapter, surface, config, device, queue, pipeline })
    }

    pub(crate) fn create_pipeline(device: &wgpu::Device, swap_chain_format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
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
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(swap_chain_format.into())],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
    }
}


#[derive(Clone, Debug)]
pub struct WinitWindowPtr
{
    window : WinitWindowPtrKind<WinitWindow>
}
impl WinitWindowPtr
{
    pub fn new(window : WinitWindow) -> Self { Self { window : WinitWindowPtrKind::new(window) }}
    pub fn winit_window(&self) -> &WinitWindow { &self.window }
}
impl Deref for WinitWindowPtr
{
    type Target=WinitWindow;
    fn deref(&self) -> &Self::Target { &self.window }
}


#[derive(PartialEq, Clone, Debug)]
pub struct WindowParam
{
    /// Title of the window, defaults to an empty string.
    title: String,
    pub(crate) rectangle : UiRect2,
}

impl Default for WindowParam
{
    fn default() -> Self 
    {
        Self 
        { 
            title: "".to_owned(), 
            rectangle: UiRect2::FULL_SCREEN.centered()
        }
    }
}


pub trait IWindowParam : SetRectangle<UiNumber,2>
{
    fn title(&self) -> &str;
    fn with_title(self, title: impl Into<String>) -> Self;
}

impl WindowParam
{
    pub fn new() -> Self { ___() }
}

impl IWindowParam for WindowParam
{
    fn title(&self) -> &str { &self.title }
    fn with_title(mut self, title: impl Into<String>) -> Self { self.title = title.into(); self }
}
impl GetPosition<UiNumber,2> for WindowParam
{
    fn pos(&self) -> Vector<UiNumber,2> {
        self.rectangle.pos
    }
}
impl SetPosition<UiNumber,2> for WindowParam
{
    fn set_pos(&mut self, pos : Vector<UiNumber,2>) -> &mut Self {
        self.rectangle.pos = pos;
        self
    }
}
impl GetRectangle<UiNumber,2> for WindowParam
{
    fn size(&self) -> Vector<UiNumber,2> {
        self.rectangle.size
    }
}
impl SetRectangle<UiNumber,2> for WindowParam
{
    fn set_size(&mut self, size : Vector<UiNumber, 2>) -> &mut Self {
        self.rectangle.size = size;
        self
    }
}


#[derive(Debug)]
pub(crate) struct WindowGraphics
{
    pub(crate) adapter  : wgpu::Adapter,
    pub(crate) surface  : wgpu::Surface<'static>,
    pub(crate) config   : wgpu::SurfaceConfiguration,
    pub(crate) device   : wgpu::Device,
    pub(crate) queue    : wgpu::Queue,
    pub(crate) pipeline : wgpu::RenderPipeline,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WindowEventKind
{
    Resize(Point2),
    Move(Point2),
    Open,
    Close,
    Destroy,
    Draw,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowEvent<K=WindowEventKind>
{
    pub id : WindowID,
    pub kind : K,
}

impl<K> WindowEvent<K>
{
    pub fn new(id : WindowID, kind  : K) -> Self { Self { id, kind }}
}
pub(crate) type WindowInternalEvent = WindowEvent<WindowInternalEventKind>;

#[derive(Debug)]
pub(crate) enum WindowInternalEventKind
{
    SurfaceCreated(WindowSurfaceResult),
}
pub(crate) type WindowSurfaceResult = Result<WindowGraphics, ()>;
