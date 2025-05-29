use crate::*;

mod disk;
pub use disk::*;


#[allow(unused_variables)]
pub trait IoFs : Sized
{
    fn save<T>(&mut self, path : Path, value : &T) where T : IoSave
    {
        if self.have_error() { return; }

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

        let mut bytes = Vec::with_capacity(1024);

        match value.save_to(&path, &mut bytes, self)
        {
            Ok(_) => {},
            Err(err) => self.add_error(err),
        }

        self.save_bytes(path, bytes);
    }

    fn have_permission_to_write(&self, path : &path) -> IoResult { Ok(()) }

    // The permission where not checked when calling this function
    unsafe fn save_bytes_unchecked(&mut self, path : &path, bytes : &[u8]) -> IoSaveResult;

    /// If there was an error, stop all next to avoid useless serialization
    fn premature_abord(&self) -> bool;

    fn commit(self) -> Result<(), Vec<IoSaveResult>>;

    fn have_error(&self) -> bool { false }
    fn add_error(&mut self, err : IoError);

    fn add_read_error (&mut self, path : impl Into<Path>, kind : IoErrorKind) { self.add_error(IoError::read (path, kind)); }
    fn add_write_error(&mut self, path : impl Into<Path>, kind : IoErrorKind) { self.add_error(IoError::write(path, kind)); }

    fn canonicalize_path(&self, path : &path) -> Option<Path>;

    fn is_canonicalized_path_valid(&self, path : &path) -> bool;
    fn is_path_valid(&self, path : &path) -> bool
    {
        let Some(canonicalized) = self.canonicalize_path(path) else { return false; };
        self.is_canonicalized_path_valid(&canonicalized)
    }
}