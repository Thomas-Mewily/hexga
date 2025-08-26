use super::*;


/// The window manager
#[derive(Debug, Default)]
pub struct WindowManager
{
    pub(crate) lookup  : WindowLookupID,
    pub(crate) windows : GenVec<WindowData>,
    pub(crate) any_dirty : bool,

    main_window : Option<Window>,
    
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
}


pub trait IContextWindows
{
    fn new_window(&mut self, param: WindowParam) -> Option<Window>;
    fn remove_window(&mut self, window : Window) -> WindowData;

    fn main_window(&self) -> Option<&Window>;
    fn main_window_mut(&mut self) -> Option<&mut Window>;
}

impl IContextWindows for WindowManager
{
    fn new_window(&mut self, param: WindowParam) -> Option<Window>
    {
        #[cfg(target_arch = "wasm32")]
        if self.windows.len() >= 1 { return None; }
        if self.windows.len() >= 32 { return None; }

        //param.rectangle.to_rectangle(window, screen)
        todo!();

        let data = WindowData { winit_window: None, winit_id: None, graphics: WindowGraphicsAsset::Loading(WindowGraphicsLoadingState::Pending), param, dirty: true, id: ___(), rectangle: todo!() };
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
impl Drop for WindowManager
{
    fn drop(&mut self) {
        self.main_window = None;
    }
}

impl WindowManager
{
    pub(crate) fn update<UserEvent>(&mut self, gfx : &Graphics, event_loop: &WinitActiveEventLoop, proxy : &WinitEventLoopProxy<AppInternalEvent<UserEvent>>) where UserEvent: IUserEvent
    {
        if !self.any_dirty { return; }
        self.any_dirty = false;
        
        for window in self.windows.values_mut()
        {
            window.update_dirty(&mut self.lookup, gfx, event_loop, proxy);
            self.any_dirty |= window.dirty;
        }
    }
}