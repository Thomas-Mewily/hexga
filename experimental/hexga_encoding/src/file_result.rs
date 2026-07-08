use super::*;

pub mod prelude
{
    pub use super::{FileError, FileResult};
}

pub type FileResult<T = ()> = Result<T, FileError>;

pub struct FileError
{
    pub path: Option<PathBuf>,
    pub kind: FileErrorKind,
}
impl FileError
{
    pub fn new(kind: impl Into<FileErrorKind>) -> Self { Self { kind: kind.into(), path: None } }
    pub fn with_path(mut self, path: Option<PathBuf>) -> Self
    {
        self.path = path;
        self
    }
}
impl Deref for FileError
{
    type Target = FileErrorKind;
    fn deref(&self) -> &Self::Target { &self.kind }
}
impl DerefMut for FileError
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.kind }
}
impl Debug for FileError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match &self.path
        {
            Some(path) => write!(f, "{:?} at {:?}", self.kind, path.display()),
            None => write!(f, "{:?} at unknow path", self.kind),
        }
    }
}
pub enum FileErrorKind
{
    /// Problem when encoding the data.
    Encode(EncodeError),
    /// Problem with the file system : File not found, out of free space...
    Io(IoError),
}
impl From<IoError> for FileErrorKind
{
    fn from(value: IoError) -> Self { Self::Io(value) }
}
impl From<IoErrorKind> for FileErrorKind
{
    fn from(value: IoErrorKind) -> Self { Self::Io(IoError::from(value)) }
}
impl From<EncodeError> for FileErrorKind
{
    fn from(value: EncodeError) -> Self { Self::Encode(value) }
}
impl Debug for FileErrorKind
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self
        {
            Self::Encode(v) => write!(f, "{:?}", v),
            Self::Io(v) => write!(f, "{:?}", v),
        }
    }
}

impl FileErrorKind
{
    pub fn is_io(&self) -> bool { matches!(self, Self::Io(_)) }
    pub fn is_encode(&self) -> bool { matches!(self, Self::Encode(_)) }
}

/*
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
*/
