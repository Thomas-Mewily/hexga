use super::*;


pub trait SaveToDisk : Save
{
    fn save_to_disk<P>(&self, path: P) -> IoResult
        where P: AsRef<Path>
    {
        let path = path.as_ref();
        let (bytes, extension) = self.save_to_bytes(path.extension_or_empty())
            .map_err(|e| IoError::new(path, e).when_writing())?;

        Io.save_bytes(&path.with_extension(&extension), &bytes)
    }
}
impl<T> SaveToDisk for T where T: Save {}