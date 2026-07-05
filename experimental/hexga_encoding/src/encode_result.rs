use super::*;

pub mod prelude
{
    pub use super::{EncodeError, EncodeResult};
}

/// Encoding = inside a buffer. No file system involved.
pub type EncodeResult<T = ()> = Result<T, EncodeError>;

#[non_exhaustive]
#[derive(Default, Clone, PartialEq, Eq)]
pub enum EncodeError
{
    #[default]
    Unknow,
    Unimplemented,
    Base64(Base64Error),
    Extension(EncodeErrorExtension),
    Utf8(EncodeErrorUtf8),
    /// Error with the Read/Write/Seek trait
    Io(IoErrorKind),
    Custom(EncodeErrorReason),
    Markup(EncodeErrorMarkup),
    Fmt(FmtError),
}

impl Debug for EncodeError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        match self
        {
            EncodeError::Unknow => write!(f, "Unknow"),
            EncodeError::Unimplemented => write!(f, "Unimplemented"),
            EncodeError::Base64(v) => write!(f, "{:?}", v),
            EncodeError::Extension(v) => write!(f, "{:?}", v),
            EncodeError::Utf8(v) => write!(f, "{:?}", v),
            EncodeError::Custom(v) => write!(f, "{:?}", v),
            EncodeError::Io(v) => write!(f, "{:?}", v),
            EncodeError::Markup(v) => write!(f, "{:?}", v),
            EncodeError::Fmt(v) => write!(f, "{:?}", v),
        }
    }
}
impl Display for EncodeError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "{:?}", self) }
}

pub type EncodeErrorReason = String;

impl From<Base64Error> for EncodeError
{
    fn from(value: Base64Error) -> Self { Self::Base64(value) }
}
impl From<EncodeErrorExtension> for EncodeError
{
    fn from(value: EncodeErrorExtension) -> Self { Self::Extension(value) }
}
impl From<EncodeErrorUtf8> for EncodeError
{
    fn from(value: EncodeErrorUtf8) -> Self { Self::Utf8(value) }
}
impl From<Utf8Error> for EncodeError
{
    fn from(value: Utf8Error) -> Self { Self::Utf8(value.into()) }
}
impl From<EncodeErrorReason> for EncodeError
{
    fn from(value: EncodeErrorReason) -> Self { Self::Custom(value) }
}
impl From<IoErrorKind> for EncodeError
{
    fn from(value: IoErrorKind) -> Self { Self::Io(value) }
}
impl From<IoError> for EncodeError
{
    fn from(value: IoError) -> Self { Self::Io(value.kind()) }
}
impl From<EncodeErrorMarkup> for EncodeError
{
    fn from(value: EncodeErrorMarkup) -> Self { Self::Markup(value) }
}
impl From<FmtError> for EncodeError
{
    fn from(value: FmtError) -> Self { Self::Fmt(value) }
}

