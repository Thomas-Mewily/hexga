use super::*;



pub trait LoadFromFs: for<'de> Deserialize<'de>
{
    fn load<P,Fs>(path: P, fs: &mut Fs) -> FileResult<Self> where P: AsRefPath, Fs: FsWrite
    {
        fs.load(path)
    }
}
impl<T> LoadFromFs for T where T: for<'de> Deserialize<'de> {}



pub trait LoadFromDisk : LoadFromFs
{
    fn load_from_disk<P>(path: P) -> FileResult<Self> where P: AsRefPath
    {
        Self::load(path, &mut FsDisk)
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
    fn load<T,P>(&mut self, path: P) -> FileResult<T> where T: for<'de> Deserialize<'de>, P: AsRefPath
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
    fn load_with_extension<T,P>(&mut self, path: P, extension: &extension) -> FileResult<T> where T: for<'de> Deserialize<'de>, P: AsRefPath
    {
        let mut path = path.as_ref().to_owned();
        if self.exists(&path).is_err()
        {
            path = self.auto_correct_extension(&path);
        }

        match extension
        {
            Io::RON => T::from_ron(&self.read_str(&path)?).map_err(|e| e.into()),

            Io::JSON => T::from_json(&self.read_str(&path)?).map_err(|e| e.into()),

            Io::XML => T::from_xml(&self.read_str(&path)?).map_err(|e| e.into()),

            Io::TXT => T::deserialize(DeserializerTxt{ txt: self.read_str(&path)? }).map_err(|e| e.into()),

            //Io::QUICK_BIN => T::from_quick_bin_buf(&self.read_bytes(&path)?).map_err(|e| e.into()),

            _ =>
            {
                let bytes = self.read_bytes(&path)?;
                let load_deserializer = DeserializerTxtOrBinary { bytes: bytes.into_owned() };
                T::deserialize(load_deserializer).map_err(|e| e.into())
            }
        }
    }
}
impl<S> FsLoad for S where S : FsRead {}

