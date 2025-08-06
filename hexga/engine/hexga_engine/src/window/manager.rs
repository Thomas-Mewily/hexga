use std::collections::HashMap;

use super::*;

declare_context!(Windows, WindowManager, window);

pub(crate) type WindowLookupId = HashMap<WinitWindowID, WindowID>;

#[derive(Debug, Default)]
pub struct WindowManager
{
    pub(crate) lookup  : WindowLookupId,
    pub(crate) windows : GenVec<WindowData>,

    main_window : Option<Window>,

    actives_stack  : Vec<WindowID>,
    pub(crate) any_dirty      : bool,
}

impl WindowManager
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

    pub(crate) fn update_dirty<UserEvent>(&mut self, gfx : &Graphics, event_loop: &WinitActiveEventLoop, proxy : &EventLoopProxy<AppInternalEvent<UserEvent>>) where UserEvent: IUserEvent
    {
        if !self.any_dirty { return; }
        self.any_dirty = false;

        for window in self.windows.values_mut()
        {
            window.update_dirty(&mut self.lookup, gfx, event_loop, proxy);
            self.any_dirty |= window.dirty;
        }
    }

    pub(crate) fn handle_internal_event(&mut self, event : WindowInternalEvent)
    {
        match event.kind
        {
            WindowInternalEventKind::SurfaceCreated(window_graphics_result) =>
            {
                let Some(window) = self.get_mut(event.id) else { return; };

                match window_graphics_result
                {
                    Ok(gfx) =>
                    {
                        window.graphics = Asset::Loaded(gfx);
                    },
                    Err(_) =>
                    {
                        window.graphics = Asset::Error(());
                    },
                }
            },
        }
    }

    pub fn handle_event(&mut self, event: WindowEvent)
    {
        let window = self.get_mut(event.id).expect("Invalid window");
        window.handle_event(event.kind);
    }
}

impl Drop for WindowManager
{
    fn drop(&mut self) {
        self.main_window = None;
    }
}

/// Thread safety : can only be called from the main thread
pub trait IWindowManager
{
    fn new_window(&mut self, param: WindowParam) -> Option<Window>;
    fn remove_window(&mut self, window : Window) -> WindowData;

    fn main_window(&self) -> Option<&Window>;
    fn main_window_mut(&mut self) -> Option<&mut Window>;
}

impl IWindowManager for WindowManager
{
    fn new_window(&mut self, param: WindowParam) -> Option<Window>
    {
        #[cfg(target_arch = "wasm32")]
        if self.windows.len() >= 1 { return None; }
        if self.windows.len() >= 32 { return None; }

        let data = WindowData { winit_window: None, winit_id: None, graphics: Asset::Pending(()), param, dirty: true };
        let id = self.windows.insert(data);
        self.windows[id].param.id = id;
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

pub(crate) type WindowSurfaceResult = Result<WindowGraphics, ()>;

#[derive(Debug)]
pub(crate) enum WindowInternalEventKind
{
    SurfaceCreated(WindowSurfaceResult),
}

pub(crate) type WindowInternalEvent = WindowEvent<WindowInternalEventKind>;

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



/*
pub struct WindowsEvent
{
    WindowEvent
}
*/

/*
pub enum WindowsEvent
{
    WindowEvent(WindowEvent)
}
*/

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

impl WindowGraphics
{
    pub fn resize(&mut self, size: Point2)
    {
        let size = size.max_with(one()).map(|v| v as _);
        if self.config.width != size.x || self.config.height != size.y
        {
            self.config.width = size.x;
            self.config.height = size.y;
            self.surface.configure(&self.device, &self.config);
        }
    }
}