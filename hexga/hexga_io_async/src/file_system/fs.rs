use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FsNodeKind
{
    File,
    Directoy,
}

impl FsNodeKind
{
    pub const fn is_file(self) -> bool { matches!(self, Self::File) }
    pub const fn is_directory(self) -> bool { matches!(self, Self::Directoy) }
}

pub trait FileSystem
{
    /// Reads the content of a file into memory.
    fn read_bytes<'a>(&'a mut self, path: &path) -> IoResult<Cow<'a, [u8]>>;

    /// Override the file content if already exist.
    /// If the file don't exist, create it.
    fn write_bytes(&mut self, path: &path, bytes: Vec<u8>) ->  IoResult;


    fn node_kind(&mut self, path: &path) -> IoResult<FsNodeKind>;
    /// Checks whether a path exists in this filesystem.
    fn exists(&mut self, path: &path) -> IoResult<bool> { self.node_kind(path).map(|_| true) }
    fn is_file(&mut self, path: &path) -> bool { self.node_kind(path).map(|e| e.is_file()).unwrap_or(false) }
    fn is_directory(&mut self, path: &path) -> bool { self.node_kind(path).map(|e| e.is_directory()).unwrap_or(false) }

    /// Lists subpaths (files/folders) under a directory.
    fn subpath(&mut self, path: &path) -> Vec<String>
    {
        let mut s = self.subpath_name(path);
        s.iter_mut().for_each(|name| *name = path.path_concat(name));
        s
    }

    /// Lists subpaths (files/folders) name under a directory. Empty if don't exist
    fn subpath_name(&mut self, path: &path) -> Vec<String>;

    /// Delete the files/folder recursively under a directory.
    fn delete(&mut self, path: &path) -> IoResult;

    /// Rename the path. If it is a folder, also move all the content
    fn change_path(&mut self, path: &path, new_path: &path) -> IoResult;

    /// Rename the directory / file name with another name.
    /// Keep the extension
    fn rename(&mut self, path: &path, name: &str) -> IoResult
    {
        let new_path = path.with_file_name(name);
        self.change_path(path, &new_path)
    }
}
