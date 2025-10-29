use serde::ser::{SerializeMap, SerializeSeq, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};

use super::*;


#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct SaveParam
{
    /// Will use multi file
    pub multi_file: bool,

    /// Will map be expended into multiple file (if multi_file is true)
    pub multi_file_map: bool,
    /// Will struct be expended into multiple file (if multi_file is true)
    pub multi_file_struct: bool,

    // #[serde(borrow)]
    // pub indent: &'static str,
    // #[serde(borrow)]
    // pub separator: &'static str,
    // pub capacity : usize,
}
impl Default for SaveParam
{
    fn default() -> Self {
        Self { multi_file: true, multi_file_map: true, multi_file_struct: false }
    }
}
impl SaveParam
{
    pub fn with_multi_file(self, multi_file: bool) -> Self { Self { multi_file, ..self }}
    pub fn with_multi_file_map(self, multi_file_map: bool) -> Self { Self { multi_file_map, ..self }}
    pub fn with_multi_file_struct(self, multi_file_struct: bool) -> Self { Self { multi_file_struct, ..self }}
}


pub(crate) struct SerializerSaveTxtOrBinOrMarkup<'a, F>
    where
    F: FsWrite,
{
    pub(crate) fs: &'a mut F,
    pub(crate) should_save: bool,
    pub(crate) path: Path,
    pub(crate) serializer: Option<SerializerMarkup>,
    pub(crate) default_capacity: usize,
    pub(crate) param: SaveParam,
}

