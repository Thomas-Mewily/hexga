use super::*;

pub struct Io;


impl Io
{
    pub fn load_bytes(self, path: &path) -> IoResult<Vec<u8>>
    {
        fs::load_bytes(path)
    }

    pub fn load_string(self, path: &path) -> IoResult<String>
    {
        let bytes = fs::load_bytes(path)?;
        String::from_utf8(bytes).map_err(|e| IoError::new(path, e))
    }

    pub fn save_bytes(self, path: &path, bytes: &[u8]) -> IoResult
    {
        fs::save_bytes(path, bytes)
    }

    pub fn save_str(self, path: &path, str: &str) -> IoResult
    {
        self.save_bytes(path, str.as_bytes())
    }




    pub fn load_bytes_async<F>(self, path: &path, on_loaded: F)
        where F: FnOnce(IoResult<Vec<u8>>) + 'static
    {
        fs::load_bytes_async(path, on_loaded)
    }

    pub fn save_bytes_async<F>(self, path: &path, bytes: Vec<u8>, on_saved: F)
        where F: FnOnce(IoResult) + 'static
    {
        fs::save_bytes_async(path, bytes, on_saved)
    }
}
