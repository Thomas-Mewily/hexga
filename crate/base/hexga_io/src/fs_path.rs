use super::*;


#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PathBuf
{
    inner: String
}
impl Into<PathBuf> for &PathBuf { fn into(self) -> PathBuf { self.clone() } }
impl From<&str> for PathBuf { fn from(value: &str) -> Self { Self::from(value.to_owned()) } }
impl From<&Path> for PathBuf { fn from(value: &Path) -> Self { value.to_owned() } }
impl From<String> for PathBuf { fn from(value: String) -> Self { Self::new(value) } }
impl From<PathBuf> for String { fn from(value: PathBuf) -> Self { value.inner } }
impl From<&Path> for String { fn from(value: &Path) -> Self { value.as_str().into() } }
impl PartialEq<Path> for PathBuf
{
    #[inline(always)]
    fn eq(&self, other: &Path) -> bool { (**self).eq(other) }
    #[inline(always)]
    fn ne(&self, other: &Path) -> bool { (**self).ne(other) }
}
impl PartialEq<str> for PathBuf
{
    #[inline(always)]
    fn eq(&self, other: &str) -> bool { self.as_str().eq(other) }
    #[inline(always)]
    fn ne(&self, other: &str) -> bool { self.as_str().ne(other) }
}
impl PartialEq<String> for PathBuf
{
    #[inline(always)]
    fn eq(&self, other: &String) -> bool { self.inner.eq(other) }
    #[inline(always)]
    fn ne(&self, other: &String) -> bool { self.inner.ne(other) }
}


impl PathBuf
{
    pub const fn empty() -> Self { Self{ inner: String::new() }}

    /// Create a new path and auto_correct it.
    pub fn new<S: Into<String>>(path: S) -> Self
    {
        let mut s = Self { inner: path.into() };
        s.auto_correct();
        s
    }

    /// '\' are replaced by '/'
    pub fn auto_correct(&mut self)
    {
        // Iterate over bytes and replace '\' with '/'
        unsafe {
            let bytes = self.inner.as_mut_vec();
            for b in bytes.iter_mut() {
                if *b == b'\\' {
                    *b = b'/';
                }
            }
        }
    }

    /// Replace the current extension by the given `extension`.
    ///
    /// This method replaces the file extension of the path with the provided one.
    /// If the path has no extension, the new one is simply appended.
    ///
    /// A leading dot (`.`) in the provided extension is ignored automatically.
    ///
    /// To remove the extension entirely, you can pass an empty string (`""`),
    /// but the the method [`Self::remove_extension`] is more appropriate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexga_io::Path;
    ///
    /// assert_eq!(Path::new("foo/bar.txt").set_extension("md").as_str(), "foo/bar.md");
    ///
    /// // Leading dot is optional
    /// assert_eq!(Path::new("foo/bar.txt").set_extension(".md").as_str(), "foo/bar.md");
    ///
    /// // Remove the extension entirely
    /// assert_eq!(Path::new("foo/bar.txt").set_extension("").as_str(), "foo/bar");
    ///
    /// // Add a new extension if none existed
    /// assert_eq!(Path::new("foo/bar").set_extension("md").as_str(), "foo/bar.md");
    ///
    /// // Handle compound extensions
    /// assert_eq!(Path::new("foo/archive").set_extension("tar.gz").as_str(), "foo/archive.tar.gz");
    /// assert_eq!(Path::new("foo/archive").set_extension(".tar.gz").as_str(), "foo/archive.tar.gz");
    /// ```
    pub fn set_extension(&mut self, extension: &extension) -> &mut Self
    {
        // Todo: opti it
        *self = self.with_extension(extension);
        self
    }

    pub fn remove_extension(&mut self) -> &mut Self
    {
        self.set_extension("")
    }

    pub fn clear(&mut self)
    {
        self.inner.clear();
    }

    pub fn push(&mut self, right: &Path)
    {
        if right.is_empty() { return; }
        if self.is_empty()
        {
            self.inner.clear();
            self.inner.push_str(right.as_str());
            return;
        }
        if !self.inner.ends_with('/')
        {
            self.inner.push('/');
        }
        self.inner.push_str(right.as_str());
    }
}