impl<'a, F> SerializerSaveTxtOrBinOrMarkup<'a, F>
    where
    F: FsWrite,
{
    pub(crate) fn new(fs: &'a mut F, path: Path) -> Self
    {
        let capacity = 1024;
        Self::new_full(fs, path, SaveParam::default(), SerializerMarkup::Ron(SerializerRon::new_serializer(capacity)), capacity)
    }

    pub(crate) fn with_param(self, param: SaveParam) -> Self { Self { param, ..self} }

    pub(crate) fn new_full(fs: &'a mut F, path: Path, param: SaveParam, serializer: SerializerMarkup, default_capacity: usize) -> Self
    {
        Self { fs, should_save: true, path, serializer: Some(serializer), default_capacity, param }
    }
}

pub(crate) enum Key
{
    String(String),
    Char(char),
}

#[doc(hidden)]
pub(crate) struct SerializerSaveCompound<'a, F, Ron,Json,Xml>
    where
    F: FsWrite,
{
    fs: &'a mut F,
    path: &'a Path,
    param: &'a SaveParam,
    parent_should_save: &'a mut bool,
    compound : SerializerMarkupOf<Ron,Json,Xml>,
    key: Option<Key>,
}

pub(crate) type SerializerMarkup = SerializerMarkupOf<SerializerRon,SerializerJson,SerializerXml>;

pub(crate) enum SerializerMarkupOf<Ron,Json,Xml>
{
    Ron(Ron),
    Json(Json),
    Xml(Xml),
}

macro_rules! dispatch_serializer {
    // mutable borrow
    (&mut $self:expr, $s:pat  => $body:expr) => {
        match $self.serializer.as_mut().unwrap() {
            SerializerMarkupOf::Ron($s) => $body,
            SerializerMarkupOf::Json($s) => $body,
            SerializerMarkupOf::Xml($s) => $body,
        }
    };

    // by value (move)
    ($self:expr, $s:pat  => $body:expr) => {
        match std::mem::replace(&mut $self.serializer, None).unwrap() {
            SerializerMarkupOf::Ron($s) => $body,
            SerializerMarkupOf::Json($s) => $body,
            SerializerMarkupOf::Xml($s) => $body,
        }
    };
}

macro_rules! dispatch_compound_serializer {
    // mutable borrow
    (&mut $self:expr, $s:pat  => $body:expr) => {
        match &mut $self.compound {
            SerializerMarkupOf::Ron($s) => $body,
            SerializerMarkupOf::Json($s) => $body,
            SerializerMarkupOf::Xml($s) => $body,
        }
    };

    // by value (move)
    ($self:expr, $s:pat  => $body:expr) => {
        match $self.compound {
            SerializerMarkupOf::Ron($s) => $body,
            SerializerMarkupOf::Json($s) => $body,
            SerializerMarkupOf::Xml($s) => $body,
        }
    };
}


impl<'a, F, Ron,Json,Xml> SerializeTuple for SerializerSaveCompound<'a,F,Ron,Json,Xml>
    where
    F: FsWrite,
    Ron: SerializeTuple,
    Json: SerializeTuple,
    Xml: SerializeTuple,
{
    type Ok=();
    type Error=IoError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        dispatch_compound_serializer!(&mut self, s => s.serialize_element(value).map_err(|e| IoError::new(self.path, FileError::from_display(e))))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        dispatch_compound_serializer!(self, s =>
            match s.end()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(IoError::new(self.path, FileError::from_display(e))),
            }
        )
    }
}
impl<'a, F, Ron,Json,Xml> SerializeSeq for SerializerSaveCompound<'a,F,Ron,Json,Xml>
    where
    F: FsWrite,
    Ron: SerializeSeq,
    Json: SerializeSeq,
    Xml: SerializeSeq,
{
    type Ok=();
    type Error=IoError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        dispatch_compound_serializer!(&mut self, s => s.serialize_element(value).map_err(|e| IoError::new(self.path, FileError::from_display(e))))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        dispatch_compound_serializer!(self, s =>
            match s.end()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(IoError::new(self.path, FileError::from_display(e))),
            }
        )
    }
}
impl<'a, F, Ron,Json,Xml> SerializeTupleStruct for SerializerSaveCompound<'a,F,Ron,Json,Xml>
    where
    F: FsWrite,
    Ron: SerializeTupleStruct,
    Json: SerializeTupleStruct,
    Xml: SerializeTupleStruct,
{
    type Ok=();
    type Error=IoError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        dispatch_compound_serializer!(&mut self, s => s.serialize_field(value).map_err(|e| IoError::new(self.path, FileError::from_display(e))))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        dispatch_compound_serializer!(self, s =>
            match s.end()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(IoError::new(self.path, FileError::from_display(e))),
            }
        )
    }
}
impl<'a, F, Ron,Json,Xml> SerializeTupleVariant for SerializerSaveCompound<'a,F,Ron,Json,Xml>
    where
    F: FsWrite,
    Ron: SerializeTupleVariant,
    Json: SerializeTupleVariant,
    Xml: SerializeTupleVariant,
{
    type Ok=();
    type Error=IoError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        dispatch_compound_serializer!(&mut self, s => s.serialize_field(value).map_err(|e| IoError::new(self.path, FileError::from_display(e))))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        dispatch_compound_serializer!(self, s =>
            match s.end()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(IoError::new(self.path, FileError::from_display(e))),
            }
        )
    }
}
impl<'a, F, Ron,Json,Xml> SerializeMap for SerializerSaveCompound<'a,F,Ron,Json,Xml>
    where
    F: FsWrite,
    Ron: SerializeMap,
    Json: SerializeMap,
    Xml: SerializeMap,
{
    type Ok=();
    type Error=IoError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        dispatch_compound_serializer!(&mut self, s => s.serialize_key(key).map_err(|e| IoError::new(self.path, FileError::from_display(e))))
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        dispatch_compound_serializer!(&mut self, s => s.serialize_value(value).map_err(|e| IoError::new(self.path, FileError::from_display(e))))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        dispatch_compound_serializer!(self, s =>
            match s.end()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(IoError::new(self.path, FileError::from_display(e))),
            }
        )
    }
}
impl<'a, F, Ron,Json,Xml> SerializeStruct for SerializerSaveCompound<'a,F,Ron,Json,Xml>
    where
    F: FsWrite,
    Ron: SerializeStruct,
    Json: SerializeStruct,
    Xml: SerializeStruct,
{
    type Ok=();
    type Error=IoError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where
        T: ?Sized + Serialize {
        dispatch_compound_serializer!(&mut self, s => s.serialize_field(key, value).map_err(|e| IoError::new(self.path, FileError::from_display(e))))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        dispatch_compound_serializer!(self, s =>
            match s.end()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(IoError::new(self.path, FileError::from_display(e))),
            }
        )
    }
}
impl<'a, F, Ron,Json,Xml> SerializeStructVariant for SerializerSaveCompound<'a,F,Ron,Json,Xml>
    where
    F: FsWrite,
    Ron: SerializeStruct,
    Json: SerializeStruct,
    Xml: SerializeStruct,
{
    type Ok=();
    type Error=IoError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where
        T: ?Sized + Serialize {
        dispatch_compound_serializer!(&mut self, s => s.serialize_field(key, value).map_err(|e| IoError::new(self.path, FileError::from_display(e))))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        dispatch_compound_serializer!(self, s =>
            match s.end()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(IoError::new(self.path, FileError::from_display(e))),
            }
        )
    }
}

