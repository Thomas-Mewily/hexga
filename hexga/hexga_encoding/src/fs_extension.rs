use super::*;

pub type Extension = String;
#[allow(non_camel_case_types)]
pub type extension = str;

pub type CowExtensionStatic = CowExtension<'static>;
pub type CowExtension<'a> = Cow<'a, str>;
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
    pub use super::{CommonExtensions, Extension, extension};
}
