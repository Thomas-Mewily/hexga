use crate::*;

pub type IoErrorKind = std::io::ErrorKind;

// Clonable, contrary to rust io error
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct IoError
{
    pub kind : IoErrorKind,
    pub reason : String,
}

impl IoError
{
    pub fn new(kind: IoErrorKind, reason : Reason) -> Self 
    {
        Self 
        {
            kind,
            reason,
        }
    }
}

impl From<std::io::Error> for IoError
{
    fn from(value: std::io::Error) -> Self 
    {
        let r = value.to_string();
        Self::new(value.kind(), r)
    }
}

impl Default for  IoError
{
    fn default() -> Self {
        Self { kind: IoErrorKind::Other, reason: "".to_owned() }
    }
}


pub type IoResult<T=()> = Result<T, IoError>;
pub struct Io;

impl Io
{
    pub fn disk_read(path: &path) -> Result<Vec<u8>, IoError> 
    {
        fs::read(path).map_err(|e| e.into())
    }

    pub fn disk_read_buf(path: &path, mut buffer : &mut Vec<u8>) -> IoResult 
    {
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;
        Ok(())
    }

    pub fn disk_write(path : &path, data : &[u8]) -> IoResult
    {
        fs::write(path, data).map_err(|e| e.into())
    }

    pub fn disk_remove_dir(path: &path) -> IoResult
    {
        let meta = fs::metadata(path)?;
        if meta.is_dir() 
        {
            fs::remove_dir_all(path)?;
        }
        Ok(())
    }

    pub fn disk_remove_file(path: &path) -> IoResult 
    {
        let meta = fs::metadata(path)?;
        if meta.is_file() 
        {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// file or folder
    pub fn disk_remove(path: &path) -> IoResult
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

    pub fn disk_append(path: &path, data: &[u8]) -> IoResult
    {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        file.write_all(data).map_err(|e| e.into())
    }
}

