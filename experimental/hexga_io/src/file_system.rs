use super::*;

/*
pub trait FileSystemGetCursor
{
    fn current_dir(&self) -> Result<PathBuf>;
}
pub trait FileSystemSetCursor
{
    fn set_current_dir(&self, path: PathBuf) -> Result<()>;
}
*/

/*
pub trait FileSystemNode
{

}
*/


pub trait FileSystemIsolated : FileSystemProvider
{
    type HostFileSystem : FileSystem;

    /// The root of this isolated file system.
    /// Cant only read and write file/dir inside this root.
    fn isolated_root(&mut self) -> PathBuf;

    fn non_isolated_file_system() -> Self::HostFileSystem;

    /// Validates that a path is within the isolated root.
    /// Returns an error if the path escapes the root.
    fn validate_path<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> {
        let path = path.as_ref();
        
        let mut fs = Self::non_isolated_file_system();

        let mut full_path = fs.canonicalize(&self.isolated_root())?;
        
        full_path.reserve(path.as_os_str().len() + 1);

        let mut depth = 0;
        //println!("{}", path.display());

        for (idx, c) in path.components().enumerate()
        {
            match c
            {
                std::path::Component::Prefix(prefix) => 
                {
                    return Err(IoError::new_with_path(IoErrorKind::InvalidFilename, format!("found prefix {:?} in the middle of the path", prefix), path)); 
                },
                std::path::Component::RootDir => 
                {
                    if idx != 0 { return Err(IoError::new_with_path(IoErrorKind::InvalidFilename, format!("found root dir in the middle of the path"), path))} 
                },
                std::path::Component::CurDir =>  { continue; } // Skip .,
                std::path::Component::ParentDir => 
                {
                    if depth == 0 {
                        return Err(IoError::new_with_path(
                            IoErrorKind::InvalidFilename,
                            "Path attempts to escape root with '..'".to_string(),
                            path
                        ));
                    }
                    depth -= 1;
                    full_path.pop();
                },
                std::path::Component::Normal(os_str) => 
                {
                    depth += 1;
                    full_path.push(os_str);
                },
            }
        }
        
        Ok(full_path)
    }
}
impl<T> FileSystemDynRead for T where T: FileSystemIsolated
{
    fn dyn_read_bytes_at(&mut self, path: &Path) -> IoResult<Cow<'static, [u8]>> {
        Self::non_isolated_file_system().dyn_read_bytes_at(&self.validate_path(path)?)
    }

    fn dyn_read_dir_at(&mut self, path: &Path) -> IoResult<Vec<IoResult<PathBuf>>> {
        Self::non_isolated_file_system().dyn_read_dir_at(&self.validate_path(path)?)
    }

    fn dyn_read_link_at(&mut self, path: &Path) -> IoResult<PathBuf> {
        Self::non_isolated_file_system().dyn_read_link_at(&self.validate_path(path)?)
    }

    fn dyn_file_type_at(&mut self, path: &Path) -> IoResult<FileType> {
        Self::non_isolated_file_system().dyn_file_type_at(&self.validate_path(path)?)
    }

    fn dyn_resolve_paths(&mut self, path: &Path) -> IoResult<Vec<PathBuf>> 
    {
        let root = self.isolated_root();
        let mut resolved = Self::non_isolated_file_system().dyn_resolve_paths(&self.validate_path(path)?)?;
        resolved.retain_mut(|p| 
            {
                match strip_prefix(&p, &root)
                {
                    Ok(stripped) => { *p = stripped; true },
                    Err(_) => false,
                }
            });
        Ok(resolved)
    }

    fn dyn_canonicalize(&mut self, path: &Path) -> IoResult<PathBuf> 
    {
        let path = self.validate_path(path)?;
        let root = self.isolated_root();
        strip_prefix(&path, &root)
    }
}

fn strip_prefix(path: &Path, root: &Path) -> IoResult<PathBuf>
{
    match path.strip_prefix(&root)
    {
        Ok(p) => Ok(p.to_path_buf()),
        Err(_err) => Err(IoError::new(
            IoErrorKind::InvalidInput,
            format!("Path '{:?}' is not within the isolated root '{:?}'", path, root))),
    }
}

impl<T> FileSystemDynWrite for T where T: FileSystemIsolated
{
    fn dyn_write_bytes_at(&mut self, path: &Path, value: &[u8]) -> IoResult {
        Self::non_isolated_file_system().dyn_write_bytes_at(&self.validate_path(path)?, value)
    }

    fn dyn_create_dir(&mut self, path: &Path) -> IoResult {
        Self::non_isolated_file_system().dyn_create_dir(&self.validate_path(path)?)
    }

    fn dyn_remove_at(&mut self, path: &Path) -> IoResult {
        Self::non_isolated_file_system().dyn_remove_at(&self.validate_path(path)?)
    }