impl<T> Div<T> for &PathBuf where T: AsRef<str>
{
    type Output=PathBuf;
    fn div(self, rhs: T) -> Self::Output {
        self.to_owned() / rhs
    }
}
impl<T> Div<T> for PathBuf where T: AsRef<str>
{
    type Output=PathBuf;
    fn div(mut self, rhs: T) -> Self::Output {
        self.push(&PathBuf::new(rhs.as_ref().to_owned()));
        self
    }
}
impl<T> DivAssign<T> for PathBuf where T: AsRef<str>
{
    fn div_assign(&mut self, rhs: T) {
        self.push(&PathBuf::new(rhs.as_ref().to_owned()));
    }
}


impl<T> Div<T> for &Path where T: AsRef<str>
{
    type Output=PathBuf;
    fn div(self, rhs: T) -> Self::Output {
        self.to_owned() / rhs
    }
}


impl std::fmt::Debug for PathBuf
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
impl std::fmt::Display for PathBuf
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Deref for PathBuf
{
    type Target=Path;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        Path::from_str(&self.inner)
    }
}
impl DerefMut for PathBuf {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        Path::from_str_mut(&mut self.inner)
    }
}


/// '\' are replaced by '/'
#[allow(non_camel_case_types)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Path
{
    path: str
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
        write!(f, "{}", &self.path)
    }
}
impl PartialEq<str> for Path
{
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq(other)
    }
    #[inline(always)]
    fn ne(&self, other: &str) -> bool {
        self.as_str().ne(other)
    }
}
impl PartialEq<String> for Path
{
    #[inline(always)]
    fn eq(&self, other: &String) -> bool {
        self.path.eq(other)
    }
    #[inline(always)]
    fn ne(&self, other: &String) -> bool {
        self.path.ne(other)
    }
}
impl PartialEq<PathBuf> for Path
{
    #[inline(always)]
    fn eq(&self, other: &PathBuf) -> bool {
        self.path.eq(&other.inner)
    }
    #[inline(always)]
    fn ne(&self, other: &PathBuf) -> bool {
        self.path.ne(&other.inner)
    }
}


impl AsRef<std::path::Path> for Path
{
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(self.as_str())
    }
}
impl AsRef<std::path::Path> for PathBuf
{
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(self.as_str())
    }
}

impl Path
{
    pub const fn empty() -> &'static Self { Self::from_str("") }

    pub const fn as_str(&self) -> &str { &self.path }

    #[inline(always)]
    pub const fn from_str(s: &str) -> &Self
    {
        // SAFETY: `path` is a `repr(transparent)`-like view over String's bytes
        unsafe { &*(s as *const str as *const Path) }
    }

    #[inline(always)]
    pub const fn from_str_mut(s: &mut str) -> &mut Self
    {
        // SAFETY: path is a transparent wrapper around str
        // We have &mut self.inner, so it's safe to produce a &mut path
        unsafe { &mut *(s as *mut str as *mut Path) }
    }
}
impl Path
{
    pub fn is_empty(&self) -> bool { self.as_str().is_empty() }

    /// Returns the file extension of the path, if it exists.
    ///
    /// The returned string slice does **not** include the dot `.`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::prelude::*;
    ///
    /// assert_eq!(Path::new("foo/bar.txt").extension_or_empty(), "txt");
    /// assert_eq!(Path::new("foo/archive.tar.gz").extension_or_empty(), "tar.gz");
    /// assert_eq!(Path::new("foo/bar").extension_or_empty(), "");
    /// ```
    pub fn extension_or_empty(&self) -> &extension { self.extension().unwrap_or("")}

    /// Returns true if the path has a file extension.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::prelude::*;
    ///
    /// assert_eq!(Path::new("foo/bar.txt").have_extension(), true);
    /// assert_eq!(Path::new("foo/archive.tar.gz").have_extension(), true);
    ///
    /// assert_eq!(Path::new("foo/bar").have_extension(), false);
    /// assert_eq!(Path::new("foo/.bar").have_extension(), false);
    /// ```
    pub fn have_extension(&self) -> bool { self.extension().is_some() }

