use super::*;


pub trait FsRead
{
    /// Reads the content of a file into memory as raw bytes.
    fn read_bytes<'a>(&'a mut self, path: &path) -> FileResult<Cow<'a, [u8]>>;

    /// Reads the content of a file into memory as a UTF-8 string.
    fn read_str(&mut self, path: &path) -> FileResult<String>
    {
        let bytes_cow: Cow<[u8]> = self.read_bytes(path)?;

        match String::from_utf8(bytes_cow.into_owned()) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into()),
        }
    }


    /// Returns the kind of a filesystem node (file, directory).
    fn node_kind(&mut self, path: &path) -> FileResult<FsNodeKind>;

    /// Checks whether a path exists in the filesystem.
    fn exists(&mut self, path: &path) -> FileResult<bool> { self.node_kind(path).map(|_| true) }
    /// Checks whether a path is a file.
    fn is_file(&mut self, path: &path) -> bool { self.node_kind(path).map(|e| e.is_file()).unwrap_or(false) }
    /// Checks whether a path is a directory.
    fn is_directory(&mut self, path: &path) -> bool { self.node_kind(path).map(|e| e.is_directory()).unwrap_or(false) }

    /// Attempts to automatically correct the file extension of a given path.
    ///
    /// This function looks up all known entries that match the given `path`
    /// regardless of extension, using [`FsRead::entries_with_any_extension`].
    ///
    /// - If **exactly one** matching entry is found, that path is returned.
    /// - If **zero or multiple** matches are found, the original `path` is returned unchanged to avoid ambiguity
    fn auto_correct_extension(&mut self, path: &path) -> Path
    {
        let auto_corrected = self.entries_with_any_extension(path);
        if auto_corrected.len() == 1
        {
            auto_corrected.into_iter().next().unwrap()
        }else
        {
            path.to_owned()
        }
    }


    /// Lists the full names (filenames or folder name) of all entries under a directory.
    ///
    /// Returns an empty vector if the directory does not exist or is empty.
    fn entries_fullname(&mut self, path: &path) -> Vec<String>;


    /// Lists all path (filenames or folder) under a directory.
    ///
    /// Returns an empty vector if the directory does not exist or is empty.
    fn entries(&mut self, path: &path) -> Vec<Path>
    {
        self.entries_fullname(path).into_iter().map(|name| path / name).collect()
    }

    /// Lists the full names (filenames or folder name) of all entries under a directory
    /// with the same name, ignoring the extension.
    ///
    /// Returns an empty vector if the directory does not exist or is empty.
    fn entries_fullname_with_any_extension(&mut self, path: &path) -> Vec<String>
    {
        let name = path.name();
        self.entries_fullname(path.parent_or_empty()).into_iter().filter(|fullname| path::from_str(&fullname).name() == name).collect()
    }

    /// Lists all path (filenames or folder) under a directory
    /// with the same name, ignoring the extension.
    ///
    /// Returns an empty vector if the directory does not exist or is empty.
    fn entries_with_any_extension(&mut self, path: &path) -> Vec<Path>
    {
        let parent_path = path.parent_or_empty();
        self.entries_fullname_with_any_extension(path).into_iter().map(|name| parent_path / name).collect()
    }
}


#[cfg(feature = "serde")]
/// Extension trait for [`FsRead`] that provides methods to **load values from the file system**,
/// independently of the actual file extension.
pub trait FsReadExtension : FsRead + Sized
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

            Io::QUICK_BIN => T::from_quick_bin_buf(&self.read_bytes(&path)?).map_err(|e| e.into()),

            Io::TXT => T::deserialize(DeserializerTxt{ txt: self.read_str(&path)? }).map_err(|e| e.into()),

            _ =>
            {
                let bytes = self.read_bytes(&path)?;
                let load_deserializer = DeserializerTxtOrBinary { bytes: bytes.into_owned() };
                T::deserialize(load_deserializer).map_err(|e| e.into())
            }
        }
    }
}
#[cfg(feature = "serde")]
impl<S> FsReadExtension for S where S : FsRead {}




