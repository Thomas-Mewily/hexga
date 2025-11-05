use super::*;


pub type SerializerRon<'se> = ron::ser::Serializer<VecWrite<'se>>;
pub type SerializerJson<'se> = serde_json::Serializer<&'se mut Vec<u8>>;
pub type SerializerXml<'se> = serde_xml_rs::Serializer<&'se mut Vec<u8>>;

// pub type SerializerAny<'se> = Markup<SerializerRon<'se>,SerializerJson<'se>>;
// pub enum SerializerAny<'se>
// {
//     Ron(SerializerRon<'se>),
//     Json(SerializerJson<'se>),
// }
// impl<'de> std::fmt::Debug for SerializerAny<'de>
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Ron(_) => f.write_str(Io::RON),
//             Self::Json(_) => f.write_str(Io::JSON),
//         }
//     }
// }

pub trait MarkupSerializer<'se>
{
    const EXTENSION : &'static str;
    fn new_serializer(src: &'se mut Vec<u8>) -> Self;
}

impl<'se> MarkupSerializer<'se> for SerializerRon<'se>
{
    const EXTENSION : &'static str = Io::RON;

    fn new_serializer(src: &'se mut Vec<u8>) -> Self {
        SerializerRon::new(VecWrite(src), Some(Default::default())).unwrap()
    }
}
impl<'se> MarkupSerializer<'se> for SerializerJson<'se>
{
    const EXTENSION : &'static str = Io::JSON;

    fn new_serializer(src: &'se mut Vec<u8>) -> Self {
        SerializerJson::new(src)
    }
}
impl<'se> MarkupSerializer<'se> for SerializerXml<'se>
{
    const EXTENSION : &'static str = Io::XML;

    fn new_serializer(src: &'se mut Vec<u8>) -> Self {
        SerializerXml::new_from_writer(src)
    }
}



#[doc(hidden)]
pub struct VecWrite<'a>(&'a mut Vec<u8>);

impl<'a> std::fmt::Write for VecWrite<'a> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.extend_from_slice(s.as_bytes());
        Ok(())
    }
}