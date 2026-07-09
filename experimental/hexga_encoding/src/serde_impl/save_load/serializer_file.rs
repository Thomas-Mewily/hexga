use core::fmt;
use std::marker::PhantomData;

/*
use hexga_utils::non_empty_stack::NonEmptyStack;

use super::*;

pub type SerializerRon<T=WriteFmt2Io<Vec<u8>>> = ron::ser::Serializer<T>;
pub type SerializerJson<T=Vec<u8>> = serde_json::Serializer<T>;
//pub type SerializerXml<T=Vec<u8>> = serde_xml_rs::Serializer<T>;


#[doc(hidden)]
pub struct WriteFmt2Io<T> { writer :  T }
impl std::fmt::Write for WriteFmt2Io<Vec<u8>>
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result
    {
        self.writer.extend_from_slice(s.as_bytes());
        Ok(())
    }
}


pub trait FsWriteExtension : FsWrite
{
    fn serialize_with_flags<P: AsRef<Path>, T: ?Sized + Serialize>(&mut self, path: P, value: &T, flags: SerializerFlags) -> FileResult
    {

        SerializerFile
        {
            fs: todo!(),
            path: todo!(),
            flags,
            serializer: todo!(),
        }
    }
}
impl<Fs> FsWriteExtension for Fs where Fs: FsWrite {}
pub struct SerializerFile<'a,FS> //= SerializerFileOf<'a,FS,&'a mut Option<SerializerMarkup>>;
{
    fs: &'a mut FS,
    path : Cow<'a,Path>,
    flags : SerializerFlags,
    serializer : &'a mut SerializerMarkup,
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SerializerFlags
{
    pub multi_file: bool,
    pub human_readable : bool,
}



// #[derive(Serialize, Deserialize)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Markup<Ron,Json>
 {
    Ron(Ron),
    Json(Json),
}

pub type SerializerMarkup = Markup<SerializerRon,SerializerJson>;




impl SerializerMarkup
{
    pub fn into_inner(self) -> Vec<u8>
    {
        match self
        {
            SerializerMarkup::Ron(serializer) => serializer.into_inner().writer,
            SerializerMarkup::Json(serializer) => serializer.into_inner(),
            //SerializerMarkup::Xml(serializer) => serializer.into_inner(),
        }
    }

    pub fn new(format: FormatMarkup, bytes: Vec<u8>) -> Self
    {
        match format
        {
            FormatMarkup::Ron => Self::Ron(SerializerRon::new(WriteFmt2Io{writer: bytes}, Some(Default::default())).expect("failed to create ron serializer")),
            FormatMarkup::Json => Self::Json(SerializerJson::new(bytes)),
            _ => todo!()
            //FormatMarkup::Xml => Self::Xml(SerializerXml::new_from_writer(bytes)),
        }
    }
}


impl<'a,FS> SerializerFile<'a,FS>
    where FS: Fs
{
    fn save(&mut self) -> FileResult
    {
        self.fs.write_bytes_at(self.path, &self.serializer.take().unwrap().into_inner()).map_err(|e| FileError::new(e).with_path(Some(self.path.to_path_buf())))
    }
}
/*
impl<'a,FS,S> SerializerFileOf<'a,FS,S>
    where FS: Fs
{
    fn extract(self) -> (SerializerFileOf<'a,FS,()>, S)
    {
        let Self { fs, path, flags, serializer } = self;
        (
            SerializerFileOf
            {
                fs,
                path,
                flags,
                serializer : (),
            },
            serializer
        )
    }

    fn save(self, bytes: &[u8]) -> FileResult
    {
        self.fs.write_bytes_at(self.path, &bytes).map_err(|e| FileError::new(e).with_path(Some(self.path.to_path_buf())))
    }
}
*/
type SerializerOk = ();
type SerializerErr = FileError;

/*
impl<'a,FS> SerializeSeq for SerializerFile<'a,FS>
    where FS: Fs
{
    type Ok = SerializerOk;
    type Error = SerializerErr;

    fn serialize_element<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Default::default())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Err(Default::default()) }
}
impl<'a,FS> SerializeTuple for SerializerFile<'a,FS>
    where FS: Fs
{
    type Ok = SerializerOk;
    type Error = SerializerErr;

    fn serialize_element<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Default::default())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Err(Default::default()) }
}
impl<'a,FS> SerializeTupleStruct for SerializerFile<'a,FS>
    where FS: Fs
{
    type Ok = SerializerOk;
    type Error = SerializerErr;

    fn serialize_field<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Default::default())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Err(Default::default()) }
}
impl<'a,FS> SerializeTupleVariant for SerializerFile<'a,FS>
    where FS: Fs
{
    type Ok = SerializerOk;
    type Error = SerializerErr;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Default::default())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Err(Default::default()) }
}
impl<'a,FS> SerializeMap for SerializerFile<'a,FS>
    where FS: Fs
{
    type Ok = SerializerOk;
    type Error = SerializerErr;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Default::default())
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Default::default())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Err(Default::default()) }
}
impl<'a,FS> SerializeStruct for SerializerFile<'a,FS>
    where FS: Fs
{
    type Ok = SerializerOk;
    type Error = SerializerErr;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Default::default())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Err(Default::default()) }
}
impl<'a,FS> SerializeStructVariant for SerializerFile<'a,FS>
    where FS: Fs
{
    type Ok = SerializerOk;
    type Error = SerializerErr;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Default::default())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Err(Default::default()) }
}
*/