impl EncodeError
{
    pub fn at_path(self, path: Option<PathBuf>) -> FileError { FileError::new(self).with_path(path) }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct EncodeErrorExtension
{
    pub got: Option<CowExtensionStatic>,
    pub expected: Vec<CowExtensionStatic>,
}
impl Debug for EncodeErrorExtension
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult
    {
        write!(
            f,
            "Extension {:?} is not supported, expected one of the following extension {:?}, ",
            self.got, self.expected
        )
    }
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct EncodeErrorUtf8
{
    pub valid_up_to: usize,
    pub error_len: Option<usize>,
}
impl From<Utf8Error> for EncodeErrorUtf8
{
    fn from(value: Utf8Error) -> Self
    {
        Self {
            valid_up_to: value.valid_up_to(),
            error_len: value.error_len(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncodeErrorMarkup
{
    pub extension: CowExtensionStatic,
    pub reason: EncodeErrorReason,
}

/*
pub type Reason = Cow<'static, str>;

// const PREFIX: &[u8] = b"custom_extension;";


#[non_exhaustive]
#[derive(Default, Clone, PartialEq, Eq)]
pub enum EncodeError
{
    #[default]
    Unknow,
    Fmt,
    NotPersistant,
    NotLoaded,
    Unimplemented,
    Markup
    {
        extension: CowExtensionStatic,
        reason: Reason,
    },
    Utf8Error
    {
        valid_up_to: usize,
        error_len: Option<usize>,
    },
    UnsupportedExtension
    {
        got: Option<CowExtensionStatic>,
        expected: HashSet<CowExtensionStatic>,
    },
    Custom(Reason),
    Base64(Base64Error),
    Io(IoErrorKind),
}
impl From<FromUtf8Error> for EncodeError
{
    fn from(value: FromUtf8Error) -> Self { value.utf8_error().into() }
}
impl From<Utf8Error> for EncodeError
{
    fn from(value: Utf8Error) -> Self
    {
        Self::Utf8Error {
            valid_up_to: value.valid_up_to(),
            error_len: value.error_len(),
        }
    }
}
impl From<Base64Error> for EncodeError
{
    fn from(value: Base64Error) -> Self { Self::Base64(value) }
}
impl From<IoError> for EncodeError
{
    fn from(value: IoError) -> Self { value.kind().into() }
}
impl From<IoErrorKind> for EncodeError
{
    fn from(kind: IoErrorKind) -> Self { Self::Io(kind) }
}
impl From<std::fmt::Error> for EncodeError
{
    fn from(_value: std::fmt::Error) -> Self { Self::Fmt }
}

impl From<String> for EncodeError
{
    fn from(custom: String) -> Self { Self::Custom(custom.into()) }
}
impl From<&'static str> for EncodeError
{
    fn from(custom: &'static str) -> Self { Self::Custom(custom.into()) }
}

impl Display for EncodeError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult
    {
        match self
        {
            EncodeError::Markup { extension, reason } =>
            {
                write!(f, "failed to convert to {extension} : {reason}")
            }
            EncodeError::Utf8Error { valid_up_to, error_len } =>
            {
                // Copied from std Utf8Error
                if let Some(error_len) = error_len
                {
                    write!(f, "invalid utf-8 sequence of {} bytes from index {}", error_len, valid_up_to)
                }
                else
                {
                    write!(f, "incomplete utf-8 byte sequence from index {}", valid_up_to)
                }
            }
            EncodeError::UnsupportedExtension { got, expected } => write!(f, "unsupported extension {got:?}, expected one of {expected:?}"),
            EncodeError::Custom(reason) => write!(f, "custom: {}", reason),
            EncodeError::Unknow => write!(f, "unknow"),
            EncodeError::Base64(base64) => write!(f, "base64: {}", base64),
            EncodeError::Io(std) => write!(f, "std: {}", std),
            EncodeError::Fmt => write!(f, "formating"),
            EncodeError::Unimplemented => write!(f, "unimplemented"),
            EncodeError::NotPersistant => write!(f, "not persistant"),
            EncodeError::NotLoaded => write!(f, "not loaded"),
        }
    }
}
impl std::fmt::Debug for EncodeError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "{}", self) }
}
*/

impl EncodeError
{
    pub fn save_unsupported_extension<T: Save + ?Sized>(got: impl Into<Option<CowExtensionStatic>>) -> Self
    {
        Self::Extension(EncodeErrorExtension {
            //name: std::any::type_name::<T>().to_owned(),
            got: got.into(),
            expected: T::save_extensions().map(|ext| ext.into()).collect(),
        })
    }

    pub fn load_unsupported_extension<T: Load + ?Sized>(got: impl Into<Option<CowExtensionStatic>>) -> Self
    {
        Self::Extension(EncodeErrorExtension {
            //name: std::any::type_name::<T>().to_owned(),
            got: got.into(),
            expected: T::load_extensions().map(|ext| ext.into()).collect(),
        })
    }

    pub fn markup<T: ?Sized>(extension: impl Into<CowExtensionStatic>, reason: impl Display) -> Self
    {
        Self::Markup(EncodeErrorMarkup {
            //name: std::any::type_name::<T>().to_owned(),
            extension: extension.into(),
            reason: reason.to_string().into(),
        })
    }

    pub fn custom(reason: impl Into<EncodeErrorReason>) -> Self { Self::Custom(reason.into()) }
    pub fn from_display(reason: impl Display) -> Self { Self::custom(reason.to_string()) }
}

/*
impl EncodeError
{
    pub fn utf8_error(valid_up_to: usize, error_len: Option<usize>) -> Self { Self::Utf8Error { valid_up_to, error_len } }

    pub fn save_unsupported_extension_with_name<T: Save + ?Sized>(got: impl Into<Option<CowExtensionStatic>>, _name: impl Into<String>) -> Self
    {
        Self::UnsupportedExtension {
            //name: name.into(),
            got: got.into(),
            expected: T::save_extensions().map(|ext| ext.into()).collect(),
        }
    }
    pub fn save_unsupported_extension<T: Save + ?Sized>(got: impl Into<Option<CowExtensionStatic>>) -> Self
    {
        Self::save_unsupported_extension_with_name::<T>(got, std::any::type_name::<T>())
    }

    pub fn load_unsupported_extension_with_name<T: Load + ?Sized>(got: impl Into<Option<CowExtensionStatic>>, _name: impl Into<String>) -> Self
    {
        Self::UnsupportedExtension {
            //name: name.into(),
            got: got.into(),
            expected: T::load_custom_extensions().map(|ext| ext.into()).collect(),
        }
    }
    pub fn load_unsupported_extension<T: Load + ?Sized>(got: impl Into<Option<CowExtensionStatic>>) -> Self
    {
        Self::load_unsupported_extension_with_name::<T>(got, std::any::type_name::<T>())
    }

    pub fn markup<T: ?Sized>(extension: impl Into<CowExtensionStatic>, reason: impl Display) -> Self
    {
        Self::Markup {
            //name: std::any::type_name::<T>().to_owned(),
            extension: extension.into(),
            reason: reason.to_string().into(),
        }
    }

    pub fn custom(reason: impl Into<Reason>) -> Self { Self::Custom(reason.into()) }
    pub fn from_display(reason: impl Display) -> Self { Self::custom(reason.to_string()) }
}

impl std::error::Error for EncodeError {}
#[cfg(feature = "serde")]
impl serde::ser::Error for EncodeError
{
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::custom(msg.to_string())
    }
}
#[cfg(feature = "serde")]
impl serde::de::Error for EncodeError
{
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::custom(msg.to_string())
    }
}
*/

impl std::error::Error for EncodeError {}
#[cfg(feature = "serde")]
impl serde::ser::Error for EncodeError
{
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::custom(msg.to_string())
    }
}
#[cfg(feature = "serde")]
impl serde::de::Error for EncodeError
{
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::custom(msg.to_string())
    }
}
