use super::*;

/// The Global I/O operations for the filesystem.
#[derive(Debug, Default)]
pub struct Io;

impl FsDynRead for Io
{
    fn dyn_try_exist_unresolved(&mut self, path: &Path) -> IoResult<bool> { let path : &Path = path.into(); Ok(path.exists()) }

    fn dyn_read_bytes_unresolved(&mut self, path: &Path) -> IoResult<Cow<'static, [u8]>>
    {
        let bytes = std::fs::read(path)?;
        Ok(Cow::Owned(bytes))
    }

    fn dyn_read_dir_unresolved(&mut self, path: &Path) -> IoResult<Vec<PathBuf>>
    {
        let entries = std::fs::read_dir(path)?;
        let paths = entries.filter_map(|entry| entry.ok()).map(|entry| entry.path().into()).collect();
        Ok(paths)
    }

    fn dyn_resolve_paths<'a>(&mut self, path: &'a Path) -> IoResult<Vec<PathBuf>>
    {
        let path = self.canonicalize(path)?;
        let path : PathBuf = path.into();
        let parent = path
            .parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "path has no parent directory"))?;
        let stem = path.file_stem().unwrap_or(path.as_os_str());

        let parent = std::fs::read_dir(parent)?;

        let matches: Vec<PathBuf> = parent
            .filter_map(|entry| entry.ok())
            .map(|entry| { let p : PathBuf = entry.path().into(); p })
            .filter(|candidate| candidate.file_stem() == Some(stem))
            .collect();
        Ok(matches)
    }

    fn dyn_canonicalize(&mut self, path: &Path) -> IoResult<PathBuf> { 
        let path : &Path = path.into();
        Ok(fs::canonicalize(path)?.into()) 
    }
    
    fn dyn_file_type_unresolved(&mut self, path: &Path) -> IoResult<FileType> {
        let path : &Path = path.into();
        let file_type = path.metadata()?.file_type();
        if file_type.is_file() { return Ok(FileType::File); }
        if file_type.is_dir() { return Ok(FileType::Dir); }
        if file_type.is_symlink() { return Ok(FileType::Symlink); }
        Err(IoError::new(IoErrorKind::InvalidFilename, "Can't gess the file type"))
    }
}
impl FsDynWrite for Io
{
    fn dyn_write_bytes_unresolved(&mut self, path: &Path, value: &[u8]) -> IoResult
    {
        std::fs::write(path, value)?;
        Ok(())
    }
}
