use super::*;


pub(crate) mod prelude
{
    pub use super::{SaveCustomExtension, Save, SaveAs};
}

pub trait SaveCustomExtension : CfgSerialize
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> { std::iter::empty() }
    fn save_to_writer_with_custom_extension<W>(&self, writer: W, extension: &extension) -> EncodeResult where W: Write
    {
        let _ = (writer, extension);
        Err(EncodeError::Unimplemented)
    }
}

pub trait SaveCustomExtensionBytes : SaveCustomExtension
{
    fn save_to_bytes_with_custom_extension(&self, extension: &extension) -> EncodeResult<Vec<u8>>
    {
        self.save_to_bytes_with_custom_extension_in(Vec::with_capacity(DEFAULT_WRITER_CAPACITY), extension)
    }
    fn save_to_bytes_with_custom_extension_in(&self, mut bytes: Vec<u8>, extension: &extension) -> EncodeResult<Vec<u8>>
    {
        self.save_to_writer_with_custom_extension(&mut bytes, extension)?;
        Ok(bytes)
    }
}
impl<T> SaveCustomExtensionBytes for T where T: SaveCustomExtension {}


const DEFAULT_WRITER_CAPACITY : usize = 1024;

pub trait Save : SaveCustomExtension
{
    fn save_extensions() -> impl Iterator<Item = &'static extension>
    {
        #[cfg(feature = "serde")]
        return Self::save_custom_extensions().chain(AnyFormat::ALL.into_iter().map(|v| v.extension()));

        #[cfg(not(feature = "serde"))]
        return Self::save_custom_extensions();
    }
    fn save_prefered_extension() -> Option<&'static extension>
    {
        Self::save_custom_extensions().next()
    }

    fn save_to_bytes<'ext>(&self, extension: &'ext extension) -> EncodeResult<(Vec<u8>, DeducedExtension<'ext>)>
    {
        self.save_to_bytes_in(Vec::with_capacity(DEFAULT_WRITER_CAPACITY), extension)
    }
    fn save_to_bytes_in<'ext>(&self, mut bytes: Vec<u8>, extension: &'ext extension) -> EncodeResult<(Vec<u8>, DeducedExtension<'ext>)>
    {
        let r = self.save_to_writer(&mut bytes, extension)?;
        Ok((bytes, r))
    }
    fn save_to_writer<'ext, W>(&self, writer: &mut W, extension: &'ext extension) -> EncodeResult<DeducedExtension<'ext>> where W: Write
    {
        if Self::save_custom_extensions().any(|e| e == extension)
        {
            return self.save_to_writer_with_custom_extension(writer, extension).map(|_| extension.into());
        }
        if extension.is_empty() && let Some(ext) = Self::save_prefered_extension()
        {
            return self.save_to_writer_with_custom_extension(writer, ext).map(|_| ext.into());
        }

        #[cfg(feature = "serde")]
        {
            let format = AnyFormat::try_from(extension).unwrap_or_default();
            format.encode_with_writer(&self, writer)?;
            return Ok(format.extension().into());
        }

        #[allow(unreachable_code)]
        Err(EncodeError::save_unsupported_extension::<Self>(extension))
    }
}
impl<T> Save for T where T:SaveCustomExtension + ?Sized {}


pub trait SaveAs
{
    type Output: SaveCustomExtension + for<'a> From<&'a Self>;
}
impl<S> SaveCustomExtension for S where S: SaveAs + CfgSerialize
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        S::Output::save_custom_extensions()
    }
    fn save_to_writer_with_custom_extension<W>(&self, writer: W, extension: &extension) -> EncodeResult where W: Write {
        S::Output::save_to_writer_with_custom_extension(&self.into(), writer, extension)
    }
}