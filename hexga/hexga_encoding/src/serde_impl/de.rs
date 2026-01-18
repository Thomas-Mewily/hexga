
// Todo: add a cfg flag to determine the SerializerPrefered
pub type  DeserializerPrefered<'se> = DeserializerRon<'se>;

pub type DeserializerRon<'de> = ron::Deserializer<'de>;
pub type DeserializerJson<'de> = serde_json::de::Deserializer<serde_json::de::SliceRead<'de>>;
pub type DeserializerXml<'de> = serde_xml_rs::de::Deserializer<&'de mut Vec<u8>>;
