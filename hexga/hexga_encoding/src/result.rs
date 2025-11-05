use super::*;


pub type EncodeResult<T=()> = Result<T,EncodeError>;

// const PREFIX: &[u8] = b"custom_extension;";

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub enum EncodeError
{
    #[default]
    Unknow,
    Markup { extension: String, reason: String },
    Utf8Error { valid_up_to : usize, error_len : Option<usize>},
    UnsupportedExtension { got : Extension, expected : Vec<Extension> },
    Custom(String),
    Base64(Base64Error),
    Std(std::io::ErrorKind),
}
impl From<FromUtf8Error> for EncodeError { fn from(value: FromUtf8Error) -> Self { value.utf8_error().into() } }
impl From<Utf8Error> for EncodeError { fn from(value: Utf8Error) -> Self { Self::Utf8Error { valid_up_to: value.valid_up_to(), error_len: value.error_len() } } }
impl From<Base64Error> for EncodeError { fn from(value: Base64Error) -> Self { Self::Base64(value) } }
impl From<std::io::Error> for EncodeError { fn from(value: std::io::Error) -> Self { value.kind().into() } }
impl From<std::io::ErrorKind> for EncodeError { fn from(kind: std::io::ErrorKind) -> Self { Self::Std(kind) } }

impl Display for EncodeError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            EncodeError::Markup { extension, reason } => write!(f,"failed to convert to {extension} : {reason}"),
            EncodeError::Utf8Error { valid_up_to, error_len } =>
            {
                // Copied from std Utf8Error
                if let Some(error_len) = error_len {
                    write!(f, "invalid utf-8 sequence of {} bytes from index {}", error_len, valid_up_to)
                } else {
                    write!(f, "incomplete utf-8 byte sequence from index {}", valid_up_to)
                }
            },
            EncodeError::UnsupportedExtension { got, expected } => write!(f, "unsupported extension {got}, expected one of {expected:?}"),
            EncodeError::Custom(reason) => write!(f, "custom: {}", reason),
            EncodeError::Unknow => write!(f, "unknow"),
            EncodeError::Base64(base64) => write!(f, "base64: {}", base64),
            EncodeError::Std(std) => write!(f, "std: {}", std)
        }
    }
}


impl EncodeError
{
    pub fn utf8_error(valid_up_to : usize, error_len : Option<usize>) -> Self
    {
        Self::Utf8Error { valid_up_to, error_len }
    }


    pub fn save_unsupported_extension_with_name<T:Save + ?Sized>(got: impl Into<Extension>, _name: impl Into<String>) -> Self
    {
        Self::UnsupportedExtension
        {
            //name: name.into(),
            got: got.into(),
            expected: T::save_extensions().map(|ext| ext.to_owned()).collect()
        }
    }
    pub fn save_unsupported_extension<T:Save + ?Sized>(got: impl Into<Extension>) -> Self { Self::save_unsupported_extension_with_name::<T>(got, std::any::type_name::<T>()) }


    pub fn load_unsupported_extension_with_name<T:Load + ?Sized>(got: impl Into<Extension>, _name: impl Into<String>) -> Self
    {
        Self::UnsupportedExtension
        {
            //name: name.into(),
            got: got.into(),
            expected: T::load_extensions().map(|ext| ext.to_owned()).collect()
        }
    }
    pub fn load_unsupported_extension<T:Load + ?Sized>(got: impl Into<Extension>) -> Self { Self::load_unsupported_extension_with_name::<T>(got, std::any::type_name::<T>()) }


    pub fn markup<T: ?Sized>(extension: impl Into<String>, reason: impl Display) -> Self
    {
        Self::Markup
        {
            //name: std::any::type_name::<T>().to_owned(),
            extension: extension.into(),
            reason: reason.to_string()
        }
    }

    pub fn custom(reason: impl Into<String>) -> Self { Self::Custom(reason.into()) }
    pub fn from_display(reason: impl Display) -> Self { Self::custom(reason.to_string())}
}


impl std::error::Error for EncodeError
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }
    fn provide<'a>(&'a self, _: &mut std::error::Request<'a>) {}
}
#[cfg(feature = "serde")]
impl serde::ser::Error for EncodeError
{
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display { Self::custom(msg.to_string()) }
}
#[cfg(feature = "serde")]
impl serde::de::Error for EncodeError
{
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display { Self::custom(msg.to_string()) }
}
