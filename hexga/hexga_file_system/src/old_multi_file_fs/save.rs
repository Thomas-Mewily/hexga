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
        self.save_to_fs(path, &mut FsDisk)
    }
}
impl<T> SaveToDisk for T where T: Serialize + ?Sized {}


pub trait FsSave : FsWrite + Sized
{
    fn save<T, P>(&mut self, value: &T, path: P) -> IoResult where T: Serialize + ?Sized, P: AsRefPath
    {
        let path = path.as_ref();
        self.save_with_param(value, path, Default::default())
    }

    fn save_with_param<T, P>(&mut self, value: &T, path: P, param: SaveParam) -> IoResult where T: Serialize + ?Sized, P: AsRefPath
    {
        let path = path.as_ref();

        if !param.multi_file
        {
            match path.extension_or_empty()
            {
                Io::RON =>
                {
                    let markup = value.to_ron().map_err(|e| IoError::new(path, e))?;
                    return self.write_str(&path, &markup).map_err(|e| IoError::new(path, e));
                },
                Io::JSON =>
                {
                    let markup = value.to_json().map_err(|e| IoError::new(path, e))?;
                    return self.write_str(&path, &markup).map_err(|e| IoError::new(path, e));
                },
                // Io::XML =>
                // {
                //     let markup = value.to_xml().map_err(|e| IoError::new(path, e))?;
                //     return self.write_str(&path, &markup).map_err(|e| IoError::new(path, e));
                // },
                Io::TXT =>
                {
                    let txt = value.serialize(SerializerTxt).map_err(|e| IoError::new(path, e))?;
                    return self.write_str(&path, &txt).map_err(|e| IoError::new(path, e));
                }

                // TODO: handle Io::BIN (raw bytes)

                _ => {}
            }
        }

        let mut ser = SerializerSave::new(self, path.to_owned(), param);
        value.serialize(&mut ser)?;
        ser.save()
    }
}
impl<S> FsSave for S where S : FsWrite {}