pub trait FsWrite : FsRead
{
    /// Creates a new directory at the specified path.
    ///
    /// - If the directory already exists, this is a **no-op**.
    /// - If any parent directories in the path do not exist, they are **created automatically**.
    /// - If the path or any parent path exists as a file, delete it and replace it by a directory.
    fn create_directory(&mut self, path: &path) ->  FileResult;

    /// Writes raw bytes to a file.
    ///
    /// - If the file already exists, its content is **overwritten**.
    /// - If the file does not exist, it and any missing parent directories are **created**.
    /// - If the path exists as a directory, the directory is **deleted** and replaced by the file.
    fn write_raw_bytes(&mut self, path: &path, bytes: &[u8]) ->  FileResult;

    /// Writes raw bytes to a file, while also deleting any ambigious existing files with the same name in the same folder.
    /// Check [`FsWrite::write_raw_bytes`] to not delete any ambigious existing files.
    ///
    /// - If the file already exists, its content is **overwritten**.
    /// - If the file does not exist, it and any missing parent directories are **created**.
    /// - If the path exists as a directory, the directory is **deleted** and replaced by the file.
    fn write_bytes(&mut self, path: &path, bytes: &[u8]) ->  FileResult
    {
        self.delete_with_any_extension(path)?;
        self.write_raw_bytes(path, bytes)
    }

    /// Writes a UTF-8 string to a file, while also deleting any ambigious existing files with the same name in the same folder.
    /// Check [`FsWrite::write_raw_str`] to not delete any ambigious existing files.
    ///
    /// - If the file already exists, its content is **overwritten**.
    /// - If the file does not exist, it and any missing parent directories are **created**.
    /// - If the path exists as a directory, the directory is **deleted** and replaced by the file.
    fn write_str(&mut self, path: &path, text: &str) -> FileResult
    {
        self.write_bytes(path, text.as_bytes())
    }

    /// Writes a UTF-8 string to a file.
    ///
    /// - If the file already exists, its content is **overwritten**.
    /// - If the file does not exist, it and any missing parent directories are **created**.
    /// - If the path exists as a directory, the directory is **deleted** and replaced by the file.
    fn write_raw_str(&mut self, path: &path, text: &str) -> FileResult
    {
        self.write_raw_bytes(path, text.as_bytes())
    }


    /// Deletes a file or a directory recursively.
    ///
    /// - If the path is a directory, all its contents are also deleted.
    /// - If the path does not exist, return Ok(()).
    fn delete(&mut self, path: &path) -> FileResult;

    /// Deletes all files and directories recursively with the same name, ignoring the extension.
    ///
    /// - If the path does not exist, return Ok(()).
    fn delete_with_any_extension(&mut self, path: &path) -> FileResult
    {
        for entry in self.entries_with_any_extension(path) {
            self.delete(&entry)?;
        }
        Ok(())
    }

    /// Moves a file or directory to a new location.
    ///
    /// - If the path is a directory, all its contents are moved.
    /// - If the target path already exists, delete it.
    fn move_to(&mut self, path: &path, new_path: &path) -> FileResult;

    /// Renames a file or directory, keeping its extension.
    ///
    /// - Only the base name of the file or directory is changed.
    /// - The new path is constructed using the same parent directory and the new name.
    fn rename(&mut self, path: &path, name: &str) -> FileResult
    {
        let new_path = path.with_name(name);
        self.move_to(path, &new_path)
    }
}



#[cfg(feature = "serde")]
pub trait FsWriteExtension : FsWrite + Sized
{
    fn save<T, P>(&mut self, value: &T, path: P) -> IoResult where T: Serialize + ?Sized, P: AsRefPath
    {
        let path = path.as_ref();
        self.save_with_param(value, path, Default::default())
    }

