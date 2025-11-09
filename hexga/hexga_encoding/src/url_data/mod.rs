use std::ops::Deref;

use super::*;

#[cfg(feature = "serde")]
mod serde_impl;
pub use serde_impl::*;

pub mod prelude
{
    pub use super::{FromUrl,ToUrl,MediaType};
    #[cfg(feature = "serde")]
    pub use super::serde_impl::{UrlSerializer,UrlDeserializer};
}


/// Represents a the metadata portion of [a data url](https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes/data) (Data URL, RFC 2397),
/// without including the payload/data.
///
/// Example Data URL:
/// ```text
/// data:image/png;base64,
/// ```
///
/// # Usage
///
/// ```rust
/// use hexga_encoding::*;
///
/// let url = UrlDataMeta::try_from("data:image/png;base64,").unwrap();
/// assert_eq!(url.scheme, "data");
/// assert_eq!(url.media_type, "image");
/// assert_eq!(url.extension, "png");
/// assert_eq!(url.encoding, Some("base64"));
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct UrlDataMeta<'a>
{
    /// The URL scheme keyword, e.g., "data"
    pub scheme: &'a str,

    /// The media type, e.g., "image"
    pub media_type: &'a str,

    /// The file extension/subtype, e.g., "png"
    pub extension: &'a str,

    /// Base64 marker if present, usually "base64"
    pub encoding: Option<&'a str>,
}
impl<'a> TryFrom<&'a str> for UrlDataMeta<'a>
{
    type Error=EncodeError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let value = value.trim();

        let (scheme, rest) = value
            .split_once(':')
            .ok_or_else(|| EncodeError::custom("URL must have a scheme"))?;

        let meta = rest.split(',').next().unwrap_or(rest);

        let (media_type_and_ext, encoding) = match meta.split_once(';') {
            Some((m, e)) => (m, Some(e)),
            None => (meta, None),
        };

        let (media_type, extension) = media_type_and_ext
            .split_once('/')
            .ok_or_else(|| EncodeError::custom("Invalid media type in URL"))?;

        Ok(UrlDataMeta {
            scheme,
            media_type,
            extension,
            encoding,
        })
    }
}

/// Represents a parsed [data url](https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes/data) (Data URL, RFC 2397).
///
///
/// Example Data URL: (single red pixel)
/// ```text
/// data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAAEElEQVR4AQEFAPr/AP8AAP8FAAH/+lyI0QAAAABJRU5ErkJggg==
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct UrlData<'a>
{
    pub meta: UrlDataMeta<'a>,
    pub data: &'a str,
}

const MIN_BYTE_SEPARATOR_SEARCH : usize = 256;

impl<'a> Deref for UrlData<'a>{ type Target=UrlDataMeta<'a>; fn deref(&self) -> &Self::Target { &self.meta } }
impl<'a> TryFrom<&'a str> for UrlData<'a>
{
    type Error = EncodeError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error>
    {
        // Limit the search to the first MIN_BYTE_SEPARATOR_SEARCH bytes
        let search_len = value.len().min(MIN_BYTE_SEPARATOR_SEARCH);
        let prefix = &value[..search_len];

        let comma_pos = prefix
            .find(',')
            .ok_or_else(|| EncodeError::custom(format!("Missing ',' separator in URL (first {MIN_BYTE_SEPARATOR_SEARCH} bytes)")))?;

        let (meta_str, data) = value.split_at(comma_pos);
        let data = &data[1..]; // skip the comma itself

        let meta = UrlDataMeta::try_from(meta_str)?;
        if meta.scheme != "data" {
            return Err(EncodeError::custom("Invalid URL scheme: expected 'data'"));
        }

        Ok(UrlData { meta, data })
    }
}


/// Represents a parsed bin_data url (similar to a Data URL, RFC 2397, but the data is in binary).
///
/// This struct stores references to the different components of a URL-like string
/// without allocating new memory. It can be used for both Base64-encoded data URLs
/// and custom binary URLs with a similar structure.
///
/// Example binary URL (custom format):
/// ```text
/// bin_data:image/png;base64,<raw bytes>
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct BinUrlData<'a>
{
    pub meta: UrlDataMeta<'a>,
    pub data: &'a [u8],
}
impl<'a> Deref for BinUrlData<'a>{ type Target=UrlDataMeta<'a>; fn deref(&self) -> &Self::Target { &self.meta } }
impl<'a> TryFrom<&'a [u8]> for BinUrlData<'a>
{
    type Error = EncodeError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        // Limit the search to the first MIN_BYTE_SEPARATOR_SEARCH bytes
        let search_len = value.len().min(MIN_BYTE_SEPARATOR_SEARCH);
        let prefix = &value[..search_len];

