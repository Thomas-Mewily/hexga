use super::*;


pub mod prelude
{
    pub use super::CameraManager;
}

#[derive(Default)]
pub struct CameraManager
{
    cameras: NonEmptyStack<Camera>,
}

impl CameraManager
{
    pub fn with_camera<C>(cam : &C) -> Self where C: ICamera { Self { cameras: NonEmptyStack::new(cam.to_camera()) } }
    pub fn new() -> Self { ___() }
}

impl CameraManager
{
    pub fn replace<C>(&mut self, cam: &C) -> &mut Self where C: ICamera { self.cameras.replace(cam.to_camera()); self }
}

impl ICamera for CameraManager
{
    fn matrix(&self) -> Mat4 { self.cameras.matrix() }
    fn have_depth(&self) -> bool { self.cameras.have_depth() }
    fn viewport(&self) -> Option<Rect2P> { self.cameras.viewport() }
}