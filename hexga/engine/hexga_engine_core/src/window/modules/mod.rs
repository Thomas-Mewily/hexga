use crate::*;

pub use hexga_engine_window::modules::*;
pub use super::window::*;

pub trait IconExtension
{
    //pub fn with_16x16_icon(&mut self, icon : Image);
}

pub trait ContextWindowExtension
{
    fn get_position(&mut self) -> Point2;
    fn set_position(&mut self, pos : Point2); 

    fn get_size(&mut self) -> Point2;
    fn set_size(&mut self, size : Point2);
}
impl<T> ContextWindowExtension for T where T : ContextWindow
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
        let (x,y) = self.get_screen_size_tuple();
        point2(x as _, y  as _)
    }

    fn set_size(&mut self, size : Point2) {
        self.set_size_tuple((size.x as _, size.y as _ ));
    }
}