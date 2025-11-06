use super::*;

pub trait SaveToFs : Serialize
{
    fn save_to_fs<P, Fs>(&self, path: P, fs: &mut Fs) -> IoResult where P: AsRefPath, Fs: FsWrite
    {
        fs.save(self, path)
    }
}
impl<T> SaveToFs for T where T: Serialize + ?Sized {}

pub trait SaveToDisk : Serialize
{
    fn save_to_disk<P>(&self, path: P) -> IoResult where P: AsRefPath
    {
        self.save_to_fs(path, &mut FsDiskNotAutoCorrected)
    }
}
impl<T> SaveToDisk for T where T: Serialize + ?Sized {}


pub trait FsSave : FsWrite + Sized
{
    fn save<T, P>(&mut self, value: &T, path: P) -> IoResult where T: Serialize + ?Sized, P: AsRefPath
    {
        let path = path.as_ref();

        match path.extension_or_empty()
        {
            Io::RON =>
            {
                let markup = value.to_ron().map_err(|e| IoError::new(path, e))?;
                self.write_str(&path, &markup).map_err(|e| IoError::new(path, e))
            },
            Io::JSON =>
            {
                let markup = value.to_json().map_err(|e| IoError::new(path, e))?;
                self.write_str(&path, &markup).map_err(|e| IoError::new(path, e))
            },
            Io::XML =>
            {
                let markup = value.to_xml().map_err(|e| IoError::new(path, e))?;
                self.write_str(&path, &markup).map_err(|e| IoError::new(path, e))
            },
            Io::TXT =>
            {
                let txt = value.serialize(SerializerTxt).map_err(|e| IoError::new(path, e))?;
                self.write_str(&path, &txt).map_err(|e| IoError::new(path, e))
            }
            _ =>
            {

                let mut bytes = Vec::<u8>::with_capacity(1024);
                let mut ron = SerializerRon::new_serializer(&mut bytes);
                let serializer = SerializerSave::new(&mut ron);
                match value.serialize(serializer).map_err(|e| IoError::new(path, e))?
                {
                    SaveOutput::Format(data) => self.write_bytes(&path.with_extension(&data.extension), &data.bytes).map_err(|e| IoError::new(path, e)),
                    SaveOutput::Markup => self.write_bytes(path, &bytes).map_err(|e| IoError::new(path, e)),
                }
            }
        }
    }
}
impl<S> FsSave for S where S : FsWrite {}
