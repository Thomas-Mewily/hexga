use super::*;

pub trait MarkupSerializer
{
    const EXTENSION: &'static str;
    type Serializer;

    /// Creates a serializer that tries to respect the given parameters.
    fn create_new(param: &MultiFileSerializerParam) -> Self::Serializer where for<'a> &'a mut Self::Serializer: Serializer;
    fn extract(serializer: Self::Serializer) -> Vec<u8>;
}

pub struct Json;
impl MarkupSerializer for Json
{
    const EXTENSION: &'static str= "json";

    type Serializer=serde_json::Serializer<Vec<u8>,serde_json::ser::PrettyFormatter<'static>>;

    fn create_new(param: &MultiFileSerializerParam) -> Self::Serializer where for<'a> &'a mut Self::Serializer: Serializer
    {
        //let f = serde_json::ser::PrettyFormatter::with_indent(param.indent.to_owned().as_bytes());
        let f = serde_json::ser::PrettyFormatter::with_indent(b"   ");
        serde_json::Serializer::with_formatter(Vec::<u8>::with_capacity(param.capacity), f)
    }

    fn extract(serializer: Self::Serializer) -> Vec<u8>
    {
        serializer.into_inner()
    }
}