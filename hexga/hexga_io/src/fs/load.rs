use super::*;



#[cfg(feature = "serde")]
pub trait LoadFromFs: Load + for<'de> Deserialize<'de>
{
    fn load<P,Fs>(path: P, fs: &mut Fs) -> FileResult<Self> where P: AsRefPath, Fs: FsWrite
    {
        fs.load(path)
    }
}
#[cfg(feature = "serde")]
impl<T> LoadFromFs for T where T: Load + for<'de> Deserialize<'de> {}



pub trait LoadFromDisk : LoadFromFs
{
    fn load_from_disk<P>(path: P) -> FileResult<Self> where P: AsRefPath
    {
        Self::load(path, &mut FsDisk)
    }
}
impl<T> LoadFromDisk for T where T: LoadFromFs {}