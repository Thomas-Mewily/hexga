use super::*;

pub type FileResult<T=()> = Result<T,FileError>;


#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub enum FileError
{
    #[default]
    Unknow,
    Unimplemented,
    NotFound,
    Custom(String),
    Std(std::io::ErrorKind),
    Encoding(EncodeError)
}
impl From<EncodeError> for FileError { fn from(value: EncodeError) -> Self { Self::Encoding(value) } }
impl From<std::io::Error> for FileError { fn from(value: std::io::Error) -> Self { value.kind().into() } }
impl From<std::io::ErrorKind> for FileError { fn from(kind: std::io::ErrorKind) -> Self { Self::Std(kind) } }
impl From<String> for FileError { fn from(custom: String) -> Self { Self::Custom(custom) } }
impl From<FromUtf8Error> for FileError { fn from(value: FromUtf8Error) -> Self { value.utf8_error().into() } }
impl From<Utf8Error> for FileError { fn from(value: Utf8Error) -> Self { FileError::Encoding(value.into()) } }
impl From<Base64Error> for FileError { fn from(value: Base64Error) -> Self { FileError::Encoding(value.into()) } }

impl FileError
{
    pub fn custom(reason: impl Into<String>) -> Self { Self::Custom(reason.into()) }
    pub fn from_display(reason: impl Display) -> Self { Self::custom(reason.to_string())}
}

impl Display for FileError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self
        {
            FileError::Unknow => f.write_str("unknow"),
            FileError::Unimplemented => f.write_str("unimplemented"),
            FileError::NotFound  => f.write_str("not found"),
            FileError::Custom(reason) => write!(f, "custom: {reason}"),
            FileError::Std(kind) => write!(f, "std: {kind}"),
            FileError::Encoding(encode_error) => write!(f, "encoding: {encode_error}"),
        }
    }
}

impl std::error::Error for FileError
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }
    fn provide<'a>(&'a self, _: &mut std::error::Request<'a>) {}
}
#[cfg(feature = "serde")]
impl serde::ser::Error for FileError
{
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display { Self::Custom(msg.to_string()) }
}
#[cfg(feature = "serde")]
impl serde::de::Error for FileError
{
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display { Self::Custom(msg.to_string()) }
}
