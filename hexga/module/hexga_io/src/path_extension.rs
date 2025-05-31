use crate::*;

pub trait StrPathExtension
{
    /// Don't include the dot '.'
    fn extension_or_empty(&self) -> &extension { self.extension().unwrap_or_default() }

    /// Don't include the dot '.'
    fn extension(&self) -> Option<&extension>;
    fn have_extension(&self) -> bool { self.extension().is_some() }

    fn without_extension(&self) -> Path { self.with_extension("") }
    /// replace the current extension by this one
    fn with_extension(&self, extension : &extension) -> Path;

    fn path_split(&self) -> Vec<Path>;

    //fn is_markup_extension(&self) -> bool;

    /*
    fn file_name_and_extension(&self) -> &str;
    fn with_file_name_and_extension(&self, file_name_and_extension : &str) -> String;

    /// Without extension
    fn file_name(&self) -> &str;
    /// Without extension
    fn with_file_name(&self, file_name_without_extension : &str) -> String;
    */
}


impl StrPathExtension for &str
{
    fn extension(&self) -> Option<&extension>
    {
        match std::path::Path::new(self).extension()
        {
            Some(ex) => ex.to_str(),
            None => None,
        }
    }

    fn with_extension(&self, extension : &extension) -> Path {
        let p = std::path::Path::new(self).with_extension(extension);
        p.into_os_string().into_string().unwrap_or(Path::new())
    }


    fn path_split(&self) -> Vec<Path> {
        self.replace('\\', "/").split('/').map(|v| v.to_owned()).collect()
    }

    //fn is_markup_extension(&self) -> bool { Io::ALL_MARKUP_LANGAGE_EXTENSION.contains(self) }
    /*
    fn file_name_and_extension(&self) -> &str {
        let path = std::path::Path::new(self);
        match path.file_name() {
            Some(file) => file.to_str().unwrap_or(self),
            None => self,
        }
    }

    fn with_file_name_and_extension(&self, file_name_and_extension : &str) -> String {
        let p = std::path::Path::new(self).with_file_name(file_name_and_extension);
        p.into_os_string().into_string().unwrap_or(String::new())
    }


    /// Without extension
    fn file_name(&self) -> &str
    {
        let path = std::path::Path::new(self);
        match path.file_stem() {
            Some(stem) => stem.to_str().unwrap_or(self),
            None => self,
        }
    }

    /// Without extension
    fn with_file_name(&self, file_name_without_extension : &str) -> String
    {
        match self.extension()
        {
            Some(ex) => self.with_file_name_and_extension(&file_name_without_extension.with_extension(ex)),
            None => self.with_file_name_and_extension(file_name_without_extension),
        }
    }*/
}