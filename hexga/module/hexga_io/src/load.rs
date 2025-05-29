use crate::*;

pub type IoLoadResult<T=()> = Result<T, IoError>;
pub use std::io::Read as IoRead;

// Can't use a reader because we are not sur about the formatting, and a reader can't go back

#[allow(unused_variables)]
pub trait IoLoad : Sized + Serialize + for<'de> Deserialize<'de>
{
    // Main function to override :

    type BasedOn : IoLoad;
    fn load_from_based_on(base : Self::BasedOn) -> IoResult<Self> { Err(IoErrorKind::FromBasedOnFailed { dest: std::any::type_name::<Self>().to_owned(), src: std::any::type_name::<Self::BasedOn>().to_owned(), reason: "bad implementation".to_string() }) }

    /// Dedicated file extension to load the value. ex `png`, `jpeg` for image
    ///
    /// Don't include the markup language extension like `json` or `ron`
    fn load_own_extensions() -> impl Iterator<Item = &'static str> { Self::BasedOn::load_own_extensions() }

    fn load_from_bytes_with_own_extension(data : &[u8], path : &path, extension : &extension) -> IoLoadResult<Self> { Self::load_from_bytes_with_own_extension_pathless(data, extension).to_load_error(path) }
    fn load_from_bytes_with_own_extension_pathless(data : &[u8], extension : &extension) -> IoResult<Self> { Err(IoErrorKind::Unimplemented) }

    const CAN_BE_LOADED_FROM_TEXT : bool = false;
    fn load_from_str_with_own_extension(data : &str, path : &path, extension : &extension) -> IoLoadResult<Self> { Self::load_from_str_with_own_extension_pathless(data, extension).to_load_error(path) }
    fn load_from_str_with_own_extension_pathless(data : &str, extension : &extension) -> IoResult<Self> { Err(IoErrorKind::Unimplemented) }





    // impl details :

    /// Don't include the markup language extension like `json` or `ron`
    fn can_open_own_extension(extension: &str) -> bool { Self::load_own_extensions().any(|ext| ext == extension) }


    /// Also include the markup language extension like `json` or `ron`
    fn load_extensions() -> impl Iterator<Item = &'static str> { Self::load_own_extensions().chain(Io::MARKUP_EXTENSIONS.iter().copied()) }

    /// Also include the markup language extension like `json` or `ron`
    fn can_open_extension(extension: &str) -> bool { Self::load_extensions().any(|ext| ext == extension) }

    /// Support bytes and str
    fn load_from_reader<R>(mut r : R, path : &path) -> IoLoadResult<Self> where R : IoRead
    {
        let mut data = Vec::with_capacity(1024);
        r.read_to_end(&mut data).to_load_error(path)?;
        Self::load_from_bytes(&data, path)
    }

    /// Support bytes and str
    fn load_from_bytes(data : &[u8], path : &path) -> IoLoadResult<Self>
    {
        let ext = path.extension().unwrap_or_default();
        Self::load_from_bytes_with_extension(data, path, ext)
    }
    /// Support bytes and str
    fn load_from_bytes_with_extension(data : &[u8], path : &path, extension : &extension) -> IoLoadResult<Self>
    {
        match extension
        {
            #[cfg(feature = "serde_ron")]
            Io::RON_EXTENSION => return Self::from_ron_buf(data).to_load_error(path),

            #[cfg(feature = "serde_json")]
            Io::JSON_EXTENSION => return Self::from_json_buf(data).to_load_error(path),

            #[cfg(feature = "serde_xml")]
            Io::XML_EXTENSION => return Self::from_xml_buf(data).to_load_error(path),

            #[cfg(feature = "serde_quick_bin")]
            Io::QUICK_BIN_EXTENSION => return Self::from_quick_bin_buf(data).to_load_error(path),

            _ => {},
        }

        if let Ok(base) = Self::BasedOn::load_from_bytes_with_extension(data, path, extension)
        {
            return Self::load_from_based_on(base).to_load_error(path);
        }

        if !Self::can_open_own_extension(extension)
        {
            return Err(IoErrorKind::unsupported_open_extension::<Self>(extension).to_load_error(path));
        }

        match Self::load_from_bytes_with_own_extension(data, path, extension)
        {
            Ok(o) => Ok(o),
            Err(e) =>
            {
                if Self::CAN_BE_LOADED_FROM_TEXT
                {
                    match str::from_utf8(data)
                    {
                        Ok(txt) => Self::load_from_str_with_own_extension(txt, path, extension),
                        Err(txt_err) => if e.kind.is_unimplemented_or_unknow()
                        {
                            Err(IoErrorKind::EncodingBadUtf8 { valid_up_to: txt_err.valid_up_to(), error_len: txt_err.error_len() }.to_load_error(path))
                        }else
                        {
                            Err(e)
                        }
                    }
                }else
                {
                    Err(e)
                }
            },
        }
    }
}

impl IoLoad for IoNotBasedOn
{
    type BasedOn = IoNotBasedOn;

    fn load_extensions() -> impl Iterator<Item = &'static str> { std::iter::empty() }
    fn load_own_extensions() -> impl Iterator<Item = &'static str> { std::iter::empty() }

    fn can_open_extension(_: &str) -> bool { false }
    fn can_open_own_extension(_: &str) -> bool { false }
    fn load_from_reader<R>(_ : R, _ : &path) -> IoLoadResult<Self> where R : Read { Err(IoError::load("", IoErrorKind::FromNotBaseOn)) }

    fn load_from_based_on(_ : Self::BasedOn) -> IoResult<Self> { Err(IoErrorKind::FromNotBaseOn) }

    fn load_from_bytes(_ : &[u8], _ : &path) -> IoLoadResult<Self> { Err(IoError::load("", IoErrorKind::FromNotBaseOn)) }
    fn load_from_bytes_with_extension(_ : &[u8], _ : &path, _ : &extension) -> IoLoadResult<Self> { Err(IoError::load("", IoErrorKind::FromNotBaseOn)) }
    fn load_from_bytes_with_own_extension(_ : &[u8], _ : &path, _ : &extension) -> IoLoadResult<Self> { Err(IoError::load("", IoErrorKind::FromNotBaseOn)) }
    fn load_from_bytes_with_own_extension_pathless(_ : &[u8], _ : &extension) -> IoResult<Self> { Err(IoErrorKind::FromNotBaseOn) }

    fn load_from_str_with_own_extension(_ : &str, _ : &path, _ : &extension) -> IoLoadResult<Self> { Err(IoError::load("", IoErrorKind::FromNotBaseOn)) }
    fn load_from_str_with_own_extension_pathless(_ : &str, _ : &extension) -> IoResult<Self> { Err(IoErrorKind::FromNotBaseOn) }
}