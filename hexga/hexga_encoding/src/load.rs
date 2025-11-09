use super::*;


pub(crate) mod prelude
{
    pub use super::{LoadExtension, Load, LoadFrom};
}

pub trait LoadExtension
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> { std::iter::empty() }
    fn load_from_reader_with_custom_extension<R>(reader: R, extension: &extension) -> EncodeResult<Self> where Self: Sized, R: Read
    {
        let _ = (reader, extension);
        Err(EncodeError::Unimplemented)
    }
}

pub trait LoadExtensionBytes : LoadExtension
{
    fn load_from_bytes_with_custom_extension(bytes: &[u8], extension: &extension) -> EncodeResult<Self> where Self: Sized
    {
        Self::load_from_reader_with_custom_extension(bytes, extension)
    }
}
impl<T> LoadExtensionBytes for T where T: LoadExtension {}



pub trait Load : LoadExtension + for<'de> CfgDeserialize<'de>
{
    fn load_extensions() -> impl Iterator<Item = &'static extension>
    {
        #[cfg(feature = "serde")]
        return Self::load_custom_extensions().chain(AnyFormat::ALL.into_iter().map(|v| v.extension()));

        #[cfg(not(feature = "serde"))]
        return Self::load_custom_extensions();
    }
    fn load_prefered_extension() -> Option<&'static extension>
    {
        Self::load_custom_extensions().next()
    }

    fn load_from_bytes(bytes: &[u8], extension: &extension) -> EncodeResult<Self> where Self: Sized
    {
        Self::load_from_reader(bytes, extension)
    }

    fn load_from_reader<R>(reader: R, extension: &extension) -> EncodeResult<Self> where Self: Sized, R: Read
    {
        if Self::load_custom_extensions().any(|e| e == extension)
        {
            return Self::load_from_reader_with_custom_extension(reader, extension);
        }

        #[cfg(feature = "serde")]
        {
            let format = AnyFormat::try_from(extension).unwrap_or_default();
            return format.from_reader(reader);
        }

        #[allow(unreachable_code)]
        Err(EncodeError::load_unsupported_extension::<Self>(extension))
    }
}
impl<T> Load for T where T: LoadExtension + for<'de> CfgDeserialize<'de> + ?Sized {}


pub trait LoadFrom : From<Self::Source>
{
    type Source: LoadExtension + Into<Self>;
}
impl<S> LoadExtension for S where S: LoadFrom
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> {
        S::Source::load_custom_extensions()
    }
    fn load_from_reader_with_custom_extension<R>(reader: R, extension: &extension) -> EncodeResult<Self> where Self: Sized, R: Read {
        S::Source::load_from_reader_with_custom_extension(reader, extension).map(|v| v.into())
    }
}
