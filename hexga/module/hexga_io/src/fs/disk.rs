use crate::*;

use std::fs as f;
use std::fs::File;

// Todo : add permission and stuff
#[derive(Clone, Debug)]
pub struct IoFsDisk
{
    premature_abord : bool,
    err : Vec<IoError>,
}

impl Default for IoFsDisk
{
    fn default() -> Self { Self::new() }
}
impl IoFsDisk
{
    pub const fn new() -> Self { Self::new_with_premature_abord(true) }
    pub const fn new_with_premature_abord(premature_abord : bool) -> Self { Self { premature_abord, err: Vec::new() } }
}

/*
impl IoFsDisk
{
    pub fn disk_read(&self, path: &path) -> IoLoadResult<Vec<u8>>
    {
        f::read(path).to_load_error(path)
    }

    pub fn disk_read_buf(&self, path: &path, mut buffer : &mut Vec<u8>) -> IoLoadResult
    {
        let mut file = File::open(path).to_load_error(path)?;
        file.read_to_end(&mut buffer).to_load_error(path)?;
        Ok(())
    }

    /*
    fn disk_write(path : &path, data : &[u8]) -> IoResult
    {
        fs::write(path, data).map_err(|e| e.into())
    }

    fn disk_remove_dir(path: &path) -> IoResult
    {
        let meta = fs::metadata(path)?;
        if meta.is_dir()
        {
            fs::remove_dir_all(path)?;
        }
        Ok(())
    }

    fn disk_remove_file(path: &path) -> IoResult
    {
        let meta = fs::metadata(path)?;
        if meta.is_file()
        {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// file or folder
    fn disk_remove(path: &path) -> IoResult
    {
        let meta = fs::metadata(path)?;
        if meta.is_file()
        {
            fs::remove_file(path)?;
        } else if meta.is_dir()
        {
            fs::remove_dir_all(path)?;
        }
        Ok(())
    }

    fn disk_append(path: &path, data: &[u8]) -> IoResult
    {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        file.write_all(data).map_err(|e| e.into())
    }
    */
}
*/

impl IoFs for IoFsDisk
{
    fn have_error(&self) -> bool { self.err.is_not_empty() }

    fn premature_abord(&self) -> bool {
        self.premature_abord
    }

    unsafe fn save_bytes_unchecked(&mut self, path : Path, data : &[u8]) -> IoSaveResult {
        f::write(&path, data).to_save_error(path)
    }

    fn commit(self) -> Result<(), Vec<IoError>> {
        if self.err.is_empty() { Ok(()) } else { Err(self.err) }
    }

    fn add_error(&mut self, err : IoError) {
        self.err.push(err);
    }
}