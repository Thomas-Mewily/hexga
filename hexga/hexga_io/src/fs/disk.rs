use std::path::PathBuf;

use super::*;

type StdPath = std::path::Path;

pub struct FsDisk;


impl FsWrite for FsDisk
{
    fn write_bytes(&mut self, path: &path, bytes: &[u8]) ->  IoResult {
        let std_path = StdPath::new(path);

        if let Some(parent) = std_path.parent()
        {
            let mut current = PathBuf::new();

            for component in parent.components() {
                current.push(component);

                if current.exists() && current.is_file() {
                    // Remove a file to make way for a directory
                    std::fs::remove_file(&current)?;
                }

                // Create directory if missing
                if !current.exists() {
                    std::fs::create_dir(&current)?;
                }
            }
        }

        if std_path.exists() && std_path.is_dir()
        {
            std::fs::remove_dir_all(std_path)?;
        }

        std::fs::write(std_path, bytes).map_err(Into::into)
    }

    fn delete(&mut self, path: &path) -> IoResult {
        let std_path = StdPath::new(path);
        if std_path.is_dir() {
            std::fs::remove_dir_all(std_path).map_err(Into::into)
        } else if std_path.is_file() {
            std::fs::remove_file(std_path).map_err(Into::into)
        } else {
            Ok(())
        }
    }

    fn move_to(&mut self, path: &path, new_path: &path) -> IoResult {
        let src = StdPath::new(path);
        let dst = StdPath::new(new_path);
        std::fs::rename(src, dst).map_err(Into::into)
    }
}


impl FsRead for FsDisk
{
    fn read_bytes<'a>(&'a mut self, path: &path) ->  IoResult<Cow<'a, [u8]>>
    {
        let std_path = StdPath::new(path);
        std::fs::read(std_path).map(Cow::Owned).map_err(Into::into)
    }

    fn entries_names(&mut self, path: &path) -> Vec<String> {
        let std_path = StdPath::new(path);
        let mut names = Vec::new();

        if let Ok(entries) = std::fs::read_dir(std_path)
        {
            for entry in entries.flatten()
            {
                if let Some(name) = entry.file_name().to_str() {
                    names.push(name.to_owned());
                }
            }
        }
        names
    }

    fn node_kind(&mut self, path: &path) -> IoResult<FsNodeKind> {
        let std_path = StdPath::new(path);
        if std_path.is_dir() { return Ok(FsNodeKind::Directoy) }
        if std_path.is_file() { return Ok(FsNodeKind::File) }
        Err(IoError::NotFound)
    }
}


pub trait SaveToDisk : Save
{
    fn save_to_disk<P>(&self, path: P) -> IoResult where P: AsRefPath
    {
        let mut disk = FsDisk;
        self.save_to(path, &mut disk)
        // commit the fs here ?
    }
}
impl<T> SaveToDisk for T where T: Save + ?Sized {}

pub trait LoadFromDisk : Load
{
    fn load_from_disk<P>(path: P) -> IoResult<Self> where P: AsRefPath
    {
        Self::load_from(path, &mut FsDisk)
    }
}
impl<T> LoadFromDisk for T where T: Load + ?Sized {}