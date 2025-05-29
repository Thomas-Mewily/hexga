use crate::*;

mod disk;
pub use disk::*;


#[allow(unused_variables)]
pub trait IoFs : Sized
{
    fn save<T>(&mut self, path : &path, value : &T) -> IoResult where T : IoSave + ?Sized
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

        match value.save_with_reader(&path_abs, &mut data, self)
        {
            Ok(_) => {},
            Err(err) => { let k = err.kind.clone(); self.add_error(err); return Err(k); }
        }

        match unsafe { self.save_bytes_unchecked(path_abs, &data) }
        {
            Ok(_) => Ok(()),
            Err(err) => { let k = err.kind.clone(); self.add_error(err); return Err(k); }
        }
    }

    fn have_permission_to_write(&self, path : &path) -> IoResult { Ok(()) }

    /// The permission was not checked when calling this function
    unsafe fn save_bytes_unchecked(&mut self, path : Path, data : &[u8]) -> IoSaveResult;

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