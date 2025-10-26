use super::*;


pub struct MultiFileSerializerParam
{
    pub develop_map: bool,
    pub develop_structure: bool,
}

pub struct JsonFileSerializer<'a, F>
    where
    F: FsWrite,
{
    fs: &'a mut F,
    should_save: bool,
    path: Path,
    serializer: JsonSerializer,
    //stack : Vec<SerializeAt>,
}

// pub struct SerializeAt
// {
//     serializer: JsonSerializer,
//     path: Path,
// }

type JsonSerializer = serde_json::Serializer<Vec<u8>>;

impl<'a, F> JsonFileSerializer<'a,F>
    where
    F: FsWrite,
{
    pub fn new(fs: &'a mut F, path: Path) -> Self
    {
        Self { fs, path, serializer: Self::new_serializer(), should_save: true }
    }

    pub fn new_and_serialize<T>(fs: &'a mut F, path: Path, val: &T) -> Result<(), AssetError>
    where
        T: Serialize,
    {
        let mut s = Self::new(fs, path);
        val.serialize(&mut s)?;
        s.save()
    }



    fn new_serializer() -> JsonSerializer
    {
        JsonSerializer::new(___())
    }

    fn save(&mut self) -> Result<(), AssetError>
    {
        if !self.should_save { return Ok(()); }

        let mut serializer = Self::new_serializer();
        std::mem::swap(&mut serializer, &mut self.serializer);
        let bytes = serializer.into_inner();
        self.fs.write_bytes(&self.path, &bytes).map_err(|kind| AssetError { path: self.path.to_owned(), kind, ..___() })
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



#[doc(hidden)]
pub struct Compound<'a, F, C>
    where
    F: FsWrite,
{
    fs: &'a mut F,
    path: &'a Path,
    parent_should_save: &'a mut bool,
    compound : C,
    name: Option<String>,
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
        T: ?Sized + Serialize
    {
        match key.serialize(IdentifierSerializer)
        {
            Ok(identifier) => { self.name = Some(identifier); Ok(()) },
            Err(v) => self.compound.serialize_key(key).map_err(|_| ___()),
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        match self.name.take()
        {
            Some(identifier) => JsonFileSerializer::new_and_serialize(self.fs, (self.path / identifier).with_extension("json"), &value),
            None =>
            {
                *self.parent_should_save = true;
                self.compound.serialize_value(value).map_err(|_| ___())
            }
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

    fn end(self) -> Result<Self::Ok, Self::Error>
    {
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
    'a: 'x
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
        self.serializer.serialize_bool(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i8(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i16(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i32(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_i64(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u8(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u16(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u32(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_u64(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_f32(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_f64(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_char(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_str(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_bytes(v).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_none().map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        self.serializer.serialize_some(value).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit().map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit_struct(name).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serializer.serialize_unit_variant(name, variant_index, variant).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        self.serializer.serialize_newtype_struct(name, value).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
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
        self.serializer.serialize_newtype_variant(name, variant_index, variant, value).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let compound = self.serializer.serialize_seq(len).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))?;
        Ok(Compound { fs: self.fs, path: &self.path, parent_should_save: &mut self.should_save, compound, name: None })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        let compound = self.serializer.serialize_tuple(len).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))?;
        Ok(Compound { fs: self.fs, path: &self.path, parent_should_save: &mut self.should_save, compound, name: None })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        let compound = self.serializer.serialize_tuple_struct(name, len).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))?;
        Ok(Compound { fs: self.fs, path: &self.path, parent_should_save: &mut self.should_save, compound, name: None })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let compound = self.serializer.serialize_tuple_variant(name, variant_index, variant, len).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))?;
        Ok(Compound { fs: self.fs, path: &self.path, parent_should_save: &mut self.should_save, compound, name: None })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let compound = self.serializer.serialize_map(len).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))?;
        self.should_save = false;
        Ok(Compound { fs: self.fs, path: &self.path, parent_should_save: &mut self.should_save, compound, name: None })
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>
    {
        let compound = self.serializer.serialize_struct(name, len).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))?;
        Ok(Compound { fs: self.fs, path: &self.path, parent_should_save: &mut self.should_save, compound, name: None })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let compound = self.serializer.serialize_struct_variant(name, variant_index, variant, len).map_err(|e| AssetError::new(&self.path, IoError::Custom(e.to_string())))?;
        Ok(Compound { fs: self.fs, path: &self.path, parent_should_save: &mut self.should_save, compound, name: None })
    }
}
