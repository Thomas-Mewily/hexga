use super::*;

// pub(crate) struct DeserializerTxtOrBinary
// {
//     pub(crate) bytes: Vec<u8>
// }
// impl<'de> Deserializer<'de> for DeserializerTxtOrBinary
// {
//     type Error=EncodeError;

//     fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         self.deserialize_byte_buf(visitor)
//     }

//     fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         visitor.visit_str(&String::try_from(self.bytes).map_err(|e| EncodeError::from(e))?)
//     }

//     fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>
//     {
//         visitor.visit_string(String::try_from(self.bytes).map_err(|e| EncodeError::from(e))?)
//     }

//     fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         visitor.visit_bytes(&self.bytes)
//     }

//     fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         visitor.visit_byte_buf(self.bytes)
//     }

//     fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_unit_struct<V>(
//         self,
//         _name: &'static str,
//         _visitor: V,
//     ) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_newtype_struct<V>(
//         self,
//         _name: &'static str,
//         _visitor: V,
//     ) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_tuple_struct<V>(
//         self,
//         _name: &'static str,
//         _len: usize,
//         _visitor: V,
//     ) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_struct<V>(
//         self,
//         _name: &'static str,
//         _fields: &'static [&'static str],
//         _visitor: V,
//     ) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_enum<V>(
//         self,
//         _name: &'static str,
//         _variants: &'static [&'static str],
//         _visitor: V,
//     ) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de> {
//         Err(Default::default())
//     }

//     fn is_human_readable(&self) -> bool {
//         false
//     }
// }