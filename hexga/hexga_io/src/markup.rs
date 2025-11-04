use super::*;

pub use std::io::{Read,BufRead};


pub mod prelude
{
    pub use super::
    {
        ToRon,FromRon,
        ToJson,FromJson,
        ToXml,FromXml,
        ToQuickBin,FromQuickBin
    };

    pub(crate) use super::MarkupOf;
}

pub(crate) enum MarkupOf<Ron,Json,Xml>
{
    Ron(Ron),
    Json(Json),
    Xml(Xml),
}


// #[derive(Serialize, Deserialize)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum MarkupLanguageSpecial
// {
//     Bin,
//     Txt,
// }
// impl MarkupLanguage
// {
//     pub const ALL: &'static [Self] = &[Self::Bin, Self::Txt];

//     pub fn extension(self) -> &'static str
//     {
//         match self
//         {
//             MarkupLanguage::Bin => Io::BIN,
//             MarkupLanguage::Txt => Io::TXT,
//         }
//     }

//     pub fn to_markup<T>(self, value: &T) -> EncodeResult<String>
//         where T: Serialize
//     {
//         match self
//         {
//             MarkupLanguage::Bin => value.to_ron(),
//             MarkupLanguage::Txt => value.to_json(),
//         }
//     }

//     pub fn from_markup<T>(self, markup: &str) -> EncodeResult<T>
//         where T: for<'de> Deserialize<'de>
//     {
//         match self
//         {
//             MarkupLanguage::Ron => T::from_ron(markup),
//             MarkupLanguage::Json => T::from_json(markup),
//             MarkupLanguage::Xml => T::from_xml(markup),
//         }
//     }

//     pub fn from_markup_buf<T>(self, buf: &[u8]) -> EncodeResult<T>
//         where
//         T: for<'de> Deserialize<'de>
//     {
//         match self
//         {
//             MarkupLanguage::Ron => T::from_ron_buf(buf),
//             MarkupLanguage::Json => T::from_json_buf(buf),
//             MarkupLanguage::Xml => T::from_xml_buf(buf),
//         }
//     }

//     pub fn from_fs<T, Fs>(self, fs: &mut Fs, path: &path) -> FileResult<T>
//         where
//         Fs: FsRead,
//         T: for<'de> Deserialize<'de>
//     {
//         let bytes = fs.read_bytes(path)?;
//         self.from_markup_buf(bytes.as_ref()).map_err(|e| e.into())
//     }
// }
// impl Display for MarkupLanguage
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }
// impl<'a> TryFrom<&'a str> for MarkupLanguage
// {
//     type Error=();
//     fn try_from(value: &'a str) -> Result<Self, Self::Error> {
//         match value
//         {
//             Io::RON => Ok(Self::Ron),
//             Io::JSON => Ok(Self::Json),
//             Io::XML => Ok(Self::Xml),
//             _ => Err(())
//         }
//     }
// }



#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MarkupLanguage
{
    #[default]
    Ron,
    Json,
    Xml,
}
impl MarkupLanguage
{
    pub const ALL: &'static [Self] = &[Self::Ron, Self::Json, Self::Xml];

    pub fn extension(self) -> &'static str
    {
        match self
        {
            MarkupLanguage::Ron => Io::RON,
            MarkupLanguage::Json => Io::JSON,
            MarkupLanguage::Xml => Io::XML,
        }
    }

    pub fn to_markup<T>(self, value: &T) -> EncodeResult<String>
        where T: Serialize
    {
        match self
        {
            MarkupLanguage::Ron => value.to_ron(),
            MarkupLanguage::Json => value.to_json(),
            MarkupLanguage::Xml => value.to_xml(),
        }
    }

    pub fn from_markup<T>(self, markup: &str) -> EncodeResult<T>
        where T: for<'de> Deserialize<'de>
    {
        match self
        {
            MarkupLanguage::Ron => T::from_ron(markup),
            MarkupLanguage::Json => T::from_json(markup),
            MarkupLanguage::Xml => T::from_xml(markup),
        }
    }

    pub fn from_markup_buf<T>(self, buf: &[u8]) -> EncodeResult<T>
        where
        T: for<'de> Deserialize<'de>
    {
        match self
        {
            MarkupLanguage::Ron => T::from_ron_buf(buf),
            MarkupLanguage::Json => T::from_json_buf(buf),
            MarkupLanguage::Xml => T::from_xml_buf(buf),
        }
    }

