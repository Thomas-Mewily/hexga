use super::*;


#[derive(Debug, Default)]
pub struct CameraManager
{
    //camera : LastStack<Camera>,
    /// The current window size
    window      : Rect2,
    screen_size : Option<Point2>,
}

impl CameraManager
{
    pub(crate) fn update(&mut self, event_loop: &WinitActiveEventLoop)
    {
        self.screen_size = event_loop.primary_monitor().map(|m| m.size().convert_point2());
        self.window.pos = zero();
        self.window.size = self.screen_size().to_vec2();
    }

    pub fn screen_size(&self) -> Point2 { self.screen_size.unwrap_or_zero() }
    pub fn screen_size_vec2(&self) -> Vec2 { self.screen_size().to_vec2() }
}

declare_context_singleton!(Cam, CameraManager, camera);


pub mod prelude
{
    pub use super::{Cam};
}