use crate::*;

pub struct Io;


impl Io
{
    /// Used for loading and saving
    pub const MARKUP_EXTENSIONS: &'static [&'static str] =
    &[
        #[cfg(feature = "serde_ron")]
        Self::RON_EXTENSION,
        #[cfg(feature = "serde_json")]
        Self::JSON_EXTENSION,
        #[cfg(feature = "serde_xml")]
        Self::XML_EXTENSION,

        /* Not one of them
        #[cfg(feature = "serde_quick_bin")]
        Self::QUICK_BIN_EXTENSION,
        */
    ];

    #[cfg(feature = "serde_json")]
    pub const JSON_EXTENSION : &'static str = "json";
    #[cfg(feature = "serde_ron")]
    pub const RON_EXTENSION  : &'static str = "ron";
    #[cfg(feature = "serde_xml")]
    pub const XML_EXTENSION  : &'static str = "xml";


    #[cfg(feature = "serde_quick_bin")]
    pub const QUICK_BIN_EXTENSION  : &'static str = "bin";
}

impl IoFsDisk for Io {}

pub trait IoFsDisk
{
    fn disk_read(path: &path) -> IoLoadResult<Vec<u8>>
    {
        fs::read(path).to_read_error(path)
    }

    fn disk_read_buf(path: &path, mut buffer : &mut Vec<u8>) -> IoLoadResult
    {
        let mut file = File::open(path).to_read_error(path)?;
        file.read_to_end(&mut buffer).to_read_error(path)?;
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