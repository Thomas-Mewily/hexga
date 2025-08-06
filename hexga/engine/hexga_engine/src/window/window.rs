use std::default;

use super::*;

pub type WindowID = GenVecID<WindowData>;

/// Thread safety : only use it in the main thread.
/// Internally this is actually a handle to a [WindowData] managed by the [WindowManager]
#[derive(Debug)]
pub struct Window
{
    pub(crate) id : WindowID,
}

impl Deref for Window
{
    type Target=WindowData;
    fn deref(&self) -> &Self::Target { ctx_or_init().window.get(self.id()).expect("Invalid window") }
}
impl DerefMut for Window
{
    fn deref_mut(&mut self) -> &mut Self::Target { ctx_mut_or_init().window.get_mut(self.id()).expect("Invalid window") }
}

impl Drop for Window
{
    fn drop(&mut self)
    {
        if self.id().is_not_null()
        {
            ctx_mut().window.remove_window(unsafe { Window::from_id(self.id()) });
            self.id.reset();
        }
    }
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

/*
pub trait WindowIDExtension
{
    fn new(param : WindowParam) -> Option<Window<'static>>;
    fn remove(self) -> Option<WindowData>;
}

impl WindowIDExtension for WindowID
{
    fn new(param : WindowParam) -> Option<Window<'static>>
    {
        ctx_mut().window.new_window(param)
    }

    fn remove(self) -> Option<WindowData>
    {
        ctx_mut().window.remove_window(self)
    }
}
*/

pub(crate) type WinitWindowID = winit::window::WindowId;

#[cfg(target_arch = "wasm32")]
pub type WinitWindowPtrKind<T> = std::rc::Rc<T>;

#[cfg(not(target_arch = "wasm32"))]
pub type WinitWindowPtrKind<T> = std::sync::Arc<T>;

pub(crate) type WinitWindow = winit::window::Window;

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

#[allow(dead_code)]
pub struct WindowData
{
    // window (and surface) are destroyed when pausing/resumed
    pub(crate) winit_window : Option<WinitWindowPtr>,
    pub(crate) winit_id     : Option<WinitWindowID>,
    pub(crate) graphics     : Asset<WindowGraphics>,

    pub(crate) param       : WindowParam,
    pub(crate) dirty : bool,
}

impl Deref for WindowData
{
    type Target=WindowParam;
    fn deref(&self) -> &Self::Target {
        &self.param
    }
}

impl Debug for WindowData
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("id", &self.id())
            .field("param", &self.param)
            .finish()
    }
}

impl WindowData
{
    pub fn id(&self) -> WindowID { self.param.id }
    pub(crate) fn winit_id(&self) -> Option<WinitWindowID> { self.winit_id }

    pub fn param(&self) -> &WindowParam { &self.param }

    pub fn physical_size(&self) -> Point2 { self.winit_window.as_ref().map(|w| w.inner_size().convert()).unwrap_or(one()) }
    pub fn logical_size(&self) -> Vec2 { self.physical_size().to_vec2() / self.param.dpi }

    pub(crate) fn winit_window(&self) -> Option<&WinitWindowPtr> { self.winit_window.as_ref() }

    pub fn childs(&self) -> &[WindowID] { &self.param.childs }
    pub fn add_child(&mut self, child: WindowID) -> bool
    {
        if !self.param.childs.contains(&child)
        {
            self.param.childs.push(child);
            true
        }else
        {
            false
        }
    }
    pub fn remove_child(&mut self, child: WindowID) -> bool
    {
        if let Some(pos) = self.param.childs.iter().position(|c| *c == child)
        {
            self.param.childs.remove(pos);
            true
        }else
        {
            false
        }
    }
}


pub(crate) type WgpuSurface = wgpu::Surface<'static>;

impl WindowData
{

