use hexga_core::io::IoResult;

use super::*;


pub trait GetPath
{
    fn get_path(&self) -> &Path;
}
impl GetPath for Path
{
    fn get_path(&self) -> &Path { self }
}
impl<P> GetPath for P where P: AsRef<Path>
{
    fn get_path(&self) -> &Path {
        self.as_ref()
    }
}


pub trait SetPath
{
    fn set_path<P : AsRef<Path>>(&mut self, path: P) -> Result<(), P>;
}
impl SetPath for PathBuf
{
    fn set_path<P : AsRef<Path>>(&mut self, path: P) -> Result<(), P> {
        *self = path.as_ref().to_owned();
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
