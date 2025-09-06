use super::*;


pub struct Pen;

impl SingletonRef for Pen
{
    type Target = Drawer;

    fn try_as_ref() -> Option<&'static <Self as SingletonRef>::Target> {
        Gpu::try_as_ref().map(|gpu| &gpu.draw)
    }
}
impl Deref for Pen
{
    type Target=Drawer;
    fn deref(&self) -> &Self::Target { &Gpu.draw }
}
impl AsRef<Drawer> for Pen
{
    fn as_ref(&self) -> &Drawer { self.deref() }
}

impl SingletonMut for Pen
{
    fn try_as_mut() -> Option<&'static mut <Self as SingletonRef>::Target> {
        Gpu::try_as_mut().map(|gpu| &mut gpu.draw)
    }
}
impl DerefMut for Pen
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
impl AsMut<Drawer> for Pen
{
    fn as_mut(&mut self) -> &mut Drawer {
        Self::try_as_mut().unwrap()
    }
}