    pub(crate) async fn request_surface<UserEvent>(instance : wgpu::Instance, surface : WgpuSurface, size : Point2, id: WindowID, proxy : EventLoopProxy<AppInternalEvent<UserEvent>>) where UserEvent: IUserEvent
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
            power_preference: wgpu::PowerPreference::default(), // Power preference for the device
            force_fallback_adapter: false, // Indicates that only a fallback ("software") adapter can be used
            compatible_surface: Some(&surface), // Guarantee that the adapter can render to this surface
        })
        .await
        .expect("Could not get an adapter (GPU).");


        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(), // Specifies the required features by the device request. Fails if the adapter can't provide them.
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::Performance,
                    trace: wgpu::Trace::Off,
                }
            )
            .await
            .expect("Failed to get device");

        //let size = window.inner_size();
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

impl WindowData
{
    pub fn handle_event(&mut self, event : WindowEventKind)
    {
        match event
        {
            WindowEventKind::Resize(size) => self.resize(size),
            WindowEventKind::Move(pos) => self.set_pos(pos),
            WindowEventKind::Open => self.open(),
            WindowEventKind::Close => self.close(),
            WindowEventKind::Destroy => {},
            WindowEventKind::Draw => {},
        }
    }

    pub(crate) fn set_dirty(&mut self)
    {
        self.dirty = true;
        Windows.any_dirty = true;
    }

    pub fn resize(&mut self, size: Point2)
    {
        self.param.size = size;
        match self.graphics.get_mut()
        {
            Some(g) => g.resize(size),
            None => self.set_dirty(),
        }
    }

    pub fn set_pos(&mut self, pos: Point2)
    {
        self.param.position = pos;
        match self.winit_window.as_mut()
        {
            Some(w) => w.set_outer_position(winit::dpi::PhysicalPosition::new(pos.x, pos.y)),
            None => self.set_dirty(),
        }
    }

    pub fn set_open(&mut self, open: bool)
    {
        if self.param.open != open
        {
            self.param.open = open;

            assert_ne!(self.winit_window.is_some(), open);
            assert_ne!(self.winit_id.is_some(), open);

            if open
            {
                self.set_dirty();
            }
            else
            {
                self.param.open = false;
                self.winit_window = None;
                Windows.lookup.remove(&self.winit_id.unwrap());
                self.winit_id = None;
                self.graphics = Asset::Pending(());
            }
        }
    }
    pub(crate) fn update_dirty<UserEvent>(&mut self, lookup: &mut WindowLookupId, gfx : &Graphics, event_loop: &WinitActiveEventLoop, proxy : &EventLoopProxy<AppInternalEvent<UserEvent>>) where UserEvent: IUserEvent
    {
        if !self.dirty { return; }
        self.dirty = false;

        if self.is_open() && self.winit_window.is_none()
        {
            // winit window creation
            debug_assert!(self.winit_id().is_none());

            let mut win_attr = WinitWindow::default_attributes();

            #[cfg(not(target_arch = "wasm32"))]
            {
                win_attr = win_attr.with_title(self.param().title());
            }

            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowAttributesExtWebSys;
                win_attr = win_attr.with_append(true);
            }

            if !self.default_size_and_position
            {
                win_attr = win_attr.with_position(winit::dpi::PhysicalPosition::new(self.position.x, self.position.y));
                win_attr = win_attr.with_inner_size(winit::dpi::PhysicalSize::new(self.size.x, self.size.y))
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
        }

        self.set_pos(self.position());
        self.resize(self.size());
        self.set_cursor_icon(self.cursor_icon());
        self.set_cursor_grab(self.cursor_grab());
        self.set_cursor_visible(self.is_cursor_visible());
        self.set_transparency_support(self.support_transparency());

        let title = self.title().to_owned();
        self.set_title(title); // I don't like this clone

        self.set_level(self.level);
    }

    pub fn set_cursor_icon(&mut self, cursor_icon : CursorIcon)
    {
        self.param.cursor = cursor_icon;
        match self.winit_window.as_mut()
        {
            Some(w) => { let _ = w.set_cursor(winit::window::Cursor::Icon(cursor_icon.into())); },
            None => self.set_dirty(),
        }
    }

    pub fn set_transparency_support(&mut self, support_transparency: bool)
    {
        self.param.transparent = support_transparency;
        match self.winit_window.as_mut()
        {
            Some(w) => { let _ = w.set_transparent(support_transparency); },
            None => self.set_dirty(),
        }
    }

