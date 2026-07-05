use super::*;

pub(crate) mod prelude
{
    pub use super::{Save, SaveExtension, SaveInto};
}

const DEFAULT_WRITER_CAPACITY: usize = 1024;

pub trait Save : CfgSerialize
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> { std::iter::empty() }
    fn save_prefered_extension() -> &'static extension { Self::save_custom_extensions().next().unwrap_or(FormatMarkup::PREFERED.extension()) }

    fn save_to_fs<FS: Fs, P: AsRef<Path>>(&self, fs: &mut FS, path: P) -> FileResult
    where
        Self: Sized,
    {
        let path = path.as_ref();
        let extension = path.extension().map(|v| v.to_str()).flatten();

        let (writer, _extension) = self.save_to_bytes_with_extension_in(Vec::with_capacity(DEFAULT_WRITER_CAPACITY), extension)?;
        fs.write_bytes(path, &writer)?;
        Ok(())
    }

    fn save_to_writer_with_custom_extension<'ext, W>(&self, writer: W, extension: Option<&'ext extension>) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
    {
        let _ = (writer, extension);
        Err(EncodeError::Unimplemented)
    }
}

pub trait SaveExtension: Save
{
    fn save_to_bytes_with_custom_extension(&self, extension: Option<&extension>) -> EncodeResult<Vec<u8>>
    {
        self.save_to_bytes_with_custom_extension_in(Vec::with_capacity(DEFAULT_WRITER_CAPACITY), extension)
    }
    fn save_to_bytes_with_custom_extension_in(&self, mut bytes: Vec<u8>, extension: Option<&extension>) -> EncodeResult<Vec<u8>>
    {
        self.save_to_writer_with_custom_extension(&mut bytes, extension)?;
        Ok(bytes)
    }
    fn save_extensions() -> impl Iterator<Item = &'static extension>
    {
        #[cfg(feature = "serde")]
        return Self::save_custom_extensions().chain(AnyFormat::ALL.into_iter().map(|v| v.extension()));

        #[cfg(not(feature = "serde"))]
        return Self::save_custom_extensions();
    }

    fn save_to_bytes(&self) -> EncodeResult<(Vec<u8>, DeducedExtension<'static>)> { self.save_to_bytes_in(Vec::with_capacity(DEFAULT_WRITER_CAPACITY)) }
    fn save_to_bytes_in(&self, bytes: Vec<u8>) -> EncodeResult<(Vec<u8>, DeducedExtension<'static>)> { self.save_to_bytes_with_extension_in(bytes, None) }

    fn save_to_bytes_with_extension<'ext>(&self, extension: Option<&'ext extension>) -> EncodeResult<(Vec<u8>, DeducedExtension<'ext>)>
    {
        self.save_to_bytes_with_extension_in(Vec::with_capacity(DEFAULT_WRITER_CAPACITY), extension)
    }
    fn save_to_bytes_with_extension_in<'ext>(&self, mut bytes: Vec<u8>, extension: Option<&'ext extension>) -> EncodeResult<(Vec<u8>, DeducedExtension<'ext>)>
    {
        let r = self.save_to_writer_with_extension(&mut bytes, extension)?;
        Ok((bytes, r))
    }
    fn save_to_writer<'ext, W>(&self, writer: &mut W) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
    {
        self.save_to_writer_with_extension(writer, None)
    }
    fn save_to_writer_with_extension<'ext, W>(&self, writer: &mut W, extension: Option<&'ext extension>) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
    {
        if Self::save_custom_extensions().any(|e| Some(e) == extension)
        {
            return self.save_to_writer_with_custom_extension(writer, extension);
        }
        if extension.is_none()
        {
            return self.save_to_writer_with_custom_extension(writer, Some(Self::save_prefered_extension()));
        }

        #[cfg(feature = "serde")]
        {
            let format = match extension
            {
                Some(ex) => AnyFormat::try_from(ex).ok(),
                None => None,
            }
            .unwrap_or_default();

            format.encode_with_writer(&self, writer)?;
            return Ok(format.extension().into());
        }

        #[allow(unreachable_code)]
        Err(EncodeError::save_unsupported_extension::<Self>(extension.map(Into::into)))
    }
}
impl<T> SaveExtension for T where T: Save + ?Sized {}

pub trait SaveInto
{
    type Output: Save + for<'a> From<&'a Self>;
}
impl<S> Save for S
where
    S: SaveInto + CfgSerialize,
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> { S::Output::save_custom_extensions() }
    fn save_to_fs<FS: Fs, P: AsRef<Path>>(&self, fs: &mut FS, path: P) -> FileResult
    where
        Self: Sized,
    {
        S::Output::save_to_fs(&self.into(), fs, path)
    }
    fn save_to_writer_with_custom_extension<'ext, W>(&self, writer: W, extension: Option<&'ext extension>) -> EncodeResult<DeducedExtension<'ext>>
    where
        W: Write,
    {
        S::Output::save_to_writer_with_custom_extension(&self.into(), writer, extension)
    }
}
