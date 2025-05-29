use crate::*;

pub type IoSaveResult<T=()> = Result<T, IoError>;
pub use std::io::Write as IoWrite;


#[allow(unused_variables)]
pub trait IoSave : Serialize
{
    // Main function to override :

    type BasedOn : ?Sized + IoSave;

    fn save_from_based_on(&self) -> Option<Self::BasedOn> { None }
    fn save_from_based_on_ref(&self) -> Option<&Self::BasedOn> { None }

    /// Dedicated file extension to save the value. ex `png`, `jpeg` for image
    ///
    /// Don't include the markup language extension like `json` or `ron`
    fn save_own_extensions() -> impl Iterator<Item = &'static str>
    { Self::BasedOn::save_own_extensions() }


    // The path is usefull the save composite file (ex: saving a gif but every frame is in a subfolder relative to the path)
    fn save_to_with_own_extension<W, Fs>(&self, path : &path, extension : &extension, w : W, fs : &mut Fs) -> IoSaveResult
        where W : IoWrite, Fs : IoFs
    { self.save_to_with_own_extension_pathless(extension, w, fs).to_save_error(path) }

    fn save_to_with_own_extension_pathless<W, Fs>(&self, extension : &extension, w : W, fs : &mut Fs) -> IoResult
        where W : IoWrite, Fs : IoFs
    {
        Err(IoErrorKind::Unimplemented)
    }




    // impl details :


    /// Don't include the markup language extension like `json` or `ron`
    fn can_save_own_extension(extension: &str) -> bool { Self::save_own_extensions().any(|ext| ext == extension) }


    /// Also include the markup language extension like `json` or `ron`
    fn save_extensions() -> impl Iterator<Item = &'static str> { Self::save_own_extensions().chain(Io::MARKUP_EXTENSIONS.iter().copied()) }

    /// Also include the markup language extension like `json` or `ron`
    fn can_save_extension(extension: &str) -> bool { Self::save_extensions().any(|ext| ext == extension) }


    fn save_to<W, Fs>(&self, path : &path, w : W, fs : &mut Fs) -> IoSaveResult
        where W : IoWrite, Fs : IoFs
    {
        self.save_to_with_extension(path, path.extension().unwrap_or_default(), w, fs)
    }

    fn save_to_with_extension<W, Fs>(&self, path : &path, extension : &extension, mut w : W, fs : &mut Fs) -> IoSaveResult
        where W : IoWrite, Fs : IoFs
    {
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

        if let Some(v) = self.save_from_based_on_ref()
        {
            return v.save_to_with_extension(path, extension, w, fs);
        }
        if let Some(v) = self.save_from_based_on()
        {
            return v.save_to_with_extension(path, extension, w, fs);
        }

        self.save_to_with_own_extension(path, extension, w, fs)
    }
}

impl IoSave for IoNotBasedOn
{
    type BasedOn = IoNotBasedOn;

    fn save_extensions() -> impl Iterator<Item = &'static str> { std::iter::empty() }
    fn save_own_extensions() -> impl Iterator<Item = &'static str> { std::iter::empty() }

    fn can_save_extension(_: &str) -> bool { false }
    fn can_save_own_extension(_: &str) -> bool { false }

    fn save_from_based_on(&self) -> Option<Self::BasedOn> { None }
    fn save_from_based_on_ref(&self) -> Option<&Self::BasedOn> { None }

    fn save_to<W, Fs>(&self, path : &path, _ : W, _ : &mut Fs) -> IoSaveResult
            where W : Write, Fs : IoFs
    { Err(IoError::save(path, IoErrorKind::FromNotBaseOn)) }

    fn save_to_with_extension<W, Fs>(&self, path : &path, _ : &extension, _ : W, _ : &mut Fs) -> IoSaveResult
            where W : Write, Fs : IoFs
    { Err(IoError::save(path, IoErrorKind::FromNotBaseOn)) }

    fn save_to_with_own_extension<W, Fs>(&self, path : &path, _ : &extension, _ : W, _ : &mut Fs) -> IoSaveResult
            where W : Write, Fs : IoFs
        { Err(IoError::save(path, IoErrorKind::FromNotBaseOn)) }

    fn save_to_with_own_extension_pathless<W, Fs>(&self, _ : &extension, _ : W, _ : &mut Fs) -> IoResult
            where W : Write, Fs : IoFs {
        Err(IoErrorKind::FromNotBaseOn)
    }
}