    fn dyn_rename_at(&mut self, from: &Path, to: &Path) -> IoResult {
        Self::non_isolated_file_system().dyn_rename_at(&self.validate_path(from)?, &self.validate_path(to)?)
    }
}


pub trait FileSystemProvider
{
    type FileSystem: FileSystem;
    fn file_system() -> Self::FileSystem;
}

#[doc(hidden)]
pub trait FileSystemDynRead
{
    #[doc(hidden)]
    #[must_use]
    fn dyn_try_exist_at(&mut self, path: &Path) -> IoResult<bool>
    {
        self.dyn_file_type_at(path)?;
        Ok(true)
    }

    #[doc(hidden)]
    #[must_use]
    fn dyn_read_bytes_at(&mut self, path: &Path) -> IoResult<Cow<'static, [u8]>>;
    #[doc(hidden)] 
    #[must_use]
    fn dyn_read_dir_at(&mut self, path: &Path) -> IoResult<Vec<IoResult<PathBuf>>>;
    #[doc(hidden)]
    #[must_use]
    fn dyn_read_link_at(&mut self, path: &Path) -> IoResult<PathBuf>;
    #[doc(hidden)]
    #[must_use]
    fn dyn_file_type_at(&mut self, path: &Path) -> IoResult<FileType>;

    /// Returns all existing files or directories with the same stem name as the given path, regardless of extension.
    #[doc(hidden)]
    #[must_use]
    fn dyn_resolve_paths(&mut self, path: &Path) -> IoResult<Vec<PathBuf>>;

    /// Resolve incomplete file extension by finding the matching file on disk.
    /// Returns an error if multiple files with the same stem exist or if the path is not valid.
    /// If no file exist with the same name, return Ok(path).
    #[doc(hidden)]
    #[must_use]
    fn dyn_resolve_path(&mut self, path: &Path) -> IoResult<PathBuf>
    {
        let mut paths = self.dyn_resolve_paths(path)?;
        if let Some(p) = paths.pop()
        {
            if !paths.is_empty()
            {
                return Err(IoError::new_with_path(IoErrorKind::InvalidInput, "Can't be resolved to multiple path", path));
            }
            return Ok(p);
        }
        Ok(path.to_owned())
    }

    #[doc(hidden)]
    #[must_use]
    /// Canonicalizes the path like `std::fs::canonicalize`, relative to the [`Self::root()`], but works even if the file doesn't exist.
    /// Returns an error when resolving above [`Self::root()`] (e.g., `/..`).
    fn dyn_canonicalize(&mut self, path: &Path) -> IoResult<PathBuf>;

    /*
    #[doc(hidden)]
    #[must_use]
    /// Canonicalizes the path like `std::fs::canonicalize`, works across different file system, and also works even if the file doesn't exist.
    /// Returns an error when resolving above root (e.g., `/..`).
    fn dyn_canonicalize_absolute(&mut self, path: &Path) -> IoResult<PathBuf>;
    */

    // The root of the current file system.
    // fn root(&mut self) -> PathBuf;
}
#[doc(hidden)]
pub trait FileSystemDynWrite: FileSystemDynRead
{
    /// Write the byte at a file.
    /// If the file or the directory don't exist, create it.
    #[doc(hidden)]
    #[must_use]
    fn dyn_write_bytes_at(&mut self, path: &Path, value: &[u8]) -> IoResult;
    #[doc(hidden)]
    #[must_use]
    /// Create recursively all dir in the path.
    /// If any element is a file on the way, delete it.
    fn dyn_create_dir(&mut self, path: &Path) -> IoResult;

    #[doc(hidden)]
    #[must_use]
    /// Remove any file or folder recursively
    fn dyn_remove_at(&mut self, path: &Path) -> IoResult;

    #[doc(hidden)]
    #[must_use]
    /// Renames a file or directory to a new name, replacing the original file if
    /// `to` already exists.
    ///
    /// This will not work if the new name is on a different mount point.
    fn dyn_rename_at(&mut self, from: &Path, to: &Path) -> IoResult;
}

pub trait FileSystemRead: FileSystemDynRead
{
    /// Resolve the path anc check if it exist.
    #[must_use]
    fn exist<P: AsRef<Path>>(&mut self, path: P) -> bool { self.try_exist(path).is_ok_and(|exist| exist) }
    /// Resolve the path anc check if it exist.
    #[must_use]
    fn try_exist<P: AsRef<Path>>(&mut self, path: P) -> IoResult<bool> { let path = self.resolve_path(path)?; self.dyn_try_exist_at(&path) }

    /// Check if the path exist.
    #[must_use]
    fn exist_at<P: AsRef<Path>>(&mut self, path: P) -> bool { self.try_exist_at(path).is_ok_and(|exist| exist) }
    /// Check if the path exist.
    #[must_use]
    fn try_exist_at<P: AsRef<Path>>(&mut self, path: P) -> IoResult<bool> { self.dyn_try_exist_at(path.as_ref()) }


