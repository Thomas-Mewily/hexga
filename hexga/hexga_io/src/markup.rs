use super::*;

pub use std::io::{Read,BufRead};

#[cfg(feature = "serde_ron")]
pub trait ToRon : Serialize
{
    fn to_ron(&self) -> IoResult<String> { ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default()).map_err(|e| IoError::markup_serializer::<Self>(Extensions::RON, format!("{:?}", e))) }
}
#[cfg(feature = "serde_ron")]
impl<T> ToRon for T where T: Serialize {}

#[cfg(feature = "serde_ron")]
pub trait FromRon : for<'de> Deserialize<'de>
{
    fn from_ron_buf(buf : &[u8]) -> IoResult<Self> { Self::from_ron_with_reader(BufReader::new(buf))  }
    fn from_ron_with_reader<R : Read>(reader : R) -> IoResult<Self> { ron::de::from_reader(reader).map_err(|e| IoError::markup_deserializer::<Self>(Extensions::RON, format!("{:?}", e)))  }
    fn from_ron    (ron : &str) -> IoResult<Self> { ron::de::from_str(ron).map_err(|e| IoError::markup_deserializer::<Self>(Extensions::RON, format!("{:?}", e))) }
}
#[cfg(feature = "serde_ron")]
impl<T> FromRon for T where T: for<'de> Deserialize<'de> {}




#[cfg(feature = "serde_json")]
pub trait ToJson : Serialize
{
    fn to_json(&self) -> IoResult<String> { serde_json::ser::to_string_pretty(&self).map_err(|e| IoError::markup_serializer::<Self>(Extensions::JSON, format!("{:?}", e))) }
}
#[cfg(feature = "serde_json")]
impl<T> ToJson for T where T: Serialize {}

#[cfg(feature = "serde_json")]
pub trait FromJson : for<'de> Deserialize<'de>
{
    fn from_json_buf(buf : &[u8]) -> IoResult<Self> { Self::from_json_with_reader(BufReader::new(buf))  }
    fn from_json_with_reader<R : Read>(reader : R) -> IoResult<Self> { serde_json::de::from_reader(reader).map_err(|e| IoError::markup_deserializer::<Self>(Extensions::JSON, format!("{:?}", e))) }
    fn from_json    (json : &str) -> IoResult<Self> { serde_json::de::from_str(json).map_err(|e| IoError::markup_deserializer::<Self>(Extensions::JSON, format!("{:?}", e))) }
}
#[cfg(feature = "serde_json")]
impl<T> FromJson for T where T: for<'de> Deserialize<'de> {}




/*
#[cfg(feature = "serde_xml")]
pub trait ToXml : Serialize
{
    fn to_xml(&self) -> IoResult<String> { quick_xml::se::to_string(&self).map_err(|e| IoError::markup_serializer::<Self>(Extensions::XML, format!("{:?}", e))) }
}
#[cfg(feature = "serde_xml")]
impl<T> ToXml for T where T: Serialize {}

#[cfg(feature = "serde_xml")]
pub trait FromXml : for<'de> Deserialize<'de>
{
    fn from_xml_buf(buf : &[u8]) -> IoResult<Self> { Self::from_xml_with_reader(BufReader::new(buf))  }
    fn from_xml_with_reader<R : BufRead>(reader : R) -> IoResult<Self> { quick_xml::de::from_reader(reader).map_err(|e| IoError::markup_deserializer::<Self>(Extensions::XML, format!("{:?}", e))) }
    fn from_xml    (xml : &str) -> IoResult<Self> { quick_xml::de::from_str(xml).map_err(|e| IoError::markup_deserializer::<Self>(Extensions::XML, format!("{:?}", e))) }
}
#[cfg(feature = "serde_xml")]
impl<T> FromXml for T where T: for<'de> Deserialize<'de> {}
*/




#[cfg(feature = "serde_quick_bin")]
pub trait ToQuickBin : Serialize
{
    /// Ideal for temporary saving to a binary format
    ///
    /// Do not use it for long term storage. Implementation can change accross version
    fn to_quick_bin(&self) -> IoResult<Vec<u8>> { bincode::serialize(self).map_err(|e| IoError::markup_serializer::<Self>(Extensions::QUICK_BIN, format!("{:?}", e))) }
}
#[cfg(feature = "serde_quick_bin")]
impl<T> ToQuickBin for T where T: Serialize {}

#[cfg(feature = "serde_quick_bin")]
pub trait FromQuickBin : for<'de> Deserialize<'de>
{
    fn from_quick_bin_buf(quick_bin : &[u8]) -> IoResult<Self> { bincode::deserialize(quick_bin).map_err(|e| IoError::markup_deserializer::<Self>(Extensions::QUICK_BIN, format!("{:?}", e))) }
    fn from_quick_bin_with_reader<R : Read>(reader : R) -> IoResult<Self> { bincode::deserialize_from(reader).map_err(|e| IoError::markup_deserializer::<Self>(Extensions::QUICK_BIN, format!("{:?}", e))) }
}
#[cfg(feature = "serde_quick_bin")]
impl<T> FromQuickBin for T where T: for<'de> Deserialize<'de> {}