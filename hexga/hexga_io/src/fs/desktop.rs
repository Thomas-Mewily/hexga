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
    if let Some(parent) = path.parent()
    {
        for prefixes in parent.iter_prefixes()
        {
            let std_path = std::path::Path::new(prefixes);

            if std_path.exists() {
                if std_path.is_file() {
                    // Remove any file that blocks the directory
                    std::fs::remove_file(std_path).map_err(|e| IoError::new(prefixes, e).when_writing())?;
                    std::fs::create_dir(std_path).map_err(|e| IoError::new(prefixes, e).when_writing())?;
                }
                // else: already a directory, nothing to do
            } else {
                std::fs::create_dir(std_path).map_err(|e| IoError::new(prefixes, e).when_writing())?;
            }
        }
    }
    std::fs::write(StdPath::new(path), bytes).map_err(|e| IoError::new(path, e).when_writing())
}

pub(crate) fn save_bytes_async<F>(path: &path, bytes: Vec<u8>, on_saved: F)
    where F: FnOnce(IoResult) + 'static
{
    on_saved(save_bytes(path, &bytes))
}