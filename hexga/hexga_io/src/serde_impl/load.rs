use super::*;


pub trait LoadFromDisk : Load
{
    fn load_from_disk<P>(path: P) -> IoResult<Self>
        where P: AsRef<Path>
    {
        Io.load(path)
    }
}
impl<T> LoadFromDisk for T where T: Load + ?Sized {}