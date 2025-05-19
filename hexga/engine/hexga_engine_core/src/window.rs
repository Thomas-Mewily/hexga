use crate::*;

pub use hexga_engine_window::{CursorIcon, LoopWindow, WindowConfig};


pub trait LoopWindowExtension
{
    fn get_position(&mut self) -> Point2;
    fn set_position(&mut self, pos : Point2); 

    fn get_size(&mut self) -> Point2;
    fn set_size(&mut self, size : Point2);
}
impl<T> LoopWindowExtension for T where T : LoopWindow
{
    fn get_position(&mut self) -> Point2 
    {
        let (x,y) = self.get_position_tuple();
        point2(x as _, y  as _)
    }

    fn set_position(&mut self, pos : Point2) {
        self.set_position_tuple((pos.x as _, pos.y as _));
    }

    fn get_size(&mut self) -> Point2 {
        let (x,y) = self.get_size_tuple();
        point2(x as _, y  as _)
    }

    fn set_size(&mut self, size : Point2) {
        self.set_size_tuple((size.x as _, size.y as _ ));
    }
}

pub mod prelude
{
    use crate::*;
    pub use super::LoopWindowExtension;
    pub use hexga_engine_window::prelude::*;
}