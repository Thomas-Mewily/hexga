use super::*;

pub type Path = String;
#[allow(non_camel_case_types)]
pub type path = str;

pub type Extension = String;
#[allow(non_camel_case_types)]
pub type extension = str;

pub trait PathExtension
{
    /// Returns the file extension of the path, or an empty string if none exists.
    ///
    /// The returned string does **not** include the dot `.`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::*;
    ///
    /// assert_eq!("file.txt".extension_or_empty(), "txt");
    /// assert_eq!("file".extension_or_empty(), "");
    /// ```
    fn extension_or_empty(&self) -> &extension { self.extension().unwrap_or_default() }

    /// Returns the file extension of the path, if it exists.
    ///
    /// The returned string slice does **not** include the dot `.`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::*;
    ///
    /// assert_eq!("foo/bar.txt".extension(), Some("txt"));
    /// assert_eq!("foo/archive.tar.gz".extension(), Some("gz"));
    /// assert_eq!("foo/bar".extension(), None);
    /// ```
    fn extension(&self) -> Option<&extension>;

    /// Returns true if the path has a file extension.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::*;
    ///
    /// assert_eq!("foo/bar.txt".have_extension(), true);
    /// assert_eq!("foo/bar".have_extension(), false);
    /// ```
    fn have_extension(&self) -> bool { self.extension().is_some() }

    /// Returns a new path with the extension removed.
    ///
    /// Equivalent to `self.with_extension("")`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::*;
    ///
    /// assert_eq!("foo/bar.txt".without_extension(), "foo/bar");
    /// assert_eq!("foo/bar".without_extension(), "foo/bar");
    /// ```
    fn without_extension(&self) -> &path;


    /// Returns a new path with the current extension replaced by `extension`.
    ///
    /// The `extension` argument should **not** include a dot `.`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::*;
    ///
    /// assert_eq!("foo/bar.txt".with_extension("md"), "foo/bar.md");
    /// assert_eq!("foo/bar.txt".with_extension(""), "foo/bar");
    /// assert_eq!("foo/bar".with_extension("md"), "foo/bar.md");
    /// ```
    fn with_extension(&self, extension : &extension) -> Path;


    /// Returns the full file name (including all extensions) of the path.
    ///
    /// If the path ends with a directory or has no file name, returns an empty string `""`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::*;
    ///
    /// assert_eq!("foo/bar.txt".path_fullname(), "bar.txt");
    /// assert_eq!("foo/.hidden".path_fullname(), ".hidden");
    /// assert_eq!("foo/archive.tar.gz".path_fullname(), "archive.tar.gz");
    /// assert_eq!("foo/bar/".path_fullname(), "");
    /// assert_eq!("file".path_fullname(), "file");
    /// ```
    fn path_fullname(&self) -> &path;


    /// Returns the name of the file or directory at the end of the path,
    /// **without the last file extension** (the portion before the final `.` in the filename).
    ///
    /// If the path has no file name, returns an empty string `""`.
    ///
    /// # Behavior
    ///
    /// - If the file name has no `.`: returns the entire file name.
    /// - If the file name begins with `.` and has no other dots: returns the entire file name (e.g., `.gitignore`).
    /// - Otherwise: returns the portion of the file name **before the final `.`**.
    /// - If the path ends with a directory separator or has no file name: returns `""`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::*;
    ///
    /// assert_eq!("foo/bar.txt".path_name(), "bar");
    /// assert_eq!("foo/.hidden".path_name(), ".hidden");
    /// assert_eq!("foo/archive.tar.gz".path_name(), "archive.tar");
    /// assert_eq!("foo/bar/".path_name(), "");
    /// assert_eq!("file".path_name(), "file");
    /// ```
    fn path_name(&self) -> &str;

    fn path_parent(&self) -> &path;

    /// Returns a new file/folder name with the base name replaced by `name`,
    /// preserving the original extension(s).
    ///
    /// The parent directory is not modified. Only the file/folder name is changed.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::StrPathExtension;
    ///
    /// assert_eq!("foo/bar.txt".path_rename("baz"), "baz.txt");
    /// assert_eq!("foo/.hidden".path_rename("config"), "config");
    /// assert_eq!("foo/archive.tar.gz".path_rename("new.gz"), "new.tar.gz"); // the name is "archive.tar"
    /// assert_eq!("file".path_rename("newfile"), "newfile");
    /// ```
    fn path_replace_name(&self, name: &str) -> Path;



    /// Splits the path into its components using `/` or `\` as separators,
    /// and returns them as a vector of strings.
    ///
    /// This method normalizes Windows-style backslashes (`\`) to forward slashes (`/`)
    /// before splitting, so it works consistently across platforms.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::*;
    ///
    /// assert_eq!("foo/bar/baz.txt".path_split(), vec!["foo", "bar", "baz.txt"]);
    /// assert_eq!("foo\\bar\\baz.txt".path_split(), vec!["foo", "bar", "baz.txt"]);
    /// assert_eq!("file.txt".path_split(), vec!["file.txt"]);
    /// assert_eq!("".path_split(), Vec::<String>::new());
    /// assert_eq!("/foo/bar/".path_split(), vec!["", "foo", "bar", ""]);
    /// ```
    fn path_split(&self) -> Vec<String>;

    /// Concatenates this path with another path segment, producing a new `Path`.
    ///
    /// This method ensures there is exactly one `/` between the two components,
    /// avoiding duplicate or missing separators. It works with both forward `/`
    /// and backward `\` slashes in the input path, normalizing them to `/`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::*; // replace with your crate/module path
    ///
    /// assert_eq!("foo/bar".path_concat("baz.txt"), "foo/bar/baz.txt");
    /// assert_eq!("foo/".path_concat("baz.txt"), "foo/baz.txt");
    /// assert_eq!("foo".path_concat("/baz.txt"), "foo/baz.txt");
    /// assert_eq!("foo\\".path_concat("baz.txt"), "foo/baz.txt");
    /// assert_eq!("".path_concat("file.txt"), "file.txt");
    /// assert_eq!("foo/bar".path_concat(""), "foo/bar/");
    /// ```
    fn path_concat(&self, right: &path) -> Path;
}


impl PathExtension for &str
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

    fn without_extension(&self) -> &path {
        match std::path::Path::new(self).file_stem() {
            Some(stem) => stem.to_str().unwrap_or(self),
            None => self,
        }
    }


    fn path_split(&self) -> Vec<Path> {
        self.replace('\\', "/").split('/').map(|v| v.to_owned()).collect()
    }

    fn path_concat(&self, right: &path) -> Path {
        let mut left = self.replace('\\', "/");
        let mut right = right.replace('\\', "/");

        if left.ends_with('/') {
            left.pop();
        }
        while right.starts_with('/') {
            right.remove(0);
        }

        if left.is_empty() {
            right
        } else if right.is_empty() {
            left
        } else {
            format!("{}/{}", left, right)
        }
    }

    fn path_fullname(&self) -> &str {
        let path = std::path::Path::new(self);
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
    }

    fn path_name(&self) -> &str
    {
        let path = std::path::Path::new(self);
        path.file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("")
    }


    fn path_replace_name(&self, name: &str) -> Path {
        let extension = self.extension_or_empty();
        let parent_name = self.path_parent();
        parent_name.path_concat(&name.with_extension(extension))
    }

    fn path_parent(&self) -> &path {
        std::path::Path::new(self)
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
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