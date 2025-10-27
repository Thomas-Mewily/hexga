use super::*;


const PREFIX: &[u8] = b"custom_extension;";


// pub trait SerializeAsFile
// {

// }
// impl<T> Serialize for T where T: SerializeAsFile

// pub trait SerializerFileExtension : Serializer
// {
//     fn serialize_file_format(self, extension: &extension, bytes: &[u8]) -> Result<Self::Ok, Self::Error>
//     {
//         todo!();
//         if self.is_human_readable()
//         {

//         }else
//         {
//             todo!("encode them in base 64")
//         }
//         Ok(())
//     }
// }
// impl<F> SerializerFileExtension for F where F: Serializer{}

pub trait BinaryVecMagicFileExtension
{
    fn push_magic_file_extension(&mut self, extension: &extension);
    fn extract_magic_file_extension_and_data(&self) -> Result<(&str, &[u8]), ()>;
}
impl BinaryVecMagicFileExtension for Vec<u8>
{
    fn push_magic_file_extension(&mut self, extension: &extension) {
        self.extend_from_slice(PREFIX);
        self.extend_from_slice(extension.as_bytes());
        self.extend_from_slice(b";");
    }

    fn extract_magic_file_extension_and_data(&self) -> Result<(&str, &[u8]), ()> {
        if !self.starts_with(PREFIX)
        {
            return Err(());
        }

        let rest = &self[PREFIX.len()..];
        let sep_pos = rest.iter().position(|&b| b == b';').ok_or(())?;

        // Split into (extension, data)
        let (ext_bytes, data) = rest.split_at(sep_pos);
        let ext = std::str::from_utf8(ext_bytes).map_err(|_| ())?;

        // Skip the ';' separator before the data
        let data = &data[1..];
        Ok((ext, data))
    }
}