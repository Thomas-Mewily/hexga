use super::*;

pub(crate) struct IdentifierSerializer;
pub struct IdentifierSerializerError;

impl std::fmt::Debug for IdentifierSerializerError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl std::fmt::Display for IdentifierSerializerError
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl std::error::Error for IdentifierSerializerError
{

}
impl serde::ser::Error for IdentifierSerializerError
{
    fn custom<T>(msg:T) -> Self where T:Display {
        IdentifierSerializerError
    }
}

impl SerializeSeq for IdentifierSerializer
{
    type Ok=String;
    type Error=IdentifierSerializerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }
}

impl SerializeTuple for IdentifierSerializer
{
    type Ok=String;
    type Error=IdentifierSerializerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }
}

impl SerializeTupleStruct for IdentifierSerializer
{
    type Ok=String;
    type Error=IdentifierSerializerError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }
}

impl SerializeTupleVariant for IdentifierSerializer
{
    type Ok=String;
    type Error=IdentifierSerializerError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }
}

impl SerializeMap for IdentifierSerializer
{
    type Ok=String;
    type Error=IdentifierSerializerError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }
}

impl SerializeStruct for IdentifierSerializer
{
    type Ok=String;
    type Error=IdentifierSerializerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }
}

impl SerializeStructVariant for IdentifierSerializer
{
    type Ok=String;
    type Error=IdentifierSerializerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }
}

impl Serializer for IdentifierSerializer
{
    type Ok=String;
    type Error=IdentifierSerializerError;

    type SerializeSeq=IdentifierSerializer;
    type SerializeTuple=IdentifierSerializer;
    type SerializeTupleStruct=IdentifierSerializer;
    type SerializeTupleVariant=IdentifierSerializer;
    type SerializeMap=IdentifierSerializer;
    type SerializeStruct=IdentifierSerializer;
    type SerializeStructVariant=IdentifierSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_owned())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(IdentifierSerializerError)
    }
}