use super::*;

pub type IoResult<T = ()> = Result<T, IoError>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IoMode
{
    Read,
    Write,
}

impl Display for IoMode
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            IoMode::Read => f.write_str("Read"),
            IoMode::Write => f.write_str("Write"),
        }
    }
}

impl IoMode
{
    pub const fn is_read(self) -> bool { matches!(self, Self::Read) }
    pub const fn is_write(self) -> bool { matches!(self, Self::Write) }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct IoError
{
    pub path: PathBuf,
    pub mode: Option<IoMode>,
    pub kind: FileError,
}
impl IoError
{
    pub fn new(path: impl Into<PathBuf>, kind: impl Into<FileError>) -> Self
    {
        Self {
            path: path.into(),
            kind: kind.into(),
            mode: None,
        }
    }
    pub fn with_mode(self, mode: Option<IoMode>) -> Self { Self { mode, ..self } }
    pub fn when_reading(self) -> Self
    {
        Self {
            mode: Some(IoMode::Read),
            ..self
        }
    }
    pub fn when_writing(self) -> Self
    {
        Self {
            mode: Some(IoMode::Write),
            ..self
        }
    }
}
impl Display for IoError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "IO error at {:?} ", self.path)?;
        if let Some(mode) = self.mode
        {
            match mode
            {
                IoMode::Read => write!(f, "when reading")?,
                IoMode::Write => write!(f, "when writing")?,
            }
        }
        write!(f, ": {}", self.kind)
    }
}
impl std::fmt::Debug for IoError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self) }
}

impl std::error::Error for IoError {}

#[cfg(feature = "serde")]
impl serde::ser::Error for IoError
{
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self {
            kind: FileError::custom(msg.to_string()),
            ..Default::default()
        }
    }
}
#[cfg(feature = "serde")]
impl serde::de::Error for IoError
{
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self {
            kind: FileError::custom(msg.to_string()),
            ..Default::default()
        }
    }
}

pub type FileResult<T = ()> = Result<T, FileError>;

#[non_exhaustive]
#[derive(Default, Clone, PartialEq, Eq)]
pub enum FileError
{
    #[default]
    Unknow,
    NotSupported,
    DownloadFailed,
    Unimplemented,
    NotFound,
    Custom(Reason),
    Std(std::io::ErrorKind),
    Encoding(EncodeError),
}
impl From<EncodeError> for FileError
{
    fn from(value: EncodeError) -> Self { Self::Encoding(value) }
}
impl From<String> for FileError
{
    fn from(custom: String) -> Self { Self::Custom(custom.into()) }
}
impl From<&'static str> for FileError
{
    fn from(custom: &'static str) -> Self { Self::Custom(custom.into()) }
}

impl From<FromUtf8Error> for FileError
{
    fn from(value: FromUtf8Error) -> Self { value.utf8_error().into() }
}
impl From<Utf8Error> for FileError
{
    fn from(value: Utf8Error) -> Self { FileError::Encoding(value.into()) }
}
impl From<Base64Error> for FileError
{
    fn from(value: Base64Error) -> Self { FileError::Encoding(value.into()) }
}
impl From<std::io::Error> for FileError
{
    fn from(value: std::io::Error) -> Self { value.kind().into() }
}
impl From<std::io::ErrorKind> for FileError
{
    fn from(kind: std::io::ErrorKind) -> Self { Self::Std(kind) }
}
impl From<std::fmt::Error> for FileError
{
    fn from(value: std::fmt::Error) -> Self { Self::Encoding(value.into()) }
}

impl FileError
{
    pub fn custom(reason: impl Into<Reason>) -> Self { Self::Custom(reason.into()) }
    pub fn from_display(reason: impl Display) -> Self { Self::custom(reason.to_string()) }

    pub fn is_encoding(&self) -> bool { matches!(self, Self::Encoding(_)) }
}

impl Display for FileError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            FileError::Unknow => f.write_str("unknow"),
            FileError::Unimplemented => f.write_str("unimplemented"),
            FileError::NotSupported => f.write_str("not supported"),
            FileError::DownloadFailed => f.write_str("download failed"),
            FileError::NotFound => f.write_str("not found"),
            FileError::Custom(reason) => write!(f, "custom: {reason}"),
            FileError::Std(kind) => write!(f, "std: {kind}"),
            FileError::Encoding(encode_error) => write!(f, "encoding: {encode_error}"),
        }
    }
}
impl std::fmt::Debug for FileError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self) }
}

impl std::error::Error for FileError {}
#[cfg(feature = "serde")]
impl serde::ser::Error for FileError
{
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::custom(msg.to_string())
    }
}
#[cfg(feature = "serde")]
impl serde::de::Error for FileError
{
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::custom(msg.to_string())
    }
}
