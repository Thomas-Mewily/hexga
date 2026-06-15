use std::ffi::OsStr;

use super::*;

pub type StdPath = std::path::Path;
pub type StdPathBuf = std::path::PathBuf;

/// Similar to [`std::path::Path`] but don't contains any method that rely on the physical file system.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[allow(non_camel_case_types)]
pub struct path
{
    path: StdPath,
}
impl std::fmt::Debug for path
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.path)
    }
}
impl std::fmt::Display for path
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path.display())
    }
}

impl<'a> From<&'a StdPath> for &'a path
{
    fn from(value: &'a StdPath) -> Self { path::new(value) }
}
impl<'a> From<&'a path> for &'a StdPath
{
    fn from(value: &'a path) -> Self { &value.path }
}

impl path
{
    pub fn new<'a, S: AsRef<StdPath> + ?Sized>(p: &'a S) -> &'a path
    {
        let std_path = p.as_ref();
        unsafe { &*(std_path as *const StdPath as *const path) }
    }

    fn new_mut<'a>(std_path: &'a mut StdPath) -> &'a mut path
    {
        // SAFETY: Path is just a wrapper around OsStr,
        // therefore converting &mut OsStr to &mut Path is safe.
        unsafe { &mut *(std_path.as_mut_os_str() as *mut OsStr as *mut Path) }
    }

    /// Makes the path absolute without accessing the filesystem.
    /// 
    /// This is an alias to [`path::absolute`](absolute).
    ///
    /// # Errors
    ///
    /// This function may return an error in the following situations:
    ///
    /// * If the path is syntactically invalid; in particular, if it is empty.
    /// * If getting the [current directory][crate::env::current_dir] fails.
    #[must_use]
    pub fn absolute(&self) -> IoResult<Path> {
        Ok(Path::from(std::path::absolute(&self.path)?))
    }

