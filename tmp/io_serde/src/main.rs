#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{env::var, io::BufWriter, ops::{Deref, DerefMut}};

use hexga::{io::{asset::AssetError, fs::{Fs, FsDisk}}, prelude::*};

use ron::ser::PrettyConfig;
use serde::ser::{SerializeMap, SerializeSeq, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, ser::SerializeStruct};

struct SerdeAssetSerializer<F,S>
{
    path: Path,
    fs: F,
    serializer: S,
    //prefered_extension: Extension,
}
impl<F,S> Deref for SerdeAssetSerializer<F,S>
{
    type Target=F;
    fn deref(&self) -> &Self::Target {
        &self.fs
    }
}
impl<F,S> DerefMut for SerdeAssetSerializer<F,S>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fs
    }
}
impl<F,S> SerdeAssetSerializer<F,S>
{
    pub fn new(path: Path, fs: F, serializer: S) -> Self { Self { path, fs, serializer }}
}
impl<F> SerdeAssetSerializer<F,ron::Serializer<String>>
{
    pub fn ron(path: Path, fs: F) -> Self { Self { path, fs, serializer: ron::Serializer::new(String::new(), None).unwrap() }}
}

impl<'a,F,S> SerializeSeq for SerdeAssetSerializer<F,S>
    where
    S: SerializeSeq
{
    type Ok = ();
    type Error = AssetError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
            todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a,F,S> SerializeTuple for SerdeAssetSerializer<F,S>
    where
    S: SerializeTuple
{
    type Ok = ();
    type Error = AssetError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}


impl<'a,F,S> SerializeTupleStruct for SerdeAssetSerializer<F,S>
    where
    S: SerializeTupleStruct
{
    type Ok = ();
    type Error = AssetError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a,F,S> SerializeTupleVariant for SerdeAssetSerializer<F,S>
    where
    S: SerializeTupleVariant
{
    type Ok = ();
    type Error = AssetError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a,F,S> SerializeMap for SerdeAssetSerializer<F,S>
    where
    S: SerializeMap
{
    type Ok = ();
    type Error = AssetError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a,F,S> SerializeStruct for SerdeAssetSerializer<F,S>
    where
    S: SerializeStruct
{
    type Ok = ();
    type Error = AssetError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a,F,S> SerializeStructVariant for SerdeAssetSerializer<F,S>
    where
    S: SerializeStructVariant
{
    type Ok = ();
    type Error = AssetError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}




// A simple example for our format: key=value;key=value
impl<'a,F,S> Serializer for SerdeAssetSerializer<F,S>
    where
    S: Serializer
{
    type Ok = ();
    type Error = AssetError;

    type SerializeSeq = SerdeAssetSerializer<F,S::SerializeSeq>;
    type SerializeTuple = SerdeAssetSerializer<F,S::SerializeTuple>;
    type SerializeTupleStruct = SerdeAssetSerializer<F,S::SerializeTupleStruct>;
    type SerializeTupleVariant = SerdeAssetSerializer<F,S::SerializeTupleVariant>;
    type SerializeMap = SerdeAssetSerializer<F,S::SerializeMap>;
    type SerializeStruct = SerdeAssetSerializer<F,S::SerializeStruct>;
    type SerializeStructVariant = SerdeAssetSerializer<F,S::SerializeStructVariant>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_bool(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_i8(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_i16(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_i32(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_i64(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_u8(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_u16(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_u32(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_u64(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_f32(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_f64(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_char(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_str(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_bytes(v).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_none().map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_some(value).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_unit().map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_unit_struct(name).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_unit_variant(name, variant_index, variant).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_newtype_struct(name, value).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
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
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let _ok = serializer.serialize_newtype_variant(name, variant_index, variant, value).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let serializer = serializer.serialize_seq(len).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(SerdeAssetSerializer{path, fs, serializer})
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let serializer = serializer.serialize_tuple(len).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(SerdeAssetSerializer{path, fs, serializer})
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let serializer = serializer.serialize_tuple_struct(name, len).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(SerdeAssetSerializer{path, fs, serializer})
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let serializer = serializer.serialize_tuple_variant(name, variant_index, variant, len).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(SerdeAssetSerializer{path, fs, serializer})
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let serializer = serializer.serialize_map(len).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(SerdeAssetSerializer{path, fs, serializer})
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>
    {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let serializer = serializer.serialize_struct(name, len).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(SerdeAssetSerializer{path, fs, serializer})
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let SerdeAssetSerializer { path, fs, serializer } = self;
        let serializer = serializer.serialize_struct_variant(name, variant_index, variant, len).map_err(|e| AssetError{ path: path.clone(), kind: IoError::from_display(e), ..___() })?;
        Ok(SerdeAssetSerializer{path, fs, serializer})
    }
}

#[derive(Serialize, Deserialize)]
struct Person
{
    age: i32,
    name: String,
}


fn t()
{
    let my_struct = Person {
        name: "Alice".into(),
        age: 30,
    };

    let mut s = String::new();
    let mut ron = ron::ser::Serializer::new(&mut s, None).unwrap();
    my_struct.serialize(&mut ron).unwrap();

    //dbg!(r);
    println!("{s}");

    let mut meta_ron = SerdeAssetSerializer::ron("./tmp/io_serde/test".to_owned(), FsDisk);
    my_struct.serialize(meta_ron);
    // let mut f = SerdeAssetSerializer::ron(FsDisk);
    // let r = my_struct.serialize(&mut f);
    // dbg!(&r);
}

fn main()
{
    t();
    println!("hello world");
}
