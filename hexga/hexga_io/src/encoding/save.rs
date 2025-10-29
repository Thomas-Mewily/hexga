use super::*;


pub trait Save
{
    fn save_extensions() -> impl Iterator<Item = &'static extension>;
    fn save(&self, extension: &extension) -> EncodeResult<Vec<u8>>
    {
        let mut bytes = Vec::with_capacity(1024);
        self.save_in(&mut bytes, extension)?;
        Ok(bytes)
    }
    fn save_in<W>(&self, writer : &mut W, extension: &extension) -> EncodeResult where W: Write;
    fn save_prefered_extension() -> Option<&'static extension> { Self::save_extensions().next() }
}


