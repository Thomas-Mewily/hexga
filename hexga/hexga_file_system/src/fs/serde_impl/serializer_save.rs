use super::*;


pub(crate) struct SerializedFormat
{
    extension:String,
    bytes: Vec<u8>
}

// pub(crate) enum SerializedData
// {
//     CustomFormat(SerializedFormat),
//     Text(String),
//     Binary(Vec<u8>),
//     Markup(String),
// }

// pub(crate) struct SerializerSave<'se>
// {
//     ron: Option<&'se mut SerializerRon<'se>>,
// }
pub(crate) struct SerializerSave<S>
{
    serializer: S,
}
impl<S> SerializerSave<S>
{
    pub fn new(serializer: S) -> Self
    {
        Self { serializer }
    }
}

pub(crate) struct SerializerSaveCompound<C>
{
    compound: C,
}
impl<C> SerializerSaveCompound<C>
{
    pub fn new(compound: C) -> Self { Self { compound } }
}



impl<C> SerializeSeq for SerializerSaveCompound<C>
    where C: SerializeSeq
{
    type Ok=();
    type Error=EncodeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        self.compound.serialize_element(value).map_err(|e| EncodeError::from_display(e))
    }

    fn end(self) -> Result<Self::Ok, Self::Error>
    {
        match self.compound.end()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(EncodeError::from_display(e)),
        }
    }
}
impl<C> SerializeTuple for SerializerSaveCompound<C>
    where C: SerializeTuple
{
    type Ok=();
    type Error=EncodeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        self.compound.serialize_element(value).map_err(|e| EncodeError::from_display(e))
    }

    fn end(self) -> Result<Self::Ok, Self::Error>
    {
        match self.compound.end()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(EncodeError::from_display(e)),
        }
    }
}
impl<C> SerializeTupleStruct for SerializerSaveCompound<C>
    where C: SerializeTupleStruct
{
    type Ok=();
    type Error=EncodeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.compound.serialize_element(value).map_err(|e| EncodeError::from_display(e))
    }

    fn end(self) -> Result<Self::Ok, Self::Error>
    {
        match self.compound.end()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(EncodeError::from_display(e)),
        }
    }
}
impl<C> SerializeTupleVariant for SerializerSaveCompound<C>
    where C: SerializeTupleVariant
{
    type Ok=();
    type Error=EncodeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.compound.serialize_element(value).map_err(|e| EncodeError::from_display(e))
    }

    fn end(self) -> Result<Self::Ok, Self::Error>
    {
        match self.compound.end()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(EncodeError::from_display(e)),
        }
    }
}
impl<C> SerializeMap for SerializerSaveCompound<C>
    where C: SerializeMap
{
    type Ok=();
    type Error=EncodeError;

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
        where
            K: ?Sized + Serialize,
            V: ?Sized + Serialize, {
        self.compound.serialize_entry(key, value).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.compound.serialize_key(key).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.compound.serialize_value(value).map_err(|e| EncodeError::from_display(e))
    }

    fn end(self) -> Result<Self::Ok, Self::Error>
    {
        match self.compound.end()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(EncodeError::from_display(e)),
        }
    }
}
impl<C> SerializeStruct for SerializerSaveCompound<C>
    where C: SerializeStruct
{
    type Ok=();
    type Error=EncodeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.compound.serialize_field(key, value).map_err(|e| EncodeError::from_display(e))
    }

    fn end(self) -> Result<Self::Ok, Self::Error>
    {
        match self.compound.end()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(EncodeError::from_display(e)),
        }
    }
}
impl<C> SerializeStructVariant for SerializerSaveCompound<C>
    where C: SerializeStructVariant
{
    type Ok=();
    type Error=EncodeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.compound.serialize_field(key, value).map_err(|e| EncodeError::from_display(e))
    }

    fn end(self) -> Result<Self::Ok, Self::Error>
    {
        match self.compound.end()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(EncodeError::from_display(e)),
        }
    }
}

impl<'se, S> Serializer for &'se mut SerializerSave<S>
    where &'se mut S: Serializer
{
    type Ok=();
    type Error=EncodeError;

    type SerializeSeq = SerializerSaveCompound<<&'se mut S as Serializer>::SerializeSeq>;
    type SerializeTuple = &'se mut SerializerSaveCompound<<&'se mut S as Serializer>::SerializeTuple>;
    type SerializeTupleStruct = &'se mut SerializerSaveCompound<<&'se mut S as Serializer>::SerializeTupleStruct>;
    type SerializeTupleVariant = &'se mut SerializerSaveCompound<<&'se mut S as Serializer>::SerializeTupleVariant>;
    type SerializeMap = &'se mut SerializerSaveCompound<<&'se mut S as Serializer>::SerializeMap>;
    type SerializeStruct = &'se mut SerializerSaveCompound<<&'se mut S as Serializer>::SerializeStruct>;
    type SerializeStructVariant = &'se mut SerializerSaveCompound<<&'se mut S as Serializer>::SerializeStructVariant>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_bool(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i8(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i16(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i32(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i64(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u8(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u16(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u32(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u64(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_f32(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_f64(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_char(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_str(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_bytes(v).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_none().map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        self.serializer.serialize_some(value).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit().map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit_struct(name).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit_variant(name, variant_index, variant).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        self.serializer.serialize_newtype_struct(name, value).map_err(|e| EncodeError::from_display(e))
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
        self.serializer.serialize_newtype_variant(name, variant_index, variant, value).map_err(|e| EncodeError::from_display(e))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SerializerSaveCompound::new(self.serializer.serialize_seq(len).map_err(|e| EncodeError::from_display(e))?))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializerSaveCompound::new(self.serializer.serialize_tuple(len).map_err(|e| EncodeError::from_display(e))?))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializerSaveCompound::new(self.serializer.serialize_tuple_struct(name, len).map_err(|e| EncodeError::from_display(e))?))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializerSaveCompound::new(self.serializer.serialize_tuple_variant(name, variant_index, variant, len).map_err(|e| EncodeError::from_display(e))?))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializerSaveCompound::new(self.serializer.serialize_map(len).map_err(|e| EncodeError::from_display(e))?))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializerSaveCompound::new(self.serializer.serialize_struct(name, len).map_err(|e| EncodeError::from_display(e))?))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializerSaveCompound::new(self.serializer.serialize_struct_variant(name, variant_index, variant, len).map_err(|e| EncodeError::from_display(e))?))
    }
}