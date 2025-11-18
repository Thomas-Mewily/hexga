use super::*;

pub struct Io;


impl Io
{
    pub fn load_bytes<P>(self, path: P) -> IoResult<Vec<u8>> where P: AsRef<Path>
    {
        fs::load_bytes(path.as_ref())
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