impl<'a,FS> SerializerFile<'a,FS>
    where FS: Fs
{
    fn encoded(&mut self, r: Result<(), EncodeError>) -> Result<(), FileError>
    {
        let path = self.path;
        match r
        {
            Ok(_) => self.save(),
            Err(e) => Err(FileError::new(FileErrorKind::Encode(e)).with_path(Some(path.to_path_buf()))),
        }
    }
}

macro_rules! dispatch_ser {
    ($self:expr, $method_name:ident $(, $value:expr)*) => {{
        match $self.serializer.as_mut().unwrap() {
            SerializerMarkup::Ron(serializer) => {
                serializer.$method_name($($value),*)
                    .map_err(|e| EncodeError::markup::<Self>(Extension::RON, e))
            }
            SerializerMarkup::Json(serializer) => {
                serializer.$method_name($($value),*)
                    .map_err(|e| EncodeError::markup::<Self>(Extension::JSON, e))
            }
            /*
            SerializerMarkup::Xml(serializer) => {
                serializer.$method_name($($value),*)
                    .map_err(|e| EncodeError::markup::<Self>(Extension::XML, e))
            }
            */
        }
    }};
}

macro_rules! impl_ser {
    ($self:expr, $method_name:ident $(, $value:expr)*) => {{
        let r = dispatch_ser!($self, $method_name $(, $value)*);
        $self.encoded(r)
    }};
}


