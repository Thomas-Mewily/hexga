use super::*;

/// The Global I/O operations for the filesystem.
/// It have access to the whole file system.
/// Use other `Io` struct to sandbox it.
#[derive(Debug, Default)]
pub struct IoGlobal;

impl FileSystemProvider for IoGlobal
{
    type FileSystem = Self;
    fn file_system() -> Self::FileSystem { Self }
}

impl FileSystemDynRead for IoGlobal
{
    fn dyn_try_exist_at(&mut self, path: &Path) -> IoResult<bool> { path.try_exists() }

    fn dyn_read_bytes_at(&mut self, path: &Path) -> IoResult<Cow<'static, [u8]>>
    {
        let bytes = std::fs::read(path)?;
        Ok(Cow::Owned(bytes))
    }

    fn dyn_read_dir_at(&mut self, path: &Path) -> IoResult<Vec<IoResult<PathBuf>>>
    {
        #[cfg(feature = "print_io")]
        println!("io : read at {}", path.display());

        let entries = std::fs::read_dir(path)?;
        let paths = entries.map(|entry| entry.map(|entry| entry.path().to_path_buf())).collect();
        Ok(paths)
    }

    fn dyn_resolve_paths(&mut self, path: &Path) -> IoResult<Vec<PathBuf>>
    {
        match self._dyn_resolve_paths(path)
        {
            Ok(v) =>
            {
                #[cfg(feature = "print_io")]
                println!(
                    "io : resolving {} -> [{}]",
                    path.display(),
                    v.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ")
                );
                Ok(v)
            }
            Err(e) =>
            {
                #[cfg(feature = "print_io")]
                println!("io : resolving failed {}", e);
                Err(e)
            }
        }
    }

    fn dyn_file_type_at(&mut self, path: &Path) -> IoResult<FileType> {
        let meta = fs::metadata(path)?;
        let file_type = meta.file_type();

        if file_type.is_dir() { return Ok(Some(FileKind::Dir)); }
        if file_type.is_file() { return Ok(Some(FileKind::File)); }
        if file_type.is_symlink() { return Ok(Some(FileKind::Symlink)); }
        Ok(None)
    }
    
    fn dyn_read_link_at(&mut self, path: &Path) -> IoResult<PathBuf> {
        path.read_link()
    }
    
    fn dyn_canonicalize(&mut self, path: &Path) -> IoResult<PathBuf> {
        match self._dyn_canonicalize(path)
        {
            Ok(canonized) =>
            {
                #[cfg(feature = "print_io")]
                println!("io : canonicalize {} -> {}", path.display(), canonized.display());
                Ok(canonized)
            }
            Err(e) =>
            {
                #[cfg(feature = "print_io")]
                println!("io : canonicalize failed {}", e);
                Err(e)
            }
        }
    }
}

impl IoGlobal
{
    fn _dyn_resolve_paths(&mut self, path: &Path) -> IoResult<Vec<PathBuf>>
    {
        let p = self.canonicalize(path)?;
        if p.extension().is_some()
        {
            return Ok(vec![p]);
        }

        let parent = match p.parent()
        {
            Some(p) => p,
            None => return Ok(vec![p]),
        };
        let stem = p.file_stem().unwrap_or(p.as_os_str());

        let parent = match std::fs::read_dir(parent)
        {
            Ok(dir) => dir,
            Err(_e) => return Ok(vec![p]),
        };

        let matches: Vec<PathBuf> = parent
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let p: PathBuf = entry.path().into();
                p
            })
            .filter(|candidate| candidate.file_stem() == Some(stem))
            .collect();
        Ok(matches)
    }

    fn _dyn_canonicalize(&mut self, path: &Path) -> IoResult<PathBuf>
    {
        let path: &Path = path.into();

        /*
        if path.exists()
        {
            return path.canonicalize();
        }
        */

        let mut resolved = std::env::current_dir()?;
        resolved.reserve(path.as_os_str().len());

        for component in path.components()
        {
            match component
            {
                std::path::Component::CurDir =>
                {
                    // Skip ".". Does nothing
                    continue;
                }
                std::path::Component::ParentDir =>
                {
                    // Pop the last component for ".."
                    if resolved.pop() == false
                    {
                        return Err(IoErrorKind::InvalidInput.into());
                    };
                }
                std::path::Component::RootDir =>
                {
                    resolved.push("/");
                }
                std::path::Component::Prefix(prefix) =>
                {
                    // Windows drive letter (e.g., "C:")
                    resolved.push(prefix.as_os_str());
                }
                std::path::Component::Normal(segment) =>
                {
                    resolved.push(segment);
                }
            }
        }

        Ok(resolved)
    }
}

impl FileSystemDynWrite for IoGlobal
{
    fn dyn_write_bytes_at(&mut self, path: &Path, value: &[u8]) -> IoResult
    {
        #[cfg(feature = "print_io")]
        println!("io : write at {}", path.display());

        if let Some(parent) = path.parent()
        {
            if !parent.exists()
            {
                std::fs::create_dir_all(parent)?;
            }
        }

        std::fs::write(path, value)?;
        Ok(())
    }

    fn dyn_create_dir(&mut self, path: &Path) -> IoResult
    {
        if path.exists()
        {
            if path.is_file()
            {
                std::fs::remove_file(path)?;
            }
        }

        std::fs::create_dir_all(path)?;
        Ok(())
    }

    fn dyn_remove_at(&mut self, path: &Path) -> IoResult
    {
        #[cfg(feature = "print_io")]
        println!("io : remove at {}", path.display());

        if path.is_dir()
        {
            std::fs::remove_dir_all(path)?;
        }
        else if path.is_file()
        {
            std::fs::remove_file(path)?;
        }
        else if !path.exists()
        {
            return Ok(());
        }
        Ok(())
    }

    fn dyn_rename_at(&mut self, from: &Path, to: &Path) -> IoResult
    {
        #[cfg(feature = "print_io")]
        println!("io : rename {} -> {}", from.display(), to.display());
        std::fs::rename(from, to)
    }
}