    pub fn from_fs<T, Fs>(self, fs: &mut Fs, path: &path) -> FileResult<T>
        where
        Fs: FsRead,
        T: for<'de> Deserialize<'de>
    {
        let bytes = fs.read_bytes(path)?;
        self.from_markup_buf(bytes.as_ref()).map_err(|e| e.into())
    }
}
impl Display for MarkupLanguage
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<'a> TryFrom<&'a str> for MarkupLanguage
{
    type Error=();
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value
        {
            Io::RON => Ok(Self::Ron),
            Io::JSON => Ok(Self::Json),
            Io::XML => Ok(Self::Xml),
            _ => Err(())
        }
    }
}

pub trait ToRon : Serialize
{
    fn to_ron(&self) -> EncodeResult<String> { ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default()).map_err(|e| EncodeError::markup::<Self>(Io::RON, e)) }
}
impl<T> ToRon for T where T: Serialize {}

pub trait FromRon : for<'de> Deserialize<'de>
{
    fn from_ron_buf(buf : &[u8]) -> EncodeResult<Self> { Self::from_ron_with_reader(BufReader::new(buf))  }
    fn from_ron_with_reader<R : Read>(reader : R) -> EncodeResult<Self> { ron::de::from_reader(reader).map_err(|e| EncodeError::markup::<Self>(Io::RON, e))  }
    fn from_ron    (ron : &str) -> EncodeResult<Self> { ron::de::from_str(ron).map_err(|e| EncodeError::markup::<Self>(Io::RON, e)) }
}
impl<T> FromRon for T where T: for<'de> Deserialize<'de> {}




pub trait ToJson : Serialize
{
    fn to_json(&self) -> EncodeResult<String> { serde_json::ser::to_string_pretty(&self).map_err(|e| EncodeError::markup::<Self>(Io::JSON, e)) }
}
impl<T> ToJson for T where T: Serialize {}

pub trait FromJson : for<'de> Deserialize<'de>
{
    fn from_json_buf(buf : &[u8]) -> EncodeResult<Self> { Self::from_json_with_reader(BufReader::new(buf))  }
    fn from_json_with_reader<R : Read>(reader : R) -> EncodeResult<Self> { serde_json::de::from_reader(reader).map_err(|e| EncodeError::markup::<Self>(Io::JSON, e)) }
    fn from_json    (json : &str) -> EncodeResult<Self> { serde_json::de::from_str(json).map_err(|e| EncodeError::markup::<Self>(Io::JSON, e)) }
}
impl<T> FromJson for T where T: for<'de> Deserialize<'de> {}




pub trait ToXml : Serialize
{
    fn to_xml(&self) -> EncodeResult<String> { serde_xml_rs::ser::to_string(&self).map_err(|e| EncodeError::markup::<Self>(Io::XML, e)) }
}
impl<T> ToXml for T where T: Serialize {}

pub trait FromXml : for<'de> Deserialize<'de>
{
    fn from_xml_buf(buf : &[u8]) -> EncodeResult<Self> { Self::from_xml_with_reader(BufReader::new(buf))  }
    fn from_xml_with_reader<R : BufRead>(reader : R) -> EncodeResult<Self> { serde_xml_rs::de::from_reader(reader).map_err(|e| EncodeError::markup::<Self>(Io::XML, e)) }
    fn from_xml    (xml : &str) -> EncodeResult<Self> { serde_xml_rs::de::from_str(xml).map_err(|e| EncodeError::markup::<Self>(Io::XML, e)) }
}
impl<T> FromXml for T where T: for<'de> Deserialize<'de> {}




pub trait ToQuickBin : Serialize
{
    /// Ideal for temporary saving to a binary format
    ///
    /// Do not use it for long term storage. Implementation can change accross version
    fn to_quick_bin(&self) -> EncodeResult<Vec<u8>> { bincode::serialize(self).map_err(|e| EncodeError::markup::<Self>(Io::BIN, e)) }
}
impl<T> ToQuickBin for T where T: Serialize {}

pub trait FromQuickBin : for<'de> Deserialize<'de>
{
    fn from_quick_bin_buf(quick_bin : &[u8]) -> EncodeResult<Self> { bincode::deserialize(quick_bin).map_err(|e| EncodeError::markup::<Self>(Io::BIN, e)) }
    fn from_quick_bin_with_reader<R : Read>(reader : R) -> EncodeResult<Self> { bincode::deserialize_from(reader).map_err(|e| EncodeError::markup::<Self>(Io::BIN, e)) }
}
impl<T> FromQuickBin for T where T: for<'de> Deserialize<'de> {}