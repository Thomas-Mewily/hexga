use super::*;

mod disk;
pub use disk::*;

pub trait IoFsCore: Sized
{
    /// If there was an error, try stop all the next operation to avoid useless serialization
    fn premature_abord(&self) -> bool;

    fn commit(self) -> IoResult;

    fn have_error(&self) -> bool { false }
    fn add_error(&mut self, err: IoError);

    fn add_read_error (&mut self, path: impl Into<Path>, kind: IoErrorKind) { self.add_error(IoError::read (path, kind)); }
    fn add_write_error(&mut self, path: impl Into<Path>, kind: IoErrorKind) { self.add_error(IoError::write(path, kind)); }

    fn absolute_path(&self, path: &path) -> Option<Path>
    {
        match std::path::absolute(path)
        {
            Ok(v) => v.to_str().map(|v| v.to_owned()),
            Err(_) => None,
        }
    }
}


pub trait IoFsWrite: IoFsCore
{
    fn save<T>(&mut self, path: &path, value: &T) -> IoResult where T: IoSave + ?Sized
    {
        self.save_with_extension(path, path.extension_or_empty(), value)
    }
    fn save_with_extension<T>(&mut self, path: &path, mut extension: &extension, value: &T) -> IoResult where T: IoSave + ?Sized
    {
        if self.premature_abord() && self.have_error() { return Err(IoErrorKind::FsPrematureAbord); }

        let mut path_abs = match self.absolute_path(&path)
        {
            Some(abs) => abs,
            None =>
            {
                self.add_write_error(path, IoErrorKind::InvalidPath);
                return Err(IoErrorKind::InvalidPath);
            }
        };

        if extension.is_empty()
        {
            extension = T::save_default_extension().unwrap_or_default();
            path_abs = path_abs.as_str().with_extension(extension);
        }

        if let Err(kind) = self.have_permission_to_write(&path_abs)
        {
            let k = kind.clone();
            self.add_write_error(path_abs, kind);
            return Err(k);
        }

        let mut data = Vec::with_capacity(1024);

        match value.save_with_reader_and_extension(&path_abs, extension, &mut data, self)
        {
            Ok(_) => {},
            Err(err) => { let k = err.kind.clone(); self.add_error(err); return Err(k); }
        }

        match unsafe { self.write_bytes_unchecked(path_abs, &data) }
        {
            Ok(_) => Ok(()),
            Err(err) => { let k = err.kind.clone(); self.add_error(err); return Err(k); }
        }
    }

    #[allow(unused_variables)]
    fn have_permission_to_write(&self, path: &path) -> IoResult { Ok(()) }

    /// The permission was not checked when calling this function
    unsafe fn write_bytes_unchecked(&mut self, path: Path, data: &[u8]) -> IoSaveResult;
}

pub trait IoFsRead: IoFsCore
{
    /// Loads a value of type `T` from the given `path`.
    /// Automatically determines the file extension from the path and delegates to `load_with_extension`.
    fn load<T>(&mut self, path: &path) -> IoResult<T> where T: IoLoad + ?Sized { self.load_with_extension(path, path.extension_or_empty()) }

    /// Loads a value of type `T` from the given `path`, or creates it and **save** it using `init` if it doesn't exist.
    fn load_or_create<T,F>(&mut self, path: &path, init: F) -> IoResult<T> where T: IoLoad + IoSave + ?Sized, Self: IoFsWrite, F:FnOnce() -> T
    {
        match self.load(path)
        {
            Ok(v) => return Ok(v),
            Err(_) =>
            {
                let val = init();
                val.save_to(path, self).map_err(|e| e.kind)?;
                Ok(val)
            },
        }
    }

    /// Loads a value of type `T` from the given `path` using a specified `extension`.
    fn load_with_extension<T>(&mut self, path: &path, extension: &extension) -> IoResult<T> where T: IoLoad + ?Sized
    {
        if self.premature_abord() && self.have_error() { return Err(IoErrorKind::FsPrematureAbord); }

        let path_abs = match self.absolute_path(&path)
        {
            Some(abs) => abs,
            None =>
            {
                self.add_write_error(path, IoErrorKind::InvalidPath);
                return Err(IoErrorKind::InvalidPath);
            }
        };

        if let Err(kind) = self.have_permission_to_read(&path_abs)
        {
            let k = kind.clone();
            self.add_read_error(path_abs, kind);
            return Err(k);
        }

        let data = unsafe { self.read_bytes_unchecked(path_abs) }?;

        match T::load_from_bytes_with_extension(&data, path, extension)
        {
            Ok(v) => Ok(v),
            Err(err) => { let k = err.kind.clone(); self.add_error(err); Err(k) }
        }
    }

    #[allow(unused_variables)]
    fn have_permission_to_read(&self, path: &path) -> IoResult { Ok(()) }

    /// The permission was not checked when calling this function
    unsafe fn read_bytes_unchecked(&mut self, path: Path) -> IoResult<Vec<u8>>;
}
