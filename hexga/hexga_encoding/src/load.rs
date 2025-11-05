use super::*;


pub trait Load
{
    fn load_extensions() -> impl Iterator<Item = &'static extension>;
    fn load_from_reader<R>(reader : &mut R, extension: &extension) -> EncodeResult<Self> where R: Read, Self: Sized
    {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).map_err(|e| EncodeError::from(e))?;
        Self::load_from_bytes(&bytes, extension)
    }
    fn load_from_bytes(bytes: &[u8], extension: &extension) -> EncodeResult<Self> where Self: Sized;
    fn load_prefered_extension() -> Option<&'static extension> { Self::load_extensions().next() }
}