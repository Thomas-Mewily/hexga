use super::*;


pub type IoResult<T=()> = Result<T,IoError>;

// TODO: impl it
pub struct IoError;


impl From<std::io::Error> for IoError
{
    fn from(_: std::io::Error) -> Self {
        Self
    }
}

impl From<std::io::ErrorKind> for IoError
{
    fn from(_: std::io::ErrorKind) -> Self {
        Self
    }
}