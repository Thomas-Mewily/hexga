use super::*;


type StdPath = std::path::Path;

pub struct FileSystemOS;


impl FileSystem for FileSystemOS
{
    fn read_bytes<'a>(&'a mut self, path: &path) ->  IoResult<Cow<'a, [u8]>>
    {
        let std_path = StdPath::new(path);
        std::fs::read(std_path).map(Cow::Owned).err_into()
    }

    fn write_bytes(&mut self, path: &path, bytes: Vec<u8>) ->  IoResult {
        let std_path = StdPath::new(path);

        // Ensure the parent directory exists
        if let Some(parent) = std_path.parent()
        {
            std::fs::create_dir_all(parent).map_err(|e| IoError::from(e))?;
        }
        std::fs::write(std_path, bytes).err_into()
    }

    fn subpath_name(&mut self, path: &path) -> Vec<String> {
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

    fn delete(&mut self, path: &path) -> IoResult {
        let std_path = StdPath::new(path);
        if std_path.is_dir() {
            std::fs::remove_dir_all(std_path).err_into()
        } else if std_path.is_file() {
            std::fs::remove_file(std_path).err_into()
        } else {
            Ok(())
        }
    }

    fn node_kind(&mut self, path: &path) -> IoResult<FsNodeKind> {
        let std_path = StdPath::new(path);
        if std_path.is_dir() { return Ok(FsNodeKind::Directoy) }
        if std_path.is_file() { return Ok(FsNodeKind::File) }
        Err(IoError)
    }

    fn change_path(&mut self, path: &path, new_path: &path) -> IoResult {
        let src = StdPath::new(path);
        let dst = StdPath::new(new_path);
        std::fs::rename(src, dst).err_into()
    }
}