    /// Produces an iterator over `Path` and its ancestors.
    ///
    /// The iterator will yield the `Path` that is returned if the [`parent`] method is used zero
    /// or more times. If the [`parent`] method returns [`None`], the iterator will do likewise.
    /// The iterator will always yield at least one value, namely `Some(&self)`. Next it will yield
    /// `&self.parent()`, `&self.parent().and_then(Path::parent)` and so on.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// let mut ancestors = Path::new("/foo/bar").ancestors();
    /// assert_eq!(ancestors.next(), Some(Path::new("/foo/bar")));
    /// assert_eq!(ancestors.next(), Some(Path::new("/foo")));
    /// assert_eq!(ancestors.next(), Some(Path::new("/")));
    /// assert_eq!(ancestors.next(), None);
    ///
    /// let mut ancestors = Path::new("../foo/bar").ancestors();
    /// assert_eq!(ancestors.next(), Some(Path::new("../foo/bar")));
    /// assert_eq!(ancestors.next(), Some(Path::new("../foo")));
    /// assert_eq!(ancestors.next(), Some(Path::new("..")));
    /// assert_eq!(ancestors.next(), Some(Path::new("")));
    /// assert_eq!(ancestors.next(), None);
    /// ```
    ///
    /// [`parent`]: Path::parent
    #[inline]
    pub fn ancestors(&self) -> std::path::Ancestors<'_> {
        self.path.ancestors()
    }

    /// Returns the final component of the `Path`, if there is one.
    ///
    /// If the path is a normal file, this is the file name. If it's the path of a directory, this
    /// is the directory name.
    ///
    /// Returns [`None`] if the path terminates in `..`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use std::ffi::OsStr;
    ///
    /// assert_eq!(Some(OsStr::new("bin")), Path::new("/usr/bin/").file_name());
    /// assert_eq!(Some(OsStr::new("foo.txt")), Path::new("tmp/foo.txt").file_name());
    /// assert_eq!(Some(OsStr::new("foo.txt")), Path::new("foo.txt/.").file_name());
    /// assert_eq!(Some(OsStr::new("foo.txt")), Path::new("foo.txt/.//").file_name());
    /// assert_eq!(None, Path::new("foo.txt/..").file_name());
    /// assert_eq!(None, Path::new("/").file_name());
    /// ```
    #[doc(alias = "basename")]
    #[must_use]
    pub fn file_name(&self) -> Option<&OsStr> {
        self.path.file_name()
    }

    /// Returns a path that, when joined onto `base`, yields `self`.
    ///
    /// # Errors
    ///
    /// If `base` is not a prefix of `self` (i.e., [`starts_with`]
    /// returns `false`), returns [`Err`].
    ///
    /// [`starts_with`]: Path::starts_with
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::{Path, PathBuf};
    ///
    /// let path = Path::new("/test/haha/foo.txt");
    ///
    /// assert_eq!(path.strip_prefix("/"), Ok(Path::new("test/haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test"), Ok(Path::new("haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test/"), Ok(Path::new("haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test/haha/foo.txt"), Ok(Path::new("")));
    /// assert_eq!(path.strip_prefix("/test/haha/foo.txt/"), Ok(Path::new("")));
    ///
    /// assert!(path.strip_prefix("test").is_err());
    /// assert!(path.strip_prefix("/te").is_err());
    /// assert!(path.strip_prefix("/haha").is_err());
    ///
    /// let prefix = PathBuf::from("/test/");
    /// assert_eq!(path.strip_prefix(prefix), Ok(Path::new("haha/foo.txt")));
    /// ```
    pub fn strip_prefix<P>(&self, base: P) -> Result<&path, std::path::StripPrefixError>
    where
        P: AsRef<path>,
    {
        Ok(Self::new(self.path.strip_prefix(&base.as_ref().path)?))
    }

    /// Determines whether `base` is a prefix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// let path = Path::new("/etc/passwd");
    ///
    /// assert!(path.starts_with("/etc"));
    /// assert!(path.starts_with("/etc/"));
    /// assert!(path.starts_with("/etc/passwd"));
    /// assert!(path.starts_with("/etc/passwd/")); // extra slash is okay
    /// assert!(path.starts_with("/etc/passwd///")); // multiple extra slashes are okay
    ///
    /// assert!(!path.starts_with("/e"));
    /// assert!(!path.starts_with("/etc/passwd.txt"));
    ///
    /// assert!(!Path::new("/etc/foo.rs").starts_with("/etc/foo"));
    /// ```
    #[must_use]
    pub fn starts_with<P: AsRef<path>>(&self, base: P) -> bool {
        self.path.starts_with(&base.as_ref().path)
    }

    /// Determines whether `child` is a suffix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// let path = Path::new("/etc/resolv.conf");
    ///
    /// assert!(path.ends_with("resolv.conf"));
    /// assert!(path.ends_with("etc/resolv.conf"));
    /// assert!(path.ends_with("/etc/resolv.conf"));
    ///
    /// assert!(!path.ends_with("/resolv.conf"));
    /// assert!(!path.ends_with("conf")); // use .extension() instead
    /// ```
    #[must_use]
    pub fn ends_with<P: AsRef<path>>(&self, child: P) -> bool {
        self.path.ends_with(&child.as_ref().path)
    }

    /// Extracts the stem (non-extension) portion of [`self.file_name`].
    ///
    /// [`self.file_name`]: Path::file_name
    ///
    /// The stem is:
    ///
    /// * [`None`], if there is no file name;
    /// * The entire file name if there is no embedded `.`;
    /// * The entire file name if the file name begins with `.` and has no other `.`s within;
    /// * Otherwise, the portion of the file name before the final `.`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// assert_eq!("foo", Path::new("foo.rs").file_stem().unwrap());
    /// assert_eq!("foo.tar", Path::new("foo.tar.gz").file_stem().unwrap());
    /// ```
    ///
    /// # See Also
    /// This method is similar to [`Path::file_prefix`], which extracts the portion of the file name
    /// before the *first* `.`
    ///
    /// [`Path::file_prefix`]: Path::file_prefix
    ///
    #[must_use]
    pub fn file_stem(&self) -> Option<&OsStr> {
        self.path.file_stem()
    }

    /// Extracts the prefix of [`self.file_name`].
    ///
    /// The prefix is:
    ///
    /// * [`None`], if there is no file name;
    /// * The entire file name if there is no embedded `.`;
    /// * The portion of the file name before the first non-beginning `.`;
    /// * The entire file name if the file name begins with `.` and has no other `.`s within;
    /// * The portion of the file name before the second `.` if the file name begins with `.`
    ///
    /// [`self.file_name`]: Path::file_name
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// assert_eq!("foo", Path::new("foo.rs").file_prefix().unwrap());
    /// assert_eq!("foo", Path::new("foo.tar.gz").file_prefix().unwrap());
    /// assert_eq!(".config", Path::new(".config").file_prefix().unwrap());
    /// assert_eq!(".config", Path::new(".config.toml").file_prefix().unwrap());
    /// ```
    ///
    /// # See Also
    /// This method is similar to [`Path::file_stem`], which extracts the portion of the file name
    /// before the *last* `.`
    ///
    /// [`Path::file_stem`]: Path::file_stem
    ///
    #[must_use]
    pub fn file_prefix(&self) -> Option<&OsStr> {
        self.path.file_prefix()
    }

    /// Extracts the extension (without the leading dot) of [`self.file_name`], if possible.
    ///
    /// The extension is:
    ///
    /// * [`None`], if there is no file name;
    /// * [`None`], if there is no embedded `.`;
    /// * [`None`], if the file name begins with `.` and has no other `.`s within;
    /// * Otherwise, the portion of the file name after the final `.`
    ///
    /// [`self.file_name`]: Path::file_name
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// assert_eq!("rs", Path::new("foo.rs").extension().unwrap());
    /// assert_eq!("gz", Path::new("foo.tar.gz").extension().unwrap());
    /// ```
    #[must_use]
    pub fn extension(&self) -> Option<&OsStr> {
        self.path.extension()
    }

    /// Creates an owned [`PathBuf`] with `path` adjoined to `self`.
    ///
    /// If `path` is absolute, it replaces the current path.
    ///
    /// On Windows:
    ///
    /// * if `path` has a root but no prefix (e.g., `\windows`), it
    ///   replaces and returns everything except for the prefix (if any) of `self`.
    /// * if `path` has a prefix but no root, `self` is ignored and `path` is returned.
    /// * if `self` has a verbatim prefix (e.g. `\\?\C:\windows`)
    ///   and `path` is not empty, the new path is normalized: all references
    ///   to `.` and `..` are removed.
    ///
    /// See [`PathBuf::push`] for more details on what it means to adjoin a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::{Path, PathBuf};
    ///
    /// assert_eq!(Path::new("/etc").join("passwd"), PathBuf::from("/etc/passwd"));
    /// assert_eq!(Path::new("/etc").join("/bin/sh"), PathBuf::from("/bin/sh"));
    /// ```
    #[must_use]
    pub fn join<P: AsRef<path>>(&self, path: P) -> Path {
        self.path.join(&path.as_ref().path).into()
    }

    /// Creates an owned [`PathBuf`] like `self` but with the given file name.
    ///
    /// See [`PathBuf::set_file_name`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::{Path, PathBuf};
    ///
    /// let path = Path::new("/tmp/foo.png");
    /// assert_eq!(path.with_file_name("bar"), PathBuf::from("/tmp/bar"));
    /// assert_eq!(path.with_file_name("bar.txt"), PathBuf::from("/tmp/bar.txt"));
    ///
    /// let path = Path::new("/tmp");
    /// assert_eq!(path.with_file_name("var"), PathBuf::from("/var"));
    /// ```
    #[must_use]
    pub fn with_file_name<S: AsRef<OsStr>>(&self, file_name: S) -> Path {
        self.path.with_file_name(file_name).into()
    }

    /// Creates an owned [`PathBuf`] like `self` but with the given extension.
    ///
    /// See [`PathBuf::set_extension`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// let path = Path::new("foo.rs");
    /// assert_eq!(path.with_extension("txt"), Path::new("foo.txt"));
    /// assert_eq!(path.with_extension(""), Path::new("foo"));
    /// ```
    ///
    /// Handling multiple extensions:
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// let path = Path::new("foo.tar.gz");
    /// assert_eq!(path.with_extension("xz"), Path::new("foo.tar.xz"));
    /// assert_eq!(path.with_extension("").with_extension("txt"), Path::new("foo.txt"));
    /// ```
    ///
    /// Adding an extension where one did not exist:
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// let path = Path::new("foo");
    /// assert_eq!(path.with_extension("rs"), Path::new("foo.rs"));
    /// ```
    pub fn with_extension<S: AsRef<OsStr>>(&self, extension: S) -> Path {
        self.path.with_extension(extension).into()
    }

    /// Creates an owned [`PathBuf`] like `self` but with the extension added.
    ///
    /// See [`PathBuf::add_extension`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::{Path, PathBuf};
    ///
    /// let path = Path::new("foo.rs");
    /// assert_eq!(path.with_added_extension("txt"), PathBuf::from("foo.rs.txt"));
    ///
    /// let path = Path::new("foo.tar.gz");
    /// assert_eq!(path.with_added_extension(""), PathBuf::from("foo.tar.gz"));
    /// assert_eq!(path.with_added_extension("xz"), PathBuf::from("foo.tar.gz.xz"));
    /// assert_eq!(path.with_added_extension("").with_added_extension("txt"), PathBuf::from("foo.tar.gz.txt"));
    /// ```
    pub fn with_added_extension<S: AsRef<OsStr>>(&self, extension: S) -> Path {
        self.path.with_added_extension(extension).into()
    }

    /// Produces an iterator over the [`Component`]s of the path.
    ///
    /// When parsing the path, there is a small amount of normalization:
    ///
    /// * Repeated separators are ignored, so `a/b` and `a//b` both have
    ///   `a` and `b` as components.
    ///
    /// * Occurrences of `.` are normalized away, except if they are at the
    ///   beginning of the path. For example, `a/./b`, `a/b/`, `a/b/.` and
    ///   `a/b` all have `a` and `b` as components, but `./a/b` starts with
    ///   an additional [`CurDir`] component.
    ///
    /// * Trailing separators are normalized away, so `/a/b` and `/a/b/` are equivalent.
    ///
    /// Note that no other normalization takes place; in particular, `a/c`
    /// and `a/b/../c` are distinct, to account for the possibility that `b`
    /// is a symbolic link (so its parent isn't `a`).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::{Path, Component};
    /// use std::ffi::OsStr;
    ///
    /// let mut components = Path::new("/tmp/foo.txt").components();
    ///
    /// assert_eq!(components.next(), Some(Component::RootDir));
    /// assert_eq!(components.next(), Some(Component::Normal(OsStr::new("tmp"))));
    /// assert_eq!(components.next(), Some(Component::Normal(OsStr::new("foo.txt"))));
    /// assert_eq!(components.next(), None)
    /// ```
    ///
    /// [`CurDir`]: Component::CurDir
    pub fn components(&self) -> std::path::Components<'_> {
        self.path.components()   
    }

    /// Produces an iterator over the path's components viewed as [`OsStr`]
    /// slices.
    ///
    /// For more information about the particulars of how the path is separated
    /// into components, see [`components`].
    ///
    /// [`components`]: Path::components
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::{self, Path};
    /// use std::ffi::OsStr;
    ///
    /// let mut it = Path::new("/tmp/foo.txt").iter();
    /// assert_eq!(it.next(), Some(OsStr::new(&path::MAIN_SEPARATOR.to_string())));
    /// assert_eq!(it.next(), Some(OsStr::new("tmp")));
    /// assert_eq!(it.next(), Some(OsStr::new("foo.txt")));
    /// assert_eq!(it.next(), None)
    /// ```
    #[inline]
    pub fn iter(&self) -> std::path::Iter<'_> {
        self.path.iter()
    }
}

/// Similar to [`std::path::PathBuf`] but don't contains any method that rely on the physical file system.
#[derive(Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Path
{
    path: StdPathBuf,
}
impl Deref for Path
{
    type Target=path;
    fn deref(&self) -> &Self::Target {
        path::new(&self.path)
    }
}
impl DerefMut for Path
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        path::new_mut(&mut self.path)
    }
}
impl std::fmt::Debug for Path
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.path)
    }
}
impl std::fmt::Display for Path
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.path.display())
    }
}
impl From<StdPathBuf> for Path
{
    fn from(path: StdPathBuf) -> Self { Self { path } }
}
impl From<Path> for StdPathBuf
{
    fn from(value: Path) -> Self { value.path }
}
impl Path
{
    pub const fn new() -> Self { Self { path: StdPathBuf::new() } }
    pub fn with_capacity(capacity: usize) -> Self { Self { path: StdPathBuf::with_capacity(capacity) } }
}
impl WithCapacity for Path
{
    type Param = ();
    fn with_capacity_and_param(capacity: usize, _param: Self::Param) -> Self {
        Self::with_capacity(capacity)
    }
}