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

    pub fn new_unsuported_extension<T>(extension : &extension) -> Self 
    {
        Self::new(std::io::ErrorKind::InvalidInput, format!("The extension .{extension} is not supported for {}", std::any::type_name::<T>()))
    }
    pub fn new_serializing<T>(extension : &extension, reason : Reason) -> Self 
    {
        Self::new(std::io::ErrorKind::InvalidInput, format!("Error while serializing {} to .{extension} : {reason}", std::any::type_name::<T>()))
    }
    pub fn new_deserialize<T>(extension : &extension, reason : Reason) -> Self 
    {
        Self::new(std::io::ErrorKind::InvalidInput, format!("Error while deserializing {} from .{extension} : {reason}", std::any::type_name::<T>()))
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
        Self { kind: IoErrorKind::Other, reason: "?".to_owned() }
    }
}


pub type IoResult<T=()> = Result<T, IoError>;
pub struct Io;

impl Io
{
    /// Used for loading and saving
    pub const ALL_MARKUP_LANGAGE_EXTENSION: &'static [&'static str] = 
    &[
        #[cfg(feature = "serde_json")]
        Self::JSON_EXTENSION,
        #[cfg(feature = "serde_ron")]
        Self::RON_EXTENSION,
        #[cfg(feature = "serde_xml")]
        Self::XML_EXTENSION,
        #[cfg(feature = "serde_quick_bin")]
        Self::QUICK_BIN_EXTENSION,
    ];

    #[cfg(feature = "serde_json")]
    pub const JSON_EXTENSION : &'static str = "json";
    #[cfg(feature = "serde_ron")]
    pub const RON_EXTENSION  : &'static str = "ron";
    #[cfg(feature = "serde_xml")]
    pub const XML_EXTENSION  : &'static str = "xml";
    #[cfg(feature = "serde_quick_bin")]
    pub const QUICK_BIN_EXTENSION  : &'static str = "bin";

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

