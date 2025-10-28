use super::*;


pub trait Decode
{
    fn decode_extensions() -> impl Iterator<Item = &'static extension>;
    fn decode_from_reader<R>(reader : &mut R, format: &extension) -> EncodeResult<Self> where R: Read, Self: Sized
    {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).map_err(|e| EncodeError::from(e))?;
        Self::decode(&bytes, format)
    }
    fn decode(bytes: &[u8], format: &extension) -> EncodeResult<Self> where Self: Sized;
    fn decode_prefered_extension() -> Option<&'static extension> { Self::decode_extensions().next() }
}

#[cfg(feature = "serde")]
pub trait DeserializerWithEncoding<'de> : Deserializer<'de>
{
    fn deserialize_with_encoding<T>(self) -> Result<T, Self::Error>
        where T: Decode + FromUrl
    {
        if self.is_human_readable()
        {
            let url = String::deserialize(self)?;
            return T::from_url(&url).map_err(serde::de::Error::custom);
        }
        let url = Vec::<u8>::deserialize(self)?;
        return T::from_url_bin(&url).map_err(serde::de::Error::custom);
    }
}
#[cfg(feature = "serde")]
impl<'de, F> DeserializerWithEncoding<'de> for F where F: Deserializer<'de> {}



