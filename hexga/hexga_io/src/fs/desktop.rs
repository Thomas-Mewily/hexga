use std::fs;
use super::*;


pub(crate) fn load_bytes(path: &Path) -> IoResult<Vec<u8>>
{
    std::fs::read(path).map_err(|e| IoError::new(path, e).when_reading())
}

pub(crate) fn save_bytes(path: &Path, bytes: &[u8]) -> Result<(), IoError> {
    if let Some(parent) = path.parent() {
        for prefix in parent.iter() {
            let std_path = Path::new(prefix);

            if std_path.exists() {
                if std_path.is_file() {
                    fs::remove_file(std_path).map_err(|e| IoError::new(std_path, e).when_writing())?;
                    fs::create_dir(std_path).map_err(|e| IoError::new(std_path, e).when_writing())?;
                }
            } else {
                fs::create_dir(std_path).map_err(|e| IoError::new(std_path, e).when_writing())?;
            }
        }
    }

    fs::write(path, bytes).map_err(|e| IoError::new(path, e).when_writing())
}

/*
pub(crate) fn save_bytes_async<F>(path: &Path, bytes: Vec<u8>, on_saved: F)
    where F: FnOnce(IoResult) + 'static
{
    on_saved(save_bytes(path, &bytes))
}

pub(crate) fn load_bytes_async<F>(path: &Path, on_loaded: F)
    where F: FnOnce(IoResult<Vec<u8>>) + 'static
{
    on_loaded(load_bytes(path))
}
*/