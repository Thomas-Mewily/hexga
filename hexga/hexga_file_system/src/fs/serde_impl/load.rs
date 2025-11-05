use super::*;



pub trait LoadFromFs: for<'de> Deserialize<'de>
{
    fn load_from_fs<P,Fs>(path: P, fs: &mut Fs) -> IoResult<Self> where P: AsRefPath, Fs: FsWrite
    {
        fs.load(path)
    }
}
impl<T> LoadFromFs for T where T: for<'de> Deserialize<'de> {}



pub trait LoadFromDisk : LoadFromFs
{
    fn load_from_disk<P>(path: P) -> IoResult<Self> where P: AsRefPath
    {
        Self::load_from_fs(path, &mut FsDisk)
    }
}
impl<T> LoadFromDisk for T where T: LoadFromFs {}



/// Extension trait for [`FsRead`] that provides methods to **load values from the file system**,
/// independently of the actual file extension.
pub trait FsLoad : FsRead + Sized
{
    /// Load a value from a file, automatically detecting the extension.
    ///
    /// If the file does not exist, attempts to auto-correct the extension.
    fn load<T,P>(&mut self, path: P) -> IoResult<T> where T: for<'de> Deserialize<'de>, P: AsRefPath
    {
        let path = path.as_ref();
        if self.exists(&path).is_err()
        {
            let path = self.auto_correct_extension(&path);
            return self.load_with_extension(&path, path.extension_or_empty());
        }
        self.load_with_extension(&path, path.extension_or_empty())
    }

    /// Load a value from a file using a specified extension, ignoring the file's actual extension.
    ///
    /// If the file does not exist, attempts to auto-correct the extension.
    fn load_with_extension<T,P>(&mut self, path: P, extension: &extension) -> IoResult<T> where T: for<'de> Deserialize<'de>, P: AsRefPath
    {
        let mut path = path.as_ref().to_owned();
        if self.exists(&path).is_err()
        {
            path = self.auto_correct_extension(&path);
        }

        match extension
        {
            Io::RON =>
            {
                let markup = self.read_string(&path).map_err(|e| IoError::new(&path, e))?;
                T::from_ron(&markup).map_err(|e| IoError::new(&path, e))
            }

            Io::JSON =>
            {
                let markup = self.read_string(&path).map_err(|e| IoError::new(&path, e))?;
                T::from_json(&markup).map_err(|e| IoError::new(&path, e))
            }

            Io::XML =>
            {
                let markup = self.read_string(&path).map_err(|e| IoError::new(&path, e))?;
                T::from_xml(&markup).map_err(|e| IoError::new(&path, e))
            }

            Io::TXT =>
            {
                let txt = self.read_string(&path).map_err(|e| IoError::new(&path, e))?;
                T::deserialize(DeserializerTxt{ txt: txt.into() }).map_err(|e| IoError::new(&path, e))
            }

            _ =>
            {
                todo!()
                // let mut de = DeserializerLoad::new(self, path.to_owned());
                // T::deserialize(&mut de)
            }
        }
    }
}
impl<S> FsLoad for S where S : FsRead {}