    /// Returns a new path with the extension removed.
    ///
    /// Equivalent to `self.with_extension("")`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::prelude::*;
    ///
    /// assert_eq!(Path::new("foo/bar.txt").without_extension().as_str(), "foo/bar");
    /// assert_eq!(Path::new("foo/archive.tar.gz").without_extension().as_str(), "foo/archive");
    /// assert_eq!(Path::new("foo/bar").without_extension().as_str(), "foo/bar");
    /// assert_eq!(Path::new("foo/.hidden").without_extension().as_str(), "foo/.hidden");
    /// ```
    pub fn without_extension(&self) -> &Path
    {
        let s = self.as_str();

        // Split off the final path component (filename)
        let end = s.len();
        let start = s.rfind('/').map(|i| i + 1).unwrap_or(0);
        let base = &s[start..end];

        // Skip leading dot (hidden file)
        let search_start = if base.starts_with('.') { start + 1 } else { start };
        let base = &s[search_start..end];

        if let Some(dot_pos) = base.find('.') {
            let cut = search_start + dot_pos;
            return Self::from_str(&s[..cut]);
        }

        self
    }

    /// Returns the full extension of the path, if it exists.
    ///
    /// The returned string slice does **not** include the first dot `.`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::prelude::*;
    ///
    /// assert_eq!(Path::new("foo/bar.txt").extension(), Some("txt"));
    /// assert_eq!(Path::new("foo/archive.tar.gz").extension(), Some("tar.gz"));
    /// assert_eq!(Path::new("foo/bar").extension(), None);
    /// ```
    pub fn extension(&self) -> Option<&extension> {
        let mut fullname: &str = self.fullname();

        if fullname.starts_with('.')
        {
            fullname = &fullname[1..];
        }

        if let Some(dot_pos) = fullname.find('.') {
            return Some(&fullname[dot_pos + 1..]);
        }
        None
    }

    /// Iter over all extensions, left to right
    ///
    /// The returned extension does **not** include the dot `.`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::prelude::*;
    /// use hexga_encoding::prelude::*;
    ///
    /// let empty = Vec::<Extension>::new();
    ///
    /// assert_eq!(Path::new("foo/bar.txt").extensions().collect::<Vec<_>>(), vec!["txt"]);
    /// assert_eq!(Path::new("foo/archive.tar.gz").extensions().collect::<Vec<_>>(), vec!["tar", "gz"]);
    /// assert_eq!(Path::new("foo/bar").extensions().collect::<Vec<_>>(), empty);
    /// assert_eq!(Path::new("foo/.bar").extensions().collect::<Vec<_>>(), empty);
    /// assert_eq!(Path::new("foo/.bar.txt").extensions().collect::<Vec<_>>(), vec!["txt"]);
    /// assert_eq!(Path::new("foo/.bar.txt").extensions().collect::<Vec<_>>(), vec!["txt"]);
    /// assert_eq!(Path::new("foo.buz/bar").extensions().collect::<Vec<_>>(), empty);
    /// ```
    pub fn extensions(&self) -> impl Iterator<Item = &extension> + DoubleEndedIterator + FusedIterator
    {
        let fullname = self.fullname();

        let mut parts = fullname.split('.');

        if fullname.starts_with('.')
        {
            parts.next();
        }
        parts.next();
        parts
    }