        let comma_pos = prefix
            .iter()
            .position(|&b| b == b',')
            .ok_or_else(|| EncodeError::custom(format!("Missing ',' separator in Bin URL (first {MIN_BYTE_SEPARATOR_SEARCH} bytes)")))?;

        let meta_bytes = &value[..comma_pos];
        let data = &value[comma_pos + 1..]; // raw payload

        let meta_str = std::str::from_utf8(meta_bytes)
            .map_err(|_| EncodeError::custom("Invalid UTF-8 in metadata"))?;

        let meta = UrlDataMeta::try_from(meta_str)?;
        if meta.scheme != "bin_data" {
            return Err(EncodeError::custom("Invalid URL scheme: expected 'bin_data'"));
        }

        Ok(BinUrlData { meta, data })
    }
}



pub trait MediaType
{
    /// [Media types (MIME types)](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/MIME_types)
    fn media_type() -> &'static str;
    fn mime_type(extension: &extension) -> String
    {
        let media = Self::media_type();
        format!("{media}/{extension}")
    }
}


impl MediaType for String
{
    fn media_type() -> &'static str { "text" }
}
impl<'a> MediaType for &'a str
{
    fn media_type() -> &'static str { String::media_type() }
}



pub trait ToUrl: MediaType + SaveCustomExtension
{
    /// Converts the encoded image into a Data URL (RFC 2397).
    ///
    /// # Parameters
    /// - `extension`: The file extension (e.g., `png`, `jpeg`).
    ///
    /// # Returns
    /// An `EncodeResult<String>` containing the Data URL, in the format:
    /// `data:<media_type>/<extension>;base64,<base64_encoded_data>`.
    ///
    /// # Errors
    /// Returns an error if the image cannot be encoded for the given extension.
    fn to_url(&self, extension: &extension) -> EncodeResult<String>
    {
        let (bytes, _deduced_extension) = self.save_to_bytes(extension)?;
        let media = Self::media_type();
        let url = bytes.to_base64_in(format!("data:{media}/{extension};base64,"));
        Ok(url)
    }

    /// Converts the encoded image into a binary url.
    ///
    /// Similar to [`Encode::to_url`], except the `<base64_encoded_data>` is in binary
    fn to_url_bin(&self, extension: &extension) -> EncodeResult<Vec<u8>>
    {
        let media = Self::media_type();
        let mut data = Vec::with_capacity(1024);
        write!(&mut data, "bin_data:{media}/{extension};base64,").map_err(|e| EncodeError::from(e))?;
        let (data, _deduced_extension) = self.save_to_bytes_in(data, extension)?;
        Ok(data)
    }
}
impl<T> ToUrl for T where T: MediaType + SaveCustomExtension{}

/// Trait for types that can be **loaded from URL-like data** or raw bytes.
///
/// This trait extends [`Load`] and provides methods to create an the value
/// from either a **Data URL (RFC 2397)**, a **binary URL**, or raw bytes.
pub trait FromUrl: LoadCustomExtension
{
    /// Loads an instance from a standard **Data URL (RFC 2397)** string.
    ///
    /// Example Data URL: (single red pixel)
    ///
    /// ```text
    /// data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAAEElEQVR4AQEFAPr/AP8AAP8FAAH/+lyI0QAAAABJRU5ErkJggg==
    /// ```
    fn from_url(url: &str) -> EncodeResult<Self> where Self: Sized
    {
        let url = UrlData::try_from(url)?;
        let bytes = Vec::<u8>::from_base64(url.data)?;
        Self::load_from_bytes_with_custom_extension(&bytes, url.extension)
    }

    /// Loads an instance from a **binary URL** (custom `bin_data:` scheme).
    ///
    /// # Example
    ///
    /// ```text
    /// bin_data:image/png;base64,<raw bytes>
    /// ```
    fn from_bin_url(url: &[u8]) -> EncodeResult<Self> where Self: Sized
    {
        let url = BinUrlData::try_from(url)?;
        Self::load_from_bytes_with_custom_extension(&url.data, url.extension)
    }

    /// Loads an instance from a **binary URL** (custom `bin_data:` scheme), falling back to raw bytes if parsing fails.
    ///
    /// This method attempts to parse the input as a binary URL first. If that fails,
    /// it treats the input as raw bytes and loads it using the provided `extension`.
    fn from_bin_url_or_bytes(bytes: &[u8], extension: &extension)  -> EncodeResult<Self> where Self: Sized
    {
        match Self::from_bin_url(bytes)
        {
            Ok(o) => Ok(o),
            Err(_) => Self::load_from_bytes_with_custom_extension(bytes, extension)
        }
    }
}
impl<T> FromUrl for T where T: LoadCustomExtension {}



