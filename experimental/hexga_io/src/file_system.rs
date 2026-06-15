use super::*;

#[doc(hidden)]
pub trait FsDynRead
{
    #[doc(hidden)]
    fn dyn_try_exist_unresolved(&mut self, path: &Path) -> IoResult<bool>;
    #[doc(hidden)]
    fn dyn_read_bytes_unresolved(&mut self, path: &Path) -> IoResult<Cow<'static, [u8]>>;
    #[doc(hidden)]
    fn dyn_read_dir_unresolved(&mut self, path: &Path) -> IoResult<Vec<PathBuf>>;

    /// Returns all existing files or directories with the same stem name as the given path, regardless of extension.
    #[doc(hidden)]
    fn dyn_resolve_paths(&mut self, path: &Path) -> IoResult<Vec<PathBuf>>;

    /// Resolve incomplete file extension by finding the matching file on disk.
    /// Returns an error if multiple files with the same stem exist or if the path is not valid.
    /// If no file exist with the same name, return Ok(path).
    #[doc(hidden)]
    fn dyn_resolve_path(&mut self, path: &Path) -> IoResult<PathBuf> 
    {
        let mut paths = self.dyn_resolve_paths(path)?;
        if let Some(p) = paths.pop()
        {
            if !paths.is_empty() { return Err(IoError::new(io::ErrorKind::InvalidInput, "Can be resolved to multiple path")) }
            return Ok(p);
        }
        Ok(path.to_owned())
    }

    #[doc(hidden)]
    fn dyn_canonicalize(&mut self, path: &Path) -> IoResult<PathBuf>;
}
#[doc(hidden)]
pub trait FsDynWrite : FsDynRead
{
    #[doc(hidden)]
    fn dyn_write_bytes_unresolved(&mut self, path: &Path, value: &[u8]) -> IoResult;
}


pub trait FsRead
{
    fn exist_unresolved<P: AsRef<Path>>(&mut self, path: P) -> bool { self.try_exist_unresolved(path).is_ok_and(|exist| exist) }
    fn try_exist_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<bool>;

    fn read_bytes_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Cow<'static, [u8]>>;
    fn read_dir_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>>;


    fn exist<P: AsRef<Path>>(&mut self, path: P) -> bool { self.try_exist(path).is_ok_and(|exist| exist) }
    fn try_exist<P: AsRef<Path>>(&mut self, path: P) -> IoResult<bool> { let path = self.resolve_path(path)?; self.try_exist_unresolved(path) }

    /// Given a Path to a file, return all occurence of the file on the disk with the same name, regardless of the extension.
    fn resolve_paths<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>>;
    /// Resolve incomplete file extension by finding the matching file on disk.
    /// Returns an error if multiple files with the same stem exist or if the path is not valid.
    /// If no file exist with the same name, return Ok(path).
    fn resolve_path<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf>;

    fn canonicalize<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf>;

    fn read_bytes<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Cow<'static, [u8]>> { let path = self.resolve_path(path)?; self.read_bytes_unresolved(path) }
    fn read_dir<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>> { let path = self.resolve_path(path)?; self.read_dir_unresolved(path) }
}
impl<T> FsRead for T where T:FsDynRead
{
    fn try_exist_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<bool> { self.dyn_try_exist_unresolved(path.as_ref()) }
    fn read_bytes_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Cow<'static, [u8]>> { self.dyn_read_bytes_unresolved(path.as_ref()) }
    fn read_dir_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>> { self.dyn_read_dir_unresolved(path.as_ref()) }
    
    fn resolve_paths<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>> { self.dyn_resolve_paths(path.as_ref()) }
    fn resolve_path<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { self.dyn_resolve_path(path.as_ref()) }
    
    fn canonicalize<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { self.dyn_canonicalize(path.as_ref()) }
}

pub trait FsWrite : FsRead + FsDynWrite
{
    fn write_bytes_unresolved<P: AsRef<Path>>(&mut self, path: P, value: &[u8]) -> IoResult;
    fn write_bytes<P: AsRef<Path>>(&mut self, path: P, value: &[u8]) -> IoResult { let path = self.resolve_path(path)?; self.write_bytes_unresolved(path, value) }
}
impl<T> FsWrite for T where T: FsRead + FsDynWrite
{
    fn write_bytes_unresolved<P: AsRef<Path>>(&mut self, path: P, value: &[u8]) -> IoResult { self.dyn_write_bytes_unresolved(path.as_ref(), value) }
}