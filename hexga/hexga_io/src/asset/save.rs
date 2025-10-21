use super::*;


#[allow(unused_variables)]
pub trait Save: Serialize
{
    /// Dedicated file extension to save the value. ex `png`, `jpeg` for image
    ///
    /// Don't include the markup language extension like `json` or `ron`
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> { std::iter::empty() }

    fn save_to_with_custom_extension(&self, path: &path, extension: &extension, fs: &mut Fs) -> IoResult;

    /// When saving, if the extension is missing
    fn save_default_extension() -> Option<&'static str> { Self::save_custom_extensions().next() }
}


pub trait SaveExtension
{
    /// Also include the markup language extension like `json` or `ron`
    fn save_extensions() -> impl Iterator<Item = &'static extension>;

    fn save_to(&self, path: &path, fs: &mut Fs) -> IoResult;
}
impl<S: ?Sized> SaveExtension for S where S: Save
{
    fn save_extensions() -> impl Iterator<Item = &'static extension> { Self::save_custom_extensions().chain(Extensions::MARKUP.iter().copied()) }

    fn save_to(&self, path: &path, fs: &mut Fs) -> IoResult
    {
        let original_extension = path.extension_or_empty();
        let mut extension = original_extension;

        if !Self::save_extensions().any(|ext|  ext == extension)
        {
            extension = Self::save_default_extension().unwrap_or("");

            if !Self::save_extensions().any(|ext|  ext == extension)
            {
                return Err(IoError::unsupported_save_extension::<Self>(original_extension));
            }
        }

        match extension
        {
            #[cfg(feature = "serde_ron")]
            Extensions::RON => return fs.write_str(path, &self.to_ron()?),

            #[cfg(feature = "serde_json")]
            Extensions::JSON => return fs.write_str(path, &self.to_json()?),

            #[cfg(feature = "serde_quick_bin")]
            Extensions::QUICK_BIN => return fs.write_bytes(path, &self.to_quick_bin()?),

            _ => {}
        }

        self.save_to_with_custom_extension(path, extension, fs)
    }
}


pub trait SaveInto
{
    type Base: Save;
    fn save_base(&self) -> Option<Self::Base> { None }
    fn save_base_ref(&self) -> Option<&Self::Base> { None }
}
impl<T> Save for T where T: SaveInto + Serialize
{
    fn save_custom_extensions() -> impl Iterator<Item = &'static extension> {
        T::Base::save_custom_extensions()
    }
    fn save_default_extension() -> Option<&'static str> {
        T::Base::save_default_extension()
    }
    fn save_to_with_custom_extension(&self, path: &path, extension: &extension, fs: &mut Fs) -> IoResult {
        if let Some(v) = self.save_base_ref()
        {
            return v.save_to_with_custom_extension(path, extension, fs);
        }
        if let Some(v) = self.save_base()
        {
            return v.save_to_with_custom_extension(path, extension, fs);
        }
        Err(IoError::save_missing_base::<T, T::Base>())
    }
}