    /// Returns a new [`Path`] with its current extension replaced by the given `extension`.
    ///
    /// This method replaces the file extension of the path with the provided one.
    /// If the path has no extension, the new one is simply appended.
    ///
    /// A leading dot (`.`) in the provided extension is ignored automatically.
    ///
    /// To remove the extension entirely, you can pass an empty string (`""`),
    /// but the the method [`Self::without_extension`] is more appropriate because it return a &[`path`].
    ///
    /// This method is a convenience wrapper over [`Self::with_extensions`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexga_io::prelude::*;
    ///
    /// assert_eq!(Path::new("foo/bar.txt").with_extension("md").as_str(), "foo/bar.md");
    ///
    /// // Leading dot is optional
    /// assert_eq!(Path::new("foo/bar.txt").with_extension(".md").as_str(), "foo/bar.md");
    ///
    /// // Remove the extension entirely
    /// assert_eq!(Path::new("foo/bar.txt").with_extension("").as_str(), "foo/bar");
    ///
    /// // Add a new extension if none existed
    /// assert_eq!(Path::new("foo/bar").with_extension("md").as_str(), "foo/bar.md");
    ///
    /// // Handle compound extensions
    /// assert_eq!(Path::new("foo/archive").with_extension("tar.gz").as_str(), "foo/archive.tar.gz");
    /// assert_eq!(Path::new("foo/archive").with_extension(".tar.gz").as_str(), "foo/archive.tar.gz");
    /// ```
    pub fn with_extension(&self, extension: &extension) -> PathBuf
    {
        self.with_extensions([extension.strip_prefix('.').unwrap_or(extension)])
    }

