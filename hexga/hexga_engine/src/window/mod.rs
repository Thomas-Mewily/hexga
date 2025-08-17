use crate::*;

pub(crate) type WindowLookupID = HashMap<WinitWindowID, WindowID>;

pub type WindowID = GenVecID<WindowData>;


declare_context_singleton!(Windows, ContextWindows, windows);

/// The window manager
#[derive(Debug, Default)]
pub struct ContextWindows
{
    pub(crate) lookup  : WindowLookupID,
    pub(crate) windows : GenVec<WindowData>,
    pub(crate) any_dirty : bool,

    main_window : Option<Window>,
}




impl ContextWindows
{
    pub fn new() -> Self { ___() }

    pub(crate) fn get(&self, id : WindowID) -> Option<&WindowData> { self.windows.get(id) }
    pub(crate) fn get_mut(&mut self, id : WindowID) -> Option<&mut WindowData> { self.windows.get_mut(id) }

    pub(crate) fn winit_id_to_window_id(&mut self, id : WinitWindowID) -> Option<WindowID>
    {
        self.lookup.get(&id).copied()
    }

    pub(crate) fn init_main_window(&mut self, param : Option<WindowParam>)
    {
        self.main_window = param.map(|p| self.new_window(p).expect("can't init main window"));
    }
}


pub trait IContextWindows
{
    fn new_window(&mut self, param: WindowParam) -> Option<Window>;
    fn remove_window(&mut self, window : Window) -> WindowData;

    fn main_window(&self) -> Option<&Window>;
    fn main_window_mut(&mut self) -> Option<&mut Window>;
}

impl IContextWindows for ContextWindows
{
    fn new_window(&mut self, param: WindowParam) -> Option<Window>
    {
        #[cfg(target_arch = "wasm32")]
        if self.windows.len() >= 1 { return None; }
        if self.windows.len() >= 32 { return None; }

        let data = WindowData { winit_window: None, winit_id: None, graphics: WindowGraphicsAsset::Loading(WindowGraphicsLoadingState::Pending), param, dirty: true, id: ___() };
        let id = self.windows.insert(data);
        self.windows[id].id = id;
        self.any_dirty = true;

        unsafe
        {
            Some(Window::from_id(id))
        }
    }

    fn main_window(&self) -> Option<&Window> { self.main_window.as_ref() }
    fn main_window_mut(&mut self) -> Option<&mut Window> { self.main_window.as_mut() }

    fn remove_window(&mut self, mut window : Window) -> WindowData
    {
        let id = window.id();
        window.id.reset();

        let data = self.windows.remove(id).expect("Invalid window");
        if let Some(winit_id) = data.winit_id()
        {
            self.lookup.remove(&winit_id);
        }
        data
    }
}
impl Drop for ContextWindows
{
    fn drop(&mut self) {
        self.main_window = None;
    }
}


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


#[derive(PartialEq, Clone, Default, Debug)]
pub struct WindowParam
{
    /// Title of the window, defaults to an empty string.
    title: String,
    size : Option<Point2>,
    position : Option<Point2>,
}



pub trait IWindowParam
{
    fn with_title(self, title: impl Into<String>) -> Self;
    fn with_size(self, size: impl Into<Option<Point2>>) -> Self;
    fn with_position(self, position: impl Into<Option<Point2>>) -> Self;
}

impl WindowParam
{
    pub fn new() -> Self { ___() }
}

impl IWindowParam for WindowParam
{
    fn with_title(mut self, title: impl Into<String>) -> Self { self.title = title.into(); self }
    fn with_size(mut self, size: impl Into<Option<Point2>>) -> Self { self.size = size.into(); self }
    fn with_position(mut self, position: impl Into<Option<Point2>>) -> Self { self.position = position.into(); self }
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
