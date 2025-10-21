// use super::*;


// pub trait AsyncFileSystem
// {
//     /// Reads the content of a file into memory.
//     fn async_read_bytes<'a>(&'a mut self, path: &path) -> DynFuture<IoResult<Cow<'a, [u8]>>>;

//     /// Override the file content if already exist.
//     /// If the file don't exist, create it.
//     fn async_write_bytes(&mut self, path: &path, bytes: Vec<u8>) ->  DynFuture<IoResult>;


//     fn async_node_kind(&mut self, path: &path) -> DynFuture<IoResult<FsNodeKind>>;
//     /// Checks whether a path exists in this filesystem.
//     fn exists(&mut self, path: &path) -> DynFuture<IoResult<bool>>;
//     fn is_file(&mut self, path: &path) -> DynFuture<bool>;
//     fn is_directory(&mut self, path: &path) -> DynFuture<bool>;

//     /// Lists subpaths (files/folders) under a directory.
//     fn subpath(&mut self, path: &path) -> DynFuture<Vec<String>>
//     {
//         let mut s = self.subpath_name(path);
//         s.iter_mut().for_each(|name| *name = path.path_concat(name));
//         s
//     }

//     /// Lists subpaths (files/folders) name under a directory. Empty if don't exist
//     fn subpath_name(&mut self, path: &path) -> Vec<String>;

//     /// Delete the files/folder recursively under a directory.
//     fn delete(&mut self, path: &path) -> IoResult;

//     /// Rename the path. If it is a folder, also move all the content
//     fn change_path(&mut self, path: &path, new_path: &path) -> IoResult;

//     /// Rename the directory / file name with another name.
//     /// Keep the extension
//     fn rename(&mut self, path: &path, name: &str) -> IoResult
//     {
//         let new_path = path.path_replace_name(name);
//         self.change_path(path, &new_path)
//     }
// }