    /// Returns a new [`Path`] with its extensions replaced by the given sequence of `extensions`.
    ///
    /// Each extension is appended to the base filename, separated by a dot (`.`).
    /// This method can be used to build paths with *compound extensions*,
    /// such as `"tar.gz"`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexga_io::prelude::*;
    ///
    /// assert_eq!(
    ///     Path::new("foo/bar.txt").with_extensions(["tar", "gz"]).as_str(),
    ///     "foo/bar.tar.gz"
    /// );
    ///
    /// assert_eq!(
    ///     Path::new("foo/bar").with_extensions(["log"]).as_str(),
    ///     "foo/bar.log"
    /// );
    ///
    /// assert_eq!(
    ///     Path::new("foo/bar.tar.gz").with_extensions([]).as_str(),
    ///     "foo/bar"
    /// );
    /// ```
    pub fn with_extensions<'a, E>(&'a self, extensions: E) -> PathBuf where E: IntoIterator<Item = &'a extension>
    {
        let mut path = self.without_extension().to_owned();
        for extension in extensions.into_iter()
        {
            if extension.is_empty() { continue; }
            path.inner.push('.');
            path.inner.push_str(extension);
        }
        path
    }

    /// Splits the path into its components using `/` as separators
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::prelude::*;
    ///
    /// assert_eq!(Path::new("foo/bar/baz.txt").components().collect::<Vec<_>>(), vec!["foo", "bar", "baz.txt"]);
    /// assert_eq!(Path::new("foo\\bar\\baz.txt").components().collect::<Vec<_>>(), vec!["foo\\bar\\baz.txt"]);
    /// assert_eq!(Path::new("file.txt").components().collect::<Vec<_>>(), vec!["file.txt"]);
    /// assert_eq!(Path::new("").components().collect::<Vec<_>>(), Vec::<&path>::new());
    /// assert_eq!(Path::new("/foo/bar/").components().collect::<Vec<_>>(), vec!["", "foo", "bar", ""]);
    /// ```
    pub fn components(&self) -> impl Iterator<Item = &Path> + DoubleEndedIterator + FusedIterator
    {
        std::iter::once(self.as_str())
            .filter(|s| !s.is_empty()) // skip empty path
            .flat_map(|s| s.split('/').map(|v| Self::from_str(v)))
    }

    /// Iterates over all incremental path prefixes separated by `/`.
    ///
    /// Each yielded item represents a cumulative subpath from the start up to
    /// that point in the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::prelude::*;
    /// use hexga_encoding::prelude::*;
    ///
    /// assert_eq!(
    ///     Path::new("foo/bar/baz.txt").iter_prefixes().collect::<Vec<_>>(),
    ///     vec!["foo", "foo/bar", "foo/bar/baz.txt"]
    /// );
    ///
    /// assert_eq!(
    ///     Path::new("foo/bar/").iter_prefixes().collect::<Vec<_>>(),
    ///     vec!["foo", "foo/bar", "foo/bar/"]
    /// );
    ///
    /// assert_eq!(
    ///     Path::new("file.txt").iter_prefixes().collect::<Vec<_>>(),
    ///     vec!["file.txt"]
    /// );
    ///
    /// assert_eq!(
    ///     Path::new("").iter_prefixes().collect::<Vec<_>>(),
    ///     Vec::<&extension>::new()
    /// );
    /// ```
    ///
    /// This differs from [`Path::iter()`](Self::iter), which yields only individual
    /// path components (e.g., `"foo"`, `"bar"`, `"baz.txt"`), while this method
    /// yields cumulative subpaths (e.g., `"foo"`, `"foo/bar"`, `"foo/bar/baz.txt"`).
    pub fn iter_prefixes(&self) -> impl Iterator<Item = &Path> + DoubleEndedIterator + FusedIterator
    {
        let s = self.as_str();

        // Empty iterator for empty path
        std::iter::once(s)
            .filter(|s| !s.is_empty())
            .flat_map(|s| {
                let slash_indices = s.match_indices('/').map(|(i, _)| i);
                slash_indices
                    .map(move |i| Self::from_str(&s[..i]))
                    .chain(std::iter::once(Self::from_str(s)))
            })
    }

    /// Returns the parent directory of this path using [`Self::parent`], or an empty path if none exists.
    pub fn parent_or_empty(&self) -> &Path
    {
        self.parent().unwrap_or( Path::from_str(""))
    }

    /// Returns the parent directory of this path, if it exists.
    ///
    /// The parent is defined as the portion of the path before the last `/` separator.
    /// If the path has no `/`, or if the parent would be empty, this returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::prelude::*;
    ///
    /// let path = Path::new("foo/bar/baz.txt");
    /// assert_eq!(path.parent().map(|p| p.as_str()), Some("foo/bar"));
    ///
    /// let path = Path::new("foo/bar");
    /// assert_eq!(path.parent().map(|p| p.as_str()), Some("foo"));
    ///
    ///
    /// let path = Path::new("foo/bar/");
    /// assert_eq!(path.parent().map(|p| p.as_str()), Some("foo/bar"));
    ///
    ///
    /// let path = Path::new("foo");
    /// assert_eq!(path.parent(), None);
    ///
    /// let path = Path::new("/");
    /// assert_eq!(path.parent(), None);
    /// ```
    pub fn parent(&self) -> Option<&Path> {

        let s = self.as_str();

        match s.rfind('/') {
            Some(pos) if pos > 0 => {
                Some(Path::from_str(&s[..pos]))
            }
            Some(_) | None => None,
        }
    }

    pub fn push(&self, right: &Self) -> PathBuf
    {
        self.to_owned() / right
    }

    /// Returns the name of the file or directory at the end of the path,
    /// **without the file extensions** (the portion before the most left final `.` in the filename).
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
    /// use hexga_io::prelude::*;
    ///
    /// assert_eq!(Path::new("foo/bar.txt").name(), "bar");
    /// assert_eq!(Path::new("foo/.hidden").name(), ".hidden");
    /// assert_eq!(Path::new("foo/archive.tar.gz").name(), "archive");
    /// assert_eq!(Path::new("foo/bar/").name(), "");
    /// assert_eq!(Path::new("file").name(), "file");
    /// ```
    pub fn name(&self) -> &str
    {
        Self::from_str(self.fullname()).without_extension().as_str()
    }

    /// Returns a new file/folder name with the base name replaced by `name`,
    /// preserving the original extension(s).
    ///
    /// The parent directory is not modified. Only the file/folder name is changed.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::prelude::*;
    ///
    /// assert_eq!(Path::new("foo/bar.txt").with_name("baz").as_str(), "foo/baz.txt");
    /// assert_eq!(Path::new("foo/.hidden").with_name("config").as_str(), "foo/config");
    /// assert_eq!(Path::new("foo/.hidden.txt").with_name("config").as_str(), "foo/config.txt");
    /// assert_eq!(Path::new("foo/archive.tar.gz").with_name("new").as_str(), "foo/new.tar.gz");
    /// assert_eq!(Path::new("file").with_name("newfile").as_str(), "newfile");
    /// ```
    pub fn with_name(&self, name: &str) -> PathBuf
    {
        let s = self.as_str();

        // Split parent and filename
        let end = s.len();
        let start = s.rfind('/').map(|i| i + 1).unwrap_or(0);
        let parent = &s[..start];      // includes trailing '/' if present
        let fullname: &str = &s[start..end]; // filename (may be empty)

        let rest_fullname = if fullname.starts_with('.') { &fullname[1..] } else { fullname };

        let ext = match rest_fullname.find('.') {
            Some(dot_pos) => &fullname[fullname.len() - rest_fullname.len() + dot_pos..],
            None => "",
        };

        let mut result = String::with_capacity(parent.len() + name.len() + ext.len());
        result.push_str(parent);
        result.push_str(name);
        result.push_str(ext);
        PathBuf::new(result)
    }

    /// Returns the full file name (including all extensions) of the path.
    ///
    /// If the path ends with a directory or has no file name, returns an empty string `""`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexga_io::prelude::*;
    ///
    /// assert_eq!(Path::new("foo/bar.txt").fullname(), "bar.txt");
    /// assert_eq!(Path::new("foo/.hidden").fullname(), ".hidden");
    /// assert_eq!(Path::new("foo/archive.tar.gz").fullname(), "archive.tar.gz");
    /// assert_eq!(Path::new("foo/bar/").fullname(), "");
    /// assert_eq!(Path::new("file").fullname(), "file");
    /// ```
    pub fn fullname(&self) -> &str
    {
        self.as_str().rsplit('/').next().unwrap_or(&self.path)
    }
}
impl<'a> From<&'a str> for &'a Path
{
    fn from(value: &'a str) -> Self
    {
        Path::from_str(value)
    }
}
impl<'a> From<&'a mut str> for &'a mut Path
{
    fn from(value: &'a mut str) -> Self
    {
        Path::from_str_mut(value)
    }
}
impl<'a> From<&'a Path> for &'a str
{
    fn from(value: &'a Path) -> Self
    {
        &value.path
    }
}
impl<'a> From<&'a mut Path> for &'a mut str
{
    fn from(value: &'a mut Path) -> Self
    {
        &mut value.path
    }
}


impl AsRef<str> for Path
{
    fn as_ref(&self) -> &str {
        &self.path
    }
}
impl AsRef<Path> for Path
{
    fn as_ref(&self) -> &Path {
        self
    }
}
impl AsRef<Path> for str
{
    fn as_ref(&self) -> &Path {
        self.path()
    }
}
impl AsRef<Path> for String
{
    fn as_ref(&self) -> &Path {
        self.path()
    }
}
impl AsMut<str> for Path
{
    fn as_mut(&mut self) -> &mut str {
        &mut self.path
    }
}
impl ToOwned for Path
{
    type Owned=PathBuf;

