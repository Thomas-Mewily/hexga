use super::*;


pub trait LoadFromDisk : Load
{
    fn load_from_disk<P>(path: P) -> IoResult<Self>
        where P: AsRef<Path>
    {
        let path = path.as_ref();
        let extension = path.extension_or_empty();

        let (bytes, extension) = match Io.load_bytes(path)
        {
            Ok(bytes) => (bytes, extension),
            Err(err) =>
            {
                let mut found = None;

                for ext in Self::load_extensions()
                {
                    if ext == extension { continue; }

                    if let Ok(bytes) = Io.load_bytes(&path.with_extension(ext))
                    {
                        found = Some((bytes, ext));
                        break;
                    }
                }
                match found
                {
                    Some(bytes_and_extension) => bytes_and_extension,
                    None => return Err(err),
                }
            },
        };
        Self::load_from_bytes(&bytes, extension).map_err(|e| IoError::new(path, e).when_reading())
    }
}
impl<T> LoadFromDisk for T where T: Load + ?Sized {}