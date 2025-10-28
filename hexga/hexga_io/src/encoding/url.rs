use super::*;

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
        let bytes = self.encode(extension)?;
        let media = Self::media_type();
        let mut data = Vec::with_capacity(1024);
        write!(&mut data, "bin_data:{media}/{extension};base64,").map_err(|e| EncodeError::from(e))?;
        self.encode_in(&mut data, extension)?;
        Ok(bytes)
    }
}
impl<T> ToUrl for T where T: MediaType + Encode{}

pub trait FromUrl: Decode
{
    fn from_url(url: &str) -> EncodeResult<Self> where Self: Sized
    {
        let url = url.trim();
        let prefix = "data:";
        if !url.starts_with(prefix) {
            return Err(EncodeError::custom("Data URL must start with 'data:'"));
        }

        let rest = &url[prefix.len()..];
        let (meta, b64data) = rest
            .split_once(',')
            .ok_or_else(|| EncodeError::custom("Missing ',' separator in Data URL"))?;

        let (media_type_and_extension, base64_marker) = meta.split_once(';').unwrap_or((meta, ""));
        if base64_marker != "base64" {
            return Err(EncodeError::custom("Only base64-encoded Data URLs are supported"));
        }

        let bytes = Vec::<u8>::from_base64(b64data)?;

        let (_media_type, extension) = media_type_and_extension
            .split_once('/')
            .ok_or_else(|| EncodeError::custom("Invalid media type in Data URL"))?;

        Self::decode(&bytes, extension)
    }
    fn from_url_bin(url: &[u8]) -> EncodeResult<Self> where Self: Sized
    {
        let prefix = b"bin_data:";
        if !url.starts_with(prefix) {
            return Err(EncodeError::custom(
                "Binary Data URL must start with 'bin_data:'",
            ));
        }

        let comma_pos = url
            .iter()
            .position(|&b| b == b',')
            .ok_or_else(|| EncodeError::custom("Missing ',' separator in binary Data URL"))?;

        let media_type_and_extension = std::str::from_utf8(&url[prefix.len()..comma_pos])
            .map_err(|_| EncodeError::custom("Invalid UTF-8 in binary URL header"))?;

        let (_media_type, extension) = media_type_and_extension
            .split_once('/')
            .ok_or_else(|| EncodeError::custom("Invalid media type in binary URL header"))?;

        let data = &url[comma_pos + 1..];

        Self::decode(data, extension)
    }
}
impl<T> FromUrl for T where T: Decode {}