use super::*;

pub struct Io;


impl Io
{
    pub fn load_bytes<P>(self, path: P) -> IoResult<Vec<u8>> where P: AsRef<Path>
    {
        fs::load_bytes(path.as_ref())
    }

    pub fn load<P,T>(self, path: P) -> IoResult<T> where P: AsRef<Path>, T: Load
    {
        let path = path.as_ref();
        let extension = path.extension_or_empty();

        let (bytes, extension) = match Io.load_bytes(path)
        {
            Ok(bytes) => (bytes, extension),
            Err(err) =>
            {
                let mut found = None;

                for ext in T::load_extensions()
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
        T::load_from_bytes(&bytes, extension).map_err(|e| IoError::new(path, e).when_reading())
    }

    pub fn load_string<P>(self, path: P) -> IoResult<String> where P: AsRef<Path>
    {
        let bytes = fs::load_bytes(path.as_ref())?;
        String::from_utf8(bytes).map_err(|e| IoError::new(path.as_ref(), e))
    }

    pub fn save_bytes<P>(self, path: P, bytes: &[u8]) -> IoResult where P: AsRef<Path>
    {
        fs::save_bytes(path.as_ref(), bytes)
    }

    pub fn save_str<P>(self, path: P, str: &str) -> IoResult where P: AsRef<Path>
    {
        self.save_bytes(path, str.as_bytes())
    }

    pub fn save<P,T>(self, path: P, value: &T) -> IoResult where P: AsRef<Path>, T: Save + ?Sized
    {
        let path = path.as_ref();
        let (bytes, extension) = value.save_to_bytes(path.extension_or_empty())
            .map_err(|e| IoError::new(path, e).when_writing())?;

        Io.save_bytes(&path.with_extension(extension.as_ref()), &bytes)
    }






    pub fn load_bytes_async<P,F>(self, path: P, on_loaded: F)
        where
        P: AsRef<Path>,
        F: FnOnce(IoResult<Vec<u8>>) + 'static
    {
        fs::load_bytes_async(path.as_ref(), on_loaded)
    }

    pub fn save_bytes_async<P,F>(self, path: P, bytes: Vec<u8>, on_saved: F)
        where
        P: AsRef<Path>,
        F: FnOnce(IoResult) + 'static
    {
        fs::save_bytes_async(path.as_ref(), bytes, on_saved)
    }
}
