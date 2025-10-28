use super::*;









pub trait FsRead
{
    /// Reads the content of a file into memory.
    fn read_bytes<'a>(&'a mut self, path: &path) -> FileResult<Cow<'a, [u8]>>;

    /// Reads the content of a file into memory.
    fn read_str(&mut self, path: &path) -> FileResult<String>
    {
        let bytes_cow: Cow<[u8]> = self.read_bytes(path)?;

        match String::from_utf8(bytes_cow.into_owned()) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into()),
        }
    }



    /// Lists subpaths (files/folders) name under a directory. Empty if don't exist
    fn entries_names(&mut self, path: &path) -> Vec<String>;

    fn node_kind(&mut self, path: &path) -> FileResult<FsNodeKind>;

    /// Checks whether a path exists in this filesystem.
    fn exists(&mut self, path: &path) -> FileResult<bool> { self.node_kind(path).map(|_| true) }
    fn is_file(&mut self, path: &path) -> bool { self.node_kind(path).map(|e| e.is_file()).unwrap_or(false) }
    fn is_directory(&mut self, path: &path) -> bool { self.node_kind(path).map(|e| e.is_directory()).unwrap_or(false) }

    /// Lists subpaths (files/folders) under a directory.
    fn entries(&mut self, path: &path) -> Vec<Path>
    {
        self.entries_names(path).into_iter().map(|name| path / name).collect()
    }
}


#[cfg(feature = "serde")]
pub trait FsReadExtension : FsRead + Sized
{
    fn load<T,P>(&mut self, path: P) -> FileResult<T> where T: for<'de> Deserialize<'de>, P: AsRefPath
    {
        let path = path.as_ref();
        let original_extension = path.extension_or_empty();
        let extension = original_extension;

        match extension
        {
            Extensions::RON => return T::from_ron(&self.read_str(path)?).map_err(|e| e.into()),

            #[cfg(feature = "serde_json")]
            Extensions::JSON => return T::from_json(&self.read_str(path)?).map_err(|e| e.into()),

            #[cfg(feature = "serde_xml")]
            Extensions::XML => return T::from_xml(&self.read_str(path)?).map_err(|e| e.into()),

            #[cfg(feature = "serde_quick_bin")]
            Extensions::QUICK_BIN => return T::from_quick_bin_buf(&self.read_bytes(path)?).map_err(|e| e.into()),

            _ => return T::from_ron(&self.read_str(path)?).map_err(|e| e.into()),
        }
    }
}
#[cfg(feature = "serde")]
impl<S> FsReadExtension for S where S : FsRead {}




pub trait FsWrite : FsRead
{
    /// Override/delete the file content if already exist.
    /// If the file don't exist, create it.
    fn write_bytes(&mut self, path: &path, bytes: &[u8]) ->  FileResult;

    /// Override the file content if already exist.
    /// If the file don't exist, create it.
    fn write_str(&mut self, path: &path, text: &str) -> FileResult
    {
        self.write_bytes(path, text.as_bytes())
    }


    /// Delete the files/folder recursively under a directory.
    fn delete(&mut self, path: &path) -> FileResult;

    /// Move the file/directory. If it is a folder, also move all the content
    fn move_to(&mut self, path: &path, new_path: &path) -> FileResult;

    /// Rename the directory / file name with another name.
    /// Keep the extension
    fn rename(&mut self, path: &path, name: &str) -> FileResult
    {
        let new_path = path.with_name(name);
        self.move_to(path, &new_path)
    }
}

#[cfg(feature = "serde")]
pub trait FsWriteExtension : FsWrite + Sized
{
    fn save<T : ?Sized, P>(&mut self, path: P, value: &T) -> FileResult where T: Serialize, P: AsRefPath
    {
        let path = path.as_ref();
        let original_extension = path.extension_or_empty();
        let extension = original_extension;


        match extension
        {
            Extensions::RON => return self.write_str(path, &value.to_ron()?),

            #[cfg(feature = "serde_json")]
            Extensions::JSON => return self.write_str(path, &value.to_json()?),

            #[cfg(feature = "serde_xml")]
            Extensions::XML => return self.write_str(path, &value.to_xml()?),

            #[cfg(feature = "serde_quick_bin")]
            Extensions::QUICK_BIN => return self.write_bytes(path, &value.to_quick_bin()?),

            _ => return self.write_str(path, &value.to_ron()?),
        }
    }
}
#[cfg(feature = "serde")]
impl<S> FsWriteExtension for S where S : FsWrite {}







// //pub type Fs = dyn FileSystem;
// pub struct Fs<'a>
// {
//     fs : &'a mut dyn FsWrite,
// }
// impl<'a> Fs<'a>
// {
//     pub fn new(fs : &'a mut dyn FsWrite) -> Self { Self { fs }}
// }
// impl<'a> FsWrite for Fs<'a>
// {
//     fn write_bytes(&mut self, path: &path, bytes: &[u8]) ->  IoResult {
//         self.fs.write_bytes(path, bytes)
//     }
//     fn write_str(&mut self, path: &path, text: &str) -> IoResult {
//         self.fs.write_str(path, text)
//     }
//     fn rename(&mut self, path: &path, name: &str) -> IoResult {
//         self.fs.rename(path, name)
//     }
//     fn delete(&mut self, path: &path) -> IoResult {
//         self.fs.delete(path)
//     }
//     fn move_to(&mut self, path: &path, new_path: &path) -> IoResult {
//         self.fs.move_to(path, new_path)
//     }
// }
// impl<'a> FsRead for Fs<'a>
// {
//     fn read_bytes<'b>(&'b mut self, path: &path) -> IoResult<Cow<'b, [u8]>> {
//         self.fs.read_bytes(path)
//     }
//     fn entries_names(&mut self, path: &path) -> Vec<String> {
//         self.fs.entries_names(path)
//     }
//     fn node_kind(&mut self, path: &path) -> IoResult<FsNodeKind> {
//         self.fs.node_kind(path)
//     }
//     fn entries(&mut self, path: &path) -> Vec<Path> {
//         self.fs.entries(path)
//     }
//     fn exists(&mut self, path: &path) -> IoResult<bool> {
//         self.fs.exists(path)
//     }
//     fn is_directory(&mut self, path: &path) -> bool {
//         self.fs.is_directory(path)
//     }
//     fn is_file(&mut self, path: &path) -> bool {
//         self.fs.is_file(path)
//     }
//     fn read_str(&mut self, path: &path) -> IoResult<String> {
//         self.fs.read_str(path)
//     }
// }