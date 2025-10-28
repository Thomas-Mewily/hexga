use super::*;


pub trait Load : for<'de> Deserialize<'de>
{
    fn load<P,Fs>(path: P, fs: &mut Fs) -> FileResult<Self> where P: AsRefPath, Fs: FsWrite
    {
        fs.load(path)
    }
}
impl<T> Load for T where T: for<'de> Deserialize<'de> {}

pub trait LoadFromDisk : for<'de> Deserialize<'de>
{
    fn load_from_disk<P>(path: P) -> FileResult<Self> where P: AsRefPath
    {
        Self::load(path, &mut FsDisk)
    }
}
impl<T> LoadFromDisk for T where T: for<'de> Deserialize<'de> {}