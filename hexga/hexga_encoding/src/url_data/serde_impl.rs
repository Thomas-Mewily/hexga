use super::*;


pub trait UrlDeserializer<'de> : Deserializer<'de>
{
    fn deserialize_with_encoding<T>(self) -> Result<T, Self::Error>
        where T: FromUrl
    {
        if self.is_human_readable()
        {
            let url = self.deserialize_byte_buf(StringVisitor)?;
            return T::from_url(&url).map_err(serde::de::Error::custom);
        }
        let url = self.deserialize_byte_buf(BytesVisitor)?;
        return T::from_bin_url_or_bytes(&url, "").map_err(serde::de::Error::custom);
    }
}

impl<'de, F> UrlDeserializer<'de> for F where F: Deserializer<'de> {}




pub trait UrlSerializer : Serializer
{
    fn serialize_with_encoding<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where T: ToUrl
    {
        self.serialize_with_encoding_and_extension(value, T::save_prefered_extension().unwrap_or_default())
    }

    fn serialize_with_encoding_and_extension<T>(self, value: &T, extension: &extension) -> Result<Self::Ok, Self::Error>
        where T: ToUrl
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

impl<F> UrlSerializer for F where F: Serializer {}

pub(crate) struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a byte buffer")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_vec())
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }
}

pub(crate) struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a UTF-8 string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_string())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }
}