use std::collections::HashMap;

use super::*;

declare_context!(Windows, WindowManager, window);

#[derive(Debug, Default)]
pub struct WindowManager
{
    lookup  : HashMap<WinitWindowID, WindowID>,
    windows : GenVec<WindowData>,

    main_window : Option<Window>,

    actives_stack  : Vec<WindowID>,
    any_dirty      : bool,
}

impl WindowManager
{
    pub fn new() -> Self { ___() }

    pub(crate) fn get(&self, id : WindowID) -> Option<&WindowData> { self.windows.get(id) }
    pub(crate) fn get_mut(&mut self, id : WindowID) -> Option<&mut WindowData> { self.windows.get_mut(id) }

    pub(crate) fn winit_id_to_window_id(&mut self, id : WinitWindowID) -> WindowID
    {
        self.lookup.get(&id).copied().unwrap_or_default()
    }

    pub(crate) fn init_main_window(&mut self, param : Option<WindowParam>)
    {
        self.main_window = param.map(|p| self.new_window(p));
    }

    pub(crate) fn update_dirty<UserEvent>(&mut self, gfx : &Graphics, event_loop: &WinitActiveEventLoop, proxy : &EventLoopProxy<AppInternalEvent<UserEvent>>) where UserEvent: IUserEvent
    {
        if !self.any_dirty { return; }
        self.any_dirty = false;

        for (id, window) in self.windows.iter_mut()
        {
            if !window.param_dirty { continue; }
            window.param_dirty = false;

            if window.winit_window.is_none()
            {
                debug_assert!(window.winit_id().is_none());

                let mut win_attr = WinitWindow::default_attributes();

                #[cfg(not(target_arch = "wasm32"))]
                {
                    win_attr = win_attr.with_title(window.param().title());
                }

                #[cfg(target_arch = "wasm32")]
                {
                    use winit::platform::web::WindowAttributesExtWebSys;
                    win_attr = win_attr.with_append(true);
                }

                let winit_window = WinitWindowPtr::new(
                    event_loop
                        .create_window(win_attr)
                        .expect("create window err."),
                );

                let winit_id = winit_window.winit_window().id();
                window.winit_id = Some(winit_id);
                self.lookup.insert(winit_id, window.id());
            }

            match &mut window.graphics
            {
                Asset::Pending(_) =>
                {
                    let winit_window = window.winit_window.as_ref().expect("winit_window should have been init just before").clone();
                    window.graphics = Asset::Loading(());
                    #[cfg(target_arch = "wasm32")]
                    wasm_bindgen_futures::spawn_local(WindowData::request_surface(gfx.instance.clone(), winit_window, id, proxy.clone()));

                    #[cfg(not(target_arch = "wasm32"))]
                    pollster::block_on(WindowData::request_surface(gfx.instance.clone(), winit_window, id, proxy.clone()));
                },
                Asset::Loading(_) => {},
                Asset::Loaded(_gfx) => {},
                Asset::Error(_) => { panic!("Can't create the window gfx"); },
            }
        }

        /*
        for (id, window) in self.windows.iter_mut()
        {
            if !window.param_dirty { continue; }
            window.param_dirty = false;

            match window.state
            {
                WindowState::Init =>
                {
                    window.state = WindowState::WaitSurface;
                    let mut win_attr = WinitWindow::default_attributes();

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        win_attr = win_attr.with_title(window.param().title.clone());
                    }

                    #[cfg(target_arch = "wasm32")]
                    {
                        use winit::platform::web::WindowAttributesExtWebSys;
                        win_attr = win_attr.with_append(true);
                    }

                    let winit_window = WinitWindowPtr::new(
                        event_loop
                            .create_window(win_attr)
                            .expect("create window err."),
                    );

                    debug_assert!(window.winit_id.is_none());
                    let winit_id = winit_window.winit_window().id();
                    window.winit_id = Some(winit_id);
                    self.lookup.insert(winit_id, window.id());

                    #[cfg(target_arch = "wasm32")]
                    wasm_bindgen_futures::spawn_local(WindowData::request_surface(gfx.instance.clone(), winit_window, id, proxy.clone()));

                    #[cfg(not(target_arch = "wasm32"))]
                    pollster::block_on(WindowData::request_surface(gfx.instance.clone(), winit_window, id, proxy.clone()));
                },
                WindowState::WaitSurface => {}
                WindowState::Close =>
                {
                    todo!()
                },
                WindowState::Initialized =>
                {
                    todo!()
                }
                WindowState::Error => todo!(),
            }
            //if window.winit_id
        }
        */
        todo!()

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
        todo!()
        /*
        match event
        {
            WindowEvent::SurfaceResult(window_surface) =>
            {
                if let Some(w) = self.windows.get_mut(window_surface.id)
                {
                    w.graphics = window_surface.graphics.ok();
                    if w.state == WindowState::WaitSurface
                    {
                        w.state = WindowState::Initialized;
                    }
                }
            }
            WindowEvent::Resize(id, size) =>
            {
                if let Some(w) = self.windows.get_mut(id)
                {
                    if let Some(gfx) = &mut w.graphics
                    {
                        gfx.config.width  = size.x.max(1) as _;
                        gfx.config.height = size.y.max(1) as _;
                        gfx.surface.configure(&gfx.device, &gfx.config);
                    }
                }
            },
            WindowEvent::Move(id, pos) =>
            {
                if let Some(w) = self.windows.get_mut(id)
                {
                    w.param.position = pos;
                }
            },
        }
        */
    }

    pub fn handle_event(&mut self, event: WindowEvent)
    {
        let window = self.get_mut(event.id).expect("Invalid window");
        window.handle_event(event.kind);
    }
}

/// Thread safety : can only be called from the main thread
pub trait IWindowManager
{
    fn new_window(&mut self, param: WindowParam) -> Window;
    fn main_window(&self) -> Option<&Window>;
    fn main_window_mut(&mut self) -> Option<&mut Window>;
    fn remove_window(&mut self, window : Window) -> WindowData;
}

impl IWindowManager for WindowManager
{
    fn new_window(&mut self, param: WindowParam) -> Window
    {
        let data = WindowData { winit_window: None, winit_id: None, graphics: Asset::Pending(()), param, param_dirty: true };
        let id = self.windows.insert(data);
        self.any_dirty = true;
        unsafe { Window::from_id(id) }
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