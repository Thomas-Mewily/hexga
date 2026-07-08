use super::*;
use std::collections::HashSet;

pub type Extension = String;
#[allow(non_camel_case_types)]
pub type extension = str;

pub type Extensions<T = CowExtensionStatic> = HashSet<T>;

pub type CowExtensionStatic = CowExtension<'static>;
pub type CowExtension<'a> = Cow<'a, extension>;
pub type DeducedExtension<'a> = CowExtension<'a>;

pub trait CommonExtensions
{
    const TXT: &'static str = "txt";

    const RON: &'static str = "ron";
    const JSON: &'static str = "json";
    const XML: &'static str = "xml";

    // Todo: add a cfg flag for the prefered extension
    const PREFERED: &'static str = Self::RON;

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

pub trait FsResolvePathWithExtension: FsRead
{
    /// Resolve incomplete file extension by finding the matching file on disk.
    /// If no file exist with the same name, return the path with the prefered extension.
    fn resolve_path_for<T, P: AsRef<Path>>(&mut self, path: P) -> PathBuf
    where
        T: PreferedExtension + ?Sized,
    {
        let path = path.as_ref();
        let mut path = self
            .resolve_path(&path)
            .unwrap_or_else(|_| self.canonicalize(path).unwrap_or_else(|_| path.to_path_buf()));
        if path.extension().is_none()
        {
            path.set_extension(T::prefered_extension());
        }
        path
    }
}
impl<T> FsResolvePathWithExtension for T where T: FsRead {}

pub trait PreferedExtension: Save
{
    fn prefered_extension() -> &'static extension { Self::save_prefered_extension() }
}
impl<T> PreferedExtension for T where T: Save + ?Sized {}
