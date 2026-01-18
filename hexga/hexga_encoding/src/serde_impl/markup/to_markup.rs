use super::*;


pub trait ToRon: Serialize
{
    fn to_ron(&self) -> EncodeResult<String> { ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default()).map_err(|e| EncodeError::markup::<Self>(Extension::RON, e)) }
    #[allow(unused_mut)] // weird LSP error
    fn to_ron_with_writer<W: Write>(&self, mut writer: W) -> EncodeResult { ron::ser::to_writer_pretty(writer.to_fmt_writer(), &self, ron::ser::PrettyConfig::default()).map_err(|e| EncodeError::markup::<Self>(Extension::RON, e)) }
}
impl<T> ToRon for T where T: Serialize {}

pub trait FromRon: for<'de> Deserialize<'de>
{
    fn from_ron_bytes(bytes: &[u8]) -> EncodeResult<Self> { Self::from_ron_with_reader(BufReader::new(bytes))  }
    fn from_ron_with_reader<R: Read>(reader: R) -> EncodeResult<Self> { ron::de::from_reader(reader).map_err(|e| EncodeError::markup::<Self>(Extension::RON, e))  }
    fn from_ron(ron: &str) -> EncodeResult<Self> { ron::de::from_str(ron).map_err(|e| EncodeError::markup::<Self>(Extension::RON, e)) }
}
impl<T> FromRon for T where T: for<'de> Deserialize<'de> {}




pub trait ToJson: Serialize
{
    fn to_json(&self) -> EncodeResult<String> { serde_json::ser::to_string_pretty(&self).map_err(|e| EncodeError::markup::<Self>(Extension::JSON, e)) }
    fn to_json_with_writer<W: Write>(&self, writer: W) -> EncodeResult { serde_json::ser::to_writer_pretty(writer, &self).map_err(|e| EncodeError::markup::<Self>(Extension::JSON, e)) }
}
impl<T> ToJson for T where T: Serialize {}

pub trait FromJson: for<'de> Deserialize<'de>
{
    fn from_json_bytes(bytes: &[u8]) -> EncodeResult<Self> { Self::from_json_with_reader(BufReader::new(bytes))  }
    fn from_json_with_reader<R: Read>(reader: R) -> EncodeResult<Self> { serde_json::de::from_reader(reader).map_err(|e| EncodeError::markup::<Self>(Extension::JSON, e)) }
    fn from_json    (json: &str) -> EncodeResult<Self> { serde_json::de::from_str(json).map_err(|e| EncodeError::markup::<Self>(Extension::JSON, e)) }
}
impl<T> FromJson for T where T: for<'de> Deserialize<'de> {}




pub trait ToXml: Serialize
{
    fn to_xml(&self) -> EncodeResult<String> { serde_xml_rs::ser::to_string(&self).map_err(|e| EncodeError::markup::<Self>(Extension::XML, e)) }
    fn to_xml_with_writer<W: Write>(&self, writer: W) -> EncodeResult { serde_xml_rs::ser::to_writer(writer, &self).map_err(|e| EncodeError::markup::<Self>(Extension::XML, e)) }
}
impl<T> ToXml for T where T: Serialize {}

pub trait FromXml: for<'de> Deserialize<'de>
{
    fn from_xml_bytes(bytes: &[u8]) -> EncodeResult<Self> { Self::from_xml_with_reader(BufReader::new(bytes))  }
    fn from_xml_with_reader<R: Read>(reader: R) -> EncodeResult<Self> { serde_xml_rs::de::from_reader(reader).map_err(|e| EncodeError::markup::<Self>(Extension::XML, e)) }
    fn from_xml(xml: &str) -> EncodeResult<Self> { serde_xml_rs::de::from_str(xml).map_err(|e| EncodeError::markup::<Self>(Extension::XML, e)) }
}
impl<T> FromXml for T where T: for<'de> Deserialize<'de> {}


/// TODO: replace bincode by [postcard](https://docs.rs/postcard/latest/postcard/) ?

/// Intended for short-term storage of data in a binary format, such as during data transfer.
///
/// Not suitable for long-term storage, as the implementation or encoding may change at any time.
pub trait ToTmpBin: Serialize
{
    /// Intended for short-term storage of data in a binary format, such as during data transfer.
    ///
    /// Not suitable for long-term storage, as the implementation or encoding may change at any time.
    fn to_tmp_bin(&self) -> EncodeResult<Vec<u8>> { bincode::serialize(self).map_err(|e| EncodeError::markup::<Self>(Extension::TMP_BIN, e)) }
        /// Intended for short-term storage of data in a binary format, such as during data transfer.
    ///
    /// Not suitable for long-term storage, as the implementation or encoding may change at any time.
    fn to_tmp_bin_with_writer<W: Write>(&self, writer: W) -> EncodeResult { bincode::serialize_into(writer, self).map_err(|e| EncodeError::markup::<Self>(Extension::TMP_BIN, e)) }
}
impl<T> ToTmpBin for T where T: Serialize {}

pub trait FromTmpBin: for<'de> Deserialize<'de>
{
    fn from_tmp_bin_bytes(bytes: &[u8]) -> EncodeResult<Self> { bincode::deserialize(bytes).map_err(|e| EncodeError::markup::<Self>(Extension::TMP_BIN, e)) }
    fn from_tmp_bin_with_reader<R: Read>(reader: R) -> EncodeResult<Self> { bincode::deserialize_from(reader).map_err(|e| EncodeError::markup::<Self>(Extension::TMP_BIN, e)) }
}
impl<T> FromTmpBin for T where T: for<'de> Deserialize<'de> {}





