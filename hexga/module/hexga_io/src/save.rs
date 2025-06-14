use crate::*;

pub type IoSaveResult<T=()> = Result<T, IoError>;
pub use std::io::Write as IoWrite;

pub trait IoSaveFrom
{
    type From : IoSave;

    fn save_from_based_on(&self) -> Option<Self::From> { None }
    fn save_from_based_on_ref(&self) -> Option<&Self::From> { None }
}

impl<T> IoSave for T where T: IoSaveFrom + Serialize
{
    fn save_own_extensions() -> impl Iterator<Item = &'static str> { T::From::save_own_extensions() }

    fn save_with_reader_and_extension<W, Fs>(&self, path : &path, extension : &extension, w : W, fs : &mut Fs) -> IoSaveResult
            where W : Write, Fs : IoFsWrite
    {
        if let Some(v) = self.save_from_based_on_ref()
        {
            return v.save_with_reader_and_extension(path, extension, w, fs);
        }
        if let Some(v) = self.save_from_based_on()
        {
            return v.save_with_reader_and_extension(path, extension, w, fs);
        }
        Err(IoErrorKind::FromBasedOnFailed { dest: std::any::type_name::<Self>().to_owned(), src: std::any::type_name::<T>().to_owned(), reason: "bad implementation".to_string() }).to_save_error(path)
    }
}

#[allow(unused_variables)]
pub trait IoSave : Serialize
{
    // Main function to override :

    /// Dedicated file extension to save the value. ex `png`, `jpeg` for image
    ///
    /// Don't include the markup language extension like `json` or `ron`
    fn save_own_extensions() -> impl Iterator<Item = &'static str> { std::iter::empty() }



    // The path is usefull the save composite file (ex: saving a gif but every frame is in a subfolder relative to the path)
    fn save_to_with_own_extension<W, Fs>(&self, path : &path, extension : &extension, w : W, fs : &mut Fs) -> IoSaveResult
        where W : IoWrite, Fs : IoFsWrite
    { self.save_to_with_own_extension_pathless(extension, w, fs).to_save_error(path) }

    fn save_to_with_own_extension_pathless<W, Fs>(&self, extension : &extension, w : W, fs : &mut Fs) -> IoResult
        where W : IoWrite, Fs : IoFsWrite
    {
        Err(IoErrorKind::Unimplemented)
    }




    // impl details :

    /// When saving, if the extension is missing, the file will use this extension by default
    fn save_default_extension() -> Option<&'static str> { Self::save_own_extensions().next() }


    /// Don't include the markup language extension like `json` or `ron`
    fn can_save_own_extension(extension: &extension) -> bool { Self::save_own_extensions().any(|ext| ext == extension) }


    /// Also include the markup language extension like `json` or `ron`
    fn save_extensions() -> impl Iterator<Item = &'static str> { Self::save_own_extensions().chain(Io::MARKUP_EXTENSIONS.iter().copied()) }

    /// Also include the markup language extension like `json` or `ron`
    fn can_save_extension(extension: &extension) -> bool { Self::save_extensions().any(|ext| ext == extension) }

    fn save_to<Fs>(&self, path : &path, fs : &mut Fs) -> IoSaveResult where Fs : IoFsWrite
    {
        self.save_to_with_extension(path, path.extension_or_empty(), fs)
    }

    fn save_to_with_extension<Fs>(&self, path : &path, extension: &extension, fs : &mut Fs) -> IoSaveResult where Fs : IoFsWrite
    {
        fs.save_with_extension(path, extension, self).to_save_error(path)
    }

    fn save_with_reader<W, Fs>(&self, path : &path, w : W, fs : &mut Fs) -> IoSaveResult
        where W : IoWrite, Fs : IoFsWrite
    {
        self.save_with_reader_and_extension(path, path.extension_or_empty(), w, fs)
    }

    #[allow(unused_mut)] // The writter must be mutable `mut w : W`
    fn save_with_reader_and_extension<W, Fs>(&self, path : &path, extension : &extension, mut w : W, fs : &mut Fs) -> IoSaveResult
        where W : IoWrite, Fs : IoFsWrite
    {
        if extension.is_empty()
        {
            return Err(IoErrorKind::missing_save_extension::<Self>().to_save_error(path));
        }

        match extension
        {
            #[cfg(feature = "serde_ron")]
            Io::RON_EXTENSION => return write!(w, "{}", self.to_ron().to_save_error(path)?).to_save_error(path),

            #[cfg(feature = "serde_json")]
            Io::JSON_EXTENSION => return write!(w, "{}", self.to_json().to_save_error(path)?).to_save_error(path),

            #[cfg(feature = "serde_xml")]
            Io::XML_EXTENSION => return write!(w, "{}", self.to_xml().to_save_error(path)?).to_save_error(path),

            #[cfg(feature = "serde_quick_bin")]
            Io::QUICK_BIN_EXTENSION =>
            {
                let buf = self.to_quick_bin().to_save_error(path)?;
                w.write(&buf).to_save_error(path)?;
                return Ok(())
            },

            _ => {}
        }

        if !Self::can_save_own_extension(extension)
        {
            return Err(IoErrorKind::unsupported_save_extension::<Self>(extension).to_save_error(path));
        }

        match self.save_to_with_own_extension(path, extension, w, fs)
        {
            Ok(o) => Ok(o),
            Err(e) => if e.kind.is_unimplemented_or_unknow() { Err(IoErrorKind::unsupported_save_extension::<Self>(extension).to_save_error(path)) } else { Err(e) },
        }
    }
}
