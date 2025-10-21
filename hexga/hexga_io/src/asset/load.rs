use super::*;

#[allow(unused_variables)]
pub trait Load: for<'de> Deserialize<'de>
{
    /// Dedicated file extension to load the value. ex `png`, `jpeg` for image
    ///
    /// Don't include the markup language extension like `json` or `ron`
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> { std::iter::empty() }

    fn load_from_with_custom_extension<Fs>(path: &path, extension: &extension, fs: &mut Fs) -> IoResult<Self> where Fs: FsRead { Err(IoError::Unimplemented) }

    /// When saving, if the extension is missing
    fn load_default_extension() -> Option<&'static str> { Self::load_custom_extensions().next() }
}

pub trait LoadExtension: Load
{
    /// Also include the markup language extension like `json` or `ron`
    fn load_extensions() -> impl Iterator<Item = &'static extension> { Self::load_custom_extensions().chain(Extensions::MARKUP.iter().copied()) }

    fn load_from_bytes<'a>(bytes: &'a [u8], extension: &extension) -> IoResult<Self>
    {
        let mut file_fs = FsFile::lambda_with_data(Cow::Borrowed(bytes));
        let mut fs = Fs::new(&mut file_fs);
        Self::load_from_with_custom_extension("", extension, &mut fs)
    }

    /// Load the value, and if it fail, init it and save it then return it. Saving the value may silenly fail, and no error are returned
    fn load_from_or_create<'a, F, Fs>(path: &path, init: F, fs: &mut Fs) -> Self where F:FnOnce() -> Self, Self: Save, Fs: FsWrite
    {
        match Self::load_from(path, fs)
        {
            Ok(v) => return v,
            Err(_) =>
            {
                let val = init();
                let _ = val.save_to(path, fs);
                val
            },
        }
    }

    fn load_from<Fs>(path: &path, fs: &mut Fs) -> IoResult<Self> where Fs: FsRead
    {
        let original_extension = path.extension_or_empty();
        let mut extension = original_extension;

        if !Self::load_extensions().any(|ext|  ext == extension)
        {
            extension = Self::load_default_extension().unwrap_or("");

            if !Self::load_extensions().any(|ext|  ext == extension)
            {
                return Err(IoError::unsupported_load_extension::<Self>(original_extension));
            }
        }

        match extension
        {
            #[cfg(feature = "serde_ron")]
            Extensions::RON => return Self::from_ron(&fs.read_str(path)?),

            #[cfg(feature = "serde_json")]
            Extensions::JSON => return Self::from_json(&fs.read_str(path)?),

            #[cfg(feature = "serde_quick_bin")]
            Extensions::QUICK_BIN => return Self::from_quick_bin_buf(&fs.read_bytes(path)?),

            _ => {}
        }

        Self::load_from_with_custom_extension(path, extension, fs)
    }
}
impl<S: ?Sized> LoadExtension for S where S: Load {}



pub trait LoadFrom: Sized
{
    type Base: Load;
    fn load_from_base(base: Self::Base) -> IoResult<Self>;
}
impl<T> Load for T where T: LoadFrom + for<'de> Deserialize<'de>
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> {
        T::Base::load_custom_extensions()
    }
    fn load_default_extension() -> Option<&'static str> {
        T::Base::load_default_extension()
    }
    fn load_from_with_custom_extension<Fs>(path: &path, extension: &extension, fs: &mut Fs) -> IoResult<Self> where Fs: FsRead
    {
        match T::Base::load_from_with_custom_extension(path, extension, fs)
        {
            Ok(base) => Self::load_from_base(base),
            Err(e) => Err(e),
        }
    }
}