pub(crate) type SerializerRon = ron::ser::Serializer<String>;
pub(crate) type SerializerJson = serde_json::Serializer<Vec<u8>>;
pub(crate) type SerializerXml = serde_xml_rs::ser::Serializer<Vec<u8>>;

pub(crate) trait MarkupSerializer
{
    fn new_serializer(capacity: usize) -> Self;
    fn extract(self) -> EncodeResult<String>;
}
impl MarkupSerializer for SerializerRon
{
    fn new_serializer(capacity: usize) -> Self
    {
        SerializerRon::new(String::with_capacity(capacity), Some(Default::default())).unwrap()
    }
    fn extract(self) -> EncodeResult<String>
    {
        todo!("into inner when it will be impl 4 ron")
    }
}
impl MarkupSerializer for SerializerJson
{
    fn new_serializer(capacity: usize) -> Self
    {
        SerializerJson::new(Vec::with_capacity(capacity))
    }

    fn extract(self) -> EncodeResult<String> {
        todo!()
    }
}
impl MarkupSerializer for SerializerXml
{
    fn new_serializer(capacity: usize) -> Self
    {
        SerializerXml::new_from_writer(Vec::with_capacity(capacity))
    }
    fn extract(self) -> EncodeResult<String> {
        Err(EncodeError::custom("xml not supported"))
    }
}

// pub(crate) struct SaveTxtOrBinUrlOrMarkup
// {
//     pub(crate) bytes: Vec<u8>,
//     pub(crate) extension: String,
// }

impl<'a, F> SerializerSaveTxtOrBinOrMarkup<'a, F>
    where F: FsWrite
{
    pub(crate) fn write_fs(&mut self, bytes: &[u8]) -> IoResult
    {
        self.fs.write_bytes(&self.path, bytes).map_err(|e| IoError::new(self.path.clone(), FileError::from(e)))
    }

    pub(crate) fn save(&mut self) -> IoResult
    {
        let markup = dispatch_serializer!(self, s => s.extract()).map_err(|e| IoError::new(self.path.clone(), FileError::from(e)))?;
        self.write_fs(markup.as_bytes())
    }
}

macro_rules! serialize_value {
    ($self:ident, $method:ident $(, $arg:expr)* $(,)?) => {{
        dispatch_serializer!(&mut $self, s =>
            match s.$method($($arg),*) {
                Ok(_) => {},
                Err(e) => Err(IoError::new($self.path.clone(), FileError::from_display(e)))?,
            }
        );
        $self.save()
    }};
}


macro_rules! dispatch_compound {
    ($self:expr, $method:ident $(, $arg:expr)*) => {{
        match $self.serializer.as_mut().unwrap() {
            SerializerMarkupOf::Ron(ser) => {
                let seq = ser.$method($($arg),*).map_err(|e| IoError::new($self.path.clone(), FileError::from_display(e)))?;
                Ok(SerializerSaveCompound {
                    fs: &mut $self.fs,
                    path: &$self.path,
                    param: &$self.param,
                    parent_should_save: &mut $self.should_save,
                    compound: SerializerMarkupOf::Ron(seq),
                    key: None,
                })
            },
            SerializerMarkupOf::Json(ser) => {
                let seq = ser.$method($($arg),*).map_err(|e| IoError::new($self.path.clone(), FileError::from_display(e)))?;
                Ok(SerializerSaveCompound {
                    fs: &mut $self.fs,
                    path: &$self.path,
                    param: &$self.param,
                    parent_should_save: &mut $self.should_save,
                    compound: SerializerMarkupOf::Json(seq),
                    key: None,
                })
            },
            SerializerMarkupOf::Xml(ser) => {
                let seq = ser.$method($($arg),*).map_err(|e| IoError::new($self.path.clone(), FileError::from_display(e)))?;
                Ok(SerializerSaveCompound {
                    fs: &mut $self.fs,
                    path: &$self.path,
                    param: &$self.param,
                    parent_should_save: &mut $self.should_save,
                    compound: SerializerMarkupOf::Xml(seq),
                    key: None,
                })
            },
        }
    }};
}

