use hexga_core::io::IoResult;

use super::*;


pub trait GetPath
{
    fn get_path(&self) -> Option<&Path>;
}

impl GetPath for Path
{
    fn get_path(&self) -> Option<&Path> { Some(self) }
}
impl GetPath for Option<&Path>
{
    fn get_path(&self) -> Option<&Path> 
    { 
        self.map(|p| p.get_path()).flatten()
    }
}
impl<E> GetPath for Result<&Path,E>
{
    fn get_path(&self) -> Option<&Path> 
    { 
        match self
        {
            Ok(p) => Some(p),
            Err(_) => None,
        }
    }
}

impl GetPath for PathBuf
{
    fn get_path(&self) -> Option<&Path> { Some(self) }
}
impl GetPath for Option<&PathBuf>
{
    fn get_path(&self) -> Option<&Path> 
    { 
        self.map(|p| p.get_path()).flatten()
    }
}
impl<E> GetPath for Result<&PathBuf,E>
{
    fn get_path(&self) -> Option<&Path> 
    { 
        match self
        {
            Ok(p) => Some(p),
            Err(_) => None,
        }
    }
}

impl GetPath for &str
{
    fn get_path(&self) -> Option<&Path> { Some(self.as_ref()) }
}
impl GetPath for Option<&str>
{
    fn get_path(&self) -> Option<&Path> 
    { 
        self.map(|p| p.as_ref())
    }
}
impl<E> GetPath for Result<&str,E>
{
    fn get_path(&self) -> Option<&Path> 
    { 
        match self
        {
            Ok(p) => Some(p.as_ref()),
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
    fn set_path<P : AsRef<Path>>(&mut self, path: impl Into<Option<P>>) -> Result<(), Option<P>>;
}
impl SetPath for PathBuf
{
    fn set_path<P : AsRef<Path>>(&mut self, path: impl Into<Option<P>>) -> Result<(), Option<P>> {
        match path.into()
        {
            Some(new_path) => { *self = new_path.as_ref().to_owned(); },
            None => {},
        }
        Ok(())
    }
}
impl SetPath for String
{
    fn set_path<P: AsRef<Path>>(&mut self, path: impl Into<Option<P>>) -> Result<(), Option<P>> {
        match path.into()
        {
            Some(new_path) => {
                *self = new_path.as_ref()
                    .to_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| new_path)?;
            },
            None => {},
        }
        Ok(())
    }
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
