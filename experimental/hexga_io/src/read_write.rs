use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum FileType
{
    File,
    Dir,
    Symlink
}


#[doc(hidden)]
pub trait FsDynRead
{
    #[doc(hidden)]
    fn dyn_try_exist_at(&mut self, path: &Path) -> IoResult<bool> { self.dyn_file_type_at(path)?; Ok(true) }
    #[doc(hidden)]
    fn dyn_read_bytes_at(&mut self, path: &Path) -> IoResult<Cow<'static, [u8]>>;
    #[doc(hidden)]
    fn dyn_read_dir_at(&mut self, path: &Path) -> IoResult<Vec<PathBuf>>;
    #[doc(hidden)]
    fn dyn_file_type_at(&mut self, path: &Path) -> IoResult<FileType>;

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
            if !paths.is_empty()
            {
                return Err(IoError::new_with_path(io::ErrorKind::InvalidInput, "Can be resolved to multiple path", path));
            }
            return Ok(p);
        }
        Ok(path.to_owned())
    }

    #[doc(hidden)]
    /// Canonicalizes the path like `std::fs::canonicalize`, but works even if the file doesn't exist.
    /// Returns an error when resolving above root (e.g., `/..`).
    fn dyn_canonicalize(&mut self, path: &Path) -> IoResult<PathBuf>;


    #[doc(hidden)]
    /// Renames a file or directory to a new name, replacing the original file if
    /// `to` already exists.
    ///
    /// This will not work if the new name is on a different mount point.
    fn dyn_rename_at(&mut self, from: &Path, to: &Path) -> IoResult;
}
#[doc(hidden)]
pub trait FsDynWrite: FsDynRead
{
    /// Write the byte at a file.
    /// If the file or the directory don't exist, create it.
    #[doc(hidden)]
    fn dyn_write_bytes_at(&mut self, path: &Path, value: &[u8]) -> IoResult;
    #[doc(hidden)]
    /// Create recursively all dir in the path.
    /// If any element is a file on the way, delete it.
    fn dyn_create_dir(&mut self, path: &Path) -> IoResult;

    #[doc(hidden)]
    /// Remove any file or folder recursively
    fn dyn_remove_at(&mut self, path: &Path) -> IoResult;
}


pub trait FsRead : FsDynRead
{
    fn exist_at<P: AsRef<Path>>(&mut self, path: P) -> bool { self.try_exist_at(path).is_ok_and(|exist| exist) }
    fn try_exist_at<P: AsRef<Path>>(&mut self, path: P) -> IoResult<bool> { self.dyn_try_exist_at(path.as_ref()) }

    fn read_bytes_at<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Cow<'static, [u8]>> { self.dyn_read_bytes_at(path.as_ref()) }
    fn read_dir_at<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>> { self.dyn_read_dir_at(path.as_ref()) }

    fn exist<P: AsRef<Path>>(&mut self, path: P) -> bool { self.try_exist(path).is_ok_and(|exist| exist) }
    fn try_exist<P: AsRef<Path>>(&mut self, path: P) -> IoResult<bool>
    {
        let path = self.resolve_path(path)?;
        self.try_exist_at(path)
    }

    fn file_type_at<P: AsRef<Path>>(&mut self, path: P) -> IoResult<FileType> { self.dyn_file_type_at(path.as_ref()) }
    fn file_type<P: AsRef<Path>>(&mut self, path: P) -> IoResult<FileType> { let path = self.resolve_path(path)?; self.file_type_at(path) }

    /// Given a path to a file, return all occurence of the file on the disk with the same name, regardless of the extension.
    /// If the path already have an extension, return it.
    fn resolve_paths<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>> { self.dyn_resolve_paths(path.as_ref()) }
    /// Resolve incomplete file extension by finding the matching file on disk.
    /// Returns an error if multiple files with the same stem exist or if the path is not valid.
    /// If no file exist with the same name, return Ok(path).
    fn resolve_path<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { self.dyn_resolve_path(path.as_ref()) }

    /// Canonicalizes the path like `std::fs::canonicalize`, but works even if the file doesn't exist.
    /// Returns an error when resolving above root (e.g., `/..`).
    fn canonicalize<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { self.dyn_canonicalize(path.as_ref()) }

    fn read_bytes<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Cow<'static, [u8]>>
    {
        let path = self.resolve_path(path)?;
        self.read_bytes_at(path)
    }
    fn read_dir<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>>
    {
        let path = self.resolve_path(path)?;
        self.read_dir_at(path)
    }
}
impl<T> FsRead for T where T: FsDynRead { }
impl FsRead for dyn FsDynRead { }
impl FsRead for dyn Fs { }


pub trait Fs : FsDynWrite + FsDynRead {}
impl<F> Fs for F where F: FsDynWrite + FsDynRead {}


pub trait FsWrite: FsRead + FsDynWrite
{
    /// Write the byte at a file.
    /// If the file or the directory don't exist, create it.
    fn write_bytes_at<P: AsRef<Path>>(&mut self, path: P, value: &[u8]) -> IoResult { self.dyn_write_bytes_at(path.as_ref(), value) }

    /// Write the byte at a file.
    /// If the file or the directory don't exist, create it.
    fn write_bytes<P: AsRef<Path>>(&mut self, path: P, value: &[u8]) -> IoResult<PathBuf>
    {
        let path = self.resolve_path(path)?;
        self.write_bytes_at(&path, value)?;
        Ok(path)
    }

    /// Create recursively all dir in the path.
    /// If any element is a file on the way, delete it.
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> IoResult { self.dyn_create_dir(path.as_ref()) }
    
    /// Remove any file or folder recursively
    fn remove_at<P: AsRef<Path>>(&mut self, path: P) -> IoResult { self.dyn_remove_at(path.as_ref()) }
    /// Remove any file or folder recursively
    fn remove<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { let path = self.resolve_path(path)?; self.remove_at(&path)?; Ok(path) }

    /// Renames a file or directory to a new name, replacing the original file if
    /// `to` already exists.
    ///
    /// This will not work if the new name is on a different mount point.
    fn rename_at<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> IoResult { self.dyn_rename_at(from.as_ref(), to.as_ref()) }

    /// Renames a file or directory to a new name, replacing the original file if
    /// `to` already exists.
    ///
    /// This will not work if the new name is on a different mount point.
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> IoResult<PathBuf>
    {
        let from = self.resolve_path(from)?;
        let to = self.resolve_path(to)?;
        self.rename_at(from, &to)?;
        Ok(to)
    }
}
impl<T> FsWrite for T where T: Fs {}
impl FsWrite for dyn Fs {}
