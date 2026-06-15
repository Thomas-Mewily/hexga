use super::*;

/// The Global I/O operations for the filesystem.
#[derive(Debug)]
pub struct Io;

impl FsDynRead for Io
{
    fn dyn_try_exist_unresolved(&mut self, path: &Path) -> IoResult<bool> {
        Ok(path.exists())
    }

    fn dyn_read_bytes_unresolved(&mut self, path: &Path) -> IoResult<Cow<'static, [u8]>> {
        let bytes = std::fs::read(path)?;
        Ok(Cow::Owned(bytes))
    }

    fn dyn_read_dir_unresolved(&mut self, path: &Path) -> IoResult<Vec<PathBuf>> {
        let entries = std::fs::read_dir(path)?;
        let paths = entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .collect();
        Ok(paths)
    }
    
    fn dyn_resolve_paths<'a>(&mut self, path: &'a Path) -> IoResult<Vec<PathBuf>> 
    {
        let parent = path.parent().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Path has no parent directory")
        })?;
        let stem = path.file_stem().unwrap_or(path.as_os_str());

        let matches: Vec<PathBuf> = std::fs::read_dir(parent)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|candidate| {
                candidate.is_file() && candidate.file_stem() == Some(stem)
            })
            .collect();

        Ok(matches)
    }
}
impl FsDynWrite for Io
{
    fn dyn_write_bytes_unresolved(&mut self, path: &Path, value: &[u8]) -> IoResult {
        std::fs::write(path, value)?;
        Ok(())
    }
}
