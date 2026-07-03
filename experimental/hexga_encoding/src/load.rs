use super::*;

pub(crate) mod prelude
{
    pub use super::{Load, LoadExtension, LoadFrom};
}

pub trait Load: Sized + for<'de> CfgDeserialize<'de>
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> { std::iter::empty() }
    fn load_prefered_extension() -> &'static extension { Self::load_custom_extensions().next().unwrap_or(FormatMarkup::PREFERED.extension()) }

    fn load_from_fs<FS: Fs, P: AsRef<Path>>(fs: &mut FS, path: P) -> FileResult<Self>
    where
        Self: Sized,
    {
        let path = path.as_ref();
        let extension = path.extension().map(|v| v.to_str()).flatten();

        let bytes = fs.read_bytes(path)?;
        let value = Self::load_from_bytes_with_extension(bytes.as_ref(), extension)?;
        Ok(value)
    }

    fn load_from_reader_with_custom_extension<R>(reader: R, extension: Option<&extension>) -> EncodeResult<Self>
    where
        R: Read,
    {
        let _ = (reader, extension);
        Err(EncodeError::Unimplemented)
    }
}

pub trait LoadExtension: Load
{
    fn load_extensions() -> impl Iterator<Item = &'static extension>
    {
        #[cfg(feature = "serde")]
        return Self::load_custom_extensions().chain(AnyFormat::ALL.into_iter().map(|v| v.extension()));

        #[cfg(not(feature = "serde"))]
        return Self::load_custom_extensions();
    }

    fn load_from_bytes(bytes: &[u8]) -> EncodeResult<Self>
    where
        Self: Sized,
    {
        Self::load_from_reader_with_extension(bytes, None)
    }

    fn load_from_bytes_with_extension(bytes: &[u8], extension: Option<&extension>) -> EncodeResult<Self>
    where
        Self: Sized,
    {
        Self::load_from_reader_with_extension(bytes, extension)
    }

    fn load_from_reader<R>(reader: R) -> EncodeResult<Self>
    where
        Self: Sized,
        R: Read,
    {
        Self::load_from_reader_with_extension(reader, None)
    }

    fn load_from_reader_with_extension<R>(reader: R, extension: Option<&extension>) -> EncodeResult<Self>
    where
        Self: Sized,
        R: Read,
    {
        if Self::load_custom_extensions().any(|e| Some(e) == extension)
        {
            return Self::load_from_reader_with_custom_extension(reader, extension);
        }
        if extension.is_none()
        {
            return Self::load_from_reader_with_custom_extension(reader, Some(Self::load_prefered_extension()));
        }

        #[cfg(feature = "serde")]
        {
            if let Some(ex) = extension
            {
                return AnyFormat::try_from(ex).unwrap_or_default().from_reader(reader);
            }
        }

        #[allow(unreachable_code)]
        return Err(EncodeError::load_unsupported_extension::<Self>(extension.map(|e| e.to_owned().into())));
    }
}
impl<T> LoadExtension for T where T: Load {}

pub trait LoadFrom: From<Self::Source>
{
    type Source: Load + Into<Self>;
}
impl<S> Load for S
where
    S: LoadFrom + for<'de> CfgDeserialize<'de>,
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> { S::Source::load_custom_extensions() }
    fn load_from_fs<FS: Fs, P: AsRef<Path>>(fs: &mut FS, path: P) -> FileResult<Self>
    where
        Self: Sized,
    {
        S::Source::load_from_fs(fs, path).map(|v| v.into())
    }
    fn load_from_reader_with_custom_extension<R>(reader: R, extension: Option<&extension>) -> EncodeResult<Self>
    where
        Self: Sized,
        R: Read,
    {
        S::Source::load_from_reader_with_custom_extension(reader, extension).map(|v| v.into())
    }
}
