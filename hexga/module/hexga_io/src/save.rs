use crate::*;

pub type IoSaveResult<T=()> = Result<T, IoError>;
pub use std::io::Write as IoWrite;

/*
#[allow(unused_variables)]
pub trait IoSave : Sized + Serialize + for<'de> Deserialize<'de>
{
    type BasedOn : IoSave;
    fn save_from_based_on(base : Self::BasedOn) -> IoResult<Self> { Err(IoErrorKind::FromBasedOnFailed { dest: std::any::type_name::<Self>().to_owned(), src: std::any::type_name::<Self::BasedOn>().to_owned(), reason: "bad implementation".to_string() }) }

    /// Dedicated file extension to save the value. ex `png`, `jpeg` for image
    ///
    /// Don't include the markup language extension like `json` or `ron`
    fn save_own_extensions() -> impl Iterator<Item = &'static str> { Self::BasedOn::save_own_extensions() }

    /// Don't include the markup language extension like `json` or `ron`
    fn can_open_own_extension(extension: &str) -> bool { Self::save_own_extensions().any(|ext| ext == extension) }


    /// Also include the markup language extension like `json` or `ron`
    fn save_extensions() -> impl Iterator<Item = &'static str> { Self::save_own_extensions().chain(Io::MARKUP_EXTENSIONS.iter().copied()) }

    /// Also include the markup language extension like `json` or `ron`
    fn can_open_extension(extension: &str) -> bool { Self::save_extensions().any(|ext| ext == extension) }


    fn save_to_bytes<W>(w : &mut, path : &path) -> IoSaveResult<Self>
    {
        let ext = path.extension().unwrap_or_default();
        Self::save_to_bytes_with_extension(data, path, ext)
    }
    fn save_to_bytes_with_extension(data : &[u8], path : &path, extension : &extension) -> IoSaveResult<Self>
    {
        match extension
        {
            #[cfg(feature = "serde_ron")]
            Io::RON_EXTENSION => Self::from_ron_buf(data).to_read_error(path),

            #[cfg(feature = "serde_json")]
            Io::JSON_EXTENSION => Self::from_json_buf(data).to_read_error(path),

            #[cfg(feature = "serde_xml")]
            Io::XML_EXTENSION => Self::from_xml_buf(data).to_read_error(path),

            #[cfg(feature = "serde_quick_bin")]
            Io::QUICK_BIN_EXTENSION => Self::from_quick_bin_buf(data).to_read_error(path),

            _ => match Self::BasedOn::save_to_bytes_with_extension(data, path, extension)
            {
                Ok(base) => Self::save_from_based_on(base).to_read_error(path),
                Err(_) => match Self::can_open_own_extension(extension)
                {
                    false => Err(IoErrorKind::unsupported_open_extension::<Self>(extension).to_read_error(path)),
                    true => match Self::save_to_bytes_with_own_extension(data, path, extension)
                    {
                        Ok(o) => Ok(o),
                        Err(e) =>
                        {
                            if Self::CAN_BE_SAVEED_FROM_TEXT
                            {
                                match str::from_utf8(data)
                                {
                                    Ok(txt) => Self::save_from_str_with_own_extension(txt, path, extension),
                                    Err(txt_err) => if e.kind.is_unimplemented_or_unknow()
                                    {
                                        Err(IoErrorKind::EncodingBadUtf8 { valid_up_to: txt_err.valid_up_to(), error_len: txt_err.error_len() }.to_read_error(path))
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
                },
            }
        }
    }

    fn save_to_bytes_with_own_extension(data : &[u8], path : &path, extension : &extension) -> IoSaveResult<Self> { Self::save_to_bytes_with_own_extension_pathless(data, extension).to_read_error(path) }
    fn save_to_bytes_with_own_extension_pathless(data : &[u8], extension : &extension) -> IoResult<Self> { Err(IoErrorKind::Unimplemented) }

    const CAN_BE_SAVEED_FROM_TEXT : bool = false;
    fn save_from_str_with_own_extension(data : &str, path : &path, extension : &extension) -> IoSaveResult<Self> { Self::save_from_str_with_own_extension_pathless(data, extension).to_read_error(path) }
    fn save_from_str_with_own_extension_pathless(data : &str, extension : &extension) -> IoResult<Self> { Err(IoErrorKind::Unimplemented) }
}

impl IoSave for IoNotBasedOn
{
    type BasedOn = IoNotBasedOn;

    fn save_extensions() -> impl Iterator<Item = &'static str> { std::iter::empty() }
    fn save_own_extensions() -> impl Iterator<Item = &'static str> { std::iter::empty() }

    fn save_from_based_on(_ : Self::BasedOn) -> IoResult<Self> { Err(IoErrorKind::FromNotBaseOn) }

    fn save_to_bytes(_ : &[u8], _ : &path) -> IoSaveResult<Self> { Err(IoError::read("", IoErrorKind::FromNotBaseOn)) }
    fn save_to_bytes_with_extension(_ : &[u8], _ : &path, _ : &extension) -> IoSaveResult<Self> { Err(IoError::read("", IoErrorKind::FromNotBaseOn)) }
    fn save_to_bytes_with_own_extension(_ : &[u8], _ : &path, _ : &extension) -> IoSaveResult<Self> { Err(IoError::read("", IoErrorKind::FromNotBaseOn)) }
    fn save_to_bytes_with_own_extension_pathless(_ : &[u8], _ : &extension) -> IoResult<Self> { Err(IoErrorKind::FromNotBaseOn) }

    fn save_from_str_with_own_extension(_ : &str, _ : &path, _ : &extension) -> IoSaveResult<Self> { Err(IoError::read("", IoErrorKind::FromNotBaseOn)) }
    fn save_from_str_with_own_extension_pathless(_ : &str, _ : &extension) -> IoResult<Self> { Err(IoErrorKind::FromNotBaseOn) }
}
    */