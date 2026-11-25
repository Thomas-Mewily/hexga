use super::*;
use std::collections::HashSet;

pub type Extension = String;
#[allow(non_camel_case_types)]
pub type extension = str;

pub type Extensions<T = CowExtensionStatic> = HashSet<T>;

pub type CowExtensionStatic = CowExtension<'static>;
pub type CowExtension<'a> = Cow<'a, extension>;
pub type DeducedExtension<'a> = CowExtension<'a>;

/// Supported markup formats.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatMarkup
{
    Ron,
    Json,
    Xml,
}

impl FormatMarkup
{
    // Todo: add a feature flag
    pub const PREFERED: Self = Self::Ron;
    pub const ALL: &'static [Self] = &[Self::Ron, Self::Json, Self::Xml];

    pub const fn extension(self) -> &'static str
    {
        match self
        {
            Self::Ron => Extension::RON,
            Self::Json => Extension::JSON,
            Self::Xml => Extension::XML,
        }
    }
}

impl Default for FormatMarkup
{
    fn default() -> Self { Self::PREFERED }
}

pub trait CommonExtensions
{
    const TXT: &'static str = "txt";

    const RON: &'static str = "ron";
    const JSON: &'static str = "json";
    const XML: &'static str = "xml";

    const PREFERED: &'static str = FormatMarkup::PREFERED.extension();

    /// Intended for short-term storage of data in a binary format, such as during data transfer.
    ///
    /// Not suitable for long-term storage, as the implementation or encoding may change at any time.
    const TMP_BIN: &'static str = "tmp";
}
impl CommonExtensions for Extension {}

pub(crate) mod prelude
{
    pub use super::traits::*;
    pub use super::{CommonExtensions, Extension, extension};
}

pub mod traits
{
    pub use super::{FsResolvePathWithExtension, PreferedExtension};
}

pub trait FsResolvePathWithExtension: FileSystemRead
{
    /// Resolve incomplete file extension by finding the matching file on disk.
    /// If no file exist with the same name, return the path with the prefered extension.
    fn resolve_path_for<T, P: AsRef<Path>>(&mut self, path: P) -> PathBuf
    where
        T: PreferedExtension + ?Sized,
    {
        let path = path.as_ref();
        let mut path = self
            .resolve_path(path)
            .unwrap_or_else(|_| self.canonicalize(path).unwrap_or_else(|_| path.to_path_buf()));
        if path.extension().is_none()
        {
            path.set_extension(T::prefered_extension());
        }
        path
    }
}
impl<T> FsResolvePathWithExtension for T where T: FileSystemRead {}

pub trait PreferedExtension: Save
{
    fn prefered_extension() -> &'static extension { Self::save_prefered_extension() }
}
impl<T> PreferedExtension for T where T: Save + ?Sized {}
