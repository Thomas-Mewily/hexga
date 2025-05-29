use crate::*;

mod disk;
pub use disk::*;

pub trait IoFs : Sized
{
    /// If there was an error, try stop all the next operation to avoid useless serialization
    fn premature_abord(&self) -> bool;

    fn commit(self) -> IoResult;

    fn have_error(&self) -> bool { false }
    fn add_error(&mut self, err : IoError);

    fn add_read_error (&mut self, path : impl Into<Path>, kind : IoErrorKind) { self.add_error(IoError::read (path, kind)); }
    fn add_write_error(&mut self, path : impl Into<Path>, kind : IoErrorKind) { self.add_error(IoError::write(path, kind)); }

    fn absolute_path(&self, path : &path) -> Option<Path>
    {
        match std::path::absolute(path)
        {
            Ok(v) => v.to_str().map(|v| v.to_owned()),
            Err(_) => None,
        }
    }
}


pub trait IoFsWrite : IoFs
{
    fn save<T>(&mut self, path : &path, value : &T) -> IoResult where T : IoSave + ?Sized
    {
        self.save_with_extension(path, path.extension_or_empty(), value)
    }
    fn save_with_extension<T>(&mut self, path : &path, extension: &extension, value : &T) -> IoResult where T : IoSave + ?Sized
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
    fn have_permission_to_write(&self, path : &path) -> IoResult { Ok(()) }

    /// The permission was not checked when calling this function
    unsafe fn write_bytes_unchecked(&mut self, path : Path, data : &[u8]) -> IoSaveResult;
}

pub trait IoFsRead : IoFs
{
    fn load<T>(&mut self, path : &path) -> IoResult<T> where T : IoLoad + ?Sized { self.load_with_extension(path, path.extension_or_empty()) }

    fn load_with_extension<T>(&mut self, path : &path, extension : &extension) -> IoResult<T> where T : IoLoad + ?Sized
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
    fn have_permission_to_read(&self, path : &path) -> IoResult { Ok(()) }

    /// The permission was not checked when calling this function
    unsafe fn read_bytes_unchecked(&mut self, path : Path) -> IoResult<Vec<u8>>;
}