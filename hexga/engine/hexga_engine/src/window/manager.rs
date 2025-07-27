use super::*;

declare_context!(Windows, WindowManager, window);

#[derive(Debug, Default)]
pub struct WindowManager
{
    windows : GenVec<Window>,
}
impl WindowManager
{
    pub fn new() -> Self { ___() }
}

pub trait IWindowManager
{
    fn new_window(&mut self, param: WindowParam) -> WindowID;
    fn window(&mut self, id : WindowID) -> Option<&Window>;
    fn window_exist(&mut self, id : WindowID) -> bool { self.window(id).is_some() }
    fn delete_window(&mut self, id : WindowID);

    fn update_window(&mut self, id : WindowID, param: WindowParam);
}

impl IWindowManager for WindowManager
{
    fn new_window(&mut self, param: WindowParam) -> WindowID
    {
        let mut w = Window { winit_window: None, winit_id: None, param, id: ___() };
        let id = self.windows.insert(w);
        self.windows[id].id = id;
        // todo emit the event to create the surface
        id
    }

    fn window(&mut self, id : WindowID) -> Option<&Window> {
        self.windows.get(id)
    }

    fn delete_window(&mut self, id : WindowID)
    {
        // Todo : delete the surface...
        self.windows.remove(id);
    }

    fn update_window(&mut self, id : WindowID, param: WindowParam) {
        self.windows.get_mut(id).map(|w| w.param = param);
    }
}