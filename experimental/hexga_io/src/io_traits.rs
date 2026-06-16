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
pub trait IoDynRead
{
    #[doc(hidden)]
    fn dyn_try_exist_unresolved(&mut self, path: &Path) -> IoResult<bool> { self.dyn_file_type_unresolved(path)?; Ok(true) }
    #[doc(hidden)]
    fn dyn_read_bytes_unresolved(&mut self, path: &Path) -> IoResult<Cow<'static, [u8]>>;
    #[doc(hidden)]
    fn dyn_read_dir_unresolved(&mut self, path: &Path) -> IoResult<Vec<PathBuf>>;
    #[doc(hidden)]
    fn dyn_file_type_unresolved(&mut self, path: &Path) -> IoResult<FileType>;

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
    fn dyn_rename_unresolved(&mut self, from: &Path, to: &Path) -> IoResult;
}
#[doc(hidden)]
pub trait IoDynWrite: IoDynRead
{
    /// Write the byte at a file.
    /// If the file or the directory don't exist, create it.
    #[doc(hidden)]
    fn dyn_write_bytes_unresolved(&mut self, path: &Path, value: &[u8]) -> IoResult;
    #[doc(hidden)]
    /// Create recursively all dir in the path.
    /// If any element is a file on the way, delete it.
    fn dyn_create_dir(&mut self, path: &Path) -> IoResult;

    #[doc(hidden)]
    /// Remove any file or folder recursively
    fn dyn_remove_unresolved(&mut self, path: &Path) -> IoResult;
}


pub trait IoRead
{
    fn exist_unresolved<P: AsRef<Path>>(&mut self, path: P) -> bool { self.try_exist_unresolved(path).is_ok_and(|exist| exist) }
    fn try_exist_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<bool>;

    fn read_bytes_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Cow<'static, [u8]>>;
    fn read_dir_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>>;

    fn exist<P: AsRef<Path>>(&mut self, path: P) -> bool { self.try_exist(path).is_ok_and(|exist| exist) }
    fn try_exist<P: AsRef<Path>>(&mut self, path: P) -> IoResult<bool>
    {
        let path = self.resolve_path(path)?;
        self.try_exist_unresolved(path)
    }

    fn file_type_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<FileType>;
    fn file_type<P: AsRef<Path>>(&mut self, path: P) -> IoResult<FileType> { let path = self.resolve_path(path)?; self.file_type_unresolved(path) }

    /// Given a path to a file, return all occurence of the file on the disk with the same name, regardless of the extension.
    /// If the path already have an extension, return it.
    fn resolve_paths<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>>;
    /// Resolve incomplete file extension by finding the matching file on disk.
    /// Returns an error if multiple files with the same stem exist or if the path is not valid.
    /// If no file exist with the same name, return Ok(path).
    fn resolve_path<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf>;

    /// Canonicalizes the path like `std::fs::canonicalize`, but works even if the file doesn't exist.
    /// Returns an error when resolving above root (e.g., `/..`).
    fn canonicalize<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf>;

    fn read_bytes<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Cow<'static, [u8]>>
    {
        let path = self.resolve_path(path)?;
        self.read_bytes_unresolved(path)
    }
    fn read_dir<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>>
    {
        let path = self.resolve_path(path)?;
        self.read_dir_unresolved(path)
    }
}
impl<T> IoRead for T
where
    T: IoDynRead,
{
    fn try_exist_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<bool> { self.dyn_try_exist_unresolved(path.as_ref()) }
    fn read_bytes_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Cow<'static, [u8]>> { self.dyn_read_bytes_unresolved(path.as_ref()) }
    fn read_dir_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>> { self.dyn_read_dir_unresolved(path.as_ref()) }

    fn resolve_paths<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>> { self.dyn_resolve_paths(path.as_ref()) }
    fn resolve_path<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { self.dyn_resolve_path(path.as_ref()) }

    fn canonicalize<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { self.dyn_canonicalize(path.as_ref()) }
    
    fn file_type_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<FileType> { self.dyn_file_type_unresolved(path.as_ref()) }
}
/*
impl IoRead for dyn IoDynRead
{
    fn try_exist_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<bool> { self.dyn_try_exist_unresolved(path.as_ref()) }
    fn read_bytes_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Cow<'static, [u8]>> { self.dyn_read_bytes_unresolved(path.as_ref()) }
    fn read_dir_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>> { self.dyn_read_dir_unresolved(path.as_ref()) }

    fn resolve_paths<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>> { self.dyn_resolve_paths(path.as_ref()) }
    fn resolve_path<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { self.dyn_resolve_path(path.as_ref()) }

    fn canonicalize<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { self.dyn_canonicalize(path.as_ref()) }
    
    fn file_type_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<FileType> { self.dyn_file_type_unresolved(path.as_ref()) }
}*/

pub trait IoProvider : IoWrite + IoRead + Default {}
impl<F> IoProvider for F where F: IoWrite + IoRead + Default {}

pub trait IoWrite: IoRead + IoDynWrite
{
    /// Write the byte at a file.
    /// If the file or the directory don't exist, create it.
    fn write_bytes_unresolved<P: AsRef<Path>>(&mut self, path: P, value: &[u8]) -> IoResult;

    /// Write the byte at a file.
    /// If the file or the directory don't exist, create it.
    fn write_bytes<P: AsRef<Path>>(&mut self, path: P, value: &[u8]) -> IoResult<PathBuf>
    {
        let path = self.resolve_path(path)?;
        self.write_bytes_unresolved(&path, value)?;
        Ok(path)
    }

    /// Create recursively all dir in the path.
    /// If any element is a file on the way, delete it.
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> IoResult;
    
    /// Remove any file or folder recursively
    fn remove_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult;
    /// Remove any file or folder recursively
    fn remove<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { let path = self.resolve_path(path)?; self.remove_unresolved(&path)?; Ok(path) }

    /// Renames a file or directory to a new name, replacing the original file if
    /// `to` already exists.
    ///
    /// This will not work if the new name is on a different mount point.
    fn rename_unresolved<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> IoResult { self.dyn_rename_unresolved(from.as_ref(), to.as_ref()) }

    /// Renames a file or directory to a new name, replacing the original file if
    /// `to` already exists.
    ///
    /// This will not work if the new name is on a different mount point.
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> IoResult<PathBuf>
    {
        let from = self.resolve_path(from)?;
        let to = self.resolve_path(to)?;
        self.rename_unresolved(from, &to)?;
        Ok(to)
    }
}
impl<T> IoWrite for T
where
    T: IoRead + IoDynWrite,
{
    fn write_bytes_unresolved<P: AsRef<Path>>(&mut self, path: P, value: &[u8]) -> IoResult { self.dyn_write_bytes_unresolved(path.as_ref(), value) }

    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> IoResult { self.dyn_create_dir(path.as_ref()) }
    fn remove_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult { self.dyn_remove_unresolved(path.as_ref()) }
}
