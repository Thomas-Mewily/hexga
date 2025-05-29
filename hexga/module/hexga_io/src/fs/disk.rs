use crate::*;

use std::fs as f;
use std::fs::File;

// Todo : add permission and stuff
#[derive(Clone, Debug, Default)]
pub struct IoFsDiskStepByStep
{
    err : Vec<IoSaveResult>,
}

impl IoFsDiskStepByStep
{

}

impl IoFsDiskStepByStep
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

impl IoFs for IoFsDiskStepByStep
{
    fn have_error(&self) -> bool { self.err.is_not_empty() }

    fn save_in_writer<W>(&mut self, w : W, path : Path) {
        if self.have_error() { return; }
        self.dis
    }
}