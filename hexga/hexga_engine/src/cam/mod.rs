use super::*;


#[derive(Debug, Default)]
pub struct CameraManager
{
    //camera : LastStack<Camera>,
    /// The current window size
    window      : Rect2P,
    screen_size : Option<Point2>,
}

impl CameraManager
{
    pub(crate) fn update(&mut self, event_loop: &WinitActiveEventLoop)
    {
        self.screen_size = event_loop.primary_monitor().map(|m| m.size().convert_point2());
        self.window.pos = zero();
        self.window.size = self.screen_size_px();
    }

    pub fn screen_px(&self) -> Rect2P { Rect2P::new_sized(self.screen_size_px()) }
    pub fn screen_size_px(&self) -> Point2 { self.screen_size.unwrap_or_zero()}

    pub fn screen(&self) -> Rect2 { self.screen_px().to_float() }
    pub fn screen_size(&self) -> Vec2 { self.screen_size_px().to_float() }

    /// The current rectangle of the current window
    pub fn window_px(&self) -> Rect2P { Windows.active().expect("No active window").rect() }
    /// The current size of the current window
    pub fn window_size_px(&self) -> Point2 { self.window_px().size() }

    /// The current rectangle of the current window
    pub fn window(&self) -> Rect2 { self.window_px().to_float() }
    /// The current size of the current window
    pub fn window_size(&self) -> Vec2 { self.window_size_px().to_float() }
}

declare_context_singleton!(Cam, CameraManager, camera);


pub mod prelude
{
    pub use super::{Cam};
}