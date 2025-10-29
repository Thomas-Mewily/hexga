use super::*;

pub trait SaveToFs : Serialize
{
    fn save<P, Fs>(&self, path: P, fs: &mut Fs) -> IoResult where P: AsRefPath, Fs: FsWrite
    {
        fs.save(self, path)
    }
}
impl<T> SaveToFs for T where T: Serialize + ?Sized {}

pub trait SaveToDisk : Serialize
{
    fn save_to_disk<P>(&self, path: P) -> IoResult where P: AsRefPath
    {
        self.save(path, &mut FsDisk)
    }
}
impl<T> SaveToDisk for T where T: Serialize + ?Sized {}

