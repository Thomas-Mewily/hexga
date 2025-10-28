use super::*;


pub trait Encode: MediaType
{
    fn encode_extensions() -> impl Iterator<Item = &'static extension>;
    fn encode(&self,extension: &extension) -> EncodeResult<Vec<u8>>
    {
        let mut bytes = Vec::with_capacity(1024);
        self.encode_in(&mut bytes, extension)?;
        Ok(bytes)
    }
    fn encode_in<W>(&self, writer : &mut W, extension: &extension) -> EncodeResult where W: Write;
    fn encode_prefered_extension() -> Option<&'static extension> { Self::encode_extensions().next() }
}


#[cfg(feature = "serde")]
pub trait SerializerWithEncoding : Serializer
{
    fn serialize_with_encoding<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where T: Encode
    {
        self.serialize_with_encoding_and_extension(value, T::encode_prefered_extension().unwrap_or_default())
    }

    fn serialize_with_encoding_and_extension<T>(self, value: &T, extension: &extension) -> Result<Self::Ok, Self::Error>
        where T: Encode
    {
        use serde::ser::Error;

        if self.is_human_readable()
        {
            let url = value.to_url(extension).map_err(Self::Error::custom)?;
            self.serialize_str(&url)
        }
        else
        {
            let url = value.to_url_bin(extension).map_err(Self::Error::custom)?;
            self.serialize_bytes(&url)
        }
    }
}
#[cfg(feature = "serde")]
impl<F> SerializerWithEncoding for F where F: Serializer {}