    pub fn set_cursor_grab(&mut self, cursor_grab : CursorGrab)
    {
        self.param.cursor_grab = cursor_grab;
        match self.winit_window.as_mut()
        {
            Some(w) => { let _ = w.set_cursor_grab(cursor_grab.into()); },
            None => self.set_dirty(),
        }
    }

    pub fn set_cursor_visible(&mut self, cursor_visible : bool)
    {
        self.param.cursor_visible = cursor_visible;
        match self.winit_window.as_mut()
        {
            Some(w) => { let _ = w.set_cursor_visible(cursor_visible); },
            None => self.set_dirty(),
        }
    }

    pub fn set_title(&mut self, title : impl Into<String>)
    {
        self.param.title = title.into();
        match self.winit_window.as_mut()
        {
            Some(w) => { let _ = w.set_title(&self.param.title); },
            None => self.set_dirty(),
        }
    }

    pub fn set_level(&mut self, level : WindowLevel)
    {
        self.param.level = level;
        match self.winit_window.as_mut()
        {
            Some(w) => { let _ = w.set_window_level(level.into()); },
            None => self.set_dirty(),
        }
    }


    pub fn set_close_when_parent_exit(&mut self, close_when_parent_exit : bool)
    {
        self.param.close_when_parent_exit = close_when_parent_exit;
    }
    /*
    pub fn set_dpi(&mut self, dpi: float)
    {
        self.param.dpi = dpi;
        todo!("update the surface");
    }
    */

    fn close(&mut self) { self.set_open(false); }
    fn open(&mut self) { self.set_open(true); }
}

#[bitindex]
#[repr(u8)]
pub enum WindowButton
{
    Close,
    Minimize,
    Maximize,
}

impl Into<winit::window::WindowButtons> for WindowButtonFlags
{
    fn into(self) -> winit::window::WindowButtons
    {
        let mut buttons = winit::window::WindowButtons::empty();
        if self.contains(WindowButton::Close) {
            buttons |= winit::window::WindowButtons::CLOSE;
        }
        if self.contains(WindowButton::Minimize) {
            buttons |= winit::window::WindowButtons::MINIMIZE;
        }
        if self.contains(WindowButton::Maximize) {
            buttons |= winit::window::WindowButtons::MAXIMIZE;
        }
        buttons
    }
}

/// A window level groups windows with respect to their z-position.
///
/// The relative ordering between windows in different window levels is fixed.
/// The z-order of a window within the same window level may change dynamically on user interaction.
///
/// ## Platform-specific
///
/// - **iOS / Android / Web / Wayland:** Unsupported.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowLevel
{
    /// The window will always be below normal windows.
    ///
    /// This is useful for a widget-based app.
    AlwaysOnBottom,

    /// The default.
    #[default]
    Normal,

    /// The window will always be on top of normal windows.
    AlwaysOnTop,
}

impl Into<winit::window::WindowLevel> for WindowLevel
{
    fn into(self) -> winit::window::WindowLevel {
        match self {
            WindowLevel::AlwaysOnBottom => winit::window::WindowLevel::AlwaysOnBottom,
            WindowLevel::Normal => winit::window::WindowLevel::Normal,
            WindowLevel::AlwaysOnTop => winit::window::WindowLevel::AlwaysOnTop,
        }
    }
}


//#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
//#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(PartialEq, PartialOrd, Clone)]
pub struct WindowParam
{
    /// Title of the window, defaults to an empty string.
    title: String,
    pub(crate) id : WindowID,

    size : Point2,
    position : Point2,

    default_size_and_position : bool,

    resizable : bool,

    transparent : bool,
    visible: bool,

    /// is the window open or close
    pub(crate) open : bool,

    buttons : WindowButtonFlags,
    level : WindowLevel,

    icon: Option<Icon>,

    cursor    : CursorIcon,
    cursor_grab    : CursorGrab,
    cursor_visible : bool,

    dpi : float,

    // TODO: remove it ?
    close_when_parent_exit : bool,

    childs : Vec<WindowID>,
}