impl<'s, 'a, F> Serializer for &'s mut SerializerSaveTxtOrBinOrMarkup<'a, F>
    where F: FsWrite,
    'a: 's
{
    type Ok=();
    type Error=IoError;

    type SerializeSeq=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeSeq,
        <&'s mut SerializerJson as Serializer>::SerializeSeq,
        <&'s mut SerializerXml as Serializer>::SerializeSeq>;

    type SerializeTuple=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeTuple,
        <&'s mut SerializerJson as Serializer>::SerializeTuple,
        <&'s mut SerializerXml as Serializer>::SerializeTuple>;

    type SerializeTupleStruct=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeTupleStruct,
        <&'s mut SerializerJson as Serializer>::SerializeTupleStruct,
        <&'s mut SerializerXml as Serializer>::SerializeTupleStruct>;

    type SerializeTupleVariant=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeTupleVariant,
        <&'s mut SerializerJson as Serializer>::SerializeTupleVariant,
        <&'s mut SerializerXml as Serializer>::SerializeTupleVariant>;

    type SerializeMap=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeMap,
        <&'s mut SerializerJson as Serializer>::SerializeMap,
        <&'s mut SerializerXml as Serializer>::SerializeMap>;


    type SerializeStruct=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeStruct,
        <&'s mut SerializerJson as Serializer>::SerializeStruct,
        <&'s mut SerializerXml as Serializer>::SerializeStruct>;

    type SerializeStructVariant=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeStructVariant,
        <&'s mut SerializerJson as Serializer>::SerializeStructVariant,
        <&'s mut SerializerXml as Serializer>::SerializeStructVariant>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error>
    {
        serialize_value!(self, serialize_bool, value)
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_i8, value)
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_i16, value)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_i32, value)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_i64, value)
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_u8, value)
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_u16, value)
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_u32, value)
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_u64, value)
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_f32, value)
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_f64, value)
    }

    fn serialize_char(self, c: char) -> Result<Self::Ok, Self::Error>
    {
        // txt
        let mut buf = [0u8; 8]; // 4 is enought, I put 8 to be sure
        let bytes = c.encode_utf8(&mut buf).as_bytes();
        self.write_fs(bytes)
    }

    fn serialize_str(self, txt: &str) -> Result<Self::Ok, Self::Error>
    {
        // txt
        self.write_fs(txt.as_bytes())
    }

    fn serialize_bytes(self, bytes: &[u8]) -> Result<Self::Ok, Self::Error>
    {
        let url = BinUrlData::try_from(bytes).map_err(|e| IoError::new(self.path.clone(), EncodeError::from(e)))?;
        let bytes = url.data.to_owned();
        self.write_fs(&bytes)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_none)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        serialize_value!(self, serialize_some, value)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_unit_struct, name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        serialize_value!(self, serialize_unit_variant, name, variant_index, variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        serialize_value!(self, serialize_newtype_struct, name, value)
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
        serialize_value!(self, serialize_newtype_variant, name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>
    {
        dispatch_compound!(self, serialize_seq, len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        dispatch_compound!(self, serialize_tuple, len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        dispatch_compound!(self, serialize_tuple_struct, name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        dispatch_compound!(self, serialize_tuple_variant, name, variant_index, variant, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        dispatch_compound!(self, serialize_map, len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        dispatch_compound!(self, serialize_struct, name, len)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        dispatch_compound!(self, serialize_struct_variant, name, variant_index, variant, len)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}