    fn to_owned(&self) -> Self::Owned {
        PathBuf::new(self.path.to_owned())
    }
}
impl AsRef<OsStr> for Path
{
    fn as_ref(&self) -> &OsStr {
        self.path.as_ref()
    }
}



impl Borrow<Path> for PathBuf
{
    fn borrow(&self) -> &Path {
        self.deref()
    }
}
impl BorrowMut<Path> for PathBuf
{
    fn borrow_mut(&mut self) -> &mut Path {
        self.deref_mut()
    }
}
impl AsRef<Path> for PathBuf
{
    fn as_ref(&self) -> &Path {
        self.deref()
    }
}
impl AsMut<Path> for PathBuf
{
    fn as_mut(&mut self) -> &mut Path {
        self.deref_mut()
    }
}
impl AsRef<str> for PathBuf
{
    fn as_ref(&self) -> &str {
        &self.inner
    }
}
impl AsMut<str> for PathBuf
{
    fn as_mut(&mut self) -> &mut str {
        &mut self.inner
    }
}
impl AsRef<OsStr> for PathBuf
{
    fn as_ref(&self) -> &OsStr {
        self.inner.as_ref()
    }
}


pub trait ToPathSlice
{
    type Output;
    fn path(self) -> Self::Output;
}
impl<'a> ToPathSlice for &'a str
{
    type Output=&'a Path;
    fn path(self) -> Self::Output {
        self.into()
    }
}
impl<'a> ToPathSlice for &'a mut str
{
    type Output=&'a mut Path;
    fn path(self) -> Self::Output {
        self.into()
    }
}
