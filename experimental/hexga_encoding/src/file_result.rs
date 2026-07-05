use super::*;

pub mod prelude
{
    pub use super::{FileError, FileResult};
}

pub type FileResult<T = ()> = Result<T, FileError>;

pub enum FileError
{
    /// Problem when encoding the data.
    Encode(EncodeFileError),
    /// Problem with the file system : File not found, out of free space...
    Io(IoError),
}
impl Debug for FileError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encode(v) => write!(f, "{:?}", v),
            Self::Io(v) => write!(f, "{:?}", v),
        }
    }
}

impl FileError
{
    pub fn is_io(&self) -> bool { matches!(self, Self::Io(_)) }
    pub fn is_encode(&self) -> bool { matches!(self, Self::Encode(_)) }
}
impl From<EncodeFileError> for FileError
{
    fn from(value: EncodeFileError) -> Self { Self::Encode(value) }
}
impl From<IoError> for FileError
{
    fn from(value: IoError) -> Self { Self::Io(value) }
}
impl From<IoErrorKind> for FileError
{
    fn from(value: IoErrorKind) -> Self { Self::Io(IoError::from(value)) }
}
impl From<EncodeError> for FileError
{
    fn from(value: EncodeError) -> Self { Self::Encode(EncodeFileError::new(value)) }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct EncodeFileError
{
    pub error: EncodeError,
    pub path: Option<PathBuf>,
}
impl Debug for EncodeFileError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.path
        {
            Some(at) => write!(f, "{:?} at {:?}", self.error, at.display()),
            None => write!(f, "{:?}", self.error),
        }
    }
}
impl EncodeFileError
{
    pub fn new(error: EncodeError) -> Self { Self { error, path: None } }
    pub fn with_path(mut self, path: Option<PathBuf>) -> Self
    {
        self.path = path;
        self
    }
}
