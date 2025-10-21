use std::{str::Utf8Error, string::FromUtf8Error};

use super::*;


pub type IoResult<T=()> = Result<T,IoError>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IoMode{Read,Write}

impl IoMode
{
    pub const fn is_read(self) -> bool { matches!(self, Self::Read) }
    pub const fn is_write(self) -> bool { matches!(self, Self::Write) }
}

// TODO: impl it
#[derive(Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IoError
{
    #[default]
    Unknow,
    Unimplemented,
    NotFound,
    Std(std::io::ErrorKind),
    Utf8Error { valid_up_to : usize, error_len : Option<usize>},
    Markup { mode: IoMode, typename: String, extension: Extension, reason: String},
    UnsupportedExtension { mode: IoMode, typename : String, got : Extension, expected : Vec<Extension> },
    MissingBase { mode: IoMode, typename_source : String, typename_dest : String },
    Encoding { reason: String }
}

impl IoError
{
    pub fn encoding(reason: String) -> Self { Self::Encoding { reason }}

    pub fn unsupported_save_extension<T: ?Sized>(ext: &extension) -> Self where T: Save { Self::UnsupportedExtension { mode:IoMode::Write, typename: std::any::type_name::<T>().to_owned(), got: ext.to_owned(), expected: T::save_extensions().map(|v| v.to_owned()).collect() }}
    pub fn unsupported_load_extension<T: ?Sized>(ext: &extension) -> Self where T: Load { Self::UnsupportedExtension { mode:IoMode::Read, typename: std::any::type_name::<T>().to_owned(), got: ext.to_owned(), expected: T::load_extensions().map(|v| v.to_owned()).collect() }}

    // pub(crate) fn missing_save_extension<T>() -> Self where T: Save + ?Sized { Self::unsupported_save_extension::<T>("") }
    // pub(crate) fn missing_load_extension<T>() -> Self where T: Load + ?Sized { Self::unsupported_load_extension::<T>("") }


    #[allow(dead_code)] // why compiler ? They are used
    pub(crate) fn markup_serializer  <T: ?Sized>(ext : &extension, err : impl std::fmt::Debug) -> Self { IoError::Markup{ mode:IoMode::Write, typename: std::any::type_name::<T>().to_owned(), extension: ext.to_owned(), reason: format!("{:?}", err) } }
    #[allow(dead_code)] // why compiler ? They are used
    pub(crate) fn markup_deserializer<T: ?Sized>(ext : &extension, err : impl std::fmt::Debug) -> Self { IoError::Markup{ mode:IoMode::Read, typename: std::any::type_name::<T>().to_owned(), extension: ext.to_owned(), reason: format!("{:?}", err) } }


    pub(crate) fn save_missing_base<Src: ?Sized, Dest: ?Sized>() -> Self { Self::MissingBase { mode:IoMode::Write, typename_source: std::any::type_name::<Src>().to_owned(), typename_dest: std::any::type_name::<Dest>().to_owned() }}
}

impl From<FromUtf8Error> for IoError
{
    fn from(value: FromUtf8Error) -> Self
    {
        value.utf8_error().into()
    }
}

impl From<Utf8Error> for IoError
{
    fn from(value: Utf8Error) -> Self
    {
        Self::Utf8Error { valid_up_to: value.valid_up_to(), error_len: value.error_len() }
    }
}

impl From<std::io::Error> for IoError
{
    fn from(value: std::io::Error) -> Self {
        value.kind().into()
    }
}

impl From<std::io::ErrorKind> for IoError
{
    fn from(kind: std::io::ErrorKind) -> Self {
        Self::Std(kind)
    }
}