use super::*;









pub trait FsRead
{
    /// Reads the content of a file into memory.
    fn read_bytes<'a>(&'a mut self, path: &path) -> IoResult<Cow<'a, [u8]>>;

    /// Reads the content of a file into memory.
    fn read_str(&mut self, path: &path) -> IoResult<String>
    {
        let bytes_cow: Cow<[u8]> = self.read_bytes(path)?;

        match String::from_utf8(bytes_cow.into_owned()) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into()),
        }
    }



    /// Lists subpaths (files/folders) name under a directory. Empty if don't exist
    fn entries_names(&mut self, path: &path) -> Vec<String>;

    fn node_kind(&mut self, path: &path) -> IoResult<FsNodeKind>;

    /// Checks whether a path exists in this filesystem.
    fn exists(&mut self, path: &path) -> IoResult<bool> { self.node_kind(path).map(|_| true) }
    fn is_file(&mut self, path: &path) -> bool { self.node_kind(path).map(|e| e.is_file()).unwrap_or(false) }
    fn is_directory(&mut self, path: &path) -> bool { self.node_kind(path).map(|e| e.is_directory()).unwrap_or(false) }

    /// Lists subpaths (files/folders) under a directory.
    fn entries(&mut self, path: &path) -> Vec<String>
    {
        let mut s = self.entries_names(path);
        s.iter_mut().for_each(|name| *name = path.path_concat(name));
        s
    }
}


pub trait FsReadExtension : FsRead + Sized
{
    fn load<T>(&mut self, path: &path) -> IoResult<T> where T: Load
    {
        T::load_from(path, self)
    }
}
impl<S> FsReadExtension for S where S : FsRead {}




pub trait FsWrite : FsRead
{
    /// Override the file content if already exist.
    /// If the file don't exist, create it.
    fn write_bytes(&mut self, path: &path, bytes: &[u8]) ->  IoResult;

    /// Override the file content if already exist.
    /// If the file don't exist, create it.
    fn write_str(&mut self, path: &path, text: &str) -> IoResult
    {
        self.write_bytes(path, text.as_bytes())
    }


    /// Delete the files/folder recursively under a directory.
    fn delete(&mut self, path: &path) -> IoResult;

    /// Move the file/directory. If it is a folder, also move all the content
    fn move_to(&mut self, path: &path, new_path: &path) -> IoResult;

    /// Rename the directory / file name with another name.
    /// Keep the extension
    fn rename(&mut self, path: &path, name: &str) -> IoResult
    {
        let new_path = path.with_file_name(name);
        self.move_to(path, &new_path)
    }
}

pub trait FsWriteExtension : FsWrite + Sized
{
    fn save<T : ?Sized>(&mut self, path: &path, value: &T) -> IoResult where T: Save
    {
        value.save_to(path, self)
    }
}
impl<S> FsWriteExtension for S where S : FsWrite {}







//pub type Fs = dyn FileSystem;
pub struct Fs<'a>
{
    fs : &'a mut dyn FsWrite,
}
impl<'a> Fs<'a>
{
    pub fn new(fs : &'a mut dyn FsWrite) -> Self { Self { fs }}
}
impl<'a> FsWrite for Fs<'a>
{
    fn write_bytes(&mut self, path: &path, bytes: &[u8]) ->  IoResult {
        self.fs.write_bytes(path, bytes)
    }
    fn write_str(&mut self, path: &path, text: &str) -> IoResult {
        self.fs.write_str(path, text)
    }
    fn rename(&mut self, path: &path, name: &str) -> IoResult {
        self.fs.rename(path, name)
    }
    fn delete(&mut self, path: &path) -> IoResult {
        self.fs.delete(path)
    }
    fn move_to(&mut self, path: &path, new_path: &path) -> IoResult {
        self.fs.move_to(path, new_path)
    }
}
impl<'a> FsRead for Fs<'a>
{
    fn read_bytes<'b>(&'b mut self, path: &path) -> IoResult<Cow<'b, [u8]>> {
        self.fs.read_bytes(path)
    }
    fn entries_names(&mut self, path: &path) -> Vec<String> {
        self.fs.entries_names(path)
    }
    fn node_kind(&mut self, path: &path) -> IoResult<FsNodeKind> {
        self.fs.node_kind(path)
    }
    fn entries(&mut self, path: &path) -> Vec<String> {
        self.fs.entries(path)
    }
    fn exists(&mut self, path: &path) -> IoResult<bool> {
        self.fs.exists(path)
    }
    fn is_directory(&mut self, path: &path) -> bool {
        self.fs.is_directory(path)
    }
    fn is_file(&mut self, path: &path) -> bool {
        self.fs.is_file(path)
    }
    fn read_str(&mut self, path: &path) -> IoResult<String> {
        self.fs.read_str(path)
    }
}