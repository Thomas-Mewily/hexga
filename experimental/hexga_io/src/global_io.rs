use super::*;

/// The Global I/O operations for the filesystem.
#[derive(Debug, Default)]
pub struct Io;

impl FsDynRead for Io
{
    fn dyn_try_exist_unresolved(&mut self, path: &Path) -> IoResult<bool> 
    { 
        //let path : &Path = path.into(); 
        Ok(path.exists()) 
    }

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
        if path.extension().is_some() { return Ok(vec![path]); }

        let parent = match path.parent()
        {
            Some(p) => p,
            None => return Ok(vec![path]),
        };
        let stem = path.file_stem().unwrap_or(path.as_os_str());

        let parent = match std::fs::read_dir(parent) 
        {
            Ok(dir) => dir,
            Err(_e) => return Ok(vec![path]),
        };

        let matches: Vec<PathBuf> = parent
            .filter_map(|entry| entry.ok())
            .map(|entry| { let p : PathBuf = entry.path().into(); p })
            .filter(|candidate| candidate.file_stem() == Some(stem))
            .collect();
        Ok(matches)
    }

    fn dyn_canonicalize(&mut self, path: &Path) -> IoResult<PathBuf> { 
        let path : &Path = path.into();

        if path.exists() {
            return path.canonicalize();
        }

        let mut resolved = PathBuf::with_capacity(path.as_os_str().len());
        for component in path.components() {
            match component {
                std::path::Component::CurDir => {
                    // Skip ".". Does nothing
                    continue;
                }
                std::path::Component::ParentDir => {
                    // Pop the last component for ".."
                    if resolved.pop() == false { return Err(IoErrorKind::InvalidInput.into()) };
                }
                std::path::Component::RootDir => {
                    resolved.push("/");
                }
                std::path::Component::Prefix(prefix) => {
                    // Windows drive letter (e.g., "C:")
                    resolved.push(prefix.as_os_str());
                }
                std::path::Component::Normal(segment) => {
                    resolved.push(segment);
                }
            }
        }
        
        Ok(resolved)
    }
    
    fn dyn_file_type_unresolved(&mut self, path: &Path) -> IoResult<FileType> {
        //let path : &Path = path.into();
        let file_type = path.metadata()?.file_type();
        if file_type.is_file() { return Ok(FileType::File); }
        if file_type.is_dir() { return Ok(FileType::Dir); }
        if file_type.is_symlink() { return Ok(FileType::Symlink); }
        Err(IoError::new_with_path(IoErrorKind::InvalidFilename, "Can't gess the file type", path))
    }
}
impl FsDynWrite for Io
{
    fn dyn_write_bytes_unresolved(&mut self, path: &Path, value: &[u8]) -> IoResult
    {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        std::fs::write(path, value)?;
        Ok(())
    }
    
    fn dyn_create_dir(&mut self, path: &Path) -> IoResult {
        if path.exists() {
            if path.is_file() {
                std::fs::remove_file(path)?;
            }
        }
        
        std::fs::create_dir_all(path)?;
        Ok(())
    }
    
    fn dyn_remove_unresolved(&mut self, path: &Path) -> IoResult {
        if path.is_dir() 
        {
            std::fs::remove_dir_all(path)?;
        } else if path.is_file() 
        {
            std::fs::remove_file(path)?;
        } else if !path.exists() 
        {
            return Ok(());
        }
        Ok(())
    }
}