    /// Read the file at path.
    #[must_use]
    fn read_bytes_at<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Cow<'static, [u8]>> { self.dyn_read_bytes_at(path.as_ref()) }
    /// Read the contents of a directory at the given path.
    #[must_use]
    fn read_dir_at<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<IoResult<PathBuf>>> { self.dyn_read_dir_at(path.as_ref()) }
    /// Read the link at the given path.
    #[must_use]
    fn read_link_at<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<IoResult<PathBuf>>> { self.dyn_read_dir_at(path.as_ref()) }

    /// Read the file at the resolved path.
    #[must_use]
    fn read_bytes<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Cow<'static, [u8]>> { let path = self.resolve_path(path)?; self.read_bytes_at(path) }
    /// Read the contents of a directory at the given resolved path.
    #[must_use]
    fn read_dir<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<IoResult<PathBuf>>> { let path = self.resolve_path(path)?; self.read_dir_at(path) }
    /// Read the link at the given resolved path.
    #[must_use]
    fn read_link<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<IoResult<PathBuf>>> { let path = self.resolve_path(path)?; self.read_link_at(path) }

    #[must_use]
    fn file_type_at<P: AsRef<Path>>(&mut self, path: P) -> IoResult<FileType> { self.dyn_file_type_at(path.as_ref()) }
    #[must_use]
    fn file_type<P: AsRef<Path>>(&mut self, path: P) -> IoResult<FileType> { let path = self.resolve_path(path)?; self.file_type_at(path) }

    /// Given a path to a file, return all occurence of the file on the disk with the same name, regardless of the extension.
    /// If the path already have an extension, return it.
    #[must_use]
    fn resolve_paths<P: AsRef<Path>>(&mut self, path: P) -> IoResult<Vec<PathBuf>> { self.dyn_resolve_paths(path.as_ref()) }
    /// Resolve incomplete file extension by finding the matching file on disk.
    /// Returns an error if multiple files with the same stem exist or if the path is not valid.
    /// If no file exist with the same name, return Ok(path).
    #[must_use]
    fn resolve_path<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { self.dyn_resolve_path(path.as_ref()) }

    /// Canonicalizes the path like `std::fs::canonicalize`, but works even if the file doesn't exist.
    /// Returns an error when resolving above root (e.g., `/..`).
    #[must_use]
    fn canonicalize<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf> { self.dyn_canonicalize(path.as_ref()) }
}
impl<T> FileSystemRead for T where T: FileSystemDynRead {}
impl FileSystemRead for dyn FileSystemDynRead {}
impl FileSystemRead for dyn FileSystem {}

pub trait FileSystem: FileSystemDynWrite + FileSystemDynRead {}
impl<F> FileSystem for F where F: FileSystemDynWrite + FileSystemDynRead {}

pub trait FileSystemWrite: FileSystemRead + FileSystemDynWrite
{
    /// Write bytes at a file at the given path.
    /// If the file or any of the parent directory don't exist, create it.
    #[must_use]
    fn write_bytes_at<P: AsRef<Path>>(&mut self, path: P, value: &[u8]) -> IoResult { self.dyn_write_bytes_at(path.as_ref(), value) }

    /// Write bytes at a file at the given resolved path.
    /// If the file or any of the parent directory don't exist, create it.
    #[must_use]
    fn write_bytes<P: AsRef<Path>>(&mut self, path: P, value: &[u8]) -> IoResult<PathBuf>
    {
        let path = self.resolve_path(path)?;
        self.write_bytes_at(&path, value)?;
        Ok(path)
    }

    /// Create recursively all dir in the path.
    /// If any element is a file on the way, delete it.
    #[must_use]
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> IoResult { self.dyn_create_dir(path.as_ref()) }

    /// Remove any file or folder recursively at the path.
    #[must_use]
    fn remove_at<P: AsRef<Path>>(&mut self, path: P) -> IoResult { self.dyn_remove_at(path.as_ref()) }
    /// Remove any file or folder recursively at the resolved path.
    #[must_use]
    fn remove<P: AsRef<Path>>(&mut self, path: P) -> IoResult<PathBuf>
    {
        let path = self.resolve_path(path)?;
        self.remove_at(&path)?;
        Ok(path)
    }

    /// Rename a file or directory at path to a new destination, replacing the original file if
    /// `to` already exists.
    ///
    /// This will not work if the new name is on a different mount point.
    #[must_use]
    fn rename_at<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> IoResult { self.dyn_rename_at(from.as_ref(), to.as_ref()) }

    /// Rename a file or directory at the resolved path to a new destination, replacing the original file if
    /// `to` already exists.
    ///
    /// This will not work if the new name is on a different mount point.
    #[must_use]
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> IoResult<PathBuf>
    {
        let from = self.resolve_path(from)?;
        let to = self.resolve_path(to)?;
        self.rename_at(from, &to)?;
        Ok(to)
    }
}
impl<T> FileSystemWrite for T where T: FileSystem {}
impl FileSystemWrite for dyn FileSystem {}
