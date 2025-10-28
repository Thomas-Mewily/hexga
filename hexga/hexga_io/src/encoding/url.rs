use std::{f32::consts::E, ops::{Deref, DerefMut}};

use super::*;

pub mod prelude
{
    pub use super::{FromUrl,ToUrl};
}


/// Represents a the metadata portion of an url (Data URL, RFC 2397),
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
/// use hexga_io::encoding::*;
///
/// let url = UrlMeta::try_from("data:image/png;base64,").unwrap();
/// assert_eq!(url.scheme, "data");
/// assert_eq!(url.media_type, "image");
/// assert_eq!(url.extension, "png");
/// assert_eq!(url.encoding, Some("base64"));
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct UrlMeta<'a>
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
impl<'a> TryFrom<&'a str> for UrlMeta<'a>
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

        Ok(UrlMeta {
            scheme,
            media_type,
            extension,
            encoding,
        })
    }
}

/// Represents a parsed data url (Data URL, RFC 2397).
///
///
/// Example Data URL: (single red pixel)
/// ```text
/// data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAAEElEQVR4AQEFAPr/AP8AAP8FAAH/+lyI0QAAAABJRU5ErkJggg==
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Url<'a>
{
    pub meta: UrlMeta<'a>,
    pub data: &'a str,
}
impl<'a> Deref for Url<'a>{ type Target=UrlMeta<'a>; fn deref(&self) -> &Self::Target { &self.meta } }
impl<'a> TryFrom<&'a str> for Url<'a>
{
    type Error = EncodeError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error>
    {
        let (meta_str, data) = value
            .split_once(',')
            .ok_or_else(|| EncodeError::custom("Missing ',' separator in URL"))?;

        let meta = UrlMeta::try_from(meta_str)?;
        if meta.scheme != "data" { return Err(EncodeError::custom("Invalid URL scheme: expected 'data'")); }

        Ok(Url {
            meta,
            data,
        })
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
pub struct BinUrl<'a>
{
    pub meta: UrlMeta<'a>,
    pub data: &'a [u8],
}
impl<'a> Deref for BinUrl<'a>{ type Target=UrlMeta<'a>; fn deref(&self) -> &Self::Target { &self.meta } }
impl<'a> TryFrom<&'a [u8]> for BinUrl<'a>
{
    type Error = EncodeError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let comma_pos = value
            .iter()
            .position(|&b| b == b',')
            .ok_or_else(|| EncodeError::custom("Missing ',' separator in URL"))?;

        let meta_bytes = &value[..comma_pos];
        let data = &value[comma_pos + 1..]; // raw payload

        let meta_str = std::str::from_utf8(meta_bytes)
            .map_err(|_| EncodeError::custom("Invalid UTF-8 in metadata"))?;

        let meta = UrlMeta::try_from(meta_str)?;
        if meta.scheme != "bin_data" { return Err(EncodeError::custom("Invalid URL scheme: expected 'bin_data'")); }

        Ok(BinUrl { meta, data })
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
impl Encode for String
{
    fn encode_extensions() -> impl Iterator<Item = &'static extension> {
        ["txt", "md", "cvs"].into_iter()
    }

    fn encode_in<W>(&self, writer : &mut W, extension: &extension) -> EncodeResult where W: Write {
        self.as_str().encode_in(writer, extension)
    }
}
impl Decode for String
{
    fn decode_extensions() -> impl Iterator<Item = &'static extension> {
        Self::encode_extensions()
    }

    fn decode(bytes: &[u8], _extension: &extension) -> EncodeResult<Self> where Self: Sized {

        match std::str::from_utf8(bytes)
        {
            Ok(s) => Ok(s.to_owned()),
            Err(e) => Err(e.into()),
        }
    }
}
impl<'a> MediaType for &'a str
{
    fn media_type() -> &'static str { String::media_type() }
}
impl<'a> Encode for &'a str
{
    fn encode_extensions() -> impl Iterator<Item = &'static extension> {
        String::encode_extensions()
    }

    fn encode_in<W>(&self, writer : &mut W, _extension: &extension) -> EncodeResult where W: Write {
        match writer.write(self.as_bytes())
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}



pub trait ToUrl: MediaType + Encode
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
        let bytes = self.encode(extension)?;
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
        self.encode_in(&mut data, extension)?;
        Ok(data)
    }
}
impl<T> ToUrl for T where T: MediaType + Encode{}

pub trait FromUrl: Decode
{
    fn from_url(url: &str) -> EncodeResult<Self> where Self: Sized
    {
        let url = Url::try_from(url)?;
        let bytes = Vec::<u8>::from_base64(url.data)?;
        Self::decode(&bytes, url.extension)
    }
    fn from_url_bin(url: &[u8]) -> EncodeResult<Self> where Self: Sized
    {
        let url = BinUrl::try_from(url)?;
        let bytes = Vec::<u8>::from_base64(url.data)?;
        Self::decode(&bytes, url.extension)
    }
}
impl<T> FromUrl for T where T: Decode {}