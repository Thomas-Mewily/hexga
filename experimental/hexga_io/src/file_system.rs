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
}
#[doc(hidden)]
pub trait FsDynWrite: FsDynRead
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


pub trait FsRead
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

    /// Read the file content and load a decode it using the provided extension.
    fn load_unresolved<T, P>(&mut self, path: P) -> IoResult<T> 
    where 
        P: AsRef<Path>, 
        T: Load + Sized,
    {
        let path = path.as_ref();
        let extension = path.extension().and_then(|e| e.to_str());
        
        let bytes = self.read_bytes_unresolved(path).map_err(|e| {
            IoError::new_with_path(IoErrorKind::NotFound, e, path)
        })?;
        
        T::load_from_bytes(&bytes, extension).map_err(|e| {
            IoError::new_with_path(IoErrorKind::InvalidData, e, path)
        })
    }

    /// Read the file content and load a decode it using the provided extension.
    fn load<T,P>(&mut self, path: P) -> IoResult<T> where P: AsRef<Path>, T: Load + Sized
    {
        let path = path.as_ref();
        let extension = path.extension().map(|e| e.to_str()).flatten();

        let (bytes, extension) = match Io.read_bytes(path)
        {
            Ok(bytes) => (bytes, extension),
            Err(err) =>
            {
                let mut found = None;

                for ext in T::load_extensions()
                {
                    if Some(ext) == extension
                    {
                        continue;
                    }

                    if let Ok(bytes) = Io.read_bytes(&path.with_extension(ext))
                    {
                        found = Some((bytes, Some(ext)));
                        break;
                    }
                }
                match found
                {
                    Some(bytes_and_extension) => bytes_and_extension,
                    None => return Err(err),
                }
            }
        };
        T::load_from_bytes(&bytes, extension).map_err(|e| IoError::new_with_path(IoErrorKind::InvalidData, e, path))
    }
}
impl<T> FsRead for T
where
    T: FsDynRead,
{
    fn try_exist_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<bool> { self.dyn_try_exist_unresolved(path.as_ref()) }
    fn read_bytes_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Cow<'static, [u8]>> { self.dyn_read_bytes_unresolved(path.as_ref()) }
    fn read_dir_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>> { self.dyn_read_dir_unresolved(path.as_ref()) }

    fn resolve_paths<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>> { self.dyn_resolve_paths(path.as_ref()) }
    fn resolve_path<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { self.dyn_resolve_path(path.as_ref()) }

    fn canonicalize<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { self.dyn_canonicalize(path.as_ref()) }
    
    fn file_type_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult<FileType> { self.dyn_file_type_unresolved(path.as_ref()) }
}


pub trait FsWrite: FsRead + FsDynWrite
{
    /// Write the byte at a file.
    /// If the file or the directory don't exist, create it.
    fn write_bytes_unresolved<P: AsRef<Path>>(&mut self, path: P, value: &[u8]) -> IoResult;
    /// Write the byte at a file.
    /// If the file or the directory don't exist, create it.
    fn write_bytes<P: AsRef<Path>>(&mut self, path: P, value: &[u8]) -> IoResult
    {
        let path = self.resolve_path(path)?;
        self.write_bytes_unresolved(path, value)
    }

    /// Create recursively all dir in the path.
    /// If any element is a file on the way, delete it.
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> IoResult;
    
    /// Remove any file or folder recursively
    fn remove_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult;
    /// Remove any file or folder recursively
    fn remove<P: AsRef<Path>>(&mut self, path: P) -> IoResult { let path = self.resolve_path(path)?; self.remove_unresolved(path) }

    /// Encode the value using the provided extension and write it to a file.
    fn save_unresolved<P, T>(&mut self, path: P, value: &T) -> IoResult
    where
        P: AsRef<Path>,
        T: Save + ?Sized,
    {
        let path = path.as_ref();
        let (bytes, _extension) = value
            .save_to_bytes(path.extension().map(|ex| ex.to_str()).flatten())
            .map_err(|e| IoError::new_with_path(IoErrorKind::InvalidData, e, path))?;

        Io.write_bytes_unresolved(&path, &bytes)
    }

    /// Encode the value using the provided extension and write it to a file.
    fn save<P, T>(&mut self, path: P, value: &T) -> IoResult
    where
        P: AsRef<Path>,
        T: Save + ?Sized,
    {
        let path = path.as_ref();
        let (bytes, extension) = value
            .save_to_bytes(path.extension().map(|ex| ex.to_str()).flatten())
            .map_err(|e| IoError::new_with_path(IoErrorKind::InvalidData, e, path))?;

        match extension
        {
            Some(ex) =>  Io.write_bytes(&path.with_extension(ex.as_ref()), &bytes),
            None => Io.write_bytes(path, &bytes),
        }
    }
}
impl<T> FsWrite for T
where
    T: FsRead + FsDynWrite,
{
    fn write_bytes_unresolved<P: AsRef<Path>>(&mut self, path: P, value: &[u8]) -> IoResult { self.dyn_write_bytes_unresolved(path.as_ref(), value) }

    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> IoResult { self.dyn_create_dir(path.as_ref()) }
    fn remove_unresolved<P: AsRef<Path>>(&mut self, path: P) -> IoResult { self.dyn_remove_unresolved(path.as_ref()) }
}
