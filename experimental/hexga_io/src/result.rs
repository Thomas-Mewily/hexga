use super::*;

pub type IoErrorKind = std::io::ErrorKind;
pub type IoError = std::io::Error;
pub type IoResult<T = ()> = Result<T, IoError>;

pub trait IoErrorExt
{
    fn new_with_path<E>(kind: IoErrorKind, error: E, path: impl AsRef<Path>) -> IoError
    where
        E: Debug + 'static;
}

impl IoErrorExt for IoError
{
    fn new_with_path<E>(kind: IoErrorKind, error: E, path: impl AsRef<Path>) -> IoError
    where
        E: Debug + 'static,
    {
        IoError::new(kind, format!("{:?} at path: {}", error, path.as_ref().display()))
    }
}
