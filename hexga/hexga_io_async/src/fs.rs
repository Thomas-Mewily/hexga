use super::*;

pub type Path = String;
#[allow(non_camel_case_types)]
pub type path = str;

pub type Extension = String;
#[allow(non_camel_case_types)]
pub type extension = str;

/// Shared behavior of a FileSystem when reading and writing
pub trait FsCore
{
    /*
    fn absolute_path(&self, path: &path) -> Option<Path>
    {
        match std::path::absolute(path)
        {
            Ok(v) => v.to_str().map(|v| v.to_owned()),
            Err(_) => None,
        }
    }
    */
}


pub type IoResult<T=()> = Result<T, ()>;



pub struct IoRead
{
    pub data : Vec<u8>,
    pub path : String,
}

pub trait FsRead : FsCore
{
    // Not compatible with &dyn
    //async fn read_data(&mut self, path: path) -> IoResult<IoRead>;
    //fn read_data(&mut self, path: &path) -> impl Future<Output = IoResult<IoRead>> + Send;
}

pub trait FsWrite : FsCore
{
    fn write_data(&mut self, path: &path, data: &[u8]) -> impl Future<Output = IoResult> + Send;
}

pub trait IoLoad
{
    /// Dedicated file extension to load the value. ex `png`, `jpeg` for image
    ///
    /// Don't include the markup language extension like `json` or `ron`
    ///
    /// The first value also determines the default extension in [IoLoad::load_default_extension].
    fn load_custom_extensions() -> impl Iterator<Item = &'static str> { std::iter::empty() }

    //async fn load_from_bytes()

}