use super::*;


#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum ExtensionParam
{
    WithExtension(String),
    GuessIt{replace_it:bool}
}
impl Default for ExtensionParam
{
    fn default() -> Self {
        Self::GuessIt { replace_it: true }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
pub enum MultiFile
{
    Yes,
    #[default]
    No,
    /// Mimic the file system. If a folder exist, use it
    Mimic,
}
impl MultiFile
{
    pub const fn is_yes(self) -> bool { matches!(self, MultiFile::Yes) }
    pub const fn is_no(self) -> bool { matches!(self, MultiFile::No) }
    pub const fn is_mimic(self) -> bool { matches!(self, MultiFile::Mimic) }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct SaveParam
{
    /// Will use multi file
    pub multi_file: bool,

    // TODO: change the bool for an enum { Yes, No, Same } (Same = if there is a file save it in a file)

    /// Will map be expended into multiple file (if multi_file is true)
    pub multi_file_map: MultiFile,
    /// Will struct be expended into multiple file (if multi_file is true)
    pub multi_file_struct: MultiFile,

    /// Do char and string will be saved in a .txt file ?
    pub text_to_txt: bool,
    /// Do &[u8] will be saved in a .bin file ?
    pub bytes_to_bin: bool,

    pub extension_param: ExtensionParam,

    pub markup: MarkupLanguage,

    // #[serde(borrow)]
    // pub indent: &'static str,
    // #[serde(borrow)]
    // pub separator: &'static str,
    // pub capacity : usize,
}

impl Default for SaveParam
{
    fn default() -> Self {
        Self
        {
            multi_file: false,
            multi_file_map: MultiFile::Yes,
            multi_file_struct: MultiFile::No,
            extension_param: ExtensionParam::default(),
            text_to_txt: true,
            bytes_to_bin: true,
            markup: MarkupLanguage::default()
        }
    }
}
impl SaveParam
{
    pub fn with_extension_param(self, extension_param: ExtensionParam) -> Self { Self { extension_param, ..self }}
    pub fn with_extension(self, extension : impl Into<Extension>) -> Self { Self { extension_param: ExtensionParam::WithExtension(extension.into()), ..self}}
    pub fn with_guess_extension(self, replace_it: bool) -> Self { Self { extension_param: ExtensionParam::GuessIt{ replace_it }, ..self}}
    pub fn with_multi_file(self, multi_file: bool) -> Self { Self { multi_file, ..self }}
    pub fn with_multi_file_map(self, multi_file_map: MultiFile) -> Self { Self { multi_file_map, ..self }}
    pub fn with_multi_file_struct(self, multi_file_struct: MultiFile) -> Self { Self { multi_file_struct, ..self }}
    pub fn with_text_to_txt(self, text_to_txt: bool) -> Self { Self { text_to_txt, ..self }}
    pub fn with_bytes_to_bin(self, bytes_to_bin: bool) -> Self { Self { bytes_to_bin, ..self }}
}


pub(crate) struct SerializerSave<'a, F>
    where
    F: FsWrite,
{
    pub(crate) fs: &'a mut F,
    pub(crate) should_save: bool,
    pub(crate) path: Path,
    pub(crate) serializer: SerializerMarkup,
    pub(crate) param: SaveParam,
}

impl<'a, F> SerializerSave<'a, F>
    where
    F: FsWrite,
{
    pub(crate) fn new(fs: &'a mut F, path: Path, mut param: SaveParam) -> Self
    {
        let capacity = 1024;

        let extension = match &param.extension_param
        {
            ExtensionParam::WithExtension(ext) => ext,
            ExtensionParam::GuessIt { replace_it:_ } => path.extension_or_empty(),
        };

        if let Ok(markup) = MarkupLanguage::try_from(extension)
        {
            param.markup = markup;
        }

        let ser = match param.markup
        {
            MarkupLanguage::Ron => SerializerMarkup::Ron(SerializerRon::new_serializer(capacity)),
            MarkupLanguage::Json => SerializerMarkup::Json(SerializerJson::new_serializer(capacity)),
            // MarkupLanguage::Xml => SerializerMarkup::Xml(SerializerXml::new_serializer(capacity)),
        };

        Self::new_full(fs, path, param, ser)
    }

    pub(crate) fn new_full(fs: &'a mut F, path: Path, param: SaveParam, serializer: SerializerMarkup) -> Self
    {
        Self { fs, should_save: true, path, serializer: serializer, param }
    }
}



#[doc(hidden)]
pub(crate) struct SerializerSaveCompound<'a, F, Ron,Json>
    where
    F: FsWrite,
{
    fs: &'a mut F,
    path: &'a Path,
    param: &'a SaveParam,
    parent_should_save: &'a mut bool,
    compound : Markup<Ron,Json>,
    key: Option<Key>,
}

pub(crate) type SerializerMarkup = Markup<SerializerRon,SerializerJson>;
// pub(crate) type SerializerXml = serde_xml_rs::ser::Serializer<Vec<u8>>;


macro_rules! dispatch_serializer {
    // mutable borrow
    (&mut $self:expr, $s:pat  => $body:expr) => {
        match &mut $self.serializer {
            Markup::Ron($s) => $body,
            Markup::Json($s) => $body,
        }
    };

    // by value (move)
    ($self:expr, $s:pat  => $body:expr) => {
        match $self.serializer {
            Markup::Ron($s) => $body,
            Markup::Json($s) => $body,
        }
    };
}

macro_rules! dispatch_compound_serializer {
    // mutable borrow
    (&mut $self:expr, $s:pat  => $body:expr) => {
        match &mut $self.compound {
            Markup::Ron($s) => $body,
            Markup::Json($s) => $body,
        }
    };

    // by value (move)
    ($self:expr, $s:pat  => $body:expr) => {
        match $self.compound {
            Markup::Ron($s) => $body,
            Markup::Json($s) => $body,
        }
    };
}


impl<'a, F, Ron,Json> SerializeTuple for SerializerSaveCompound<'a,F,Ron,Json>
    where
    F: FsWrite,
    Ron: SerializeTuple,
    Json: SerializeTuple,
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
impl<'a, F, Ron,Json> SerializeSeq for SerializerSaveCompound<'a,F,Ron,Json>
    where
    F: FsWrite,
    Ron: SerializeSeq,
    Json: SerializeSeq,
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
impl<'a, F, Ron,Json> SerializeTupleStruct for SerializerSaveCompound<'a,F,Ron,Json>
    where
    F: FsWrite,
    Ron: SerializeTupleStruct,
    Json: SerializeTupleStruct,
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
impl<'a, F, Ron,Json> SerializeTupleVariant for SerializerSaveCompound<'a,F,Ron,Json>
    where
    F: FsWrite,
    Ron: SerializeTupleVariant,
    Json: SerializeTupleVariant,
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
impl<'a, F, Ron,Json> SerializeMap for SerializerSaveCompound<'a,F,Ron,Json>
    where
    F: FsWrite,
    Ron: SerializeMap,
    Json: SerializeMap,
{
    type Ok=();
    type Error=IoError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        // TODO: handle MultiFile::Mimic in self.param.multi_file_map
        if self.param.multi_file && self.param.multi_file_map.is_yes()
        {
            if let Ok(key) = key.serialize(IdentifierSerializer)
            {
                self.key = Some(key);
                return Ok(());
            }
        }
        dispatch_compound_serializer!(&mut self, s => s.serialize_key(key).map_err(|e| IoError::new(self.path, FileError::from_display(e))))
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        if let Some(k) = self.key.take()
        {
            let path = (self.path.without_extension() / k.clone().to_string()).with_extension(self.path.extension_or_empty());
            match self.fs.save_with_param(&value, path, self.param.clone())
            {
                Ok(o) => return Ok(o),
                Err(_) =>
                {
                    *self.parent_should_save = true;
                    match k
                    {
                        Key::String(k) => dispatch_compound_serializer!(&mut self, s => s.serialize_key(&k).map_err(|e| IoError::new(self.path, FileError::from_display(e)))?),
                        Key::Char(k) => dispatch_compound_serializer!(&mut self, s => s.serialize_key(&k).map_err(|e| IoError::new(self.path, FileError::from_display(e)))?),
                    }
                },
            }
        }
        *self.parent_should_save = true;
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
impl<'a, F, Ron,Json> SerializeStruct for SerializerSaveCompound<'a,F,Ron,Json>
    where
    F: FsWrite,
    Ron: SerializeStruct,
    Json: SerializeStruct,
{
    type Ok=();
    type Error=IoError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where
        T: ?Sized + Serialize
    {
        // TODO: handle MultiFile::Mimic in self.param.multi_file_map
        if self.param.multi_file && self.param.multi_file_struct.is_yes()
        {
            if let Ok(k) = key.serialize(IdentifierSerializer)
            {
                let path = (self.path.without_extension() / k.to_string()).with_extension(self.path.extension_or_empty());
                match self.fs.save_with_param(value, path, self.param.clone())
                {
                    Ok(o) => return Ok(o),
                    Err(_) => {},
                }
            }
        }
        *self.parent_should_save = true;
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
impl<'a, F, Ron,Json> SerializeStructVariant for SerializerSaveCompound<'a,F,Ron,Json>
    where
    F: FsWrite,
    Ron: SerializeStruct,
    Json: SerializeStruct,
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


impl MarkupSerializer for SerializerRon
{
    const EXTENSION : &'static str = Io::RON;

    fn new_serializer(capacity: usize) -> Self
    {
        SerializerRon::new(String::with_capacity(capacity), Some(Default::default())).unwrap()
    }
    fn extract(self) -> EncodeResult<String>
    {
        todo!("into_inner() when it will be impl 4 ron")
    }
}
impl MarkupSerializer for SerializerJson
{
    const EXTENSION : &'static str = Io::JSON;

    fn new_serializer(capacity: usize) -> Self
    {
        SerializerJson::new(Vec::with_capacity(capacity))
    }

    fn extract(self) -> EncodeResult<String>
    {
        String::try_from(self.into_inner()).map_err(|e| e.into())
    }
}
// impl MarkupSerializer for SerializerXml
// {
//     const EXTENSION : &'static str = Io::XML;

//     fn new_serializer(capacity: usize) -> Self
//     {
//         SerializerXml::new_from_writer(Vec::with_capacity(capacity))
//     }
//     fn extract(self) -> EncodeResult<String> {
//         Err(EncodeError::custom("xml not supported"))
//     }
// }

// pub(crate) struct SaveTxtOrBinUrlOrMarkup
// {
//     pub(crate) bytes: Vec<u8>,
//     pub(crate) extension: String,
// }

impl<'a, F> SerializerSave<'a, F>
    where F: FsWrite
{
    fn final_path(path: &path, param: &SaveParam, deduced_extension: Option<&str>) -> IoResult<Path>
    {
        match &param.extension_param
        {
            ExtensionParam::WithExtension(ext) => Ok(path.with_extension(&ext)),
            ExtensionParam::GuessIt { replace_it } => if *replace_it
            {
                match &deduced_extension
                {
                    Some(ext) => Ok(path.with_extension(&ext)),
                    None =>
                    {
                        if path.extension().is_some()
                        {
                            Ok(path.to_owned())
                        }
                        else
                        {
                            Err(IoError::new(path, EncodeError::custom("Unable to guess the file extension")))
                        }
                    }
                }
            }else
            {
                Ok(path.to_owned())
            }
        }
    }

    pub(crate) fn write_fs(&mut self, bytes: &[u8], deduced_extension: Option<&str>) -> IoResult
    {
        self.should_save = false;

        let path = Self::final_path(&self.path, &self.param, deduced_extension)?;
        self.fs.write_bytes(&path, bytes).map_err(|e| IoError::new(path.clone(), FileError::from(e)))
    }

    pub(crate) fn save(self) -> IoResult
    {
        if !self.should_save { return Ok(()); }

        let extension = match self.serializer
        {
            Markup::Ron(_) => SerializerRon::EXTENSION,
            Markup::Json(_) => SerializerJson::EXTENSION,
            // Markup::Xml(_) => SerializerXml::EXTENSION,
        };

        let markup = dispatch_serializer!(self, s => s.extract()).map_err(|e| IoError::new(self.path.clone(), FileError::from(e)))?;
        let mut path = Self::final_path(&self.path, &self.param, None)?;

        let path_dir = path.without_extension();
        if self.fs.is_directory(&path_dir)
        {
            path = (path_dir / keyword::MOD).with_extension(extension);
        }
        self.fs.write_bytes(&path, markup.as_bytes()).map_err(|e| IoError::new(self.path, e))
    }
}


macro_rules! serialize_value
{
    ($self:ident, $method:ident $(, $arg:expr)* $(,)?) => {{
        dispatch_serializer!(&mut $self, s =>
            match s.$method($($arg),*) {
                Ok(_) => Ok(()),
                Err(e) => Err(IoError::new($self.path.clone(), FileError::from_display(e))),
            }
        )
    }};
}


macro_rules! dispatch_compound {
    ($self:expr, $method:ident $(, $arg:expr)*) => {{
        match &mut $self.serializer {
            Markup::Ron(ser) => {
                let seq = ser.$method($($arg),*).map_err(|e| IoError::new($self.path.clone(), FileError::from_display(e)))?;
                Ok(SerializerSaveCompound {
                    fs: &mut $self.fs,
                    path: &$self.path,
                    param: &$self.param,
                    parent_should_save: &mut $self.should_save,
                    compound: Markup::Ron(seq),
                    key: None,
                })
            },
            Markup::Json(ser) => {
                let seq = ser.$method($($arg),*).map_err(|e| IoError::new($self.path.clone(), FileError::from_display(e)))?;
                Ok(SerializerSaveCompound {
                    fs: &mut $self.fs,
                    path: &$self.path,
                    param: &$self.param,
                    parent_should_save: &mut $self.should_save,
                    compound: Markup::Json(seq),
                    key: None,
                })
            },
            // Markup::Xml(ser) => {
            //     let seq = ser.$method($($arg),*).map_err(|e| IoError::new($self.path.clone(), FileError::from_display(e)))?;
            //     Ok(SerializerSaveCompound {
            //         fs: &mut $self.fs,
            //         path: &$self.path,
            //         param: &$self.param,
            //         parent_should_save: &mut $self.should_save,
            //         compound: Markup::Xml(seq),
            //         key: None,
            //     })
            // },
        }
    }};
}

impl<'s, 'a, F> Serializer for &'s mut SerializerSave<'a, F>
    where F: FsWrite,
    'a: 's
{
    type Ok=();
    type Error=IoError;

    type SerializeSeq=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeSeq,
        <&'s mut SerializerJson as Serializer>::SerializeSeq,
        // <&'s mut SerializerXml as Serializer>::SerializeSeq
        >;

    type SerializeTuple=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeTuple,
        <&'s mut SerializerJson as Serializer>::SerializeTuple,
        // <&'s mut SerializerXml as Serializer>::SerializeTuple
        >;

    type SerializeTupleStruct=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeTupleStruct,
        <&'s mut SerializerJson as Serializer>::SerializeTupleStruct,
        // <&'s mut SerializerXml as Serializer>::SerializeTupleStruct
        >;

    type SerializeTupleVariant=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeTupleVariant,
        <&'s mut SerializerJson as Serializer>::SerializeTupleVariant,
        // <&'s mut SerializerXml as Serializer>::SerializeTupleVariant
        >;

    type SerializeMap=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeMap,
        <&'s mut SerializerJson as Serializer>::SerializeMap,
        // <&'s mut SerializerXml as Serializer>::SerializeMap
        >;


    type SerializeStruct=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeStruct,
        <&'s mut SerializerJson as Serializer>::SerializeStruct,
        // <&'s mut SerializerXml as Serializer>::SerializeStruct
        >;

    type SerializeStructVariant=SerializerSaveCompound<'s,F,
        <&'s mut SerializerRon as Serializer>::SerializeStructVariant,
        <&'s mut SerializerJson as Serializer>::SerializeStructVariant,
        // <&'s mut SerializerXml as Serializer>::SerializeStructVariant
        >;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
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
        if !self.param.text_to_txt
        {
            return serialize_value!(self, serialize_char, c);
        }
        let mut buf = [0u8; 8]; // 4 is enought, I put 8 to be sure
        let bytes = c.encode_utf8(&mut buf).as_bytes();
        self.write_fs(bytes, Some("txt"))
    }

    fn serialize_str(self, txt: &str) -> Result<Self::Ok, Self::Error>
    {
        if !self.param.text_to_txt
        {
            return serialize_value!(self, serialize_str, txt);
        }
        self.write_fs(txt.as_bytes(), Some("txt"))
    }

    fn serialize_bytes(self, bytes: &[u8]) -> Result<Self::Ok, Self::Error>
    {
        self.should_save = false;
        if let Ok(url) = BinUrlData::try_from(bytes)
        {
            let bytes = url.data.to_owned();
            return self.write_fs(&bytes, Some(url.extension));
        }
        if !self.param.bytes_to_bin
        {
            return serialize_value!(self, serialize_bytes, bytes);
        }
        self.write_fs(bytes, Some("bin"))
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

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error>
    {
        // TODO: handle MultiFile::Mimic in self.param.multi_file_map
        self.should_save = !(self.param.multi_file && self.param.multi_file_map.is_yes());
        dispatch_compound!(self, serialize_map, len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>
    {
        // TODO: handle MultiFile::Mimic in self.param.multi_file_map
        self.should_save = !(self.param.multi_file && self.param.multi_file_struct.is_yes());
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



#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Key
{
    String(String),
    Char(char),
}
impl Key
{
    fn to_string(self) -> String
    {
        match self
        {
            Key::String(s) => s,
            Key::Char(c) => c.to_string(),
        }
    }
}


pub(crate) struct IdentifierSerializer;
pub struct IdentifierSerializerError;

impl std::fmt::Debug for IdentifierSerializerError
{
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl std::fmt::Display for IdentifierSerializerError
{
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl std::error::Error for IdentifierSerializerError
{

}
impl serde::ser::Error for IdentifierSerializerError
{
    fn custom<T>(_msg:T) -> Self where T:Display {
        IdentifierSerializerError
    }
}

impl SerializeSeq for IdentifierSerializer
{
    type Ok=Key;
    type Error=IdentifierSerializerError;

    fn serialize_element<T>(&mut self, _: &T) -> Result<(), Self::Error>
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
    type Ok=Key;
    type Error=IdentifierSerializerError;

    fn serialize_element<T>(&mut self, _: &T) -> Result<(), Self::Error>
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
    type Ok=Key;
    type Error=IdentifierSerializerError;

    fn serialize_field<T>(&mut self, _: &T) -> Result<(), Self::Error>
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
    type Ok=Key;
    type Error=IdentifierSerializerError;

    fn serialize_field<T>(&mut self, _: &T) -> Result<(), Self::Error>
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
    type Ok=Key;
    type Error=IdentifierSerializerError;

    fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
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
    type Ok=Key;
    type Error=IdentifierSerializerError;

    fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
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
    type Ok=Key;
    type Error=IdentifierSerializerError;

    fn serialize_field<T>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
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
    type Ok=Key;
    type Error=IdentifierSerializerError;

    type SerializeSeq=IdentifierSerializer;
    type SerializeTuple=IdentifierSerializer;
    type SerializeTupleStruct=IdentifierSerializer;
    type SerializeTupleVariant=IdentifierSerializer;
    type SerializeMap=IdentifierSerializer;
    type SerializeStruct=IdentifierSerializer;
    type SerializeStructVariant=IdentifierSerializer;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Key::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>
    {
        if v == keyword::MOD || v == keyword::PARAM
        {
            Err(IdentifierSerializerError)
        }else
        {
            Ok(Key::String(v.to_owned()))
        }
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize {
        Err(IdentifierSerializerError)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(IdentifierSerializerError)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(IdentifierSerializerError)
    }
}


// IDEA: don't hardcode the format ?
// use https://crates.io/crates/erased-serde to have &dyn Serializer
// but right now it is impossible to have &dyn Deserializer, so don't do it