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
impl WindowData
{
    pub(crate) fn update_dirty<UserEvent>(&mut self, lookup: &mut WindowLookupID, gfx : &Graphics, event_loop: &WinitActiveEventLoop, proxy : &WinitEventLoopProxy<AppInternalEvent<UserEvent>>) where UserEvent: IUserEvent
    {
        if !self.dirty { return; }
        self.dirty = false;

        if self.winit_window.is_none()
        {
            // winit window creation
            debug_assert!(self.winit_id().is_none());

            let mut win_attr = WinitWindow::default_attributes();

            // Get the monitor where the window will be created and its physical size
            if let Some(monitor) = event_loop.primary_monitor() {
                let size = monitor.size();
                // size is of type winit::dpi::PhysicalSize<u32>
                // You can use size.width and size.height as needed
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                win_attr = win_attr.with_title(self.param.title());
            }

            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowAttributesExtWebSys;
                win_attr = win_attr.with_append(true);
            }

            /* 
            if !self.default_size_and_position
            {
                win_attr = win_attr.with_position(WinitPhysicalPosition::new(self.position.x, self.position.y));
                win_attr = win_attr.with_inner_size(WinitPhysicalSize::new(self.size.x, self.size.y))
            }

            let winit_window = event_loop
                    .create_window(win_attr)
                    .expect("create window err.");

            if self.default_size_and_position
            {
                self.param.position = winit_window.outer_position().map(|v| v.convert()).unwrap_or_zero();
                let size = winit_window.outer_size();
                self.param.size = point2(size.width as _, size.height as _)
            }

            let winit_window_ptr = WinitWindowPtr::new(winit_window);

            let winit_id = winit_window_ptr.winit_window().id();
            self.winit_id = Some(winit_id);
            self.winit_window = Some(winit_window_ptr);
            lookup.insert(winit_id, self.id());


            // wgpu async surface creation
            match &mut self.graphics
            {
                Asset::Pending(_) =>
                {
                    let winit_window = self.winit_window.as_ref().expect("winit_window should have been init just before").clone();
                    self.graphics = Asset::Loading(());

                    let instance = gfx.instance.clone();
                    let surface: WgpuSurface = instance.create_surface(winit_window.window.clone()).unwrap();
                    let size = self.size();

                    crate::spawn_task(Self::request_surface(gfx.instance.clone(), surface, size, self.id(), proxy.clone()));
                },
                Asset::Loading(_) => {},
                Asset::Loaded(_gfx) => {},
                Asset::Error(_) => { panic!("Can't create the window gfx"); },
            }
            */
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
    rectangle : UiRect2,
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
