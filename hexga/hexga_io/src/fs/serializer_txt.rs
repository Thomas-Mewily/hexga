use serde::ser::{SerializeMap, SerializeSeq, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};

use super::*;



// pub(crate) struct SerializerTxt;

// impl SerializeSeq for SerializerTxt{
//     type Ok=String;
//     type Error=EncodeError;

//     fn serialize_element<T>(&mut self, _: &T) -> Result<(), Self::Error>
//     where
//         T: ?Sized + Serialize {
//         Err(Default::default())
//     }

//     fn end(self) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }
// }
// impl SerializeTuple for SerializerTxt{
//     type Ok=String;
//     type Error=EncodeError;

//     fn serialize_element<T>(&mut self, _: &T) -> Result<(), Self::Error>
//     where
//         T: ?Sized + Serialize {
//         Err(Default::default())
//     }

//     fn end(self) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }
// }
// impl SerializeTupleStruct for SerializerTxt{
//     type Ok=String;
//     type Error=EncodeError;

//     fn serialize_field<T>(&mut self, _: &T) -> Result<(), Self::Error>
//     where
//         T: ?Sized + Serialize {
//         Err(Default::default())
//     }

//     fn end(self) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }
// }
// impl SerializeTupleVariant for SerializerTxt{
//     type Ok=String;
//     type Error=EncodeError;

//     fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
//     where
//         T: ?Sized + Serialize {
//         Err(Default::default())
//     }

//     fn end(self) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }
// }
// impl SerializeMap for SerializerTxt{
//     type Ok=String;
//     type Error=EncodeError;

//     fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
//     where
//         T: ?Sized + Serialize {
//         Err(Default::default())
//     }

//     fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
//     where
//         T: ?Sized + Serialize {
//         Err(Default::default())
//     }

//     fn end(self) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }
// }
// impl SerializeStruct for SerializerTxt{
//     type Ok=String;
//     type Error=EncodeError;

//     fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
//     where
//         T: ?Sized + Serialize {
//         Err(Default::default())
//     }

//     fn end(self) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }
// }
// impl SerializeStructVariant for SerializerTxt{
//     type Ok=String;
//     type Error=EncodeError;

//     fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
//     where
//         T: ?Sized + Serialize {
//         Err(Default::default())
//     }

//     fn end(self) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }
// }


// impl Serializer for SerializerTxt
// {
//     type Ok=String;
//     type Error=EncodeError;

//     type SerializeSeq=SerializerTxt;
//     type SerializeTuple=SerializerTxt;
//     type SerializeTupleStruct=SerializerTxt;
//     type SerializeTupleVariant=SerializerTxt;
//     type SerializeMap=SerializerTxt;
//     type SerializeStruct=SerializerTxt;
//     type SerializeStructVariant=SerializerTxt;

//     fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_char(self, c: char) -> Result<Self::Ok, Self::Error> {
//         Ok(c.to_string())
//     }

//     fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
//         Ok(v.to_owned())
//     }

//     fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error>
//     {
//         Err(Default::default())
//     }

//     fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
//     where
//         T: ?Sized + Serialize {
//         Err(Default::default())
//     }

//     fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_unit_variant(
//         self,
//         _name: &'static str,
//         _variant_index: u32,
//         _variant: &'static str,
//     ) -> Result<Self::Ok, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_newtype_struct<T>(
//         self,
//         _name: &'static str,
//         _value: &T,
//     ) -> Result<Self::Ok, Self::Error>
//     where
//         T: ?Sized + Serialize {
//         Err(Default::default())
//     }

//     fn serialize_newtype_variant<T>(
//         self,
//         _name: &'static str,
//         _variant_index: u32,
//         _variant: &'static str,
//         _value: &T,
//     ) -> Result<Self::Ok, Self::Error>
//     where
//         T: ?Sized + Serialize {
//         Err(Default::default())
//     }

//     fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_tuple_struct(
//         self,
//         _name: &'static str,
//         _len: usize,
//     ) -> Result<Self::SerializeTupleStruct, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_tuple_variant(
//         self,
//         _name: &'static str,
//         _variant_index: u32,
//         _variant: &'static str,
//         _len: usize,
//     ) -> Result<Self::SerializeTupleVariant, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_struct(
//         self,
//         _name: &'static str,
//         _len: usize,
//     ) -> Result<Self::SerializeStruct, Self::Error> {
//         Err(Default::default())
//     }

//     fn serialize_struct_variant(
//         self,
//         _name: &'static str,
//         _variant_index: u32,
//         _variant: &'static str,
//         _len: usize,
//     ) -> Result<Self::SerializeStructVariant, Self::Error> {
//         Err(Default::default())
//     }

//     fn is_human_readable(&self) -> bool {
//         false
//     }
// }




