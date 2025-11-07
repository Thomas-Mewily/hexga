use super::*;


pub(crate) fn load_bytes(path: &path) -> IoResult<Vec<u8>>
{
    std::fs::read(StdPath::new(path)).map_err(|e| IoError::new(path, e).when_reading())
}

pub(crate) fn load_bytes_async<F>(path: &path, on_loaded: F)
    where F: FnOnce(IoResult<Vec<u8>>) + 'static
{
    on_loaded(load_bytes(path))
}


pub(crate) fn save_bytes(path: &path, bytes: &[u8]) -> IoResult
{
    std::fs::write(StdPath::new(path), bytes).map_err(|e| IoError::new(path, e).when_writing())
}

pub(crate) fn save_bytes_async<F>(path: &path, bytes: Vec<u8>, on_saved: F)
    where F: FnOnce(IoResult) + 'static
{
    on_saved(save_bytes(path, &bytes))
}