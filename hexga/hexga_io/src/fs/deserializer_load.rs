use super::*;




pub(crate) struct DeserializerLoad<'de,Fs>
    where
        Fs: FsRead,
{
    pub(crate) fs: &'de mut Fs,
    pub(crate) path: Path,
    pub(crate) bytes: Option<Vec<u8>>
    // pub(crate) deserializer: DeserializerMarkup<'de>, //json: serde_json::de::Deserializer<Vec<u8>>
}



// pub(crate) enum DeserializerMarkup<'de>
// {
//     Ron(DeserializerRon<'de>),
//     Json(DeserializerJson),
//     Xml(DeserializerXml),
// }
pub(crate) type DeserializerRon<'de> = ron::Deserializer<'de>;
pub(crate) type DeserializerJson<'de> = serde_json::de::Deserializer<serde_json::de::SliceRead<'de>>;

impl<'de, Fs> DeserializerLoad<'de,Fs>
    where
    Fs: FsRead,
{
    pub(crate) fn new(fs: &'de mut Fs, path: Path) -> Self
    {
        let bytes = fs.read_bytes(&path).ok().map(|v| v.into_owned());
        Self { fs, path, bytes }
    }

    pub(crate) fn markup(&self) -> Option<MarkupLanguage>
    {
        MarkupLanguage::try_from(self.path.extension_or_empty()).ok()
    }
    pub(crate) fn markup_or_default(&self) -> MarkupLanguage
    {
        self.markup().unwrap_or_default()
    }

    pub(crate) fn from_fs<T>(&mut self) -> IoResult<T>
        where
        T: for<'de2> Deserialize<'de2>
    { self.from_fs_txt(false) }

    pub(crate) fn read_bytes(&mut self) -> IoResult<Cow<'_,[u8]>>
    {
        self.fs.read_bytes(&self.path).map_err(|e| IoError::new(self.path.clone(), e))
    }
    // pub(crate) fn read_string(&mut self) -> IoResult<String>
    // {
    //     self.fs.read_string(&self.path).map_err(|e| IoError::new(self.path.clone(), e))
    // }

    pub(crate) fn from_fs_txt<T>(&mut self, txt: bool) -> IoResult<T>
        where
        T: for<'de2> Deserialize<'de2>
    {
        let markup = self.markup();

        let bytes = self.read_bytes()?;
        let buf = bytes.as_ref();

        if let Some(m) = markup
        {
            return m.from_markup_buf(buf).map_err(|e| IoError::new(self.path.clone(), e));
        }
        if txt
        {
            if let Ok(txt) = str::from_utf8(&bytes)
            {
                if let Ok(t) = T::deserialize(DeserializerTxt{ txt  })
                {
                    return Ok(t);
                }
            }
        }
        for m in MarkupLanguage::ALL
        {
            match m.from_markup_buf(buf)
            {
                Ok(o) => return Ok(o),
                Err(_) => {},
            }
        }
        Err(IoError::new(self.path.clone(), EncodeError::custom("Can't guess markup extension")))
    }
}

macro_rules! dispatch {
    ($self:expr, $r:ident => $body:expr) => {{
        let markup = $self.markup_or_default();
        let bytes = $self
            .bytes
            .as_ref()
            .ok_or_else(|| IoError::new($self.path.clone(), FileError::NotFound))?;

        match markup {
            MarkupLanguage::Ron => {
                let mut $r = DeserializerRon::from_bytes(bytes.as_ref())
                    .map_err(|e| IoError::new(
                        $self.path.clone(),
                        EncodeError::Markup {
                            extension: "ron".to_owned(),
                            reason: e.to_string(),
                        },
                    ))?;
                $body.map_err(|e| IoError::new(
                    $self.path.clone(),
                    EncodeError::Markup {
                        extension: "ron".to_owned(),
                        reason: e.to_string(),
                    },
                ))
            }
            MarkupLanguage::Json => {
                let slice = serde_json::de::SliceRead::new(bytes);
                let mut $r = DeserializerJson::new(slice);
                $body.map_err(|e| IoError::new(
                    $self.path.clone(),
                    EncodeError::Markup {
                        extension: "json".to_owned(),
                        reason: e.to_string(),
                    },
                ))
            }
            // MarkupLanguage::Xml => todo!(),
        }
    }};
}


impl<'de, 'x, Fs> Deserializer<'de> for &'x mut DeserializerLoad<'de,Fs>
    where
        Fs: FsRead,
        'x: 'de
{
    type Error=IoError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_bool(self.from_fs()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_i8(self.from_fs()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_i16(self.from_fs()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_i32(self.from_fs()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_i64(self.from_fs()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_u8(self.from_fs()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_u16(self.from_fs()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_u32(self.from_fs()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_u64(self.from_fs()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_f32(self.from_fs()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_f64(self.from_fs()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_char(self.from_fs_txt(true)?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let txt : String = self.from_fs_txt(true)?;
        visitor.visit_str(&txt)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_string(self.from_fs_txt(true)?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_bytes(self.read_bytes()?.as_ref())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_byte_buf(self.read_bytes()?.into_owned())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        dispatch!(self, r => r.deserialize_option(visitor))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        dispatch!(self, r => r.deserialize_seq(visitor))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        dispatch!(self, r => r.deserialize_tuple(len, visitor))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        dispatch!(self, r => r.deserialize_tuple_struct(name, len, visitor))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        dispatch!(self, r => r.deserialize_map(visitor))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        dispatch!(self, r => r.deserialize_struct(name, fields, visitor))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        dispatch!(self, r => r.deserialize_enum(name, variants, visitor))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        dispatch!(self, r => r.deserialize_identifier(visitor))
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_unit()
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}