impl Debug for WindowParam
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowParam")
            .field("title", &self.title)
            .field("size", &self.size)
            .field("position", &self.position)
            .field("resizable", &self.resizable)
            .field("visible", &self.visible)
            .field("transparent", &self.transparent)
            .field("buttons", &self.buttons)
            .field("level", &self.level)
            .field("icon", &self.icon)
            .field("transparent", &self.transparent)
            .field("cursor_icon", &self.cursor)
            .field("cursor_grab", &self.cursor_grab)
            .field("cursor_visible", &self.cursor_visible)
            .field("dpi", &self.dpi)
            .field("close_when_parent_exit", &self.close_when_parent_exit)
            .field("childs", &self.childs)
            .finish()
    }
}

impl Into<winit::window::WindowAttributes> for WindowParam
{
    fn into(self) -> winit::window::WindowAttributes {
        let mut att = winit::window::Window::default_attributes();

        att.title = self.title;
        att.inner_size = self.default_size_and_position.then(|| winit::dpi::Size::Logical(winit::dpi::LogicalSize::new(self.size.x as _, self.size.y as _)));
        att.position = self.default_size_and_position.then(|| winit::dpi::Position::Logical(winit::dpi::LogicalPosition::new(self.position.x as _, self.position.y as _)));
        att.resizable = self.resizable;
        att.visible = self.visible;
        att.transparent = self.transparent;
        att.decorations = self.buttons != WindowButtonFlags::ZERO;
        att.window_level = self.level.into();
        att.window_icon = self.icon.map(|icon|
        {
            let (size, rgba) = icon.image.into_size_and_values();
            let rgba2 = rgba.into_iter().map(|v| v.to_array4()).flatten().collect();
            winit::window::Icon::from_rgba(rgba2, size.x as _, size.y as _).unwrap()
        });
        att.cursor = winit::window::Cursor::Icon(self.cursor.into());
        att.enabled_buttons = self.buttons.into();
        att
    }
}

impl Default for WindowParam
{
    fn default() -> Self { Self::new() }
}
impl WindowParam
{
    pub fn new() -> Self
    {
        Self
        {
            title: ___(),
            size: ___(),
            position: ___(),
            default_size_and_position: true,
            resizable: true,
            visible: true,
            buttons: WindowButton::Close | WindowButton::Minimize | WindowButton::Maximize,
            level: ___(),
            icon: ___(),
            open: true,
            cursor: ___(),
            cursor_grab: ___(),
            dpi: 1.,
            cursor_visible: true,
            close_when_parent_exit: true,
            childs: ___(),
            id: ___(),
            transparent: false,
        }
    }

    pub fn title(&self) -> &str { &self.title }
    //pub fn set_title(&mut self, title : impl Into<String>) -> &mut Self { self.title = title.into(); self }
    pub fn with_title(mut self, title: impl Into<String>) -> Self { self.title = title.into(); self }

    pub fn size(&self) -> Point2 { self.size }
    //pub fn set_size(&mut self, size : Point2) -> &mut Self { self.size = size; self }
    pub fn with_size(mut self, size: Point2) -> Self { self.size = size; self }

    pub fn position(&self) -> Point2 { self.position }
    //pub fn set_position(&mut self, position : Point2) -> &mut Self { self.position = position; self }
    pub fn with_position(mut self, position: Point2) -> Self { self.position = position; self }

    pub fn have_default_size_and_position(&self) -> bool { self.default_size_and_position }
    //pub fn set_default_size_and_position(&mut self, default_size_and_position : bool) -> &mut Self { self.default_size_and_position = default_size_and_position; self }
    pub fn with_default_size_and_position(mut self, default_size_and_position : bool) -> Self { self.default_size_and_position = default_size_and_position; self }

    pub fn resizable(&self) -> bool { self.resizable }
    //pub fn set_resizable(&mut self, resizable: bool) -> &mut Self { self.resizable = resizable; self }
    pub fn with_resizable(mut self, resizable: bool) -> Self { self.resizable = resizable; self }

    pub fn buttons(&self) -> WindowButtonFlags { self.buttons }
    //pub fn set_buttons(&mut self, buttons: impl Into<WindowButtonFlags>) -> &mut Self { self.buttons = buttons.into(); self }
    pub fn with_buttons(mut self, buttons: impl Into<WindowButtonFlags>) -> Self { self.buttons = buttons.into(); self }

