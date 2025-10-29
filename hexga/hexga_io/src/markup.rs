use super::*;

pub use std::io::{Read,BufRead};

pub trait ToRon : Serialize
{
    fn to_ron(&self) -> EncodeResult<String> { ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default()).map_err(|e| EncodeError::markup::<Self>(Extensions::RON, e)) }
}
impl<T> ToRon for T where T: Serialize {}

pub trait FromRon : for<'de> Deserialize<'de>
{
    fn from_ron_buf(buf : &[u8]) -> EncodeResult<Self> { Self::from_ron_with_reader(BufReader::new(buf))  }
    fn from_ron_with_reader<R : Read>(reader : R) -> EncodeResult<Self> { ron::de::from_reader(reader).map_err(|e| EncodeError::markup::<Self>(Extensions::RON, e))  }
    fn from_ron    (ron : &str) -> EncodeResult<Self> { ron::de::from_str(ron).map_err(|e| EncodeError::markup::<Self>(Extensions::RON, e)) }
}
impl<T> FromRon for T where T: for<'de> Deserialize<'de> {}




pub trait ToJson : Serialize
{
    fn to_json(&self) -> EncodeResult<String> { serde_json::ser::to_string_pretty(&self).map_err(|e| EncodeError::markup::<Self>(Extensions::JSON, e)) }
}
impl<T> ToJson for T where T: Serialize {}

pub trait FromJson : for<'de> Deserialize<'de>
{
    fn from_json_buf(buf : &[u8]) -> EncodeResult<Self> { Self::from_json_with_reader(BufReader::new(buf))  }
    fn from_json_with_reader<R : Read>(reader : R) -> EncodeResult<Self> { serde_json::de::from_reader(reader).map_err(|e| EncodeError::markup::<Self>(Extensions::JSON, e)) }
    fn from_json    (json : &str) -> EncodeResult<Self> { serde_json::de::from_str(json).map_err(|e| EncodeError::markup::<Self>(Extensions::JSON, e)) }
}
impl<T> FromJson for T where T: for<'de> Deserialize<'de> {}




pub trait ToXml : Serialize
{
    fn to_xml(&self) -> EncodeResult<String> { serde_xml_rs::ser::to_string(&self).map_err(|e| EncodeError::markup::<Self>(Extensions::XML, e)) }
}
impl<T> ToXml for T where T: Serialize {}

pub trait FromXml : for<'de> Deserialize<'de>
{
    fn from_xml_buf(buf : &[u8]) -> EncodeResult<Self> { Self::from_xml_with_reader(BufReader::new(buf))  }
    fn from_xml_with_reader<R : BufRead>(reader : R) -> EncodeResult<Self> { serde_xml_rs::de::from_reader(reader).map_err(|e| EncodeError::markup::<Self>(Extensions::XML, e)) }
    fn from_xml    (xml : &str) -> EncodeResult<Self> { serde_xml_rs::de::from_str(xml).map_err(|e| EncodeError::markup::<Self>(Extensions::XML, e)) }
}
impl<T> FromXml for T where T: for<'de> Deserialize<'de> {}




pub trait ToQuickBin : Serialize
{
    /// Ideal for temporary saving to a binary format
    ///
    /// Do not use it for long term storage. Implementation can change accross version
    fn to_quick_bin(&self) -> EncodeResult<Vec<u8>> { bincode::serialize(self).map_err(|e| EncodeError::markup::<Self>(Extensions::QUICK_BIN, e)) }
}
impl<T> ToQuickBin for T where T: Serialize {}

pub trait FromQuickBin : for<'de> Deserialize<'de>
{
    fn from_quick_bin_buf(quick_bin : &[u8]) -> EncodeResult<Self> { bincode::deserialize(quick_bin).map_err(|e| EncodeError::markup::<Self>(Extensions::QUICK_BIN, e)) }
    fn from_quick_bin_with_reader<R : Read>(reader : R) -> EncodeResult<Self> { bincode::deserialize_from(reader).map_err(|e| EncodeError::markup::<Self>(Extensions::QUICK_BIN, e)) }
}
impl<T> FromQuickBin for T where T: for<'de> Deserialize<'de> {}