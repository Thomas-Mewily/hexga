use crate::*;

pub struct Io;

pub type IoDiskOk = ();
pub type IoDiskError = std::io::Error;

pub type IoDiskResult = Result<IoDiskOk,IoDiskError>;

impl Io
{
    pub fn disk_read(path: &str) -> Result<Vec<u8>, IoDiskError> 
    {
        fs::read(path)
    }

    pub fn disk_read_buf(path: &str, mut buffer : &mut Vec<u8>) -> Result<IoDiskOk, IoDiskError> 
    {
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;
        Ok(())
    }

    pub fn disk_write(path : &str, data : &[u8]) -> Result<IoDiskOk, IoDiskError>
    {
        fs::write(path, data)
    }

    pub fn disk_remove_dir(path: &str) -> Result<IoDiskOk, IoDiskError> 
    {
        let meta = fs::metadata(path)?;
        if meta.is_dir() 
        {
            fs::remove_dir_all(path)?;
        }
        Ok(())
    }

    pub fn disk_remove_file(path: &str) -> Result<IoDiskOk, IoDiskError> 
    {
        let meta = fs::metadata(path)?;
        if meta.is_file() 
        {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// file or folder
    pub fn disk_remove(path: &str) -> Result<IoDiskOk, IoDiskError> 
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

    pub fn disk_append(path: &str, data: &[u8]) -> Result<IoDiskOk, IoDiskError> 
    {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        file.write_all(data)
    }
}

