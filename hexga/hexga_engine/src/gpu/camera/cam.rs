use super::*;

pub mod prelude
{
    pub use super::Cam;
}

pub struct Cam;

impl SingletonRef for Cam
{
    type Target = CameraManager;

    fn try_as_ref() -> Option<&'static <Self as SingletonRef>::Target> {
        Pen::try_as_ref().map(|pen| pen.camera())
    }
}
impl Deref for Cam
{
    type Target=CameraManager;
    fn deref(&self) -> &Self::Target { Self::try_as_ref().unwrap() }
}
impl AsRef<CameraManager> for Cam
{
    fn as_ref(&self) -> &CameraManager { self.deref() }
}

impl SingletonMut for Cam
{
    fn try_as_mut() -> Option<&'static mut <Self as SingletonRef>::Target> {
        Pen::try_as_mut().map(|pen| pen.camera_mut())
    }
}
impl DerefMut for Cam
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
impl AsMut<CameraManager> for Cam
{
    fn as_mut(&mut self) -> &mut CameraManager {
        Self::try_as_mut().unwrap()
    }
}