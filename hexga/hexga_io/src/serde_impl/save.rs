use super::*;


pub trait SaveToDisk : Save
{
    fn save_to_disk<P>(&self, path: P) -> IoResult
        where P: AsRef<Path>
    {
        Io.save(path, self)
    }
}
impl<T> SaveToDisk for T where T: Save {}