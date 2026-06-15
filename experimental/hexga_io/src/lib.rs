use std::borrow::Cow;
pub use std::{path::{Path, PathBuf}, io, fs};

mod file_system;
pub use file_system::*;

mod global_io;
pub use global_io::*;

//mod path_extension;
//pub use path_extension::*;


pub type IoErrorKind = std::io::ErrorKind;
pub type IoError = std::io::Error;
pub type IoResult<T=()> = Result<T, IoError>; 

pub mod prelude
{
    pub use super::traits::*;
    pub use super::{Io, IoResult, IoError, IoErrorKind};
}

pub mod traits
{
    pub use super::{FsRead,FsDynRead,FsWrite,FsDynWrite};    
}