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

impl GetPosition<float,3> for CameraManager
{
    fn pos(&self) -> Vector<float,3> { self.cameras.pos() }
}
impl SetPosition<float,3> for CameraManager
{
    fn set_pos(&mut self, pos : Vector<float,3>) -> &mut Self { self.cameras.set_pos(pos); self }
}
impl GetScale<float,3> for CameraManager
{
    fn scale(&self) -> Vector<float,3> { self.cameras.scale() }
}
impl SetScale<float,3> for CameraManager
{
    fn set_scale(&mut self, scale : Vector<float,3>) -> &mut Self { self.cameras.set_scale(scale); self }
}
impl RotationX<float> for CameraManager
{
    fn rotate_x(&mut self, angle : AngleOf<float>) -> &mut Self { self.cameras.rotate_x(angle); self }
}
impl RotationY<float> for CameraManager
{
    fn rotate_y(&mut self, angle : AngleOf<float>) -> &mut Self { self.cameras.rotate_y(angle); self }
}
impl RotationZ<float> for CameraManager
{
    fn rotate_z(&mut self, angle : AngleOf<float>) -> &mut Self { self.cameras.rotate_z(angle); self }
}