    fn save_with_param<T, P>(&mut self, value: &T, path: P, param: SaveParam) -> IoResult where T: Serialize + ?Sized, P: AsRefPath
    {
        let path = path.as_ref();
        let mut ser = SerializerSaveTxtOrBinOrMarkup::new(self, path.to_owned(), param);
        value.serialize(&mut ser)?;
        ser.save()

        // if let Some(ext) = extension
        // {
        //     if !param.multi_file
        //     {
        //         match ext
        //         {
        //             Extensions::RON =>
        //             {
        //                 let markup = value.to_ron().map_err(|e| IoError::new(path, e))?;
        //                 self.write_str(&path, &markup).map_err(|e| IoError::new(path, e))
        //             },

        //             Extensions::JSON =>
        //             {
        //                 let markup = value.to_json().map_err(|e| IoError::new(path, e))?;
        //                 self.write_str(&path, &markup).map_err(|e| IoError::new(path, e))
        //             }

        //             Extensions::XML =>
        //             {
        //                 let markup = value.to_xml().map_err(|e| IoError::new(path, e))?;
        //                 self.write_str(&path, &markup).map_err(|e| IoError::new(path, e))
        //             }

        //             Extensions::QUICK_BIN =>
        //             {
        //                 let bin = value.to_quick_bin().map_err(|e| IoError::new(path, e))?;
        //                 self.write_bytes(&path, &bin).map_err(|e| IoError::new(path, e))
        //             }

        //             Extensions::TXT =>
        //             {
        //                 let txt = value.serialize(SerializerTxt).map_err(|e| IoError::new(path, e))?;
        //                 self.write_str(&path, &txt).map_err(|e| IoError::new(path, e))
        //             }

        //             _ => Err(IoError::new(&path, EncodeError::UnsupportedExtension { got: (), expected: () }))
        //         }
        //     }else
        //     {

        //     }
        // }


        // match extension
        // {
        //     Extensions::RON =>
        //     {
        //         let markup = value.to_ron().map_err(|e| IoError::new(path, e))?;
        //         self.write_str(&path, &markup).map_err(|e| IoError::new(path, e))
        //     },

        //     Extensions::JSON =>
        //     {
        //         let markup = value.to_json().map_err(|e| IoError::new(path, e))?;
        //         self.write_str(&path, &markup).map_err(|e| IoError::new(path, e))
        //     }

        //     Extensions::XML =>
        //     {
        //         let markup = value.to_xml().map_err(|e| IoError::new(path, e))?;
        //         self.write_str(&path, &markup).map_err(|e| IoError::new(path, e))
        //     }

        //     Extensions::QUICK_BIN =>
        //     {
        //         let bin = value.to_quick_bin().map_err(|e| IoError::new(path, e))?;
        //         self.write_bytes(&path, &bin).map_err(|e| IoError::new(path, e))
        //     }

        //     Extensions::TXT =>
        //     {
        //         let txt = value.serialize(SerializerTxt).map_err(|e| IoError::new(path, e))?;
        //         self.write_str(&path, &txt).map_err(|e| IoError::new(path, e))
        //     }

        //     _ =>
        //     {
        //         let mut ser = SerializerSaveTxtOrBinOrMarkup::new(self, path.to_owned()).with_param(param);
        //         value.serialize(&mut ser)

        //         //todo
        //         // let SaveTxtOrBinUrlOrMarkup { bytes, extension: save_extension } = value.serialize(SerializerSaveTxtOrBinOrMarkup{ extension }).map_err(|e| FileError::from(e))?;
        //         // if extension == GUESS_EXTENSION
        //         // {
        //         //     self.write_bytes(&path.with_extension(&save_extension), &bytes)
        //         // }else
        //         // {
        //         //     self.write_bytes(path, &bytes)
        //         // }
        //     },
        // }
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