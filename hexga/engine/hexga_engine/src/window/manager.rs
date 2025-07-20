use super::*;

pub struct WindowManager
{
    actives : Vec<WindowID>,
}
impl WindowManager
{

}

pub trait IWindowManager
{
    fn new_window(&mut self, param : WindowParam) -> WindowID;
    fn window(&mut self, id : WindowID) -> Option<&Window>;
    fn window_exist(&mut self, id : WindowID) -> bool { self.window(id).is_some() }
    fn delete_window(&mut self, id : WindowID);


}