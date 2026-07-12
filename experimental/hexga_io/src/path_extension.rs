use hexga_core::io::IoResult;

use super::*;

pub trait GetPath
{
    fn get_path(&self) -> Option<PathBuf>;
}

impl GetPath for Path
{
    fn get_path(&self) -> Option<PathBuf> { Some(self.to_path_buf()) }
}
impl GetPath for Option<&Path>
{
    fn get_path(&self) -> Option<PathBuf> { self.map(|p| p.get_path()).flatten() }
}
impl<E> GetPath for Result<&Path, E>
{
    fn get_path(&self) -> Option<PathBuf>
    {
        match self
        {
            Ok(p) => Some(p.to_path_buf()),
            Err(_) => None,
        }
    }
}

impl GetPath for PathBuf
{
    fn get_path(&self) -> Option<PathBuf> { Some(self.clone()) }
}
impl GetPath for Option<&PathBuf>
{
    fn get_path(&self) -> Option<PathBuf> { self.map(|p| p.get_path()).flatten() }
}
impl<E> GetPath for Result<&PathBuf, E>
{
    fn get_path(&self) -> Option<PathBuf>
    {
        match self
        {
            Ok(p) => Some((*p).clone()),
            Err(_) => None,
        }
    }
}

impl GetPath for &str
{
    fn get_path(&self) -> Option<PathBuf> { Some(self.into()) }
}
impl GetPath for Option<&str>
{
    fn get_path(&self) -> Option<PathBuf> { self.map(|p| p.into()) }
}
impl<E> GetPath for Result<&str, E>
{
    fn get_path(&self) -> Option<PathBuf>
    {
        match self
        {
            Ok(p) => Some(p.into()),
            Err(_) => None,
        }
    }
}

/*
impl<P> GetPath for P where P: AsRef<Path>
{
    fn get_path(&self) -> Option<&Path> {
        Some(self.as_ref())
    }
}
*/

pub trait SetPath
{
    /// Change the path without moving the old file.
    fn set_path<P: AsRef<Path>>(&mut self, path: Option<P>) -> IoResult;

    /// Change the path and move the old file.
    /// Do not change the file path if the renaming failed.
    fn rename_path<P: AsRef<Path>>(&mut self, to: P) -> IoResult;
}
impl SetPath for PathBuf
{
    fn set_path<P: AsRef<Path>>(&mut self, path: Option<P>) -> IoResult
    {
        match path
        {
            Some(new_path) =>
            {
                *self = new_path.as_ref().to_owned();
            }
            None =>
            {}
        }
        Ok(())
    }

    fn rename_path<P: AsRef<Path>>(&mut self, to: P) -> IoResult { self.set_path(Some(to)) }
}
impl SetPath for String
{
    fn set_path<P: AsRef<Path>>(&mut self, path: Option<P>) -> IoResult
    {
        match path
        {
            Some(new_path) =>
            {
                *self = new_path
                    .as_ref()
                    .to_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| IoError::new(IoErrorKind::InvalidFilename, "Path contains invalid UTF-8"))?;
            }
            None =>
            {}
        }
        Ok(())
    }
    fn rename_path<P: AsRef<Path>>(&mut self, to: P) -> IoResult { self.set_path(Some(to)) }
}

/*
pub trait PathExtension
{
    //fn extension_or_empty(&self) -> &str;
    fn exists<F>(&self, fs: &mut F) -> bool where F: FsRead;
}
impl PathExtension for Path
{
    fn exists<F>(&self, fs: &mut F) -> bool where F: FsRead { fs.exist(self) }

    /*
    fn extension_or_empty(&self) -> &str
    {
        match self.extension()
        {
            Some(ex) => match ex.to_str()
            {
                Some(e) => e,
                None => "",
            },
            None => "",
        }
    }*/
}
*/

/*
pub trait FsPath : AsRef<Path> //+ Into<PathBuf> {}
impl<P> FsPath for P where P: AsRef<Path> //+ Into<PathBuf> {}
*/
