use super::*;

pub(crate) mod prelude
{
    pub use super::{Save, SaveAs, SaveExtension};
}

pub trait SaveExtension: CfgSerialize
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> { std::iter::empty() }
    fn save_to_writer_with_custom_extension<W>(
        &self,
        writer: W,
        extension: Option<&extension>,
    ) -> EncodeResult
    where
        W: Write,
    {
        let _ = (writer, extension);
        Err(EncodeError::Unimplemented)
    }
}

pub trait SaveExtensionBytes: SaveExtension
{
    fn save_to_bytes_with_custom_extension(
        &self,
        extension: Option<&extension>,
    ) -> EncodeResult<Vec<u8>>
    {
        self.save_to_bytes_with_custom_extension_in(
            Vec::with_capacity(DEFAULT_WRITER_CAPACITY),
            extension,
        )
    }
    fn save_to_bytes_with_custom_extension_in(
        &self,
        mut bytes: Vec<u8>,
        extension: Option<&extension>,
    ) -> EncodeResult<Vec<u8>>
    {
        self.save_to_writer_with_custom_extension(&mut bytes, extension)?;
        Ok(bytes)
    }
}
impl<T> SaveExtensionBytes for T where T: SaveExtension {}

const DEFAULT_WRITER_CAPACITY: usize = 1024;

pub trait Save: SaveExtension
{
    fn save_extensions() -> impl Iterator<Item = &'static extension>
    {
        #[cfg(feature = "serde")]
        return Self::save_custom_extensions()
            .chain(AnyFormat::ALL.into_iter().map(|v| v.extension()));

        #[cfg(not(feature = "serde"))]
        return Self::save_custom_extensions();
    }
    fn save_prefered_extension() -> Option<&'static extension>
    {
        Self::save_custom_extensions().next()
    }

    fn save_to_bytes<'ext>(
        &self,
        extension: Option<&'ext extension>,
    ) -> EncodeResult<(Vec<u8>, Option<DeducedExtension<'ext>>)>
    {
        self.save_to_bytes_in(Vec::with_capacity(DEFAULT_WRITER_CAPACITY), extension)
    }
    fn save_to_bytes_in<'ext>(
        &self,
        mut bytes: Vec<u8>,
        extension: Option<&'ext extension>,
    ) -> EncodeResult<(Vec<u8>, Option<DeducedExtension<'ext>>)>
    {
        let r = self.save_to_writer(&mut bytes, extension)?;
        Ok((bytes, r))
    }
    fn save_to_writer<'ext, W>(
        &self,
        writer: &mut W,
        extension: Option<&'ext extension>,
    ) -> EncodeResult<Option<DeducedExtension<'ext>>>
    where
        W: Write,
    {
        if Self::save_custom_extensions().any(|e| Some(e) == extension)
        {
            return self
                .save_to_writer_with_custom_extension(writer, extension)
                .map(|_| extension.map(Into::into));
        }
        if extension.is_none()
            && let Some(ext) = Self::save_prefered_extension()
        {
            return self
                .save_to_writer_with_custom_extension(writer, Some(ext))
                .map(|_| Some(ext.into()));
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
            return Ok(Some(format.extension().into()));
        }

        #[allow(unreachable_code)]
        Err(EncodeError::save_unsupported_extension::<Self>(
            extension.map(Into::into),
        ))
    }
}
impl<T> Save for T where T: SaveExtension + ?Sized {}

pub trait SaveAs
{
    type Output: SaveExtension + for<'a> From<&'a Self>;
}
impl<S> SaveExtension for S
where
    S: SaveAs + CfgSerialize,
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension>
    {
        S::Output::save_custom_extensions()
    }
    fn save_to_writer_with_custom_extension<W>(
        &self,
        writer: W,
        extension: Option<&extension>,
    ) -> EncodeResult
    where
        W: Write,
    {
        S::Output::save_to_writer_with_custom_extension(&self.into(), writer, extension)
    }
}
