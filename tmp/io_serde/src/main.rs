#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{any::{Any, TypeId}, io::BufWriter, ops::{Deref, DerefMut}};

use hexga::{io::{asset::AssetError, fs::{Fs, FsDisk}}, prelude::*};

use ron::ser::PrettyConfig;
use serde::ser::{SerializeMap, SerializeSeq, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};


struct FileSerializer<'a, F, S>
    where
    F: FsWrite,
    for<'s> &'s mut S: Serializer
{
    fs: &'a mut F,
    path: String,
    serializer : NonEmptyStack<S>,//serde_json::Serializer<String>,
}

impl<'a, F, S> FileSerializer<'a,F,S>
    where
    F: FsWrite,
    for<'s> &'s mut S: Serializer
{
    pub fn new(fs: &'a mut F, path: String, serializer : S) -> Self
    {
        Self { fs, path, serializer: NonEmptyStack::new(serializer) }
    }

    #[inline]
    pub(crate) fn _serialize_primitive<T>(&mut self, val: &T) -> Result<<Self as Serializer>::Ok, <Self as Serializer>::Error> where T: Serialize
    {
        todo!()
    }
}

struct Compound<'a, F, S, C>
    where
    F: FsWrite,
    for<'s> &'s mut S: Serializer
{
    fs: &'a mut FileSerializer<'a, F, S>,
    compound : C,
}

// impl Drop for FileSerializer


impl<'a, F, S, C> SerializeSeq for Compound<'a, F, S, C>
    where
    F: FsWrite,
    for<'s> &'s mut S: Serializer<SerializeSeq = C>,
    C: SerializeSeq
{
    type Ok=();
    type Error=AssetError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize
    {
        match self.compound.serialize_element(value)
        {
            Ok(v) => Ok(()),
            Err(e) => Err(___()),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.compound.end()
        {
            Ok(v) => Ok(()),
            Err(e) => Err(___()),
        }
    }
}

impl<'a, F, S, C> SerializeTuple for Compound<'a, F, S, C>
    where
    F: FsWrite,
    for<'s> &'s mut S: Serializer<SerializeTuple = C>,
    C: SerializeTuple
{
    type Ok=();
    type Error=AssetError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize
    {
        match self.compound.serialize_element(value)
        {
            Ok(v) => Ok(()),
            Err(e) => Err(___()),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.compound.end()
        {
            Ok(v) => Ok(()),
            Err(e) => Err(___()),
        }
    }
}

impl<'a, F, S, C> SerializeTupleStruct for Compound<'a, F, S, C>
    where
    F: FsWrite,
    for<'s> &'s mut S: Serializer<SerializeTupleStruct = C>,
    C: SerializeTupleStruct
{
    type Ok=();
    type Error=AssetError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
        T: ?Sized + Serialize
    {
        match self.compound.serialize_field(value)
        {
            Ok(v) => Ok(()),
            Err(e) => Err(___()),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.compound.end()
        {
            Ok(v) => Ok(()),
            Err(e) => Err(___()),
        }
    }
}

impl<'a, F, S, C> SerializeTupleVariant for Compound<'a, F, S, C>
    where
    F: FsWrite,
    for<'s> &'s mut S: Serializer<SerializeTupleVariant = C>,
    C: SerializeTupleVariant
{
    type Ok=();
    type Error=AssetError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        match self.compound.serialize_field(value)
        {
            Ok(v) => Ok(()),
            Err(e) => Err(___()),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.compound.end()
        {
            Ok(v) => Ok(()),
            Err(e) => Err(___()),
        }
    }
}

impl<'a, F, S, C> SerializeMap for Compound<'a, F, S, C>
    where
    F: FsWrite,
    for<'s> &'s mut S: Serializer<SerializeMap = C>,
    C: SerializeMap
{
    type Ok=();
    type Error=AssetError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.compound.serialize_key(key).map_err(|_| ___())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        self.compound.serialize_value(value).map_err(|_| ___())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.compound.end()
        {
            Ok(v) => Ok(()),
            Err(e) => Err(___()),
        }
    }
}

impl<'a, F, S, C> SerializeStruct for Compound<'a, F, S, C>
    where
    F: FsWrite,
    for<'s> &'s mut S: Serializer<SerializeStruct = C>,
    C: SerializeStruct
{
    type Ok=();
    type Error=AssetError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        match self.compound.serialize_field(key, value)
        {
            Ok(v) => Ok(()),
            Err(e) => Err(___()),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.compound.end()
        {
            Ok(v) => Ok(()),
            Err(e) => Err(___()),
        }
    }
}

impl<'a, F, S, C> SerializeStructVariant for Compound<'a, F, S, C>
    where
    F: FsWrite,
    for<'s> &'s mut S: Serializer<SerializeStructVariant = C>,
    C: SerializeStructVariant
{
    type Ok=();
    type Error=AssetError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        match self.compound.serialize_field(key, value)
        {
            Ok(v) => Ok(()),
            Err(e) => Err(___()),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.compound.end()
        {
            Ok(v) => Ok(()),
            Err(e) => Err(___()),
        }
    }
}

impl<'a, 'f, F, S> Serializer for &'a mut FileSerializer<'f, F, S>
where
    F: FsWrite,
    for<'s> &'s mut S: Serializer
{
    type Ok=();
    type Error=AssetError;

    type SerializeSeq=Compound<'a,F,S,<&'a mut S as Serializer>::SerializeSeq>;
    type SerializeTuple=Compound<'a,F,S,<&'a mut S as Serializer>::SerializeTuple>;
    type SerializeTupleStruct=Compound<'a,F,S,<&'a mut S as Serializer>::SerializeTupleStruct>;
    type SerializeTupleVariant=Compound<'a,F,S,<&'a mut S as Serializer>::SerializeTupleVariant>;
    type SerializeMap=Compound<'a,F,S,<&'a mut S as Serializer>::SerializeMap>;
    type SerializeStruct=Compound<'a,F,S,<&'a mut S as Serializer>::SerializeStruct>;
    type SerializeStructVariant=Compound<'a,F,S,<&'a mut S as Serializer>::SerializeStructVariant>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self._serialize_primitive(&v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        self._serialize_primitive(&Some(value))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
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
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}


#[derive(Serialize, Deserialize)]
struct Person
{
    age: i32,
    name: String,
}

fn test_serialize<T>(val: &T) where T: Serialize
{
    let mut fs = FsDisk;
    let mut serializer = serde_json::Serializer::new(Vec::<u8>::new());
    let f = FileSerializer::new(&mut fs, "./tmp/io_serde/test".to_owned(), &mut serializer);

    val.serialize(&mut serializer).unwrap();
}

fn main()
{

    //let c : serde_json::ser::Compound<'static,String,serde_json::ser::CompactFormatter> = serde_json::ser::Compound::
    //test_serialize(true);
    //test_serialize("ok");

    let alice = Person {
        name: "Alice".into(),
        age: 30,
    };

    println!("{}", alice.to_ron().unwrap());
    test_serialize(&alice);


    println!("hello world");
}
