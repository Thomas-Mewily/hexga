use super::*;

pub trait IoLoad : IoRead
{
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
        let path = &self.resolve_path(path)?;
        let extension = path.extension().and_then(|e| e.to_str());
        
        let bytes = self.read_bytes_unresolved(path).map_err(|e| {
            IoError::new_with_path(IoErrorKind::NotFound, e, path)
        })?;
        
        T::load_from_bytes(&bytes, extension).map_err(|e| {
            IoError::new_with_path(IoErrorKind::InvalidData, e, path)
        })
    }
}


impl<I> IoLoad for I
where
    I: IoRead,
{
}

pub trait IoSave: IoWrite + IoLoad
{
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
    fn save<P, T>(&mut self, path: P, value: &T) -> IoResult<PathBuf>
    where
        P: AsRef<Path>,
        T: Save + ?Sized,
    {
        let path = path.as_ref();
        let (bytes, extension) = value
            .save_to_bytes(path.extension().map(|ex| ex.to_str()).flatten())
            .map_err(|e| IoError::new_with_path(IoErrorKind::InvalidData, e, path))?;

        let pathbuf = match extension
        {
            Some(ex) =>  path.with_extension(ex.as_ref()),
            None => path.to_owned(),
        };

        Io.write_bytes(&pathbuf, &bytes)?;
        Ok(pathbuf)
    }

    /// Read the file content and load a decode it using the provided extension.
    /// If the file don't exist, the value is created, saved, and returned.
    /// It's ok if saving fail.
    fn load_or_create_unresolved<T,P,F>(&mut self, path: P, init: F) -> T where P: AsRef<Path>, T: Load + Save + Sized, F: FnOnce() -> T
    {
        let path = path.as_ref();
        match self.load_unresolved(path)
        {
            Ok(v) => v,
            Err(_) => 
            {
                let value = init();
                let _ = self.save_unresolved(path, &value);
                value
            },
        }
    }

    /// Read the file content and load a decode it using the provided extension.
    /// If the file don't exist, the value is created, saved, and returned.
    /// It's ok if saving fail.
    fn load_or_create<T,P,F>(&mut self, path: P, init: F) -> T where P: AsRef<Path>, T: Load + Save + Sized, F: FnOnce() -> T
    {
        let path = path.as_ref();
        match self.load(path)
        {
            Ok(v) => v,
            Err(_) => 
            {
                let value = init();
                let _ = self.save(path, &value);
                value
            },
        }
    }

    /// Read the file content and load a decode it using the provided extension.
    /// If the file don't exist, the value is created using [`Default`], saved, and returned.
    /// It's ok if saving fail.
    fn load_or_default<T,P>(&mut self, path: P) -> T where P: AsRef<Path>, T: Load + Save + Sized + Default
    {
        self.load_or_create(path, || Default::default())
    }

    /// Read the file content and load a decode it using the provided extension.
    /// If the file don't exist, the value is created using [`Default`], saved, and returned.
    /// It's ok if saving fail.
    fn load_or_default_unresolved<T,P>(&mut self, path: P) -> T where P: AsRef<Path>, T: Load + Save + Sized + Default
    {
        self.load_or_create_unresolved(path, || Default::default())
    }
}
impl<I> IoSave for I
where
    I: IoWrite,
{}
