use crate::*;

pub type IoLoadResult<T=()> = Result<T, IoError>;
pub use std::io::Read as IoRead;

// Can't use a reader because we are not sur about the formatting, and a reader can't go back

pub trait IoLoadFrom : Sized
{
    type From : IoLoad;
    fn load_from_based_on(base : Self::From) -> IoResult<Self>;
}
impl<T> IoLoad for T where T : IoLoadFrom + for<'de> Deserialize<'de>
{
    fn load_own_extensions() -> impl Iterator<Item = &'static str> { T::From::load_own_extensions() }
    const CAN_BE_LOADED_FROM_TEXT : bool = T::CAN_BE_LOADED_FROM_TEXT;

    fn load_from_bytes_with_extension(data : &[u8], path : &path, extension : &extension) -> IoLoadResult<Self>
    {
        if let Ok(base) = T::From::load_from_bytes_with_extension(data, path, extension)
        {
            Self::load_from_based_on(base)
        }else
        {
            Err(IoErrorKind::FromBasedOnFailed { dest: std::any::type_name::<Self>().to_owned(), src: std::any::type_name::<T>().to_owned(), reason: "bad implementation".to_string() })
        }.to_load_error(path)
    }
}

#[allow(unused_variables)]
pub trait IoLoad : Sized + for<'de> Deserialize<'de>
{
    // Main function to override :


    /// Dedicated file extension to load the value. ex `png`, `jpeg` for image
    ///
    /// Don't include the markup language extension like `json` or `ron`
    fn load_own_extensions() -> impl Iterator<Item = &'static str> { std::iter::empty() }

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