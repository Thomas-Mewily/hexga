use std::io::BufRead;

use crate::*;


#[cfg(feature = "serde_ron")]
pub trait MarkupExtensionRon : Sized + Serialize + for<'de> Deserialize<'de>
{
    fn to_ron (&self) -> IoResult<String> { ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::___()).map_err(|e| IoErrorKind::serialize::<Self>(Io::RON_EXTENSION, e.to_debug())) }
    fn from_ron_buf(buf : &[u8]) -> IoResult<Self> { Self::from_ron_with_reader(BufReader::new(buf))  }
    fn from_ron_with_reader<R : IoRead>(reader : R) -> IoResult<Self> { ron::de::from_reader(reader).map_err(|e| IoErrorKind::deserialize::<Self>(Io::RON_EXTENSION, e.to_debug()))  }
    fn from_ron    (ron : &str) -> IoResult<Self> { ron::de::from_str(ron).map_err(|e| IoErrorKind::deserialize::<Self>(Io::RON_EXTENSION, e.to_debug())) }
}
#[cfg(feature = "serde_ron")]
impl<T> MarkupExtensionRon for T where T : Sized + Serialize + for<'de> Deserialize<'de>{}


#[cfg(feature = "serde_json")]
pub trait MarkupExtensionJson : Sized + Serialize + for<'de> Deserialize<'de>
{
    fn to_json(&self) -> IoResult<String> { serde_json::ser::to_string_pretty(&self).map_err(|e| IoErrorKind::serialize::<Self>(Io::JSON_EXTENSION, e.to_debug())) }
    fn from_json_buf(buf : &[u8]) -> IoResult<Self> { Self::from_json_with_reader(BufReader::new(buf))  }
    fn from_json_with_reader<R : IoRead>(reader : R) -> IoResult<Self> { serde_json::de::from_reader(reader).map_err(|e| IoErrorKind::deserialize::<Self>(Io::JSON_EXTENSION, e.to_debug())) }
    fn from_json    (json : &str) -> IoResult<Self> { serde_json::de::from_str(json).map_err(|e| IoErrorKind::deserialize::<Self>(Io::JSON_EXTENSION, e.to_debug())) }
}
#[cfg(feature = "serde_json")]
impl<T> MarkupExtensionJson for T where T : Sized + Serialize + for<'de> Deserialize<'de>{}


#[cfg(feature = "serde_xml")]
pub trait MarkupExtensionXml : Sized + Serialize + for<'de> Deserialize<'de>
{
    fn to_xml (&self) -> IoResult<String> { quick_xml::se::to_string(&self).map_err(|e| IoErrorKind::serialize::<Self>(Io::XML_EXTENSION, e.to_debug())) }
    fn from_xml_buf(buf : &[u8]) -> IoResult<Self> { Self::from_xml_with_reader(BufReader::new(buf))  }
    fn from_xml_with_reader<R : BufRead>(reader : R) -> IoResult<Self> { quick_xml::de::from_reader(reader).map_err(|e| IoErrorKind::deserialize::<Self>(Io::XML_EXTENSION, e.to_debug())) }
    fn from_xml    (xml : &str) -> IoResult<Self> { quick_xml::de::from_str(xml).map_err(|e| IoErrorKind::deserialize::<Self>(Io::XML_EXTENSION, e.to_debug())) }
}
#[cfg(feature = "serde_xml")]
impl<T> MarkupExtensionXml for T where T : Sized + Serialize + for<'de> Deserialize<'de>{}


#[cfg(feature = "serde_quick_bin")]
pub trait MarkupExtensionQuickBin : Sized + Serialize + for<'de> Deserialize<'de>
{
    /// Ideal for temporary saving to a binary format
    ///
    /// Do not use it for long term storage. Implementation can change accross version
    fn to_quick_bin(&self) -> IoResult<Vec<u8>> { bincode::serialize(self).map_err(|e| IoErrorKind::serialize::<Self>(Io::QUICK_BIN_EXTENSION, e.to_debug())) }
    fn from_quick_bin_buf(quick_bin : &[u8]) -> IoResult<Self> { bincode::deserialize(quick_bin).map_err(|e| IoErrorKind::deserialize::<Self>(Io::QUICK_BIN_EXTENSION, e.to_debug())) }
    fn from_quick_bin_with_reader<R : IoRead>(reader : R) -> IoResult<Self> { bincode::deserialize_from(reader).map_err(|e| IoErrorKind::deserialize::<Self>(Io::XML_EXTENSION, e.to_debug())) }
}
#[cfg(feature = "serde_quick_bin")]
impl<T> MarkupExtensionQuickBin for T where T : Sized + Serialize + for<'de> Deserialize<'de>{}