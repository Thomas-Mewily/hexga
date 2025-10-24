#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{any::{Any, TypeId}, io::BufWriter, ops::{Deref, DerefMut}};

use hexga::{io::{asset::AssetError, fs::{Fs, FsDisk}}, prelude::*};

use ron::ser::PrettyConfig;
use serde::ser::{SerializeMap, SerializeSeq, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

// pub trait MetaStringSerializer
// {
//     fn into_markup(self) -> String;
// }
// impl MetaStringSerializer for serde_json::ser::Serializer<String>
// {
//     fn into_markup(self) -> String
//     {
//         let inner = self.into_inner();
//     }
// }

// pub trait MetaSerializerGenerator
// {

// }

struct JsonFileSerializer<'a, F>
    where
    F: FsWrite,
{
    fs: &'a mut F,
    stack : Vec<SerializeAt>,
}

pub struct SerializeAt
{
    serializer: JsonSerializer,
    path: Path,
}

type JsonSerializer = serde_json::Serializer<Vec<u8>>;

impl<'a, F> JsonFileSerializer<'a,F>
    where
    F: FsWrite,
{
    pub fn new(fs: &'a mut F, path: String) -> Self
    {
        Self { fs, stack: vec![SerializeAt{ serializer: JsonSerializer::new(___()), path }] }
    }

    pub(crate) fn new_and_serialize<T>(fs: &'a mut F, path: String, val: &T) -> Result<(), AssetError>
        where T:Serialize
    {
        let mut s: JsonFileSerializer<'a, F> = Self::new(fs, path);
        {
            let s_mut = &mut s;
            val.serialize(s_mut)?;
        }
        s.save()

        // let mut s = Self::new(fs, path);
        // {
        //     let s_mut = &mut s;
        //     val.serialize(s_mut)?;
        // }
        // s.save()
    }

    #[inline]
    pub(crate) fn _serialize_primitive<T>(&mut self, val: &T) -> Result<(), AssetError>
        where
        T: Serialize,
    {
        val.serialize(&mut self.stack.last_mut().unwrap().serializer).map_err(|_| AssetError::___())
    }

    pub(crate) fn new_serializer(&mut self, name: &path)
    {
        let path = self.stack.last_mut().unwrap().path.as_str().path_concat(name);
        self.stack.push(SerializeAt { serializer: JsonSerializer::new(___()), path });
    }

    pub fn save(&mut self) -> Result<(), AssetError>
    {
        while let Some(SerializeAt { serializer, path }) = self.stack.pop()
        {
            let bytes = serializer.into_inner();
            self.fs.write_bytes(&path, &bytes).map_err(|_| ___())?;
        }
        Ok(())
    }
}

// impl<'a, F> Drop for JsonFileSerializer<'a,F>
//     where
//     F: FsWrite
// {
//     fn drop(&mut self) {
//         self.save().unwrap(); // FIXME: Should return a result
//     }
// }




struct Compound<'a, F, C>
    where
    F: FsWrite,
{
    fs: &'a mut F,
    compound : C,
}



impl<'a, F, C> SerializeSeq for Compound<'a, F, C>
    where
    F: FsWrite,
    C: SerializeSeq,
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

impl<'a, F, C> SerializeTuple for Compound<'a, F, C>
    where
    F: FsWrite,
    C: SerializeTuple,
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

impl<'a, F, C> SerializeTupleStruct for Compound<'a, F, C>
    where
    F: FsWrite,
    C: SerializeTupleStruct,
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

impl<'a, F, C> SerializeTupleVariant for Compound<'a, F, C>
    where
    F: FsWrite,
    C: SerializeTupleVariant,
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

impl<'a, F, C> SerializeMap for Compound<'a, F, C>
    where
    F: FsWrite,
    C: SerializeMap,
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

impl<'a, F, C> SerializeStruct for Compound<'a, F, C>
    where
    F: FsWrite,
    C: SerializeStruct,
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

impl<'a, F, C> SerializeStructVariant for Compound<'a, F, C>
    where
    F: FsWrite,
    C: SerializeStructVariant,
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

impl<'x, 'a, F> Serializer for &'x mut JsonFileSerializer<'a, F>
where
    F: FsWrite,
    'x: 'a
{
    type Ok=();
    type Error=AssetError;

    type SerializeSeq=Compound<'x,F,<&'x mut JsonSerializer as Serializer>::SerializeSeq>;
    type SerializeTuple=Compound<'x,F,<&'x mut JsonSerializer as Serializer>::SerializeTuple>;
    type SerializeTupleStruct=Compound<'x,F,<&'x mut JsonSerializer as Serializer>::SerializeTupleStruct>;
    type SerializeTupleVariant=Compound<'x,F,<&'x mut JsonSerializer as Serializer>::SerializeTupleVariant>;
    type SerializeMap=Compound<'x,F,<&'x mut JsonSerializer as Serializer>::SerializeMap>;
    type SerializeStruct=Compound<'x,F,<&'x mut JsonSerializer as Serializer>::SerializeStruct>;
    type SerializeStructVariant=Compound<'x,F,<&'x mut JsonSerializer as Serializer>::SerializeStructVariant>;

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
    ) -> Result<Self::SerializeStruct, Self::Error>
    {
        self.new_serializer(name);
        let SerializeAt { serializer, path } = self.stack.last_mut().unwrap();
        let compound = serializer.serialize_struct(name, len).map_err(|_| ___())?;
        Ok(Compound { fs: self.fs, compound })
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
    JsonFileSerializer::new_and_serialize(&mut fs, "./tmp/io_serde/test".to_owned(), val).unwrap();
}

fn test_it()
{
    //let c : serde_json::ser::Compound<'static,String,serde_json::ser::CompactFormatter> = serde_json::ser::Compound::
    //test_serialize(true);
    //test_serialize("ok");

    let alice = Person {
        name: "Alice".into(),
        age: 30,
    };
    //test_serialize(&alice);
    test_serialize(&64);

    //println!("{}", alice.to_ron().unwrap());
}

fn main()
{
    test_it();


    println!("hello world");
}