    pub fn level(&self) -> WindowLevel { self.level }
    //pub fn set_level(&mut self, level: WindowLevel) -> &mut Self { self.level = level; self }
    pub fn with_level(mut self, level: WindowLevel) -> Self { self.level = level; self }

    pub fn icon(&self) -> Option<&Icon> { self.icon.as_ref() }
    pub fn icon_mut(&mut self) -> Option<&mut Icon> { self.icon.as_mut() }
    //pub fn set_icon(&mut self, icon: Option<Icon>) -> &mut Self { self.icon = icon; self }
    pub fn with_icon(mut self, icon: Option<Icon>) -> Self { self.icon = icon; self }




    pub fn cursor_icon(&self) -> CursorIcon { self.cursor }
    //pub fn set_cursor_icon(&mut self, cursor: CursorIcon) -> &mut Self { self.cursor = cursor; self }
    pub fn with_cursor_icon(mut self, cursor: CursorIcon) -> Self { self.cursor = cursor; self }

    pub fn cursor_grab(&self) -> CursorGrab { self.cursor_grab }
    //pub fn set_cursor_grab(&mut self, cursor_grab: CursorGrab) -> &mut Self { self.cursor_grab = cursor_grab; self }
    pub fn with_cursor_grab(mut self, cursor_grab: CursorGrab) -> Self { self.cursor_grab = cursor_grab; self }

    pub fn is_cursor_visible(&self) -> bool { self.cursor_visible }
    //pub fn set_cursor_visible(&mut self, cursor_visible: bool) -> &mut Self { self.cursor_visible = cursor_visible; self }
    pub fn with_cursor_visible(mut self, cursor_visible: bool) -> Self { self.cursor_visible = cursor_visible; self }

    pub fn is_open(&self) -> bool { self.open }
    //pub fn set_open(&mut self, open: bool) -> &mut Self { self.open = open; self }
    pub fn with_open(mut self, open: bool) -> Self { self.open = open; self }

    pub fn is_close(&self) -> bool { !self.open }
    //pub fn set_close(&mut self, close: bool) -> &mut Self { self.open = !close; self }
    pub fn with_close(mut self, close: bool) -> Self { self.open = !close; self }

    pub fn dpi(&self) -> float { self.dpi }
    //pub fn set_dpi(&mut self, dpi: float) -> &mut Self { self.dpi = dpi; self }
    pub fn with_dpi(mut self, dpi: float) -> Self { self.dpi = dpi; self }

    /// Do the window support transparency
    pub fn support_transparency(&self) -> bool { self.transparent }
    /// Do the window support transparency
    //pub fn set_transparency_support(&mut self, transparent: bool) -> &mut Self { self.transparent = transparent; self }
    /// Do the window support transparency
    pub fn with_transparency_support(mut self, transparent: bool) -> Self { self.transparent = transparent; self }

    pub fn will_close_when_parent_exit(&self) -> bool { self.close_when_parent_exit }
    //pub fn set_close_when_parent_exit(&mut self, close_when_parent_exit: bool) -> &mut Self { self.close_when_parent_exit = close_when_parent_exit; self }
    pub fn with_close_when_parent_exit(mut self, close_when_parent_exit: bool) -> Self { self.close_when_parent_exit = close_when_parent_exit; self }

    pub fn child(&self) -> &[WindowID] { &self.childs }
    /*
    pub fn with_childs(mut self, childs: Vec<WindowID>) -> Self {
        self.childs = childs;
        self
    }
    */
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
/// Icon image in three levels of detail.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct Icon {
    /*
    /// 16 * 16 image of RGBA pixels (each 4 * u8) in row-major order.
    pub rgba_16x16: [u8; 16 * 16 * 4],
    /// 32 x 32 image of RGBA pixels (each 4 * u8) in row-major order.
    pub rgba_32x32: [u8; 32 * 32 * 4],
    /// 64 x 64 image of RGBA pixels (each 4 * u8) in row-major order.
    pub rgba_64x64: [u8; 64 * 64 * 4],
    */

    image : Image,
}

impl Icon
{
    pub fn from_image(image : Image) -> Self { Self { image }}
}
