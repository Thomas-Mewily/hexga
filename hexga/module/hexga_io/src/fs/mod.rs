use crate::*;

mod disk;
pub use disk::*;


#[allow(unused_variables)]
pub trait IoFs : Sized
{
    fn save<T>(&mut self, path : Path, value : &T) where T : IoSave
    {
        if self.premature_abord() && self.have_error() { return; }

        let Some(path) = self.canonicalize_path(&path) else
        {
            self.add_write_error(path, IoErrorKind::InvalidPath);
            return;
        };

        if let Err(e) = self.have_permission_to_write(&path)
        {
            self.add_write_error(path, e);
            return;
        }

        let mut data = Vec::with_capacity(1024);

        match value.save_to(&path, &mut data, self)
        {
            Ok(_) => {},
            Err(err) => self.add_error(err),
        }

        match unsafe { self.save_bytes_unchecked(path, &data) }
        {
            Ok(_) => todo!(),
            Err(err) => self.add_error(err),
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

    fn canonicalize_path(&self, path : &path) -> Option<Path>
    {
        match std::fs::canonicalize(path)
        {
            Ok(v) => v.to_str().map(|v| v.to_owned()),
            Err(_) => None,
        }
    }
}