use std::default;

use crate::*;

pub type IoErrorInternal = std::io::ErrorKind;

pub type IoResult<T=()> = Result<T, IoErrorKind>;

#[derive(Default, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum IoErrorKind
{
    #[default]
    Unknow,
    Unimplemented,
    Internal(IoErrorInternal),
    FromNotBaseOn,
    FromBasedOnFailed    { dest : TypeName, src : TypeName, reason : Reason },
    UnsupportedExtension { name : TypeName, got : Extension, expected : Vec<Extension> },
    EncodingBadUtf8      { valid_up_to : usize, error_len : Option<usize>},
    Encoding(Reason),
    Markup(TypeName, Extension, Reason),
}

impl IoErrorKind
{
    pub const fn is_unknow(&self) -> bool { matches!(self, Self::Unknow) }
    pub const fn is_unimplemented(&self) -> bool { matches!(self, Self::Unimplemented) }
    pub const fn is_from_not_base_on(&self) -> bool { matches!(self, Self::FromNotBaseOn) }

    pub const fn is_unimplemented_or_unknow(&self) -> bool { self.is_unimplemented() || self.is_unknow() }

}

impl IoErrorKind
{
    pub fn unsupported_open_extension<T>(ext : &extension) -> Self where T : IoLoad { Self::UnsupportedExtension { name: std::any::type_name::<T>().to_owned(), got: ext.to_owned(), expected: T::load_extensions().map(|v| v.to_owned()).collect() }}
    pub fn unsupported_save_extension<T>(ext : &extension) -> Self where T : IoSave { Self::UnsupportedExtension { name: std::any::type_name::<T>().to_owned(), got: ext.to_owned(), expected: T::load_extensions().map(|v| v.to_owned()).collect() }}

    pub fn serialize<T>(ext : &extension, err : impl ToDebug) -> Self { IoErrorKind::Markup(std::any::type_name::<T>().to_owned(), ext.to_owned(), err.to_debug()) }
    pub fn deserialize<T>(ext : &extension, err : impl ToDebug) -> Self { IoErrorKind::Markup(std::any::type_name::<T>().to_owned(), ext.to_owned(), err.to_debug()) }
}

// Todo : do a better job for io error
#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct IoError
{
    pub mode : IoMode,
    pub path : Path,
    pub kind : IoErrorKind,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum IoMode
{
    Read, Write,
}

impl IoError
{
    pub fn new(mode : IoMode, path : impl Into<Path>, kind : IoErrorKind) -> Self { Self { mode, path : path.into(), kind } }
    pub fn read(path : impl Into<Path>, kind : IoErrorKind) -> Self { Self::new(IoMode::Read, path, kind) }
    pub fn write(path : impl Into<Path>, kind : IoErrorKind) -> Self { Self::new(IoMode::Write, path, kind) }
}

pub trait ToReadError
{
    type Output;
    fn to_read_error(self, path : &path) -> Self::Output;
}

impl ToReadError for IoErrorKind
{
    type Output = IoError;
    fn to_read_error(self, path : &path) -> Self::Output { IoError::read(path, self) }
}
impl<T,E> ToReadError for Result<T,E> where E : ToReadError
{
    type Output = Result<T,E::Output>;
    fn to_read_error(self, path : &path) -> Self::Output { self.map_err(|e| e.to_read_error(path)) }
}
impl ToReadError for IoErrorInternal
{
    type Output=IoError;
    fn to_read_error(self, path : &path) -> Self::Output {
        IoError::read(path, IoErrorKind::Internal(self))
    }
}
impl ToReadError for std::io::Error
{
    type Output=IoError;
    fn to_read_error(self, path : &path) -> Self::Output {
        self.kind().to_read_error(path)
    }
}