impl<'a,FS> Serializer for &'a mut SerializerFile<'a,FS>
    where FS: Fs
{
    type Ok = SerializerOk;
    type Error = SerializerErr;

    type SerializeSeq = SerializerFileSeq<'a,FS>;
    type SerializeTuple = SerializerFileTuple<'a,FS>;
    type SerializeTupleStruct = SerializerFileTupleStruct<'a,FS>;
    type SerializeTupleVariant = SerializerFileTupleVariant<'a,FS>;
    type SerializeMap = SerializerFileMap<'a,FS>;
    type SerializeStruct = SerializerFileStruct<'a,FS>;
    type SerializeStructVariant = SerializerFileStructVariant<'a,FS>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_bool, v) }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_i8, v) }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_i16, v) }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_i32, v) }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_i64, v) }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_u8, v) }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_u16, v) }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_u32, v) }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_u64, v) }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_f32, v) }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_f64, v) }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_char, v) }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_str, v) }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> { 
        todo!("check for url / binary url (depending if binary is set here)");
        impl_ser!(self, serialize_bytes, v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_none) }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_unit) }
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> { impl_ser!(self, serialize_unit_struct, name) }
    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error>  { impl_ser!(self, serialize_unit_variant, name, variant_index, variant) }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize 
    {
        todo!()
    }
    fn serialize_some<T>(mut self, v: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + Serialize 
    { 
        todo!()
    }
    fn serialize_newtype_variant<T>(mut self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize
    {         
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> 
    {
        let seq = match self.serializer.as_mut().unwrap() {
            SerializerMarkup::Ron(ser) => {
                match ser.serialize_seq(len)
                {
                    Ok(s) => Ok(SerializerMarkupSeq::Ron(s)),
                    Err(e) => Err(FileError::new(EncodeError::markup::<Self>(Extension::RON, e)).with_path(Some(self.path.to_path_buf()))),
                }
            }
            SerializerMarkup::Json(ser) => 
            {
                match ser.serialize_seq(len)
                {
                    Ok(s) => Ok(SerializerMarkupSeq::Json(s)),
                    Err(e) => Err(FileError::new(EncodeError::markup::<Self>(Extension::JSON, e)).with_path(Some(self.path.to_path_buf()))),
                }
            }
        }?;

        Ok(
            SerializerFileSeq
            {
                file: self,
                seq,
            }
        )
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> { Err(Default::default()) }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { Err(Default::default()) }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error>
    {
        Err(Default::default())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { Err(Default::default()) }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> { Err(Default::default()) }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error>
    {
        Err(Default::default())
    }

    fn is_human_readable(&self) -> bool { self.flags.human_readable }
}





pub type SerializerRonSeq<'a,T=WriteFmt2Io<Vec<u8>>> = ::ron::ser::Compound<'a,T>;
pub type SerializerJsonSeq<'a,T=Vec<u8>> = ::serde_json::ser::Compound<'a,T,serde_json::ser::CompactFormatter>;
pub type SerializerMarkupSeq<'a> = Markup<SerializerRonSeq<'a>,SerializerJsonSeq<'a>>;

pub struct SerializerFileSeq<'a,FS>
{
    file : &'a mut SerializerFile<'a,FS>, 
    seq : SerializerMarkupSeq<'a>
}

//pub type SerializerFileSeq<'a,FS> = SerializerFileOf<'a,FS,(&'a mut SerializerFile<'a,FS>, SerializerMarkupSeq<'a>)>;
impl<'a,FS> SerializeSeq for SerializerFileSeq<'a,FS>
    where FS: Fs
{
    type Ok = SerializerOk;
    type Error=SerializerErr;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        match &mut self.serializer
        {
            Markup::Ron(s) => 
            {
                //SerializeSeq::serialize_element(s, value).map_err(|e| FileError::new(EncodeError::markup::<Self>(Extension::RON, e)).with_path(Some(self.path.to_path_buf()))),
                
            }
            Markup::Json(s) => SerializeSeq::serialize_element(s, value).map_err(|e| FileError::new(EncodeError::markup::<Self>(Extension::JSON, e)).with_path(Some(self.path.to_path_buf()))),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.serializer
        {
            Markup::Ron(s) => SerializeSeq::end(s).map_err(|e| FileError::new(EncodeError::markup::<Self>(Extension::RON, e)).with_path(Some(self.path.to_path_buf()))),
            Markup::Json(s) => SerializeSeq::end(s).map_err(|e| FileError::new(EncodeError::markup::<Self>(Extension::JSON, e)).with_path(Some(self.path.to_path_buf()))),
        }
    }
}


pub type SerializerRonTuple<'a,T=WriteFmt2Io<Vec<u8>>> = ::ron::ser::Compound<'a,T>;
pub type SerializerJsonTuple<'a,T=Vec<u8>> = ::serde_json::ser::Compound<'a,T,serde_json::ser::CompactFormatter>;
pub type SerializerMarkupTuple<'a> = Markup<SerializerRonTuple<'a>,SerializerJsonTuple<'a>>;
pub type SerializerFileTuple<'a,FS> = SerializerFileOf<'a,FS,SerializerMarkupTuple<'a>>;


pub type SerializerRonTupleStruct<'a,T=WriteFmt2Io<Vec<u8>>> = ::ron::ser::Compound<'a,T>;
pub type SerializerJsonTupleStruct<'a,T=Vec<u8>> = ::serde_json::ser::Compound<'a,T,serde_json::ser::CompactFormatter>;
pub type SerializerMarkupTupleStruct<'a> = Markup<SerializerRonTupleStruct<'a>,SerializerJsonTupleStruct<'a>>;
pub type SerializerFileTupleStruct<'a,FS> = SerializerFileOf<'a,FS,SerializerMarkupTupleStruct<'a>>;


pub type SerializerRonTupleVariant<'a,T=WriteFmt2Io<Vec<u8>>> = ::ron::ser::Compound<'a,T>;
pub type SerializerJsonTupleVariant<'a,T=Vec<u8>> = ::serde_json::ser::Compound<'a,T,serde_json::ser::CompactFormatter>;
pub type SerializerMarkupTupleVariant<'a> = Markup<SerializerRonTupleVariant<'a>,SerializerJsonTupleVariant<'a>>;
pub type SerializerFileTupleVariant<'a,FS> = SerializerFileOf<'a,FS,SerializerMarkupTupleVariant<'a>>;



pub type SerializerRonMap<'a,T=WriteFmt2Io<Vec<u8>>> = ::ron::ser::Compound<'a,T>;
pub type SerializerJsonMap<'a,T=Vec<u8>> = ::serde_json::ser::Compound<'a,T,serde_json::ser::CompactFormatter>;
pub type SerializerMarkupMap<'a> = Markup<SerializerRonMap<'a>,SerializerJsonMap<'a>>;
pub type SerializerFileMap<'a,FS> = SerializerFileOf<'a,FS,SerializerMarkupMap<'a>>;



pub type SerializerRonStruct<'a,T=WriteFmt2Io<Vec<u8>>> = ::ron::ser::Compound<'a,T>;
pub type SerializerJsonStruct<'a,T=Vec<u8>> = ::serde_json::ser::Compound<'a,T,serde_json::ser::CompactFormatter>;
pub type SerializerMarkupStruct<'a> = Markup<SerializerRonStruct<'a>,SerializerJsonStruct<'a>>;
pub type SerializerFileStruct<'a,FS> = SerializerFileOf<'a,FS,SerializerMarkupStruct<'a>>;



pub type SerializerRonStructVariant<'a,T=WriteFmt2Io<Vec<u8>>> = ::ron::ser::Compound<'a,T>;
pub type SerializerJsonStructVariant<'a,T=Vec<u8>> = ::serde_json::ser::Compound<'a,T,serde_json::ser::CompactFormatter>;
pub type SerializerMarkupStructVariant<'a> = Markup<SerializerRonStructVariant<'a>,SerializerJsonStructVariant<'a>>;
pub type SerializerFileStructVariant<'a,FS> = SerializerFileOf<'a,FS,SerializerMarkupStructVariant<'a>>;



*/